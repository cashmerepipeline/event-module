use chrono::Utc;
use log::debug;
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
use crate::field_ids::*;
use crate::manage_ids::*;
use crate::protocols::*;

#[async_trait]
pub trait HandleListenEventType {
    async fn handle_listen_event_type(
        &self,
        request: Request<ListenEventTypeRequest>,
    ) -> StreamResponseResult<ListenEventTypeResponse> {
        let (account_id, _groups, role_group) = request_account_context(request.metadata());

        let listener_id = &request.get_ref().listener_id;
        let type_id = &request.get_ref().type_id;

        if !view::can_collection_read(
            &account_id,
            &role_group,
            &EVENT_LISTENERS_MANAGE_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let listener_manager = majordomo_arc
            .get_manager_by_id(EVENT_LISTENERS_MANAGE_ID)
            .await
            .unwrap();
        let event_type_manager = majordomo_arc
            .get_manager_by_id(EVENT_TYPES_MANAGE_ID)
            .await
            .unwrap();

        // 存在检查
        if !listener_manager
            .entity_exists(&bson::doc! {ID_FIELD_ID.to_string():listener_id.clone()})
            .await
        {
            return Err(Status::not_found(t!("监听器不存在")));
        }
        if !event_type_manager
            .entity_exists(&bson::doc! {ID_FIELD_ID.to_string():type_id.clone()})
            .await
        {
            return Err(Status::not_found(t!("事件类型不存在")));
        }

        // 检查是否可监听
        let listener_entity = match listener_manager.get_entity_by_id(listener_id).await {
            Ok(r) => r,
            Err(e) => {
                return Err(Status::aborted(format!(
                    "{}: {}, {}",
                    t!("获取监听器失败 "),
                    e.operation(),
                    e.details()
                )))
            }
        };
        // 事件类型需要匹配
        if let Ok(id) = listener_entity.get_str(EVENT_LISTENERS_TYPE_ID_FIELD_ID.to_string()) {
            if &id.to_string() != type_id {
                return Err(Status::aborted(format!(
                    "{}: {}: {}, {}: {}",
                    t!("不可监听该事件类型"),
                    t!("监听者"),
                    listener_id,
                    t!("事件类型"),
                    type_id
                )));
            }
        }

        // 取得转发器
        let mut dispatcher_arc = match get_dispatcher(type_id) {
            Some(r) => r,
            None => return Err(Status::aborted(format!("{}", t!("获取转发器失败 ")))),
        };
        let mut dispatcher = dispatcher_arc.write();

        // 创建监听事件管道
        let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<EventEchoWrapper>(4);
        dispatcher.add_listener_sender(&listener_id, event_tx);

        // 创建返回流
        let (resp_tx, resp_rx) = tokio::sync::mpsc::channel(4);

        // 转发事件
        tokio::spawn(async move {
            while let event_echo_wraper = event_rx.recv().await {
                if event_echo_wraper.is_none() {
                    break;
                }

                let event_echo_wraper = event_echo_wraper.unwrap();
                debug!("{}: {}", t!("监听到事件"), event_echo_wraper.event.serial_number);

                let mut resp = ListenEventTypeResponse {
                    event: Some(event_echo_wraper.event.clone()),
                };
                resp_tx.send(Ok(resp)).await.unwrap();

                if let Some(echo_sender) = event_echo_wraper.echo_sender {
                    let echo_event = Event {
                        type_id: event_echo_wraper.event.type_id,
                        emitter_id: (event_echo_wraper.event.emitter_id.parse::<u32>().unwrap()
                            + 1)
                        .to_string(),
                        timestamp: Utc::now().timestamp_millis() as u64,
                        serial_number: event_echo_wraper.event.serial_number + 1,
                        context: vec![],
                    };
                    echo_sender.send(echo_event).await.unwrap();
                }
            }
        });

        let resp_stream = tokio_stream::wrappers::ReceiverStream::new(resp_rx);
        Ok(Response::new(
            Box::pin(resp_stream) as ResponseStream<ListenEventTypeResponse>
        ))
    }
}
