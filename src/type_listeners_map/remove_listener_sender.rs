use log::info;

use crate::type_listeners_map::get_type_listeners_senders_map;

pub fn remove_listener_sender(type_id: &String, listener_id: &String, sender_index: usize) {
    info!("{}: {}", t!("移除事件监听器"), listener_id);

    let type_listeners_senders_map_arc = get_type_listeners_senders_map();
    let type_listeners_senders_map = type_listeners_senders_map_arc.read();

    type_listeners_senders_map
        .get(type_id)
        .and_then(|listener_senders_map_arc| {
            let listener_senders_map = listener_senders_map_arc.read();
            listener_senders_map
                .get(listener_id)
                .and_then(|senders_map_arc| {
                    let mut senders_map = senders_map_arc.write();
                    senders_map.remove(&sender_index);
                    Some(())
                })
        });
}
