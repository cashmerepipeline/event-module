/*
Author: 闫刚 (yes7rose@sina.com)
events_map.rs (c) 2020
Desc: 事件映射表
Created:  2020-12-03T15:16:03.522Z
Modified: !date!
*/

use bson::{self, Document};
use parking_lot::RwLock;

use std::collections::HashMap;
use std::sync::Arc;
use futures::stream::StreamExt;


use manage_define::manage_ids::EVENTS_MANAGE_ID;

use database::MongodbResult;
use entity;


// {event_id:[target_queues]}
type EventsMap = HashMap<i64, Vec<i64>>;

/// 事件映射， 事件管理不通过总管
static mut EVENTS_MAP: Option<Arc<RwLock<EventsMap>>> = None;
pub async fn get_events_map() -> Arc<RwLock<EventsMap>> {
    unsafe {
        if EVENTS_MAP.is_some() {
            EVENTS_MAP.clone().unwrap()
        } else {
            EVENTS_MAP.get_or_insert(build_events_map().await);
            EVENTS_MAP.clone().unwrap()
        }
    }
}

// 创建事件映射表，从数据库
async fn build_events_map() -> Arc<RwLock<EventsMap>> {
    //1. 取得所有事件定义
    let events_collection = database::get_collection_by_id(&EVENTS_MANAGE_ID.to_string())
        .await
        .expect("取得事件集合错误");

    let cusor = events_collection
        .find(None, None)
        .await
        .expect("获取事件数据错误");

    //2. 构造
    let events: Vec<MongodbResult<Document>> = cusor.collect().await;
    let mut result: EventsMap = HashMap::new();

    for doc in events {
        let e_doc = doc.unwrap();
        let event_id: i64 = entity::get_entity_id(&e_doc).unwrap().parse().unwrap();

        let queues: Vec<i64> = match entity::get_entity_field(&e_doc, "queues")
        {
            Some(r) => r
                .as_array()
                .unwrap()
                .iter()
                .map(|x| {
                    let y: i64 = bson::from_bson(x.clone()).unwrap();
                    y
                })
                .collect(),
            None => vec![],
        };

        // 插入事件
       result.insert(event_id, queues);
    }
    Arc::new(RwLock::new(result))
}