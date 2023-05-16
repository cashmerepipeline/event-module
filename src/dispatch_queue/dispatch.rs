use std::sync::Arc;
use parking_lot::lock_api::RwLock;
use parking_lot::RawRwLock;
use log::warn;
use crate::dispatch_queue::event_type_queue_map::DispatchQueue;
use crate::event_services::get_event_runtime;
use crate::event_type_listeners_map::get_event_type_listener_map;
use crate::listener_senders_map::get_listener_sender_map;

pub fn dispatch(
    type_id: &String,
    dispatch_queue_arc: Arc<RwLock<RawRwLock, DispatchQueue>>,
) {
    let dispatch_queue = dispatch_queue_arc.read();
    for event_echo_wrapper in dispatch_queue.iter() {
        let event_echo_wrapper = event_echo_wrapper.to_owned();

        // 取得事件类型的监听者
        let event_listener_map_arc = get_event_type_listener_map(type_id);
        let event_listener_map = event_listener_map_arc.read();

        // 没有监听者，不需要转发
        if event_listener_map.is_empty() {
            warn!("{}: {}", t!("事件类型没有监听者"), type_id);
            break;
        }

        // 对每个监听者，取得监听者发送器表，遍历表中的发送器，发送事件
        for (listner_index, listener_id) in event_listener_map.iter() {
            let listener_sender_map_arc = get_listener_sender_map(listener_id.as_ref());
            let listener_sender_map = listener_sender_map_arc.read();

            for item in listener_sender_map.iter() {
                let sender_index = item.0.to_owned();
                let sender = item.1.to_owned();

                let type_id = type_id.clone();
                let listener_id = listener_id.clone();
                let listner_index = *listner_index;

                let event_echo_wrapper = event_echo_wrapper.clone();

                let rt = get_event_runtime();
                rt.spawn(async move {
                    let max_retry_times = 5;
                    let mut retry_times = 1;
                    loop {
                        let result = sender.clone().send(event_echo_wrapper.clone()).await;
                        if result.is_ok() {
                            break;
                        } else {
                            if retry_times > max_retry_times {
                                warn!("{}: {}", t!("发送事件失败"), type_id);
                                break;
                            } else {
                                retry_times += 1;
                            }
                        }
                    }
                });
            }
        }
    }
}
