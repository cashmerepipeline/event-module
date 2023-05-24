use dependencies_sync::chrono::Utc;
use dependencies_sync::log::debug;

use dependencies_sync::tonic::{async_trait, Request, Response, Status};
use dependencies_sync::bson;
use dependencies_sync::tokio;
use dependencies_sync::tokio_stream;

use majordomo::{self, get_majordomo};

use manage_define::general_field_ids::*;

use managers::traits::ManagerTrait;

use request_utils::request_account_context;
use view;


use service_utils::types::{ResponseStream, StreamResponseResult};


use crate::event_inner_wrapper::EventInnerWrapper;
use crate::event_types_map::get_event_type;
use crate::field_ids::*;
use crate::manage_ids::*;
use crate::protocols::*;
use crate::type_dispatcher_map::get_dispatcher;

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

        // 事件类型存在检查
        if get_event_type(type_id).await.is_none() {
            return Err(Status::not_found(t!("事件类型不存在")));
        }

        let majordomo_arc = get_majordomo().await;
        let listener_manager = majordomo_arc
            .get_manager_by_id(EVENT_LISTENERS_MANAGE_ID)
            .await
            .unwrap();

        // 存在检查
        if !listener_manager
            .entity_exists(&bson::doc! {ID_FIELD_ID.to_string():listener_id.clone()})
            .await
        {
            return Err(Status::not_found(t!("监听器不存在")));
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
        let dispatcher_arc = match get_dispatcher(type_id) {
            Some(r) => r,
            None => return Err(Status::aborted(format!("{}", t!("获取转发器失败 ")))),
        };

        // 创建监听事件管道
        let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<EventInnerWrapper>(4);
        dispatcher_arc.add_listener_sender(listener_id, event_tx);

        // 创建返回流
        let (resp_tx, resp_rx) = tokio::sync::mpsc::channel(4);

        // 转发事件
        let listener_id = listener_id.clone();
        tokio::spawn(async move {
            while let Some(event_wraper) = event_rx.recv().await {
                debug!("{}: {}", t!("监听到事件"), event_wraper.event.serial_number);

                let resp = ListenEventTypeResponse {
                    event: Some(event_wraper.event.clone()),
                };

                // TODO: 重试发送
                if let Err(e) = resp_tx.send(Ok(resp)).await {
                    debug!("{}: {}", t!("发送事件失败"), e);
                    // 反馈发送失败
                    if let Some(echo_sender) = event_wraper.echo_sender {
                        let echo_name = format!(
                            "echo-{}-{}-{}",
                            listener_id,
                            event_wraper.event.type_id,
                            event_wraper.event.emitter_instance_name
                        );
                        let echo_event = Event {
                            type_id: event_wraper.event.type_id,
                            emitter_id: (event_wraper.event.emitter_id.parse::<u32>().unwrap() + 1)
                                .to_string(),
                            emitter_instance_name: echo_name,
                            timestamp: Utc::now().timestamp_millis() as u64,
                            serial_number: event_wraper.event.serial_number + 1,
                            context: "send failed".as_bytes().to_vec(),
                            need_echo: false,
                        };
                        echo_sender.send(echo_event).await.unwrap();
                    };
                    break;
                };

                if let Some(echo_sender) = event_wraper.echo_sender {
                    let echo_name = format!(
                        "echo-{}-{}-{}",
                        listener_id,
                        event_wraper.event.type_id,
                        event_wraper.event.emitter_instance_name
                    );
                    let echo_event = Event {
                        type_id: event_wraper.event.type_id,
                        emitter_id: (event_wraper.event.emitter_id.parse::<u32>().unwrap() + 1)
                            .to_string(),
                        emitter_instance_name: echo_name,
                        timestamp: Utc::now().timestamp_millis() as u64,
                        serial_number: event_wraper.event.serial_number + 1,
                        context: "send success".as_bytes().to_vec(),
                        need_echo: false,
                    };
                    echo_sender.send(echo_event).await.unwrap();
                };
            }
        });

        let resp_stream = tokio_stream::wrappers::ReceiverStream::new(resp_rx);
        Ok(Response::new(
            Box::pin(resp_stream) as ResponseStream<ListenEventTypeResponse>
        ))
    }
}
