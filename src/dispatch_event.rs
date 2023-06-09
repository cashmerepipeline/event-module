use dependencies_sync::{log::{warn, debug}, tokio};

use crate::{
    event_inner_wrapper::EventInnerWrapper, event_type_listeners_map::get_event_type_listener_map,
    listener_senders_map::{get_listener_sender_map, remove_listener_senders}, event_services::get_event_runtime,
};

pub fn dispatch_event(event_type_id: String, event_echo_wrapper: EventInnerWrapper) {
    debug!("{}: {}", t!("开始转发事件"), event_type_id);

    let event_listener_map_arc = get_event_type_listener_map(&event_type_id);

    // 取得事件类型的监听者
    let is_empty = {
        let event_listener_map = event_listener_map_arc.read();
        event_listener_map.is_empty()
    };
    // 没有监听者，不需要转发
    if is_empty {
        warn!("{}: {}", t!("事件类型没有监听者"), event_type_id);
        return;
    }

    let event_listener_map = event_listener_map_arc.read();
    // 对每个监听者，取得监听者发送器表，遍历表中的发送器，发送事件
    for (listner_index, listener_id) in event_listener_map.iter() {
        let listener_sender_map_arc = get_listener_sender_map(listener_id.as_ref());

        // 记录已经失效的事件发送器
        let mut invalid_sender_indexes = vec![];

        {
            // 发送
            let listener_sender_map = listener_sender_map_arc.read();

            for (sender_index, sender) in listener_sender_map.iter() {
                let sender_index = sender_index.to_owned();
                let sender = sender.to_owned();

                // 事件发送器不存在，不需要转发
                if sender.is_none() {
                    // 记录移除已经不存在的事件发送器
                    invalid_sender_indexes.push(sender_index);
                    continue;
                }

                // 事件发送器已经关闭，不需要转发
                let sender = sender.unwrap();
                if sender.is_closed() {
                    // 移除已经关闭的事件发送器
                    invalid_sender_indexes.push(sender_index);
                    continue;
                }

                let type_id = event_type_id.clone();
                let _listener_id = listener_id.clone();
                let _listner_index = *listner_index;

                let event_echo_wrapper = event_echo_wrapper.clone();

                let rt = get_event_runtime();
                rt.spawn(async move {
                    let max_retry_times = 5;
                    let mut retry_times = 1;
                    loop {
                        let result = sender.clone().send(event_echo_wrapper.clone()).await;
                        if result.is_ok() {
                            break;
                        } else if retry_times > max_retry_times {
                            warn!(
                                "{}, {}: {}",
                                t!("发送事件失败"),
                                t!("超出重试次数"),
                                type_id
                            );
                            break;
                        } else {
                            // 发送失败，等待一段时间后重试
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                            retry_times += 1;
                        }
                    }
                });
            }
        }
        {
            // 移除已经失效的事件发送器
            remove_listener_senders(listener_id, invalid_sender_indexes);
        }
    }
}
