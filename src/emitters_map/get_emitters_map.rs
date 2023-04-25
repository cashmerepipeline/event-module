use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::emitter::Emitter;
use crate::types::{EventTypeEmittersMap, EventTypesMap, EventTypeListenersMap};

/// 发送者列表
static mut EMITTERS_MAP: Option<Arc<RwLock<EventTypeEmittersMap>>> = None;

/// 取得发送者映射
pub fn get_emitters_map() -> Arc<RwLock<EventTypeEmittersMap>> {
    if let Some(_map) = unsafe { EMITTERS_MAP.as_ref() } {
        _map.clone()
    } else {
        let _map = Arc::new(RwLock::new(BTreeMap::new()));
        unsafe {
            EMITTERS_MAP = Some(_map.clone());
        }
        _map
    }
}

