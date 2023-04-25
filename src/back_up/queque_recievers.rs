use tokio::{stream::StreamExt, sync::mpsc::Receiver, sync::mpsc::Sender};
use crate::event::Event;
use cash_result::*;
use runtime_handle::get_runtime_handle;
use crate::queue::get_queue_handles;
use crate::queues_map::get_event_queues_map;
use std::sync::Arc;
use std::borrow::BorrowMut;

/// 队列事件接收器列表
static mut EVENTS_QUEUE_RECEIVERS: Option<Arc<Vec<(i64, Receiver<Event>)>>> = None;

pub fn push_reciever(id: i64, new_receiver: Receiver<Event>) -> Result<(), ()> {
    unsafe {
        // 空表新建
        if EVENTS_QUEUE_RECEIVERS.is_none() {
            let mut receivers: Vec<(i64, Receiver<Event>)> = Vec::new();
            receivers.push((id, new_receiver));
            EVENTS_QUEUE_RECEIVERS.replace(Arc::new(receivers);
        } else {
            let mut receivers = EVENTS_QUEUE_RECEIVERS.unwrap();
            receivers.push((id, new_receiver));
            EVENTS_QUEUE_RECEIVERS.replace(receivers);
        }

        Ok(())
    }
}

pub async fn spawn_receivers_tasks() -> Result<(), ()> {
    let handle = get_runtime_handle();
    unsafe {
        let mut receivers = EVENTS_QUEUE_RECEIVERS.clone().unwrap();

        for i in 0..receivers.len(){
            let (id, mut receiver) = receivers.pop().unwrap();
            handle.spawn(async move {
                while let Some(event) = receiver.recv().await {
                    println!("got = {:?}", event);
                    // 取得事件处理列表,
                    let queue_handles_map = get_queue_handles(&id).await;
                    // 执行所有处理
                    let event_id: i64 = event.event_id.clone().parse().unwrap();
                    let target_handles = queue_handles_map.get(&event_id).unwrap();
                    for handle_id in target_handles {
                        send_event_to_handle(event.clone(), *handle_id);
                    }
                }
            }).await;
        }

        // for (id, mut receiver) in receivers {
        //     handle.spawn(async move {
        //         while let Some(event) = receiver.recv().await {
        //             println!("got = {:?}", event);
        //             // 取得事件处理列表,
        //             let queue_handles_map = get_queue_handles(&id).await;
        //             // 执行所有处理
        //             let event_id: i64 = event.event_id.clone().parse().unwrap();
        //             let target_handles = queue_handles_map.get(&event_id).unwrap();
        //             for handle_id in target_handles {
        //                 send_event_to_handle(event.clone(), *handle_id);
        //             }
        //         }
        //     }).await;
        // }
    }

    Ok(())

    // if result.is_ok() {
    //     Ok(operation_succeed("ok"))
    // } else {
    //     Err(operation_failed("spawn_start_recieve", "启动事件队列线程失败"))
    // }
}

