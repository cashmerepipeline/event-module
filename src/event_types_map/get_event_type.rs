use dependencies_sync::bson;
use std::sync::Arc;

use manage_define::cashmere::Name;
use manage_define::general_field_ids::{DESCRIPTION_FIELD_ID, NAME_MAP_FIELD_ID};
use managers::ManagerTrait;

use crate::event_types_map::get_event_types_map;

use crate::ids_codes::manage_ids::EVENT_TYPES_MANAGE_ID;
use crate::protocols::EventType;

pub async fn get_event_type(type_id: &String) -> Option<Arc<EventType>> {
    // 如果存在，直接返回缓存
    {
        let event_types_map_arc = get_event_types_map();
        let event_types_map = event_types_map_arc.read();

        if let Some(event_type) = event_types_map.get(type_id.as_str()) {
            return Some(event_type.clone());
        }
    }

    // 如果不存在，从数据库中获取
    let manager = {
        let majar = majordomo::get_majordomo();
        majar.get_manager_by_id(EVENT_TYPES_MANAGE_ID).unwrap()
    };

    if let Ok(type_doc) = manager.get_entity_by_id(type_id, &vec![]).await {
        let name: Name = bson::from_document(
            type_doc
                .get_document(NAME_MAP_FIELD_ID.to_string())
                .unwrap()
                .to_owned(),
        )
        .unwrap();

        let description = type_doc
            .get_str(DESCRIPTION_FIELD_ID.to_string())
            .unwrap()
            .to_owned();

        let event_type = EventType {
            type_id: type_id.clone(),
            name: Some(name),
            description,
        };
        let event_type_arc = Arc::new(event_type);

        let event_types_map_arc = get_event_types_map();
        let mut event_types_map = event_types_map_arc.write();
        event_types_map.insert(type_id.clone(), event_type_arc.clone());

        Some(event_type_arc)
    } else {
        // 数据库中不存在
        None
    }
}
