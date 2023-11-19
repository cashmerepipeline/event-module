use dependencies_sync::bson::{self};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::debug;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio;

use dependencies_sync::tonic::{async_trait, Request, Response, Status};

use majordomo::{self, get_majordomo};

use manage_define::general_field_ids::*;

use managers::ManagerTrait;

use request_utils::request_account_context;
use view;

use service_utils::types::UnaryResponseResult;

use crate::event_inner_wrapper::EventInnerWrapper;
use crate::event_types_map::get_event_serial_number;
use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::protocols::*;

use super::dispatch_to_listener_instance::dispatch_to_listener_instance;

#[async_trait]
pub trait HandleEmitEventToInstance {
    async fn handle_emit_event_to_instance(
        &self,
        request: Request<EmitEventToInstanceRequest>,
    ) -> UnaryResponseResult<EmitEventResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_emit_event_to_instance)
            .await
    }
}

async fn validate_view_rules(
    request: Request<EmitEventToInstanceRequest>,
) -> Result<Request<EmitEventToInstanceRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = EVENT_EMITTERS_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
        view::validates::validate_collection_can_read(&manage_id, &role_group).await?;
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<EmitEventToInstanceRequest>,
) -> Result<Request<EmitEventToInstanceRequest>, Status> {
    let event = &request.get_ref().event;
    let _listener_id = &request.get_ref().listener_id;
    let _instance_index = &request.get_ref().instance_index;

    // 有效性检查
    if event.is_none() {
        return Err(Status::invalid_argument(t!("事件不能为空")));
    }

    Ok(request)
}

async fn handle_emit_event_to_instance(
    request: Request<EmitEventToInstanceRequest>,
) -> UnaryResponseResult<EmitEventResponse> {
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let event = &request.get_ref().event;
    let listener_id = &request.get_ref().listener_id;
    let instance_index = &request.get_ref().instance_index;

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
    if event.need_echo {
        // 创建回馈事件管道，不使用oneshot，需要匹配管道类型
        let (echo_tx, mut echo_rx) = tokio::sync::mpsc::channel::<Event>(16);

        let event_wrapper = EventInnerWrapper {
            event: event.clone(),
            echo_sender: Some(echo_tx),
        };

        // 转发事件
        if let Err(e) =
            dispatch_to_listener_instance(event_wrapper, listener_id, *instance_index).await
        {
            return Err(Status::aborted(format!(
                "{}: {}",
                t!("发送事件失败 "),
                e.details()
            )));
        };

        // 等待反馈
        let result = tokio::time::timeout(std::time::Duration::from_secs(10), async {
            if let Some(event) = echo_rx.recv().await {
                debug!("{}, {}", t!("接收到事件反馈"), event.serial_number);

                let resp = EmitEventResponse { event: Some(event) };
                return Ok(Response::new(resp));
            }

            Err(Status::aborted(t!("等待事件反馈超时")))
        })
        .await;

        match result {
            Ok(r) => r,
            Err(e) => Err(Status::aborted(format!(
                "{}: {}",
                t!("等待事件反馈失败"),
                e
            ))),
        }
    } else {
        let event_wrapper = EventInnerWrapper {
            event: event.clone(),
            echo_sender: None,
        };

        if let Err(e) =
            dispatch_to_listener_instance(event_wrapper, listener_id, *instance_index).await
        {
            return Err(Status::aborted(format!(
                "{}: {}",
                t!("发送事件失败 "),
                e.details()
            )));
        };

        // 反馈事件已经发送
        let resp = EmitEventResponse { event: None };

        Ok(Response::new(resp))
    }
}
