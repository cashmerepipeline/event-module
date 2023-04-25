use crate::event_message::EventMessageField;

use super::event_echo_type::EventEchoType;

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    // 事件类型
    pub type_id: u32,
    // 发射者
    pub emitter_id: u64,
    pub serial_number: u64,
    // 发射时间
    pub timestamp: u64,
    pub context: Bson,
}
