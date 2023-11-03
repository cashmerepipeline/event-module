use configs::ConfigTrait;

use crate::{
    event_types_map::get_event_types_map,
    listener_instances_map::{get_listener_instance_map, get_listener_instances_map}, EventServiceConfigs,
};

pub fn has_max_listener_instance_count_reached(listener_id: &String) -> bool {
    let max_count = EventServiceConfigs::get().max_listener_instance_count as usize;
    let current_count = {
        let instance_map_arc = get_listener_instance_map(listener_id);
        let instance_map = instance_map_arc.read();
        instance_map.len()
    };

    if current_count >= max_count {
        true
    } else {
        false
    }
}
