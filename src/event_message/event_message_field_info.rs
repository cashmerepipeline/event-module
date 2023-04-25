use crate::event_message::event_message_field_data_type::EventMessageFieldDataType;

/// 消息描述信息
#[derive(Debug, Clone)]
pub struct EventMessageFieldInfo {
    data_type: EventMessageFieldDataType,
    description: String,
}