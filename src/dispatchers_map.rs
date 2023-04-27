use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use crate::dispatcher::EventDispatcher;
use crate::event_protocol::Event;

type DispatcherType = Arc<RwLock<EventDispatcher>>;
type DispatcherMapType = HashMap<String, DispatcherType>;

static mut DISPATCHERS_MAP: Option<Arc<RwLock<DispatcherMapType>>> = None;

pub fn get_dispatchers_map() -> Arc<RwLock<DispatcherMapType>> {
    unsafe {
        if DISPATCHERS_MAP.is_some() {
            DISPATCHERS_MAP.clone().unwrap()
        } else {
            DISPATCHERS_MAP.get_or_insert(build_dispatchers());
            DISPATCHERS_MAP.clone().unwrap()
        }
    }
}

fn build_dispatchers() -> Arc<RwLock<DispatcherMapType>> {
    let result: DispatcherMapType = DispatcherMapType::new();
    Arc::new(RwLock::new(result))
}

/// 取得事件分发器, 如果不存在则创建新的
pub fn get_dispatcher(type_id: &String) -> Option<DispatcherType> {
    let dispatchers_arc = get_dispatchers_map();
    let mut dispatchers = dispatchers_arc.write();

    if dispatchers.contains_key(type_id) {
        let dispatcher = dispatchers.get(type_id).unwrap();
        Some(dispatcher.clone())
    } else {
        let new_dispatcher = Arc::new(RwLock::new(EventDispatcher::new(type_id.to_owned())));
        dispatchers.insert(type_id.to_owned(), new_dispatcher.clone());
        Some(new_dispatcher)
    }
}

// 取得事件分发器
pub async fn get_dispatcher_receive_sender(type_id: &String) -> Option<Sender<Event>> {
    if let Some(dispatcher_arc) = get_dispatcher(&type_id.to_owned()) {
        let dispatcher = dispatcher_arc.read();
        Some(dispatcher.dispatch_sender.clone())
    } else {
        None
    }
}
