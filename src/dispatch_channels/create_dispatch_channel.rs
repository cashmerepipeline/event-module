use dependencies_sync::{tokio, log::{self, info}};

use crate::{event_services::get_event_runtime, dispatch_event::dispatch_event};

use super::{
    event_type_dispatch_sender_map::{
        get_event_type_dispatch_sender_map, insert_event_type_dispatch_sender,
    },
};

pub fn create_dispatch_channel(
    event_type_id: &String,
    channel_buffer_size: usize,
) {
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

    insert_event_type_dispatch_sender(&event_type_id, sender.clone());

    let event_type_id = event_type_id.clone();
    let rt = get_event_runtime();
    rt.spawn(async move {
        info!("{}: {}", t!("开始监听事件"), event_type_id);
        while let Some(event) = receiver.recv().await {
            dispatch_event(event_type_id.clone(), event);
        }
    });
}
