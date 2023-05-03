use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::sync::Arc;

use super::{get_event_type, get_event_types_map};

pub type SerialNumber = u64;

static mut EVENT_SERIAL_NUMBER_MAP: Option<
    Arc<RwLock<BTreeMap<String, Arc<RwLock<SerialNumber>>>>>,
> = None;

pub fn get_event_serial_number_map() -> Arc<RwLock<BTreeMap<String, Arc<RwLock<SerialNumber>>>>> {
    unsafe {
        if EVENT_SERIAL_NUMBER_MAP.is_none() {
            EVENT_SERIAL_NUMBER_MAP = Some(Arc::new(RwLock::new(BTreeMap::new())));
        }

        EVENT_SERIAL_NUMBER_MAP.as_ref().unwrap().clone()
    }
}

pub async fn get_event_serial_number(type_id: &String) -> Option<SerialNumber> {
    // 检查事件类型是否存在
    {
        if let Some(_event_type) = get_event_type(type_id).await {
            // do nothing
        } else {
            return None;
        }
    }

    let map = get_event_serial_number_map();
    let mut map = map.write();

    let serial_number = match map.get(type_id) {
        Some(serial_number) => {
            let mut serial_number = serial_number.write();
            *serial_number += 1;
            serial_number.clone()
        }

        None => {
            // 初始化为 1
            let new_serial_number = Arc::new(RwLock::new(1));
            map.insert(type_id.to_string(), new_serial_number.clone());
            1u64
        }
    };

    Some(serial_number)
}
