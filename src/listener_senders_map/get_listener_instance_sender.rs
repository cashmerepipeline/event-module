
use dependencies_sync::tokio::sync::mpsc::Sender;

use crate::event_inner_wrapper::EventInnerWrapper;

use crate::listener_senders_map::get_listener_sender_map;

pub fn get_listener_instance_sender(
    listener_id: &String,
    index: u32,
) -> Option<Sender<EventInnerWrapper>> {
    let instance_index_sender_map = get_listener_sender_map(listener_id);
    let instance_index_sender_map = instance_index_sender_map.read();

    match instance_index_sender_map.get(&index) {
        Some(r) => r.clone(),
        None => None,
    }
}
