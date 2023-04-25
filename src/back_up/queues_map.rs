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
use sled;

use cash_result::*;
use manage_define::manage_ids::EVENT_QUEUES_MANAGE_ID;

use crate::{Event, queue::EventQueue, queue::extract_queue_handle_ids};
use crate::queue::spawn_recieve_task;

// {queue_id:event_id:[handle_id...]}
pub type EventQueuesMap = HashMap<i64, EventQueue>;

/// 队列缓存表
static mut EVENT_QUEUES_MAP: Option<Arc<RwLock<EventQueuesMap>>> = None;

/// 取得事件队列映射表
pub async fn get_event_queues_map() -> Arc<RwLock<EventQueuesMap>> {
    unsafe {
        if EVENT_QUEUES_MAP.is_some() {
            EVENT_QUEUES_MAP.clone().unwrap()
        } else {
            EVENT_QUEUES_MAP.get_or_insert(init_event_queues_map().await);
            EVENT_QUEUES_MAP.clone().unwrap()
        }
    }
}

/// 从数据库读取事件队列并且建表
async fn init_event_queues_map() -> Arc<RwLock<EventQueuesMap>> {
    // 从数据库取得所有事件队列
    let event_queues_docs =
        match entity::get_entities(&EVENT_QUEUES_MANAGE_ID.to_string(), &None).await {
            Ok(r) => r,
            Err(_e) => panic!("初始化事件队列映射表失败")
        };

    // 收集所有事件队列
    let mut event_queues: Vec<(i64, String)> = vec![];
    for d in event_queues_docs.iter() {
        let (id, name) = (entity::get_entity_id(d).unwrap(), entity::get_entity_name(d).unwrap());
        event_queues.push((id.parse().unwrap(), name));
    }

    // 构造
    let mut result = EventQueuesMap::new();
    let configs = configs::get_configs();
    let events_dbs_root_dir = format!(
        "{}/{}",
        configs.server.root_dir, configs.server.events_dbs_dir
    );

    for d in event_queues_docs {
        let (sd, rc) = tokio::sync::mpsc::channel::<Event>(100);

        let id: i64 = entity::get_entity_id(&d).unwrap().parse().unwrap();

        let name = entity::get_entity_name(&d).unwrap();

        let handles = extract_queue_handle_ids(&d).unwrap();
        let mut handles_arc_map: HashMap<i64, Arc<RwLock<Vec<i64>>>> = HashMap::new();
        handles.iter().map(|(k, v)| {
            handles_arc_map.insert(*k, Arc::new(RwLock::new(v.clone())))
        });

        let handles_arc_map = Arc::new(RwLock::new(handles_arc_map));

        let db_path = format!("{}/{}.db", events_dbs_root_dir, name);
        let db = sled::open(&db_path).expect("打开事件数据库错误");

        let event_q = EventQueue {
            name,
            database: db,
            handles: handles_arc_map.clone(),
            sender: sd,
        };

        // 启动新接收任务
        spawn_recieve_task(id, rc, handles_arc_map.clone()).await;

        // 添加到映射表
        result.insert(id, event_q);
    }

    Arc::new(RwLock::new(result))
}


// 更新事件队列表
// TODO: 需要详细处理异常
pub async fn update_event_queues_map(new_doc: &Document) -> Result<OperationResult, OperationResult> {
    let id = match entity::get_entity_id(new_doc) {
        Some(r) => r,
        None => return Err(operation_failed("update_event_queues_map", "新实体没有指定id"))
    };
    let name = match entity::get_entity_name(new_doc) {
        Some(r) => r,
        None => return Err(operation_failed("update_event_queues_map", "新实体没有指定名字"))
    };
    let handles = extract_queue_handle_ids(new_doc).unwrap();
    let mut handles_arc_map: HashMap<i64, Arc<RwLock<Vec<i64>>>> = HashMap::new();
    handles.iter().map(|(k, v)| {
        handles_arc_map.insert(*k, Arc::new(RwLock::new(v.clone())))
    });

    let handles_arc_map = Arc::new(RwLock::new(handles_arc_map));

    let server_configs = configs::get_server_configs();
    let events_dbs_root_dir = format!(
        "{}/{}",
        server_configs.root_dir, server_configs.events_dbs_dir
    );

    let (sd, _rc) = tokio::sync::mpsc::channel::<Event>(100);

    let db_path = format!("{}/{}.db", events_dbs_root_dir, name);
    let db = sled::open(&db_path).expect("打开事件数据库错误");

    let event_q = EventQueue {
        name,
        database: db,
        handles: handles_arc_map,
        sender: sd,
    };

    // 添加到映射表
    {
        let queues_map_arc = get_event_queues_map().await;
        let mut queues_map = queues_map_arc.write();
        queues_map.insert(id.parse().unwrap(), event_q);
    }

    Ok(operation_succeed("ok"))
    // 启动新接收任务
    // spawn_recieve_task(id.parse().unwrap(), event_q.,rc).await
}


// /// 添加队列到映射
// pub async fn add_event_queue(
//     manage_id: &i32,
//     queue_name: &String,
// ) -> Result<OperationResult, OperationResult> {
//     if event_queue_exists_by_id(manage_id, queue_name).await {
//         return Err(target_already_exists("add_event_queue"));
//     }
//     // 取得全局映射
//     let event_q_map_lock = get_event_queues_map().await;
//     let mut queues_map = event_q_map_lock.write();
//
//     // 取得设置和数据库
//     let configs = configs::get_configs();
//     let events_dbs_root_dir = format!(
//         "{}/{}",
//         configs.server.root_dir, configs.server.events_dbs_dir
//     );
//     let (sd, rc) = tokio::sync::mpsc::channel::<Event>(100);
//     let db_path = format!("{}/{}.db", events_dbs_root_dir, queue_name);
//     let db = sled::open(&db_path).expect("打开事件数据库错误");
//
//     // 构造 添加
//     let event_q = EventQueue {
//         name: queue_name.clone(),
//         database: db,
//         handles: vec![],
//         sender: sd,
//         receiver: rc,
//     };
//     let mut value: HashMap<String, EventQueue> = HashMap::new();
//     value.insert(queue_name.clone(), event_q);
//     queues_map.insert(manage_id.clone(), value);
//
//     // TODO:  添加到管理数据库
//
//     Ok(operation_succeed("succeed"))
// }


/// 事件队列是否存在
async fn event_queue_exists_by_id(id: &i64) -> bool {
    let event_q_map_arc = get_event_queues_map().await;
    let queues_map_lock = event_q_map_arc.read();

    queues_map_lock.contains_key(id)
}

/// 取得管理事件映射队列 发射端
async fn get_queue_event_sender(
    id: &i64,
) -> Option<Sender<Event>> {
    if !event_queue_exists_by_id(id).await {
        return None;
    }
    let event_q_map_lock = get_event_queues_map().await;
    let queues_map = event_q_map_lock.read();
    queues_map.get(id).map(|r| r.sender.clone())
}

