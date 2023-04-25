use crate::event_message::event_message_field_data::EventMessageFieldData;
use crate::event_message::event_message_field_data_type::EventMessageFieldDataType;

/// 信息
#[derive(Debug, Clone, PartialEq)]
pub struct EventMessageField{
    name:String,
    value: EventMessageFieldData,
}
