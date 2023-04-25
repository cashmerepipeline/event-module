use std::collections::BTreeMap;

use crate::event_message::EventMessageFieldDataType;
use crate::event_message::EventMessageFieldInfo;
use crate::event_type::EventType;
use crate::event_types_map;
use crate::event_types_map::get_event_types_map;

/// 注册事件类型
/// 量不大，不需要做流
pub fn register_event_type(
    type_name: String,
    schema: BTreeMap<String, EventMessageFieldInfo>,
    description: String
) -> String {
    event_types_map::register_event_type(type_name, schema, description)
}
