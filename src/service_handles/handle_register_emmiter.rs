use dependencies_sync::tonic::{async_trait, Request, Response, Status};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::bson;

use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::*;
use managers::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;


use service_utils::validate_name;
use service_utils::types::UnaryResponseResult;

use crate::protocols::*;
use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;

#[async_trait]
pub trait HandleRegisterEventEmitter {
    async fn handle_register_event_emitter(
        &self,
        request: Request<RegisterEventEmitterRequest>,
    ) -> UnaryResponseResult<RegisterEventEmitterResponse> {
        let (account_id, _groups, role_group) = request_account_context(request.metadata());

        let event_type = &request.get_ref().event_type;
        let name = &request.get_ref().name;
        let description = &request.get_ref().description;

        if !validate_name(name) {
            return Err(Status::data_loss(t!("名字不能为空")));
        }
        let name = name.as_ref().unwrap();

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc
            .get_manager_by_id(EVENT_EMITTERS_MANAGE_ID)
            .unwrap();

        // 新建条目
        let mut new_entity_doc = if let Some(r) = make_new_entity_document(&manager, &account_id).await {
            r
        } else {
            return Err(Status::aborted(format!(
                "{}: 管理 {}",
                t!("新建实体文档失败"),
                EVENT_EMITTERS_MANAGE_ID
            )));
        };
        new_entity_doc.insert(
            NAME_MAP_FIELD_ID.to_string(),
            bson::to_document(name).unwrap(),
        );

        new_entity_doc.insert(EVENT_EMITTERS_TYPE_ID_FIELD_ID.to_string(), event_type.clone());
        new_entity_doc.insert(DESCRIPTION_FIELD_ID.to_string(), description.clone());

        let new_id = new_entity_doc
            .get_str(ID_FIELD_ID.to_string())
            .unwrap()
            .to_owned();

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(RegisterEventEmitterResponse {
                result: new_id,
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
