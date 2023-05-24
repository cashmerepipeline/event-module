use std::sync::Arc;



use cash_result::OperationResult;




use dependencies_sync::tokio::sync::Semaphore;

use crate::dispatch_queue::make_disptch_queue_ready;
use crate::event_service_configs::EventServiceConfigs;



/// 初始化事件服务
pub async fn initialize_event_service(configs: EventServiceConfigs) -> Result<(), OperationResult> {
    // 最多同时并发队列
    let max_concurrent_queue = configs.max_concurrent_queue;

    let semaphore = Arc::new(Semaphore::new(max_concurrent_queue as usize));

    make_disptch_queue_ready(semaphore);

    Ok(())
}
