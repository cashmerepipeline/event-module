/*
Author: 闫刚 (yes7rose@sina.com)
queue.rs (c) 2020
Desc: 队列数据结构
Created:  2020-12-04T02:01:31.527Z
Modified: !date!
*/

use bson::Document;
use std::collections::HashMap;
use tokio::{self, sync::mpsc::Receiver, sync::mpsc::Sender};

use crate::handles_map::get_handle_sender_by_id;
use crate::queues_map::get_event_queues_map;
use crate::Event;
use cash_result::*;

use parking_lot::RwLock;
use std::sync::Arc;

type QueueHandlesMap = Arc<RwLock<HashMap<i64, Arc<RwLock<Vec<i64>>>>>>;

/// 事件队列
pub struct EventQueue {
    pub name: String,
    pub database: sled::Db,
    // event_id:[handle_id...]}
    pub handles: QueueHandlesMap,
    pub sender: Sender<Event>,
}

impl EventQueue {
    pub fn get_sender(&self) -> Sender<Event> {
        self.sender.clone()
    }
}

/// 取得队列的事件处理器列表, 储存形式{queue_id:event_id:[handle_id...]}
pub fn extract_queue_handle_ids(queue_doc: &Document) -> Option<HashMap<i64, Vec<i64>>> {
    let handles_field_id = 1008; // TODO: 使用属性名字？
                                 // 事件处理器id使用整数作为编号
    let _id = entity::get_entity_id(queue_doc);
    let b = entity::get_entity_field(queue_doc, &handles_field_id.to_string()).unwrap();
    // 直接转换??
    // event_id:[handle_id...]
    let result: HashMap<i64, Vec<i64>> = bson::from_bson(b).unwrap();
    Some(result)
}

/// 取得队列的处理器列表
pub async fn get_queue_handles(queue_id: &i64) -> QueueHandlesMap {
    let queues_map_arc = get_event_queues_map().await;
    let queues_map_lock = queues_map_arc.read();
    let queue = queues_map_lock.get(queue_id).unwrap();
    queue.handles.clone()
}

/// 启动接收端
pub async fn spawn_recieve_task(
    _id: i64,
    _receiver: Receiver<Event>,
    _handles_map: QueueHandlesMap,
) -> Result<OperationResult, OperationResult> {
    // let handle = get_runtime_handle();
    // // let queue_handles_map = get_queue_handles(&id).await;

    // let result = handle
    //     .spawn(async move {
    //         while let Some(event) = receiver.recv().await {
    //             println!("got = {:?}", event);

    //             let event_id: i64 = event.event_id.clone().parse().unwrap();
    //             // 取得事件处理列表
    //             let mut target_handles: Vec<i64> = Vec::new();
    //             {
    //                 let handles_map_lock = handles_map.read();
    //                 let target_handles_arc = handles_map_lock.get(&event_id).unwrap();
    //                 let target_handles_lock = target_handles_arc.read();
    //                 target_handles = target_handles_lock.clone();
    //             }
    //             // 执行所有处理
    //             for handle_id in target_handles.iter() {
    //                 send_event_to_handle(event.clone(), *handle_id).await;
    //             }
    //         }
    //     })
    //     .await;

    // if result.is_ok() {
        Ok(operation_succeed("ok"))
    // } else {
    //     Err(operation_failed(
    //         "spawn_start_recieve",
    //         "启动事件队列线程失败",
    //     ))
    // }
}

/// 发送事件到处理器
async fn send_event_to_handle(event: Event, handle_id: i64) -> Result<(), OperationResult> {
    let sender = match get_handle_sender_by_id(handle_id).await {
        Ok(s) => s,
        Err(e) => {
            return Err(add_call_name_to_chain(
                e,
                "send_event_to_handle".to_string(),
            ))
        }
    };

    sender.send(event);
    Ok(())
}
