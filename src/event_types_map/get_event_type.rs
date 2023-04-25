use crate::event_type::EventType;
use crate::event_types_map::get_event_types_map;

pub fn get_event_type(type_id: &String) -> Option<EventType> {
    let event_types_map_arc = get_event_types_map();
    let event_types_map = event_types_map_arc.read();

    // 复制返回，提前释放锁
    event_types_map.get(type_id.as_str()).cloned()
}
