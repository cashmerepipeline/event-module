use std::sync::Arc;

use log::info;
use parking_lot::RwLock;
use tokio::sync::mpsc::Sender;

use crate::event_inner_wrapper::EventInnerWrapper;

use super::{
    get_type_listener_senders_map, get_type_listeners_senders_map, ListenerSendersMap,
    SendersMapType, TypeListenersMapType,
};

pub fn add_listener_sender(
    type_id: &String,
    listener_id: &String,
    listener_sender: Sender<EventInnerWrapper>,
) {
    info!("{}: {}", t!("添加事件监听者"), listener_id);

    let senders_map_arc = get_type_listener_senders_map(type_id, listener_id);
    let mut senders_map = senders_map_arc.write();
    let index = senders_map.len() + 1;
    senders_map.insert(index, listener_sender);

    // // 一个事件类型对应多个监听者 
    // let listener_senders_map_arc = if let Some(m) = type_listeners_senders_map.get(type_id){
    //     m.clone()
    // } else {
    //     // 不存在类型表, 新建并插入
    //     let new_type_listeners_senders_map = TypeListenersMapType::new();
    //     let new_listener_senders_map = ListenerSendersMap::new();
    //     let new_sendrs_map = SendersMapType::new();

    //     let new_type_listeners_senders_map_arc = Arc::new(RwLock::new(new_type_listeners_senders_map));
    //     type_listeners_senders_map.insert(type_id.to_owned(), new_type_listeners_senders_map_arc.clone());

    //     new_type_listeners_senders_map_arc
    // };
    
    // // 一个listener对应多个监听者实例
    // let listener_senders_map_arc = if let Some(m) = type_listeners_senders_map.get(listener_id){
    //     m.clone()
    // } else {
    //     // 不存在监听者表, 新建并插入
    //     let new_listener_senders_map = ListenerSendersMap::new();

    //     let new_listener_senders_map_arc = Arc::new(RwLock::new(new_listener_senders_map));
    //     listener_senders_map_arc.insert(listener_id.to_owned(), new_listeners_senders_map_arc.clone());

    //     new_listener_senders_map_arc
    // };
    
    // let mut listener_senders_map = listener_senders_map_arc.write();
    // // 发送者实例表
    // let senders_map_arc = if let Some(m) = listener_senders_map.get(listener_id){
    //     m.clone()
    // } else {
    //     // 不存在监听者实例表, 新建并插入
//     let new_senders_map = SendersMapType::new();

    //     let new_senders_map_arc = Arc::new(RwLock::new(new_senders_map));
    //     listener_senders_map.insert(listener_id.to_owned(), new_senders_map_arc.clone());

    //     new_senders_map_arc
    // };

    // // 插入发送器
    // let mut senders_map = senders_map_arc.write();
}
