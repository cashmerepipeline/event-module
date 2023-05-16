use std::collections::BTreeMap;
use std::sync::Arc;

use tokio::sync::mpsc::Sender;
use parking_lot::RwLock;

use crate::event_inner_wrapper::EventInnerWrapper;
use crate::protocols::Event;

// 不同注册的监听者可能监听同一类型的事件，所以需要多个监听者
// 每个注册的监听者可能在多个地方登录, 所以需要有这里每个监听编号对应多个发送
// {type_id:{listener_id: {instance_id: sender1, instance_id: sender2, ...}}}
pub type SendersMapType = BTreeMap<usize, Sender<EventInnerWrapper>>;
pub type ListenerSendersMap = BTreeMap<String, Arc<RwLock<SendersMapType>>>;
pub type TypeListenersMapType = BTreeMap<String, Arc<RwLock<ListenerSendersMap>>>;

static mut TYPE_LISTENERS_SENDERS_MAP: Option<Arc<RwLock<TypeListenersMapType>>> = None;

pub fn get_type_listeners_senders_map() -> Arc<RwLock<TypeListenersMapType>> {
    unsafe {
        if TYPE_LISTENERS_SENDERS_MAP.is_some() {
            TYPE_LISTENERS_SENDERS_MAP.clone().unwrap()
        } else {
            TYPE_LISTENERS_SENDERS_MAP.get_or_insert(build_type_listener_senders_map());
            TYPE_LISTENERS_SENDERS_MAP.clone().unwrap()
        }
    }
}

fn build_type_listener_senders_map() -> Arc<RwLock<TypeListenersMapType>> {
    let result: TypeListenersMapType = TypeListenersMapType::new();
    Arc::new(RwLock::new(result))
}

/// 取得事件类型监听者表, 如果不存在则创建新的
pub fn get_type_listeners_map(type_id: &String) -> Arc<RwLock<ListenerSendersMap>> {
    let type_listeners_senders_map_arc = get_type_listeners_senders_map();
    let mut type_listeners_senders_map = type_listeners_senders_map_arc.write();

    if type_listeners_senders_map.contains_key(type_id) {
        let listener_senders_map = type_listeners_senders_map.get(type_id).unwrap();
        listener_senders_map.clone()
    } else {
        let new_listener_senders_map: ListenerSendersMap = ListenerSendersMap::new();
        let new_listener_senders_map = Arc::new(RwLock::new(new_listener_senders_map));
        type_listeners_senders_map.insert(type_id.to_owned(), new_listener_senders_map.clone());
        new_listener_senders_map
    }
}

/// 取得事件分发器, 如果不存在则创建新的
pub fn get_type_listener_senders_map(type_id:&String, listener_id: &String) -> Arc<RwLock<SendersMapType>> {
    let type_listeners_senders_map_arc = get_type_listeners_senders_map();
    let mut type_listeners_senders_map = type_listeners_senders_map_arc.write();

    if type_listeners_senders_map.contains_key(type_id) {
        let listener_senders_map = type_listeners_senders_map.get(type_id).unwrap();
        let mut listener_senders_map = listener_senders_map.write();

        if listener_senders_map.contains_key(listener_id) {
            let senders_map = listener_senders_map.get(listener_id).unwrap();
            senders_map.clone()
        } else {
            let new_senders_map: SendersMapType = SendersMapType::new();
            let new_senders_map = Arc::new(RwLock::new(new_senders_map));
            listener_senders_map.insert(listener_id.to_owned(), new_senders_map.clone());
            new_senders_map
        }
    } else {
        let new_senders_map: SendersMapType = SendersMapType::new();
        let new_senders_map = Arc::new(RwLock::new(new_senders_map));
        let mut listener_senders_map: ListenerSendersMap = ListenerSendersMap::new();
        listener_senders_map.insert(listener_id.to_owned(), new_senders_map.clone());
        type_listeners_senders_map.insert(type_id.to_owned(), Arc::new(RwLock::new(listener_senders_map)));
        new_senders_map
    }
}
