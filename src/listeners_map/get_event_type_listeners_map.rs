use std::{sync::Arc, collections::BTreeMap};

use parking_lot::RwLock;

use crate::types::ListenersMap;

use super::get_listeners_map;

pub fn get_event_type_listeners_map(event_type_id: &String) -> Arc<RwLock<ListenersMap>> {
    let listeners_map_arc = get_listeners_map();
    let mut listeners_map = listeners_map_arc.write();

    match listeners_map.get(event_type_id) {
      Some(m) => m.clone(),
      None => {
        let new_map: Arc<RwLock<ListenersMap>> = Arc::new(RwLock::new(BTreeMap::new()));
        listeners_map.insert(event_type_id.clone(), new_map.clone());
        new_map
      }
    }
}
