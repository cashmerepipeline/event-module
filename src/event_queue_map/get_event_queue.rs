use crate::event_queue_map::get_events_queue_map;
use crate::types::EventDeQue;
use parking_lot::RwLock;
use std::collections::VecDeque;
use std::sync::Arc;

/// 如果队列不存在，则创建
pub(crate) fn get_event_queue(event_type_id: &String) -> EventDeQue {
    let events_queue_map_arc = get_events_queue_map();
    let events_queue_map = events_queue_map_arc.read();
    let events_queue = events_queue_map.get(event_type_id).cloned();

    if let Some(events_queue) = events_queue {
        events_queue
    } else {
        let events_queue = Arc::new(RwLock::new(VecDeque::new()));
        let mut events_queue_map = events_queue_map_arc.write();
        events_queue_map.insert(event_type_id.clone(), events_queue.clone());
        events_queue
    }
}
