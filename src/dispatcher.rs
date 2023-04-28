use cash_result::{OperationResult, operation_failed};
use futures::channel::mpsc::SendError;
use log::{info, warn, error};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Sender};

use parking_lot::RwLock;

use crate::event_echo_wrapper::EventEchoWrapper;
use crate::{dispatch_local_set::get_dispatcher_localset, event_protocol::Event};

// 每个注册的监听者可能在多个地方登录, 所以需要有这里每个监听编号对应多个发送
type ListenerSenderMapType = HashMap<String, Arc<RwLock<HashMap<usize, Sender<EventEchoWrapper>>>>>;

#[derive(Clone)]
pub struct EventDispatcher {
    pub type_id: String,
    pub dispatch_sender: Sender<EventEchoWrapper>,
    // 每个注册的监听者可能在多个地方登录
    pub listener_sender_map: Arc<RwLock<ListenerSenderMapType>>,
}

/// 事件分发器
impl EventDispatcher {
    pub fn new(type_id: String) -> Self {
        // 创建事件接收通道
        info!("{}: {}", t!("开始创建事件类型转发通道"), type_id);
        let (receive_sender, mut receive_receiver) = channel::<EventEchoWrapper>(8);
        let listener_senders_map = ListenerSenderMapType::new();
        let listener_sender_map_arc = Arc::new(RwLock::new(listener_senders_map));

        let new_dispatcher = EventDispatcher {
            type_id: type_id.clone(),
            dispatch_sender: receive_sender.clone(),
            listener_sender_map: listener_sender_map_arc.clone(),
        };

        // 启动事件转发
        let dispatch_local_set = get_dispatcher_localset();
        let local_set = dispatch_local_set.write();
        local_set.spawn_local(async move {
            while let Some(event) = receive_receiver.recv().await {
                {
                    let listener_senders_arc = listener_sender_map_arc.clone();
                    let listener_senders = listener_senders_arc.read();

                    while let Some((_t, senders_arc)) = listener_senders.iter().next() {
                        let senders = senders_arc.read();
                        for (_index, s) in senders.iter().next() {
                            if let Err(e) = s.send(event.clone()).await {
                                error!("{}: {}", t!("事件转发失败"), e);
                            };
                        }
                    }
                    // listener_senders
                    //     .iter()
                    //     .for_each(|(listener_id, listener_sender)| async {
                    //         listener_sender.send(event).await;
                    //     });
                }
            }

            info!("EventDispatcher: {} is closed", type_id);
        });

        new_dispatcher
    }

    pub fn add_listener_sender(
        &mut self,
        listener_id: &String,
        listener_sender: Sender<EventEchoWrapper>,
    ) {
        info!("{}: {}", t!("添加事件监听器"), listener_id);

        let mut listener_sender_map = self.listener_sender_map.write();
        // 一个listener 可能有多个监听端
        if listener_sender_map.contains_key(listener_id) {
            let senders_arc = listener_sender_map.get(listener_id).unwrap();
            let mut senders = senders_arc.write();
            let index = senders.len() + 1;
            senders.insert(index, listener_sender);
        } else {
            // 新建表并插入
            let mut senders = HashMap::new();
            senders.insert(1, listener_sender);
            let senders_arc = Arc::new(RwLock::new(senders));
            listener_sender_map.insert(listener_id.to_owned(), senders_arc);
        }
    }

    pub fn remove_listener_sender(&mut self, listener_id: &String, sender_index: usize) {
        info!("{}: {}", t!("移除事件监听器"), listener_id);

        let mut listener_sender_map = self.listener_sender_map.write();
        if listener_sender_map.contains_key(listener_id) {
            let senders_arc = listener_sender_map.get(listener_id).unwrap();
            let mut senders = senders_arc.write();
            senders.remove(&sender_index);
        } else {
            warn!("{}: {}", t!("事件监听器不存在"), listener_id);
        }
    }

    pub async fn send_event(
        &self,
        event_echo_wrapper: EventEchoWrapper,
    ) -> Result<(), OperationResult> {
        if let Err(e) = self.dispatch_sender.send(event_echo_wrapper).await{
            log::error!("{}: {}", t!("事件发送失败"), e);
            return Err(operation_failed("send_event", t!("事件发送失败")));
        }

        Ok(())
    }
}
