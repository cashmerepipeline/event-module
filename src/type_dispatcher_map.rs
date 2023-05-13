use log::debug;
use parking_lot::RwLock;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use crate::dispatcher::EventDispatcher;
use crate::protocols::Event;

type DispatcherType = Arc<EventDispatcher>;
type DispatcherMapType = BTreeMap<String, DispatcherType>;

static mut TYPE_DISPATCHER_MAP: Option<Arc<RwLock<DispatcherMapType>>> = None;

pub fn get_dispatchers_map() -> Arc<RwLock<DispatcherMapType>> {
    unsafe {
        if TYPE_DISPATCHER_MAP.is_some() {
            TYPE_DISPATCHER_MAP.clone().unwrap()
        } else {
            TYPE_DISPATCHER_MAP.get_or_insert(build_dispatchers());
            TYPE_DISPATCHER_MAP.clone().unwrap()
        }
    }
}

fn build_dispatchers() -> Arc<RwLock<DispatcherMapType>> {
    let result: DispatcherMapType = DispatcherMapType::new();
    Arc::new(RwLock::new(result))
}

/// 取得事件分发器, 如果不存在则创建新的
pub fn get_dispatcher(type_id: &String) -> Option<DispatcherType> {
    let dispatcher_map_arc = get_dispatchers_map();
    let mut dispatcher_map = dispatcher_map_arc.write();

    if dispatcher_map.contains_key(type_id) {
        let dispatcher = dispatcher_map.get(type_id).unwrap();

        Some(dispatcher.clone())
    } else {
        debug!("{}: {}", t!("创建新的事件分发器"), type_id);

        let new_dispatcher = Arc::new(EventDispatcher::new(type_id.to_owned()));
        dispatcher_map.insert(type_id.to_owned(), new_dispatcher.clone());

        Some(new_dispatcher)
    }
}
