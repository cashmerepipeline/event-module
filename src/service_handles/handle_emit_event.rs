use tonic::{async_trait, Request, Response, Status};

use majordomo::{self, get_majordomo};

use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;
use view;

use service_common_handles::name_utils::validate_name;
use service_common_handles::{ResponseStream, StreamResponseResult};

use crate::dispatcher;
use crate::dispatchers_map::get_dispatcher;
use crate::event_echo_wrapper::EventEchoWrapper;
use crate::event_protocol::*;
use crate::field_ids::*;
use crate::manage_ids::*;

#[async_trait]
pub trait HandleEmitEvent {
    async fn handle_emit_event(
        &self,
        request: Request<EmitEventRequest>,
    ) -> StreamResponseResult<EmitEventResponse> {
        let (account_id, _groups, role_group) = request_account_context(request.metadata());

        let event = &request.get_ref().event;

        // 有效性检查
        if event.is_none() {
            return Err(Status::invalid_argument(t!("事件不能为空")));
        }
        let event = event.as_ref().unwrap();

        if !view::can_collection_read(
            &account_id,
            &role_group,
            &EVENT_EMITTERS_MANAGE_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated(t!("用户不具有可读权限")));
        }

        let majordomo_arc = get_majordomo().await;
        let emitter_manager = majordomo_arc
            .get_manager_by_id(EVENT_EMITTERS_MANAGE_ID)
            .await
            .unwrap();
        let event_type_manager = majordomo_arc
            .get_manager_by_id(EVENT_TYPES_MANAGE_ID)
            .await
            .unwrap();

        // 存在检查
        if !emitter_manager
            .entity_exists(&bson::doc! {ID_FIELD_ID.to_string():event.emitter_id.clone()})
            .await
        {
            return Err(Status::not_found(t!("发射者不存在")));
        }
        if !event_type_manager
            .entity_exists(&bson::doc! {ID_FIELD_ID.to_string():event.type_id.clone()})
            .await
        {
            return Err(Status::not_found(t!("事件类型不存在")));
        }

        // 检查是否可发送
        let emitter_entity = match emitter_manager.get_entity_by_id(&event.emitter_id).await {
            Ok(r) => r,
            Err(e) => {
                return Err(Status::aborted(format!(
                    "{}: {}, {}",
                    t!("获取发送者失败 "),
                    e.operation(),
                    e.details()
                )))
            }
        };

        if !emitter_entity
            .get_array(EVENT_EMITTERS_EMITTABLE_EVENT_TYPES_FIELD_ID.to_string())
            .unwrap()
            .contains(&bson::to_bson(&event.type_id.clone()).unwrap())
        {
            return Err(Status::aborted(format!(
                "{}: {}, {}",
                t!("发送者不可发送该事件类型"),
                event.emitter_id,
                event.type_id
            )));
        };

        // 取得转发器
        let mut dispatcher_arc = match get_dispatcher(&event.type_id) {
            Some(r) => r,
            None => return Err(Status::aborted(format!("{}", t!("获取转发器失败 ")))),
        };

        let event_sender = {
        let dispatcher = dispatcher_arc.read();
            dispatcher.dispatch_sender.clone()
        };

        // 创建回馈事件管道
        let (echo_tx, mut echo_rx) = tokio::sync::mpsc::channel::<Event>(4);

        // 创建返回流
        let (resp_tx, resp_rx) = tokio::sync::mpsc::channel(4);

        // 发送
        let event_echo_wrapper = EventEchoWrapper {
            event: event.clone(),
            echo_sender: Some(echo_tx),
        };
        if let Err(e) = event_sender.send(event_echo_wrapper).await{
            return Err(Status::aborted(format!(
                "{}: {}",
                t!("发送事件失败 "),
                e
            )))
        };

        // 转发事件
        tokio::spawn(async move {
            while let Some(event) = echo_rx.recv().await {
                let mut resp = EmitEventResponse { event: Some(event) };
                resp_tx.send(Ok(resp)).await.unwrap();
            }
        });

        let resp_stream = tokio_stream::wrappers::ReceiverStream::new(resp_rx);
        Ok(Response::new(
            Box::pin(resp_stream) as ResponseStream<EmitEventResponse>
        ))
    }
}
