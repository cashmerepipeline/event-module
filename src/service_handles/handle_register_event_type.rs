use dependencies_sync::bson;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::{async_trait, Request, Response, Status};

use majordomo::{self, get_majordomo};

use manage_define::general_field_ids::*;

use managers::utils::make_new_entity_document;
use managers::entity_interface::EntityInterface;
use request_utils::request_account_context;

use service_utils::types::UnaryResponseResult;
use validates::validate_name;

use crate::event_services::{has_max_evnet_type_count_reached, register_event_type};
use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::protocols::*;

#[async_trait]
pub trait HandleRegisterEventType {
    async fn handle_register_event_type(
        &self,
        request: Request<RegisterEventTypeRequest>,
    ) -> UnaryResponseResult<RegisterEventTypeResponse> {
        validate_view_rulse(request)
            .and_then(validate_request_params)
            .and_then(handle_register_event_type)
            .await
    }
}

async fn validate_view_rulse(
    request: Request<RegisterEventTypeRequest>,
) -> Result<Request<RegisterEventTypeRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) =
            view::validates::validate_collection_can_write(&EVENT_TYPES_MANAGE_ID, &role_group)
                .await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<RegisterEventTypeRequest>,
) -> Result<Request<RegisterEventTypeRequest>, Status> {
    let name = &request.get_ref().name;
    validate_name(name)?;

    Ok(request)
}

async fn handle_register_event_type(
    request: Request<RegisterEventTypeRequest>,
) -> UnaryResponseResult<RegisterEventTypeResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let name = &request.get_ref().name;
    let has_echo = &request.get_ref().has_echo;
    let description = &request.get_ref().description;

    let name = name.as_ref().unwrap();

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(EVENT_TYPES_MANAGE_ID)
        .unwrap();

    // 检查类型注册数量是否已经最大
    if has_max_evnet_type_count_reached() {
        return Err(Status::aborted(t!("监听事件类型超出最大限制").to_string()));
    }

    // 新建条目
    let mut new_entity_doc = if let Ok(r) = make_new_entity_document(&manager, &account_id).await
    {
        r
    } else {
        return Err(Status::aborted(format!(
            "{}: 管理 {}",
            t!("新建实体文档失败"),
            EVENT_TYPES_MANAGE_ID
        )));
    };

    new_entity_doc.insert(
        NAME_MAP_FIELD_ID.to_string(),
        bson::to_document(name).unwrap(),
    );
    new_entity_doc.insert(EVENT_TYPES_HAS_ECHO_FIELD_ID.to_string(), *has_echo);
    new_entity_doc.insert(DESCRIPTION_FIELD_ID.to_string(), bson::to_document(description).unwrap());
    let new_id = new_entity_doc
        .get_str(ID_FIELD_ID.to_string())
        .unwrap()
        .to_owned()
        .parse::<u32>()
        .unwrap();

    if new_id % 2 == 0 {
        new_entity_doc.insert(ID_FIELD_ID.to_string(), (new_id + 1).to_string());
    }

    let result = manager
        .sink_entity(&mut new_entity_doc, &account_id, &role_group)
        .await;

    match result {
        Ok(r) => {
            // 更新缓存
            let new_event_type = EventType {
                type_id: r.clone(),
                name: Some(name.clone()),
                description: description.clone(),
            };
            if register_event_type(new_event_type).is_none() {
                // 更新缓存失败
                return Err(Status::aborted(format!(
                    "{}: {}, {}",
                    t!("更新缓存失败"),
                    t!("事件类型"),
                    t!("可能需要重新启动事件服务")
                )));
            };

            Ok(Response::new(RegisterEventTypeResponse {
                result: r,
            }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
