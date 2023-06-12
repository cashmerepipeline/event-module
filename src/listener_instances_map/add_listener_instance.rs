use dependencies_sync::tokio::sync::mpsc::Sender;

use crate::event_inner_wrapper::EventInnerWrapper;
use crate::listener_instance::ListenerInstance;
use crate::listener_instances_map::get_first_none_index::get_first_none_index;

use super::get_listener_instance_map;

pub fn add_listener_sender(listener_id: &String, instance_name: String, sender: Sender<EventInnerWrapper>) {
    let index = get_first_none_index(listener_id);
    
    let instance = Some(ListenerInstance{
        index,
        sender,
        name: instance_name,
    });

    let instance_index_sender_map_arc = get_listener_instance_map(listener_id);
    let mut instance_index_sender_map = instance_index_sender_map_arc.write();


    instance_index_sender_map.insert(index, instance);
}