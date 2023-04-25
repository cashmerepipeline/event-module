use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::types::EventTypesMap;

static mut EVENT_TYPES_MAP: Option<Arc<RwLock<EventTypesMap>>> = None;

/// 取得事件类型映射
pub fn get_event_types_map() -> Arc<RwLock<EventTypesMap>> {
    if let Some(_map) = unsafe { EVENT_TYPES_MAP.as_ref() } {
        _map.clone()
    } else {
        let _map = Arc::new(RwLock::new(BTreeMap::new()));
        unsafe {
            EVENT_TYPES_MAP = Some(_map.clone());
        }
        _map
    }
}
