use std::sync::Arc;
use md5::{Digest, Md5};
use parking_lot::RwLock;

use crate::{event_types_map::get_event_type, listener::Listener, listeners_map::get_event_type_listeners_map};

/// 监听事件类型
pub fn listen_event_type(
    event_type_id: &String,
    listener_name: &String,
) -> Result<Listener, ()> {
    let event_type = get_event_type(event_type_id);
    if event_type.is_none() {
        return Err(());
    }

    let event_type = event_type.unwrap();

    let mut hasher = Md5::new();
    hasher.update(format!("{}-{}", event_type_id, listener_name));
    let hash_result = hasher.finalize();
    let uid = uuid::Uuid::from_slice(hash_result.as_slice())
        .and_then(|r| Ok(r.simple().to_string()))
        .unwrap()
        .to_owned();

    let reciever = Arc::new(RwLock::new(event_type.subscribe()));
    let listener = Listener {
        uid: uid.clone(),
        name: listener_name.clone(),
        event_type_uid: event_type_id.clone(),
        receiver: reciever,
    };
    
    let listener_map_arc = get_event_type_listeners_map(event_type_id);
    let mut listener_map = listener_map_arc.write();
    listener_map.insert(uid.clone(), listener.clone());

    Ok(listener)
}
