/*
Author: 闫刚 (yes7rose@sina.com)
handle.rs (c) 2020
Desc: 事件处理器
Created:  2020-11-08T01:34:20.457Z
Modified: !date!
*/

use tokio::{sync::mpsc::Receiver, sync::mpsc::Sender};

use crate::event::Event;
use cash_result::*;

/// 事件处理器
pub struct EventHandle {
    pub name: String,
    pub event_id: i64,
    pub sender: Sender<Event>,
}

pub const EVENT_ID_FIELD_ID: i64 = 1008;

/// 启动接收端
pub async fn spawn_recieve_task(_id: i64, _receiver: Receiver<Event>) -> Result<OperationResult, OperationResult> {
    // let handle = get_runtime_handle();
    // let result = handle.spawn(async move {
    //     while let Some(event) = receiver.recv().await {
    //         println!("got = {:?}", event);

    //         // 执行处理
    //     }
    // }).await;

    // if result.is_ok() {
        Ok(operation_succeed("ok"))
    // } else {
    //     Err(operation_failed("spawn_start_recieve", "启动事件队列线程失败"))
    // }
}