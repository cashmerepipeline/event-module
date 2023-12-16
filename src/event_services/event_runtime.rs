use std::sync::Arc;

use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::{tokio, log};
use dependencies_sync::tokio::sync::oneshot;
use dependencies_sync::tokio::runtime::Runtime;

static mut EVENT_RUNTIME: Option<Arc<Runtime>> = None;

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

pub fn build_event_runtime() -> Arc<Runtime> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let rt_arc = Arc::new(rt);
    let result = rt_arc.clone();
    
    let (tx, shutdown_rx) = oneshot::channel();
    let _sig = tokio::spawn(server_utils::wait_for_terminat_signal(tx));

    // 在新线程中启动一个新的tokio运行时
    std::thread::spawn(move || {
        rt_arc.block_on(async {
            shutdown_rx.await.unwrap();
            log::info!(t!("事件运行时开始关闭..."));
        });
    });

    result
}
