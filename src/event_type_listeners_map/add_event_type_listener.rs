use std::ops::Deref;
use std::sync::Arc;

use crate::event_type_listeners_map::{get_event_type_listener_map, get_event_type_listeners_map};

pub fn add_event_type_listener(event_type: String, listener_id: String) {
    let event_type_listener_map = get_event_type_listener_map::get_event_type_listener_map(&event_type);
    let mut listener_map = event_type_listener_map.write();

    if listener_map
        .values()
        // 查找是否已经存在该监听者
        .find(|x| x.deref().deref() == &listener_id)
        // .find(|x| x[..] == listener_id)
        .is_some()
    {
        return;
    } else {
        let index = listener_map.len() as u32;
        listener_map.insert(index, Arc::new(listener_id.clone()));
    }
}
