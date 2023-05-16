use crate::event_inner_wrapper::EventInnerWrapper;

use super::event_type_queue_map::get_event_type_dispatch_queue;

/// 将事件包装器添加到队列中
pub fn add_event_echo_wrapper_to_queue(event_echo_wrapper: EventInnerWrapper) {
    let event_type_id = event_echo_wrapper.event.type_id.clone();

    let event_type_dispatch_queue_arc = get_event_type_dispatch_queue(&event_type_id);
    let mut event_type_dispatch_queue = event_type_dispatch_queue_arc.write();

    event_type_dispatch_queue.push_back(event_echo_wrapper);
}
