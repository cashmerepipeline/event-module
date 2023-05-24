use std::sync::Arc;
use crate::protocols::EventType;
use crate::event_types_map::get_event_types_map;


pub fn get_event_types() -> Vec<Arc<EventType>> {
    let event_types_map_arc = get_event_types_map();
    let event_types_map = event_types_map_arc.read();

    // 复制返回，提前释放锁
    event_types_map.values().cloned().collect()
}
