
use dependencies_sync::tokio::sync::mpsc::Sender;

use crate::event_inner_wrapper::EventInnerWrapper;

use super::get_listener_instance_map;

pub fn get_listener_instance_sender(
    listener_id: &String,
    instance_index: u32,
) -> Option<Sender<EventInnerWrapper>> {
    let instance_index_sender_map = get_listener_instance_map(listener_id);
    let instance_index_sender_map = instance_index_sender_map.read();

    match instance_index_sender_map.get(&instance_index) {
        Some(r) =>{
            if let Some(r) = r {
                return Some(r.sender.clone());
            } else {
                return None;
            }
        },
        None => None,
    }
}
