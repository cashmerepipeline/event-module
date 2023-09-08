use cash_result::OperationResult;

use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::log::info;

use crate::event_service_configs::EventServiceConfigs;
use crate::event_types_map::init_event_types_map;
use crate::init_event_service_configs;

/// 初始化事件服务
pub async fn initialize_event_service(configs: EventServiceConfigs) -> Result<(), OperationResult> {
    info!("{}", t!("初始化事件服务"));

    init_event_service_configs(configs.clone());

    // 最多同时并发队列
    let _max_envent_queue_size = configs.max_event_type_count;

    init_event_types_map().await?;

    #[cfg(feature = "use_queue_dispatch")]
    {
        use crate::dispatch_queue::make_disptch_queue_ready;
        use dependencies_sync::tokio::sync::Semaphore;
        use std::sync::Arc;

        let max_concurrent_queue = configs.max_concurrent_queue;

        info!("{}", t!("初始化事件服务，使用队列分发"));
        let semaphore = Arc::new(Semaphore::new(max_concurrent_queue as usize));
        make_disptch_queue_ready(semaphore);
    }

    // #[cfg(feature = "use_channel_dispatch")]
    // {
    //     info!("{}", t!("初始化事件服务，使用通道分发"));
    //     make_dispatch_channels_ready(max_envent_queue_size as usize).await?;
    // }

    info!("{}", t!("事件服务初始化完成"));

    Ok(())
}
