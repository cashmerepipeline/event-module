use std::ops::Deref;

use crate::protocols::EventType;

use crate::event_types_map::get_event_types_map;

pub fn deregister_event_type(type_id: &String) -> Option<EventType>{
  let types_map_arc = get_event_types_map();
  let mut types_map = types_map_arc.write();

  if let Some(result) = types_map.remove(type_id){
    Some(result.deref().to_owned())
  } else{
    None
  }
}
