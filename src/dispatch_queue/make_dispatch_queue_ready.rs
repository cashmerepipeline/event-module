use std::sync::Arc;

use dependencies_sync::tokio::sync::Semaphore;
use dependencies_sync::tokio;

use crate::event_services::get_event_runtime;

use super::event_type_queue_map::get_event_type_dispatch_queue_map;

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
                
                // 每个事件类型启动一个线程
                for (type_id, dispatch_queue) in event_type_dispatch_queue_map.iter() {
                    let type_id = type_id.to_owned();
                    let disptch_queue = dispatch_queue.clone();

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

            // 完成一次循环后，等待一段时间, 避免占用过多CPU
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    });
}
