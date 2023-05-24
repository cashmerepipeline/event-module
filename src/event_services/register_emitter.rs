use std::fmt::format;
use std::hash::Hash;
use std::str::FromStr;
use std::sync::Arc;

use dependencies_sync::parking_lot::RwLock;

use md5::{Digest, Md5};

use crate::emitter::Emitter;
use crate::emitters_map::{get_emitters_map, get_event_type_emitters_map};

/// 注册发送者
/// 返回事件发送者标识uid
/// 标识uid为 event_type_id + emitter_name 的hash
pub fn register_emitter(event_type_id: &u32, emitter_name: &String) -> Option<String> {
    let mut hasher = Md5::new();
    hasher.update(format!("{}-{}", event_type_id, emitter_name).as_bytes());
    let hash_result = hasher.finalize().to_ascii_lowercase();
    let emitter_id = uuid::Uuid::from_slice(hash_result.as_slice())
        .unwrap()
        .simple()
        .to_string();

    // new Emitter
    let emitter = Emitter {
        id: emitter_id.clone(),
        type_id: event_type_id.clone(),
        name: emitter_name.clone(),
    };

    // 不存在则新建
    let emitters_map_arc = get_event_type_emitters_map(&event_type_id);
    let mut emitters_map = emitters_map_arc.write();
    if !emitters_map.contains_key(&emitter_id){
        emitters_map.insert(emitter_id.clone(), emitter);
    }

    Some(emitter_id)
}
