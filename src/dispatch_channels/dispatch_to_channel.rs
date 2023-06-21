use dependencies_sync::log::{error, debug};
use dependencies_sync::rust_i18n::{self, t};

use crate::event_inner_wrapper::EventInnerWrapper;

use super::event_type_dispatch_sender_map::get_event_type_dispatch_sender;

pub async fn dispatch_to_channel(event_wrapper: EventInnerWrapper) {
    let event_type_id = event_wrapper.event.type_id.clone();

    if let Some(type_dispatch_sender) = get_event_type_dispatch_sender(&event_type_id) {
        debug!("{}: {}", t!("通过管道发送事件"), event_wrapper.event.serial_number);
        let _ = type_dispatch_sender.send(event_wrapper).await;
    } else {
        error!("{}: {}", t!("事件类型转发通道不存在"), event_type_id);
    }
}
