use std::{collections::BTreeMap, sync::Arc};

use dependencies_sync::{parking_lot::RwLock, tokio::sync::mpsc::Sender};

use crate::event_inner_wrapper::EventInnerWrapper;

type EventTypeDispatchSenderMap = BTreeMap<String, Sender<EventInnerWrapper>>;

static mut EVENT_TYPE_DISPATCH_SENDER_MAP: Option<Arc<RwLock<EventTypeDispatchSenderMap>>> = None;

pub fn get_event_type_dispatch_sender_map() -> Arc<RwLock<EventTypeDispatchSenderMap>> {
    unsafe {
        if EVENT_TYPE_DISPATCH_SENDER_MAP.is_some() {
            EVENT_TYPE_DISPATCH_SENDER_MAP.clone().unwrap()
        } else {
            EVENT_TYPE_DISPATCH_SENDER_MAP.get_or_insert(build_event_type_dispatch_sender_map());
            EVENT_TYPE_DISPATCH_SENDER_MAP.clone().unwrap()
        }
    }
}

fn build_event_type_dispatch_sender_map() -> Arc<RwLock<EventTypeDispatchSenderMap>> {
    let event_type_dispatch_sender_map = EventTypeDispatchSenderMap::new();
    Arc::new(RwLock::new(event_type_dispatch_sender_map))
}

pub fn insert_event_type_dispatch_sender(event_type_id: &String, sender: Sender<EventInnerWrapper>) {
    let event_type_dispatch_sender_map = get_event_type_dispatch_sender_map();
    let mut event_type_dispatch_sender_map = event_type_dispatch_sender_map.write();
    event_type_dispatch_sender_map.insert(event_type_id.to_string(), sender);
}

pub fn get_event_type_dispatch_sender(event_type_id: &String) -> Option<Sender<EventInnerWrapper>> {
    {
        let event_type_dispatch_sender_map = get_event_type_dispatch_sender_map();
        let event_type_dispatch_sender_map = event_type_dispatch_sender_map.read();
        let event_type_dispatch_sender = event_type_dispatch_sender_map.get(event_type_id);
        if let Some(q) = event_type_dispatch_sender {
            return Some(q.clone());
        }
    }
    
    None
}
