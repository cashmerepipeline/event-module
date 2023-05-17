use tokio::sync::mpsc::Sender;

use crate::event_inner_wrapper::EventInnerWrapper;
use crate::listener_senders_map::get_first_none_index::get_first_none_index;
use crate::listener_senders_map::get_listener_sender_map;

pub fn add_listener_sender(listener_id: &String, listener_sender: Sender<EventInnerWrapper>) {
    let index = get_first_none_index(listener_id);

    let instance_index_sender_map = get_listener_sender_map(listener_id);
    let mut instance_index_sender_map = instance_index_sender_map.write();


    instance_index_sender_map.insert(index, Some(listener_sender));
}