use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use dependencies_sync::parking_lot::RwLock;

use crate::event_inner_wrapper::EventInnerWrapper;

pub type DispatchQueue = VecDeque<EventInnerWrapper>;
type EventTypeDispatchQueueMap = BTreeMap<String, Arc<RwLock<DispatchQueue>>>;

static mut EVENT_TYPE_DISPATCH_QUEUE_MAP: Option<Arc<RwLock<EventTypeDispatchQueueMap>>> = None;

pub fn get_event_type_dispatch_queue_map() -> Arc<RwLock<EventTypeDispatchQueueMap>> {
    unsafe {
        if EVENT_TYPE_DISPATCH_QUEUE_MAP.is_some() {
            EVENT_TYPE_DISPATCH_QUEUE_MAP.clone().unwrap()
        } else {
            EVENT_TYPE_DISPATCH_QUEUE_MAP.get_or_insert(build_event_type_dispatch_queue_map());
            EVENT_TYPE_DISPATCH_QUEUE_MAP.clone().unwrap()
        }
    }
}

fn build_event_type_dispatch_queue_map() -> Arc<RwLock<EventTypeDispatchQueueMap>> {
    let event_type_dispatch_queue_map = EventTypeDispatchQueueMap::new();
    Arc::new(RwLock::new(event_type_dispatch_queue_map))
}

pub fn get_event_type_dispatch_queue(event_type_id: &String) -> Arc<RwLock<DispatchQueue>> {
    {
        let event_type_dispatch_queue_map = get_event_type_dispatch_queue_map();
        let event_type_dispatch_queue_map = event_type_dispatch_queue_map.read();
        let event_type_dispatch_queue = event_type_dispatch_queue_map.get(event_type_id);
        if event_type_dispatch_queue.is_some() {
            return event_type_dispatch_queue.unwrap().clone();
        }
    }
    {
        // 不存在则创建
        let event_type_dispatch_queue_map = get_event_type_dispatch_queue_map();
        let mut event_type_dispatch_queue_map = event_type_dispatch_queue_map.write();
        let event_type_dispatch_queue = event_type_dispatch_queue_map.get(event_type_id);
        if event_type_dispatch_queue.is_some() {
            event_type_dispatch_queue.unwrap().clone()
        } else {
            let event_type_dispatch_queue = build_event_type_dispatch_queue();
            event_type_dispatch_queue_map
                .insert(event_type_id.to_string(), event_type_dispatch_queue.clone());
            event_type_dispatch_queue
        }
    }
}

fn build_event_type_dispatch_queue() -> Arc<RwLock<DispatchQueue>> {
    let event_type_dispatch_queue = DispatchQueue::new();
    Arc::new(RwLock::new(event_type_dispatch_queue))
}
