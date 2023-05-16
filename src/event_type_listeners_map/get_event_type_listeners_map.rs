use std::{collections::BTreeMap, sync::Arc};

use parking_lot::RwLock;

pub type IndexListenerMap = BTreeMap<u32, Arc<String>>;
pub type EventTypeListenersMap = BTreeMap<String, Arc<RwLock<IndexListenerMap>>>;

static mut EVENT_TYPE_LISTENERS_MAP: Option<Arc<RwLock<EventTypeListenersMap>>> = None;

pub fn get_event_type_listeners_map() -> Arc<RwLock<EventTypeListenersMap>> {
    unsafe {
        if EVENT_TYPE_LISTENERS_MAP.is_some() {
            EVENT_TYPE_LISTENERS_MAP.clone().unwrap()
        } else {
            EVENT_TYPE_LISTENERS_MAP.get_or_insert(build_event_type_listeners_map());
            EVENT_TYPE_LISTENERS_MAP.clone().unwrap()
        }
    }
}

fn build_event_type_listeners_map() -> Arc<RwLock<EventTypeListenersMap>> {
    let mut event_type_listeners_map = EventTypeListenersMap::new();
    Arc::new(RwLock::new(event_type_listeners_map))
}

pub fn get_event_type_listener_map(type_id: &String) -> Arc<RwLock<IndexListenerMap>> {
    let listeners_map = get_event_type_listeners_map();
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