use manage_define::cashmere::Name;
use manage_define::general_field_ids::{DESCRIPTIONS_FIELD_ID, ID_FIELD_ID, NAME_MAP_FIELD_ID};
use managers::ManagerTrait;
use std::sync::Arc;

use crate::event_types_map::get_event_types_map;
use crate::field_ids::EVENT_TYPES_HAS_ECHO_FIELD_ID;
use crate::manage_ids::EVENT_TYPES_MANAGE_ID;
use crate::protocols::EventType;

pub async fn get_event_type(type_id: &String) -> Option<Arc<EventType>> {
    // 如果存在，直接返回缓存
    {
        let event_types_map_arc = get_event_types_map();
        let event_types_map = event_types_map_arc.read();

        if let Some(event_type) = event_types_map.get(type_id.as_str()) {
            return Some(event_type.clone());
        } else {
        }
    }

    // 如果不存在，从数据库中获取
    let manager = {
        let majar = majordomo::get_majordomo().await;
        majar
            .get_manager_by_id(EVENT_TYPES_MANAGE_ID)
            .await
            .unwrap()
    };

    if let Ok(type_doc) = manager.get_entity_by_id(type_id).await {
        let name: Name = bson::from_document(
            type_doc
                .get_document(NAME_MAP_FIELD_ID.to_string())
                .unwrap()
                .to_owned(),
        )
        .unwrap();
        let has_echo = type_doc
            .get_bool(EVENT_TYPES_HAS_ECHO_FIELD_ID.to_string())
            .unwrap();
        let description = type_doc
            .get_str(DESCRIPTIONS_FIELD_ID.to_string())
            .unwrap()
            .to_owned();

        let event_type = EventType {
            type_id: type_id.clone(),
            name: Some(name),
            has_echo: has_echo,
            description: description,
        };
        let event_type_arc = Arc::new(event_type.clone());

        let event_types_map_arc = get_event_types_map();
        let mut event_types_map = event_types_map_arc.write();
        event_types_map.insert(type_id.clone(), event_type_arc.clone());

        return Some(event_type_arc);
    } else {
        // 数据库中不存在
        return None;
    };
}
