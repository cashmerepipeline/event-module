use cash_result::{operation_failed, OperationResult};
use configs::get_server_configs;

use dependencies_sync::{bson};
use dependencies_sync::rust_i18n::{self, t};

use majordomo::get_majordomo;
use manage_define::{
    cashmere::Name,
    general_field_ids::{DESCRIPTION_FIELD_ID, ID_FIELD_ID, NAME_MAP_FIELD_ID},
};
use managers::entity_interface::EntityInterface;

use crate::ids_codes::{manage_ids::EVENT_TYPES_MANAGE_ID};
use crate::protocols::EventType;

use crate::event_services::register_event_type;

pub async fn init_event_types_map() -> Result<(), OperationResult> {
    let majordomo = get_majordomo();

    let manager = if let Ok(m) = majordomo.get_manager_by_id(EVENT_TYPES_MANAGE_ID) {
        m
    } else {
        return Err(operation_failed(
            "init_event_types_map",
            format!("{}: 管理 {}", t!("取得管理者失败"), EVENT_TYPES_MANAGE_ID),
        ));
    };

    let event_types_docs = if let Ok(ds) = manager.get_entities_by_filter(&None).await {
        ds
    } else {
        return Err(operation_failed(
            "init_event_types_map",
            format!("{}: 管理 {}", t!("取得管理者失败"), EVENT_TYPES_MANAGE_ID),
        ));
    };

    for type_doc in event_types_docs {
        let _lang_code = &get_server_configs().language_code;

        let id = type_doc.get_str(ID_FIELD_ID.to_string()).unwrap();
        let description = type_doc.get_document(DESCRIPTION_FIELD_ID.to_string()).unwrap().to_owned();

        let name: Name = bson::from_document(
            type_doc
                .get_document(NAME_MAP_FIELD_ID.to_string())
                .unwrap()
                .to_owned(),
        )
        .unwrap();
        
        let event_type = EventType {
            name: Some(name),
            type_id: id.to_string(),
            description: bson::from_document(description).unwrap(),
        };

        register_event_type(event_type);
    }

    Ok(())
}
