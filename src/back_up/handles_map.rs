/*
Author: 闫刚 (yes7rose@sina.com)
queue_map (c) 2020
Desc: 事件队列
Created:  2020-11-08T03:18:08.151Z
Modified: !date!
*/

use std::collections::HashMap;
use std::sync::Arc;

use bson::{self, Document};
use parking_lot::RwLock;

use tokio::{sync::mpsc::Sender};


use cash_result::*;
use  manage_define::manage_ids::EVENT_HANDLES_MANAGE_ID;

use crate::{Event};
use crate::handle::{EventHandle, EVENT_ID_FIELD_ID, spawn_recieve_task};

// {queue_id:event_id:[handle_id...]}
pub type EventHandlesMap = HashMap<i64, EventHandle>;

/// 队列缓存表
static mut EVENT_HANDLES_MAP: Option<Arc<RwLock<EventHandlesMap>>> = None;

/// 取得事件队列映射表
pub async fn get_event_handles_map() -> Arc<RwLock<EventHandlesMap>> {
    unsafe {
        if EVENT_HANDLES_MAP.is_some() {
            EVENT_HANDLES_MAP.clone().unwrap()
        } else {
            EVENT_HANDLES_MAP.get_or_insert(init_event_handle_map().await);
            EVENT_HANDLES_MAP.clone().unwrap()
        }
    }
}

/// 从数据库读取事件队列并且建表
async fn init_event_handle_map() -> Arc<RwLock<EventHandlesMap>> {
    // 从数据库取得所有事件队列
    let event_handles_docs =
        match entity::get_entities(&EVENT_HANDLES_MANAGE_ID.to_string(), &None).await {
            Ok(r) => r,
            Err(_e) => panic!("初始化事件队列映射表失败")
        };

    // 收集所有事件队列
    let mut event_handles: Vec<(i64, String)> = vec![];
    for d in event_handles_docs.iter() {
        let (id, name) = (entity::get_entity_id(d).unwrap(), entity::get_entity_name(d).unwrap());
        event_handles.push((id.parse().unwrap(), name));
    }

    // 构造
    let mut result = EventHandlesMap::new();

    for d in event_handles_docs {
        let (sd, rc) = tokio::sync::mpsc::channel::<Event>(100);

        let id: i64 = entity::get_entity_id(&d).unwrap().parse().unwrap();

        let name = entity::get_entity_name(&d).unwrap();
        let event_id = extract_event_id(&d).unwrap();

        let event_q = EventHandle {
            name,
            event_id,
            sender: sd,
        };

        // 启动新接收任务
        spawn_recieve_task(id, rc).await;

        // 添加到映射表
        result.insert(id, event_q);
    }

    Arc::new(RwLock::new(result))
}


/// 取得事件处理器的事件发射端
pub async fn get_handle_sender_by_id(handle_id: i64) -> Result<Sender<Event>, OperationResult> {
    let handles_map_arc = get_event_handles_map().await;
    let handles_map_lock = handles_map_arc.read();

    match handles_map_lock.get(&handle_id) {
        Some(r) => Ok(r.sender.clone()),
        None => Err(operation_failed("get_handle_sender_by_id", "取得处理器事件Sender失败"))
    }
}

//  从数据取得事件编号
fn extract_event_id(doc: &Document) -> Option<i64> {
    match entity::get_entity_field(doc, EVENT_ID_FIELD_ID.to_string()) {
        Some(r) => r.as_i64(),
        None => None
    }
}