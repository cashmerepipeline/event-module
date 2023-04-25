use crate::event_queue_map::get_events_queue_map;
use crate::types::EventDeQue;

use super::event::Event;

/// 事件队列
pub struct EventQueue {
    type_id: u32,
    // sender: Sender<Event>,
    queue: EventDeQue,
}

impl EventQueue {
    pub fn push_back(&self, event: Event) {
        let mut q = self.queue.write();
        q.push_back(event)
    }
}

