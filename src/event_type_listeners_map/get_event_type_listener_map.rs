use std::sync::Arc;
use parking_lot::RwLock;
use crate::event_type_listeners_map;
use crate::event_type_listeners_map::IndexListenerMap;

/// 获取事件类型的监听者表
pub fn get_event_type_listener_map(type_id: &String) -> Arc<RwLock<IndexListenerMap>> {
    let listeners_map = event_type_listeners_map::get_event_type_listeners_map();
    let mut listeners_map = listeners_map.write();

    match listeners_map.get(type_id) {
        Some(m) => m.clone(),
        None => {
            let new_map = Arc::new(RwLock::new(IndexListenerMap::new()));
            listeners_map.insert(type_id.clone(), new_map.clone());
            new_map
        }
    }
}
