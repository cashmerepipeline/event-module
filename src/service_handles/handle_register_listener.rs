use tonic::{async_trait, Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;
use view;

use service_common_handles::name_utils::validate_name;
use service_common_handles::UnaryResponseResult;

use crate::event_protocol::*;
use crate::field_ids::*;
use crate::manage_ids::*;

#[async_trait]
pub trait HandleRegisterEventListener {
    async fn handle_register_event_listener(
        &self,
        request: Request<RegisterListenerRequest>,
    ) -> UnaryResponseResult<RegisterListenerResponse> {
        let (account_id, _groups, role_group) = request_account_context(request.metadata());

        let event_type = &request.get_ref().envent_type;
        let name = &request.get_ref().name;
        let description = &request.get_ref().description;

        if validate_name(name).is_err() {
            return Err(Status::data_loss(format!("{}{}-{}", t!("监听者"), t!("名字不能为空").to_string(), event_type)));
        }
        let name = name.as_ref().unwrap();

        if !view::can_manage_write(
            &account_id,
            &role_group,
            &EVENT_LISTENERS_MANAGE_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(EVENT_LISTENERS_MANAGE_ID)
            .await
            .unwrap();

        // 新建条目
        let mut new_entity_doc = if let Some(r) = make_new_entity_document(&manager).await {
            r
        } else {
            return Err(Status::aborted(format!(
                "{}: 管理 {}",
                t!("新建实体文档失败"),
                EVENT_LISTENERS_MANAGE_ID
            )));
        };
        new_entity_doc.insert(
            NAME_MAP_FIELD_ID.to_string(),
            bson::to_document(name).unwrap(),
        );

        new_entity_doc.insert(EVENT_LISTENERS_TYPE_ID_FIELD_ID.to_string(), event_type.clone());
        new_entity_doc.insert(DESCRIPTIONS_FIELD_ID.to_string(), description.clone());

        let new_id = new_entity_doc
            .get_str(ID_FIELD_ID.to_string())
            .unwrap()
            .to_owned();

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(RegisterListenerResponse {
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
