use std::collections::BTreeMap;

use tokio::sync::broadcast;

use crate::protocols::Event;
use crate::protocols::EventType;
use crate::event_types_map::get_event_types_map;
use crate::types::EventTypesMap;

pub fn register_event_type(
    type_name: String,
    schema: BTreeMap<String, EventMessageFieldInfo>,
    description: String,
) -> String {
    let event_types_map_arc = get_event_types_map();
    let mut event_types_map = event_types_map_arc.write();

    let new_type = EventType::new(
        type_name,
        tx,
        Some(uid.clone()),
        Some(description),
        Some(schema),
    );

    event_types_map.insert(uid.clone(), new_type);

    uid.clone()
}
