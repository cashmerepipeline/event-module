use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::BTreeMap;
use crate::types::EventTypeListenersMap;

/// 监听者列表
static mut LISTENERS_MAP: Option<Arc<RwLock<EventTypeListenersMap>>> = None;

/// 取得监听映射
pub fn get_listeners_map() -> Arc<RwLock<EventTypeListenersMap>> {
    if let Some(_map) = unsafe { LISTENERS_MAP.as_ref() } {
        _map.clone()
    } else {
        let _map = Arc::new(RwLock::new(BTreeMap::new()));
        unsafe {
            LISTENERS_MAP = Some(_map.clone());
        }
        _map
    }
}
