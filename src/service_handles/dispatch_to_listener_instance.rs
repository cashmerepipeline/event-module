use cash_result::{OperationResult, operation_failed};
use dependencies_sync::log::{info, error};

use crate::{event_inner_wrapper::EventInnerWrapper, listener_instances_map::get_listener_instance_sender};

/// 不经过队列直接分发事件
pub async fn dispatch_to_listener_instance(
    event_wrapper: EventInnerWrapper,
    listener_id: &String,
    sender_index: u32,
) -> Result<(), OperationResult> {
    if let Some(listener_sender) = get_listener_instance_sender(listener_id, sender_index) {
        if let Ok(_r) = listener_sender.send(event_wrapper).await {
            error!("{}: {}", t!("事件分发成功"), listener_id);
            Ok(())
        } else {
            error!("{}: {}", t!("事件分发失败"), listener_id);
            Err(operation_failed(
                "dispatch_to_listener_instance",
                t!("事件分发失败, 发送失败"),
            ))
        }
    } else {
        info!("{}: {}", t!("取得发送器失败"), listener_id);
        Err(operation_failed(
            "dispatch_to_listener_instance",
            t!("事件分发失败, 取得发送器失败"),
        ))
    }
}

