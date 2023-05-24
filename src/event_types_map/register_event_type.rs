
use std::sync::Arc;

use dependencies_sync::log::warn;


use crate::event_types_map::get_event_types_map;


use crate::protocols::EventType;


pub fn register_event_type(new_event_type: EventType) -> Option<String> {
    let type_id = new_event_type.type_id.clone();

    let event_types_map_arc = get_event_types_map();
    {
        let event_types_map = event_types_map_arc.read();

        // 如果已存在，返回已存在的事件类型
        if event_types_map.contains_key(&new_event_type.type_id) {
            warn!("{}: {}", t!("事件类型已存在"), new_event_type.type_id);
            return Some(type_id);
        }
    }

    // 注册事件类型
    let mut event_types_map = event_types_map_arc.write();

    let type_id = new_event_type.type_id.clone();
    let new_type = Arc::new(new_event_type);
    event_types_map.insert(type_id.clone(), new_type);

    Some(type_id)
}
