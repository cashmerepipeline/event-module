use cash_result::{operation_failed, OperationResult};
use futures::channel::mpsc::SendError;
use log::{debug, error, info, warn};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Sender};
use tokio::task::LocalSet;

use parking_lot::RwLock;

use crate::dispatcher;
use crate::event_inner_wrapper::EventInnerWrapper;
use crate::event_services::get_event_runtime;
use crate::type_listeners_map::{
    add_listener_sender, get_type_listener_senders_map, get_type_listeners_map,
    get_type_listeners_senders_map, remove_listener_sender,
};
use crate::protocols::Event;
use crate::type_dispatcher_map::{get_dispatcher, get_dispatchers_map};

#[derive(Clone)]
pub struct EventDispatcher {
    pub type_id: String,
    pub dispatch_sender: Sender<EventInnerWrapper>,
}

/// 事件分发器
impl EventDispatcher {
    pub fn new(type_id: String) -> Self {
        // 创建事件接收通道
        info!("{}: {}", t!("开始创建事件类型转发通道"), type_id);

        let (dispatch_sender, mut dispatch_receiver) = channel::<EventInnerWrapper>(512);

        let new_dispatcher = EventDispatcher {
            type_id: type_id.clone(),
            dispatch_sender: dispatch_sender.clone(),
        };

        // 事件转发线程
        let rt = get_event_runtime();

        rt.spawn(async move {
            while let Some(event_wrapper) = dispatch_receiver.recv().await {
                dispatch(event_wrapper);
            }

            info!("EventDispatcher: {} is closed", type_id);
        });

        new_dispatcher
    }

    pub fn add_listener_sender(
        &self,
        listener_id: &String,
        listener_sender: Sender<EventInnerWrapper>,
    ) {
        let type_id = self.type_id.clone();
        add_listener_sender(&type_id, listener_id, listener_sender);
    }

    pub fn remove_listener_sender(self, listener_id: &String, sender_index: usize) {
        info!("{}: {}", t!("移除事件监听器"), listener_id);

        let type_id = self.type_id.clone();
        remove_listener_sender(&type_id, listener_id, sender_index);
    }
}

fn dispatch(event_wrapper: EventInnerWrapper) {
    let type_id = event_wrapper.event.type_id.clone();
    let serial_number: u64 = event_wrapper.event.serial_number;
    let emitter_id = event_wrapper.event.emitter_id.clone();

    debug!("{}: {}-{}", t!("转发事件"), type_id, serial_number);

    let type_listeners_map_arc = get_type_listeners_map(&type_id);
    let type_listeners_map = type_listeners_map_arc.read();

    let mut sender_futures = Vec::new();

    // 事件转发到不同的监听者
    for (listener_id, senders_arc) in type_listeners_map.iter() {
        debug!("{}: {}-{}", t!("转发到监听者"), listener_id, serial_number);

        {
            let mut senders = senders_arc.write();
            // 检查接收端是否关闭，如果关闭，移除发送器
            let keys = senders
                .iter()
                .filter(|(i, s)| s.is_closed())
                .map(|(i, _s)| *i)
                .collect::<Vec<usize>>();

            keys.iter().for_each(|(i)| {
                if senders.remove(i).is_none() {
                    warn!("{}: {} {}", t!("移除已关闭的发送者失败"), listener_id, i);
                };
            });
        }

        let senders = senders_arc.read();

        // 事件转发到同一个监听者的不同实例
        // for (index, sender) in senders.iter() {
        //     debug!("{}: {}-{}", t!("转发到监听者实例"), index, serial_number);
        //     if sender.is_closed() {
        //         continue;
        //     }

        //     // 发送事件到终点
        //     if let Err(e) = sender.send(event_wrapper.clone()).await {
        //         error!("{}: {}", t!("事件转发失败"), e);
        //     };
        // }

        // 发射器映射为 futures
        let listener_futures = senders
            .iter()
            .filter(|(_i, s)| !s.is_closed())
            .map(|(i, s)| {
                let sender = s.to_owned();
                let event_wrapper = event_wrapper.clone();
                async move {
                    if let Err(e) = sender.send(event_wrapper).await {
                        error!("{}: {}", t!("事件转发失败"), e);
                    };
                }
            })
            .collect::<Vec<_>>();

        sender_futures.extend(listener_futures);
    }

    let rt = get_event_runtime();
    rt.spawn(async move {
        let _ = futures::future::join_all(sender_futures).await;
    });

    debug!("{}: {}-{}", t!("转发事件完成"), type_id, serial_number);
}
