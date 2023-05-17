use crate::listener_senders_map::get_listener_sender_map;

pub fn remove_listener_sender(listener_id: &String, index: u32) {
    let instance_index_sender_map = get_listener_sender_map(listener_id);
    let mut instance_index_sender_map = instance_index_sender_map.write();

    instance_index_sender_map.remove(&index);
}