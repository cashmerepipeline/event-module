use crate::event::Event;
use crate::event_queue_map::get_event_queue;

/// 发送事件
pub async fn push_event(event: Event){
    // 1. 取得事件队列
    let event_que_arc = get_event_queue(&event.type_uid);
    let mut event_queue = event_que_arc.write();
    
    // 2. 将事件添加到事件队列中
    event_queue.push_back(event)
}
