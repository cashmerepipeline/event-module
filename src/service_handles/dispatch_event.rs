use dependencies_sync::log::debug;

use crate::{event_inner_wrapper::EventInnerWrapper, type_dispatcher_map::get_dispatcher};

// 发送事件
pub async fn dispatch_event(event_wrapper: EventInnerWrapper) -> Result<(), String> {
    let event = event_wrapper.event.clone();

    let dispatcher_arc = match get_dispatcher(&event.type_id) {
        Some(r) => r,
        None => return Err(format!("{}: {}", t!("获取转发器失败 "), event.type_id)),
    };

    let dispatch_sender = {
        let dispatcher = dispatcher_arc;
        dispatcher.dispatch_sender.clone()
    };

    // 发送
    debug!("{}, {}", t!("开始发送事件"), event.serial_number);
    if let Err(e) = dispatch_sender.send(event_wrapper).await {
        return Err(format!("{}: {}", t!("发送事件失败 "), e));
    };
    debug!("{}, {}", t!("完成发送事件"), event.serial_number);

    Ok(())
}
