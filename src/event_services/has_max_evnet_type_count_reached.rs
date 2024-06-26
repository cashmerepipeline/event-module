use configs::ConfigTrait;

use crate::{event_types_map::get_event_types_map, EventServiceConfigs};

pub fn has_max_evnet_type_count_reached() -> bool {
    let max_types_count = EventServiceConfigs::get().max_event_type_count as usize;

    let current_type_count = {
        let event_types_map_arc = get_event_types_map();
        let event_types_map = event_types_map_arc.read();
        event_types_map.len()
    };

    current_type_count >= max_types_count
}
