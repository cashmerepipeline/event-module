use std::{sync::Arc, collections::BTreeMap};

use parking_lot::RwLock;

use crate::emitter::Emitter;

use super::get_emitters_map;

/// 取得发送者表
pub fn get_event_type_emitters_map(event_type_id: &String) -> Arc<RwLock<BTreeMap<String, Emitter>>> {
    let emitters_map_arc = get_emitters_map();
    let mut emitters_map = emitters_map_arc.write();

    if emitters_map.contains_key(event_type_id) {
        return emitters_map.get(event_type_id).unwrap().clone();
    } else {
        // new
        let emitters_arc: Arc<RwLock<BTreeMap<String, Emitter>>> = Arc::new(RwLock::new(BTreeMap::new()));
        emitters_map.insert(event_type_id.clone(), emitters_arc.clone());
        return emitters_arc;
    }
}
