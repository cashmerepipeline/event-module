use dependencies_sync::log::{self, info};
use dependencies_sync::rust_i18n::{self, t};

use dependencies_sync::tokio::sync::mpsc::{channel, Sender};
use server_utils::get_shutdown_cancellation_token;

use crate::event_inner_wrapper::EventInnerWrapper;
use crate::event_services::get_event_runtime;
use crate::event_type_listeners_map::add_event_type_listener;
use crate::listener_instances_map::add_listener_sender;
// use crate::type_listeners_map::{
//     add_listener_sender, get_type_listener_senders_map, get_type_listeners_map,
//     get_type_listeners_senders_map, remove_listener_sender,
// };

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

        let (dispatch_sender, mut dispatch_reciever) = channel::<EventInnerWrapper>(512);

        let new_dispatcher = EventDispatcher {
            type_id: type_id.clone(),
            dispatch_sender,
        };

        // 事件转发线程
        let rt = get_event_runtime();
        let shutdown_cancelation = get_shutdown_cancellation_token();

        rt.spawn(async move {
            while let Some(event_wrapper) = dispatch_reciever.recv().await {
                #[cfg(feature = "use_queue_dispatch")]
                {
                    use crate::dispatch_queue::add_event_echo_wrapper_to_queue;
                    add_event_echo_wrapper_to_queue(event_wrapper);
                }

                #[cfg(feature = "use_channel_dispatch")]
                {
                    use crate::dispatch_channels::dispatch_to_channel;
                    dispatch_to_channel(event_wrapper).await;
                }

                if shutdown_cancelation.is_cancelled() {
                    log::warn!(
                        "{}: {}",
                        t!("开始退出分发循环，所有未发送的事件将取消发送"),
                        type_id
                    );
                    break;
                }
            }

            info!("{}: {}", t!("事件发送器关闭"), type_id);
        });

        new_dispatcher
    }

    pub fn add_listener_sender(
        &self,
        listener_id: &String,
        instance_name: String,
        listener_sender: Sender<EventInnerWrapper>,
    ) {
        let type_id = self.type_id.clone();
        info!("{}: {}", t!("添加事件监听器"), listener_id);
        add_event_type_listener(type_id, listener_id.clone());
        add_listener_sender(listener_id, instance_name, listener_sender);
    }
}
