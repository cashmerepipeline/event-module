use cash_result::{operation_failed, OperationResult};
use futures::channel::mpsc::SendError;
use log::{debug, error, info, warn};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Sender};
use tokio::task::LocalSet;

use parking_lot::RwLock;
use crate::dispatch_queue::add_event_echo_wrapper_to_queue;

use crate::dispatcher;
use crate::event_inner_wrapper::EventInnerWrapper;
use crate::event_services::get_event_runtime;
use crate::event_type_listeners_map::add_event_type_listener;
use crate::listener_senders_map::{add_listener_sender, remove_listener_sender};
// use crate::type_listeners_map::{
//     add_listener_sender, get_type_listener_senders_map, get_type_listeners_map,
//     get_type_listeners_senders_map, remove_listener_sender,
// };

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
        info!("{}: {}", t!("添加事件监听器"), listener_id);
        add_event_type_listener(type_id, listener_id.clone());
        add_listener_sender( listener_id, listener_sender);
    }

    pub fn remove_listener_sender(self, listener_id: &String, sender_index: usize) {
        info!("{}: {}", t!("移除事件监听器"), listener_id);

        remove_listener_sender(listener_id, sender_index as u32);
    }
}

fn dispatch(event_wrapper: EventInnerWrapper) {
    add_event_echo_wrapper_to_queue(event_wrapper);
}
