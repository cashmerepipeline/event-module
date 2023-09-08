use crate::{event_types_map::get_event_types_map, get_event_service_configs};

pub fn has_max_evnet_type_count_reached() -> bool {
    let max_types_count = get_event_service_configs().max_event_type_count as usize;

    let current_type_count = {
        let event_types_map_arc = get_event_types_map();
        let event_types_map = event_types_map_arc.read();
        event_types_map.len()
    };

    if current_type_count >= max_types_count {
      true
    } else{
      false
    }
}
