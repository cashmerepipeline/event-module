use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Sender};

use parking_lot::RwLock;

use crate::{
    dispatcher,
    dispatchers_map::{get_dispatcher, get_dispatchers_map},
    event_protocol::Event,
};

type ListenerSenderMapType = HashMap<String, Sender<Event>>;

#[derive(Clone)]
pub struct EventDispatcher {
    pub type_id: String,
    pub dispatch_sender: Sender<Event>,
    pub listener_sender_map: Arc<RwLock<ListenerSenderMapType>>,
}

/// 事件分发器
impl EventDispatcher {
    pub async fn new(type_id: String) -> Self {
        // 创建事件接收通道
        info!("{}: {}", t!("开始创建事件类型转发通道"), type_id);
        let (receive_sender, mut receive_receiver) = channel::<Event>(32);
        let listener_senders_map = ListenerSenderMapType::new();
        let listener_sender_map_arc = Arc::new(RwLock::new(listener_senders_map));

        let new_dispatcher = EventDispatcher {
            type_id: type_id.clone(),
            dispatch_sender: receive_sender.clone(),
            listener_sender_map: listener_sender_map_arc.clone(),
        };

        // 启动事件转发
        // TODO： 需要重构支持spaw, 可能有性能问题
        tokio::task::spawn_local(async move {
            while let Some(event) = receive_receiver.recv().await {
                {
                    let listener_senders_arc = listener_sender_map_arc.clone();
                    let listener_senders = listener_senders_arc.read();

                    while let Some((t, s)) = listener_senders.iter().next() {
                        s.send(event.clone()).await;
                    }
                    // listener_senders.iter().for_each(|(listener_id, listener_sender)| async {
                    //     listener_sender.send(event.clone()).await;
                    // });
                }
            }

            info!("EventDispatcher: {} is closed", type_id);
        })
        .await;

        new_dispatcher
    }

    pub fn add_listener(&mut self, listener_id: &String, listener_sender: Sender<Event>) {
        info!("{}: {}", t!("添加事件监听器"), listener_id);

        let mut listener_sender_map = self.listener_sender_map.write();
        listener_sender_map.insert(listener_id.to_owned(), listener_sender);
    }

    pub fn remove_listener(&mut self, listener_id: &String) {
        info!("{}: {}", t!("移除事件监听器"), listener_id);

        let mut listener_sender_map = self.listener_sender_map.write();
        listener_sender_map.remove(listener_id);
    }
}
