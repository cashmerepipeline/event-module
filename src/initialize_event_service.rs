use std::sync::Arc;
use std::thread;

use bson::doc;
use cash_result::OperationResult;
use futures::future;
use futures::FutureExt;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use tokio::sync::Semaphore;

use crate::event_service_configs::EventServiceConfigs;
use crate::{manage_ids::EVENT_TYPES_MANAGE_ID, protocols::EventType};
use crate::dispatch_queue::make_disptch_queue_ready;
use crate::type_dispatcher_map::{get_dispatcher, get_dispatchers_map};

/// 初始化事件服务
pub async fn initialize_event_service(configs: EventServiceConfigs) -> Result<(), OperationResult> {
    // 最多同时并发队列
    let max_concurrent_queue = configs.max_concurrent_queue;

    let semaphore = Arc::new(Semaphore::new(max_concurrent_queue as usize));

    make_disptch_queue_ready(semaphore.clone());

    Ok(())
}
