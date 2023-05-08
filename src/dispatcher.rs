use cash_result::{operation_failed, OperationResult};
use futures::channel::mpsc::SendError;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Sender};
use tokio::task::LocalSet;

use parking_lot::RwLock;

use crate::event_echo_wrapper::EventEchoWrapper;
use crate::{dispatch_local_set::get_dispatcher_localset, protocols::Event};

// 不同注册的监听者可能监听同一类型的事件，所以需要多个监听者
// 每个注册的监听者可能在多个地方登录, 所以需要有这里每个监听编号对应多个发送
// {listener_id: {instance_id: sender1, instance_id: sender2, ...}}
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

        let (dispatch_sender, mut dispatch_receiver) = channel::<EventEchoWrapper>(8);
        let listener_senders_map = ListenerSenderMapType::new();
        let listener_sender_map_arc = Arc::new(RwLock::new(listener_senders_map));

        let new_dispatcher = EventDispatcher {
            type_id: type_id.clone(),
            dispatch_sender: dispatch_sender.clone(),
            listener_sender_map: listener_sender_map_arc.clone(),
        };

        // 事件转发线程
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        std::thread::spawn(move || {
            let local_set = LocalSet::new();
            local_set.spawn_local(async move {
                while let event_wrapper = dispatch_receiver.recv().await {
                    let event_wrapper = event_wrapper.unwrap();
                    let type_id = event_wrapper.event.type_id.clone();
                    let serial_number = event_wrapper.event.serial_number.clone();
                    let emitter_id = event_wrapper.event.emitter_id.clone();

                    debug!("{}: {}-{}", t!("转发事件"), type_id, serial_number);

                    let listener_senders_map = listener_sender_map_arc.read();

                    // 事件转发到不同的监听者
                    for listeners in listener_senders_map.iter() {
                        // if sender.is_none() {
                        //     break;
                        // }

                        let (listener_id, senders_arc) = listeners;
                        debug!("{}: {}-{}", t!("转发到监听者"), listener_id, serial_number);

                        let mut senders = senders_arc.write();
                        // 检查接收端是否关闭，如果关闭，移除发送器
                        let keys = senders
                            .iter()
                            .filter(|(i, s)| s.is_closed())
                            .map(|(i, s)| *i)
                            .collect::<Vec<usize>>();
                        keys.iter().for_each(|(i)| {
                            if senders.remove(i).is_none() {
                                warn!("{}: {} {}", t!("移除已关闭的发送者失败"), listener_id, i);
                            };
                        });

                        // 事件转发到同一个监听者的不同实例
                        for (index, sender) in senders.iter() {
                            debug!("{}: {}-{}", t!("转发到监听者实例"), index, serial_number);
                            if sender.is_closed() {
                                continue;
                            }

                            // 发送事件到终点
                            if let Err(e) = sender.send(event_wrapper.clone()).await {
                                error!("{}: {}", t!("事件转发失败"), e);
                            };
                        }
                    }

                    debug!("{}: {}-{}", t!("转发事件完成"), type_id, serial_number);
                }

                info!("EventDispatcher: {} is closed", type_id);
            });

            rt.block_on(local_set);
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

        let listener_sender_map = self.listener_sender_map.write();
        if listener_sender_map.contains_key(listener_id) {
            let senders_arc = listener_sender_map.get(listener_id).unwrap();
            let mut senders = senders_arc.write();
            senders.remove(&sender_index);
        } else {
            warn!("{}: {}", t!("事件监听器不存在"), listener_id);
        }
    }
}
