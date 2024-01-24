use std::sync::{Arc, OnceLock};

use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio::runtime::Runtime;
use dependencies_sync::tokio::select;
use dependencies_sync::tokio::sync::oneshot;
use dependencies_sync::{log, tokio};

static mut EVENT_RUNTIME: Option<Arc<Runtime>> = None;
static mut EVENT_RUNTIME_SHUTDOWNED: OnceLock<bool> = OnceLock::new();

pub fn get_event_runtime() -> Arc<Runtime> {
    unsafe {
        if EVENT_RUNTIME.is_some() {
            EVENT_RUNTIME.clone().unwrap()
        } else {
            EVENT_RUNTIME.get_or_insert(build_event_runtime());
            EVENT_RUNTIME.clone().unwrap()
        }
    }
}

pub fn is_event_runtime_shutdowned() -> bool {
    unsafe { *EVENT_RUNTIME_SHUTDOWNED.get_or_init(|| false) }
}

pub fn build_event_runtime() -> Arc<Runtime> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let rt_arc = Arc::new(rt);
    let result = rt_arc.clone();

    let (tx, _shutdown_rx) = oneshot::channel();
    let _sig = tokio::spawn(server_utils::wait_for_terminat_signal(tx));
    let cancel_token = server_utils::get_shutdown_cancellation_token();

    // 在新线程中启动一个新的tokio运行时
    std::thread::spawn(move || {
        rt_arc.block_on(async {
            log::info!("{}", t!("事件运行时启动"));
            select! {
                _ = cancel_token.cancelled() => {
                    log::info!("{}", t!("事件运行时开始退出..."));
                    // unsafe { EVENT_RUNTIME_SHUTDOWNED.set(true) };
                },
            }
            // shutdown_rx.await.unwrap();
            // unsafe {
            // let _ = EVENT_RUNTIME_SHUTDOWNED.set(false);
            // };

            log::info!("{}", t!("事件运行时开始退出"));
        });
    });

    result
}
