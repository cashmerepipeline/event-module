use dependencies_sync::log::info;

use crate::listener_senders_map::get_listener_sender_map;

pub fn remove_listener_senders(listener_id: &String, invalid_sender_indexes: Vec<u32>) {
    let listener_sender_map_arc = get_listener_sender_map(listener_id);
    let mut listener_sender_map = listener_sender_map_arc.write();
    for index in invalid_sender_indexes.iter() {
        info!("{}: {}", t!("移除已经失效的事件发送器"), index);
        listener_sender_map.remove(index);
    }
}
