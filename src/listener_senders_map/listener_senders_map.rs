use std::{collections::BTreeMap, sync::Arc};

use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::tokio::sync::mpsc::Sender;

use crate::event_inner_wrapper::EventInnerWrapper;

pub type InstanceIndexSenderMap = BTreeMap<u32, Option<Sender<EventInnerWrapper>>>;
type ListenerSendersMap = BTreeMap<String, Arc<RwLock<InstanceIndexSenderMap>>>;

static mut LISTENER_SENDERS_MAP: Option<Arc<RwLock<ListenerSendersMap>>> = None;

pub fn get_listener_senders_map() -> Arc<RwLock<ListenerSendersMap>> {
    unsafe {
        if LISTENER_SENDERS_MAP.is_some() {
            LISTENER_SENDERS_MAP.clone().unwrap()
        } else {
            LISTENER_SENDERS_MAP.get_or_insert(build_listener_senders_map());
            LISTENER_SENDERS_MAP.clone().unwrap()
        }
    }
}

fn build_listener_senders_map() -> Arc<RwLock<ListenerSendersMap>> {
    let listener_senders_map = ListenerSendersMap::new();
    Arc::new(RwLock::new(listener_senders_map))
}

pub fn get_listener_sender_map(listender_id: &String) -> Arc<RwLock<InstanceIndexSenderMap>> {
    {
        let listener_senders_map = get_listener_senders_map();
        let listener_senders_map = listener_senders_map.read();

        if let Some(m) = listener_senders_map.get(listender_id) {
            return m.clone();
        }
    }
    {
        // 不存在则创建
        let listener_senders_map = get_listener_senders_map();
        let mut listener_senders_map = listener_senders_map.write();

        if let Some(m) = listener_senders_map.get(listender_id) {
            m.clone()
        } else {
            let listener_sender_map = build_listener_sender_map();
            listener_senders_map.insert(listender_id.to_string(), listener_sender_map.clone());
            listener_sender_map
        }
    }
}

fn build_listener_sender_map() -> Arc<RwLock<InstanceIndexSenderMap>> {
    let listener_sender_map = InstanceIndexSenderMap::new();
    Arc::new(RwLock::new(listener_sender_map))
}
