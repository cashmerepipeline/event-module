use crate::types::EventsQueueMap;
use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::sync::Arc;

/// 事件队列映射
static mut EVENTS_QUEUE_MAP: Option<Arc<RwLock<EventsQueueMap>>> = None;

/// 取得事件队列映射
pub(crate) fn get_events_queue_map() -> Arc<RwLock<EventsQueueMap>> {
    if let Some(events_queue_map) = unsafe { EVENTS_QUEUE_MAP.as_ref() } {
        events_queue_map.clone()
    } else {
        let events_queue_map = Arc::new(RwLock::new(BTreeMap::new()));
        unsafe {
            EVENTS_QUEUE_MAP = Some(events_queue_map.clone());
        }
        events_queue_map
    }
}
