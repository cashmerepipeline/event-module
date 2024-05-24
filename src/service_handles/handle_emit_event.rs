use dependencies_sync::bson::{self};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::debug;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio;
use dependencies_sync::tokio_stream;

use dependencies_sync::tonic::{async_trait, Request, Response, Status};

use majordomo::{self, get_majordomo};

use manage_define::general_field_ids::*;

use managers::entity_interface::EntityInterface;

use request_utils::request_account_context;

use service_utils::types::{ResponseStream, StreamResponseResult};

use crate::event_inner_wrapper::EventInnerWrapper;
use crate::event_types_map::get_event_serial_number;
use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::protocols::*;

use super::dispatch_event::dispatch_event;

#[async_trait]
pub trait HandleEmitEvent {
    async fn handle_emit_event(
        &self,
        request: Request<EmitEventRequest>,
    ) -> StreamResponseResult<EmitEventResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_emit_event)
            .await
    }
}

async fn validate_view_rules(
    request: Request<EmitEventRequest>,
) -> Result<Request<EmitEventRequest>, Status> {
    debug!("{}: {}", t!("验证视图规则"), EVENT_TYPES_MANAGE_ID);
    Ok(request)
}

async fn validate_request_params(
    request: Request<EmitEventRequest>,
) -> Result<Request<EmitEventRequest>, Status> {
    debug!("{}: {}", t!("验证参数"), EVENT_TYPES_MANAGE_ID);
    Ok(request)
}

async fn handle_emit_event(
    request: Request<EmitEventRequest>,
) -> StreamResponseResult<EmitEventResponse> {
    debug!("{}: {}", t!("处理请求"), EVENT_TYPES_MANAGE_ID);

    let (_account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let event = &request.get_ref().event;

    // 有效性检查
    if event.is_none() {
        return Err(Status::invalid_argument(t!("事件不能为空")));
    }

    let mut event = event.as_ref().unwrap().to_owned();
    if let Some(serial_number) = get_event_serial_number(&event.type_id).await {
        debug!(
            "{}: {} {} {}",
            t!("取得事件序号"),
            event.type_id,
            event.emitter_id,
            serial_number
        );
        event.serial_number = serial_number;
    } else {
        return Err(Status::invalid_argument(t!(
            "事件类型不支持，或者取得事件序号失败"
        )));
    }

    let majordomo_arc = get_majordomo();
    let emitter_manager = majordomo_arc
        .get_manager_by_id(EVENT_EMITTERS_MANAGE_ID)
        .unwrap();
    let event_type_manager = majordomo_arc
        .get_manager_by_id(EVENT_TYPES_MANAGE_ID)
        .unwrap();

    // 存在检查
    if emitter_manager
        .entity_exists(&bson::doc! {ID_FIELD_ID.to_string():event.emitter_id.clone()})
        .await
        .is_none()
    {
        return Err(Status::not_found(t!("发射者不存在")));
    }
    if event_type_manager
        .entity_exists(&bson::doc! {ID_FIELD_ID.to_string():event.type_id.clone()})
        .await
        .is_none()
    {
        return Err(Status::not_found(t!("事件类型不存在")));
    }

    // 检查是否可发送
    let emitter_entity = match emitter_manager.get_entity_by_id(&event.emitter_id, &vec![], &vec![]).await {
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

    // 事件类型需要匹配
    if let Ok(id) = emitter_entity.get_str(EVENT_EMITTERS_TYPE_ID_FIELD_ID.to_string()) {
        if *id != event.type_id {
            return Err(Status::aborted(format!(
                "{}: {}, {}",
                t!("发送者不可发送该事件类型"),
                event.emitter_id,
                event.type_id
            )));
        }
    }

    // 创建返回流
    let (resp_tx, resp_rx) = tokio::sync::mpsc::channel(16);

    if event.need_echo {
        // 创建回馈事件管道
        let (echo_tx, mut echo_rx) = tokio::sync::mpsc::channel::<Event>(16);

        let event_wrapper = EventInnerWrapper {
            event: event.clone(),
            echo_sender: Some(echo_tx),
        };

        if let Err(e) = dispatch_event(event_wrapper).await {
            return Err(Status::aborted(format!("{}: {}", t!("发送事件失败 "), e)));
        };

        // 转发事件
        tokio::spawn(async move {
            while let Some(event) = echo_rx.recv().await {
                debug!("{}, {}", t!("接收到事件反馈"), event.serial_number);
                let resp = EmitEventResponse { event: Some(event) };
                if let Err(e) = resp_tx.send(Ok(resp)).await {
                    debug!("{}: {}", t!("发送事件反馈失败"), e);
                    // 反馈失败，尝试下一个反馈
                    continue;
                };
            }
        });
    } else {
        let event_wrapper = EventInnerWrapper {
            event: event.clone(),
            echo_sender: None,
        };

        if let Err(e) = dispatch_event(event_wrapper).await {
            return Err(Status::aborted(format!("{}: {}", t!("发送事件失败 "), e)));
        };

        // 反馈事件已经发送
        let resp = EmitEventResponse { event: None };
        let _ = resp_tx.send(Ok(resp)).await;
    }

    let resp_stream = tokio_stream::wrappers::ReceiverStream::new(resp_rx);
    Ok(Response::new(
        Box::pin(resp_stream) as ResponseStream<EmitEventResponse>
    ))
    // validate_view_rules(request)
    //     .and_then(validate_request_params)
    //     .and_then(handle_emit_event)
    //     .await
}
