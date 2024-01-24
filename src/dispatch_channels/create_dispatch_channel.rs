use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::{
    log::{self, info},
    tokio,
};
use server_utils::get_shutdown_cancellation_token;

use crate::{dispatch_event::dispatch_event, event_services::get_event_runtime};

use super::event_type_dispatch_sender_map::{
    get_event_type_dispatch_sender_map, insert_event_type_dispatch_sender,
};

pub fn create_dispatch_channel(event_type_id: &String, channel_buffer_size: usize) {
    log::info!("{}: {}", t!("创建发送通道"), event_type_id);

    {
        let event_type_dispatch_sender_map_arc = get_event_type_dispatch_sender_map();
        let event_type_dispatch_sender_map = event_type_dispatch_sender_map_arc.read();
        if event_type_dispatch_sender_map.contains_key(event_type_id) {
            log::warn!("{}: {}", t!("事件类型已存在"), event_type_id);
            return;
        }
    }

    let (sender, mut receiver) = tokio::sync::mpsc::channel(channel_buffer_size);

    insert_event_type_dispatch_sender(event_type_id, sender.clone());

    let event_type_id = event_type_id.clone();

    let rt = get_event_runtime();
    let shutdow_cancelation = get_shutdown_cancellation_token();

    rt.spawn(async move {
        info!("{}: {}", t!("开始监听事件"), event_type_id);
        while let Some(event) = receiver.recv().await {
            dispatch_event(event_type_id.clone(), event);
            if shutdow_cancelation.is_cancelled() {
                log::warn!("{}: {}", t!("监听事件通道开始退出"), event_type_id);
                break;
            }
        }
        log::info!("{}: {}", t!("监听事件通道退出"), event_type_id);
    });
}
