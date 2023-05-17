use std::sync::Arc;

use log::warn;
use parking_lot::{lock_api::RwLock, RawRwLock};
use tokio::sync::Semaphore;

use crate::{
    event_inner_wrapper,
    event_services::get_event_runtime,
    event_type_listeners_map::{get_event_type_listener_map, get_event_type_listeners_map},
    listener_senders_map::get_listener_sender_map,
};

use super::event_type_queue_map::{DispatchQueue, get_event_type_dispatch_queue_map};

/// 启动转发线程，等待事件转发
pub fn make_disptch_queue_ready(semaphore: Arc<Semaphore>) {
    let rt = get_event_runtime();
    rt.spawn(async move {
        loop {
            // 最多同时启动 max_disptch_thread_num 个转发线程
            let _ = semaphore.acquire().await.unwrap();

            {
                let event_type_dispatch_queue_map_arc = get_event_type_dispatch_queue_map();
                let event_type_dispatch_queue_map = event_type_dispatch_queue_map_arc.read();

                for item in event_type_dispatch_queue_map.iter() {
                    let type_id = item.0.to_owned();
                    let disptch_queue = item.1.to_owned();

                    let is_empty = {
                        let disptch_queue = disptch_queue.read();
                        disptch_queue.is_empty()
                    };
                    if is_empty {
                        continue;
                    }

                    // 每个事件类型启动一个线程
                    super::dispatch(&type_id, disptch_queue.clone());
                }
            }
            // 完成一次循环后，等待一段时间，响应可能拆入新的事件类型
            // tokio::time::sleep(tokio::time::Duration::from_micros(10)).await;
        }
    });
}
