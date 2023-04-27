use parking_lot::RwLock;
use std::sync::Arc;
use tokio::task::LocalSet;

static mut DISPATCH_LOCALSET: Option<Arc<RwLock<LocalSet>>> = None;

pub fn get_dispatcher_localset() -> Arc<RwLock<LocalSet>> {
    unsafe {
        if DISPATCH_LOCALSET.is_some() {
            DISPATCH_LOCALSET.clone().unwrap()
        } else {
            DISPATCH_LOCALSET.get_or_insert(Arc::new(RwLock::new(LocalSet::new())));
            DISPATCH_LOCALSET.clone().unwrap()
        }
    }
}
