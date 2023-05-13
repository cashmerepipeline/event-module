use bson::doc;
use cash_result::OperationResult;
use futures::future;
use std::thread;

use futures::FutureExt;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;

use crate::type_dispatcher_map::{get_dispatcher, get_dispatchers_map};
use crate::{manage_ids::EVENT_TYPES_MANAGE_ID, protocols::EventType};

/// 初始化事件服务
pub async fn initialize_event_service() -> Result<(), OperationResult> {
    // let major = majordomo::get_majordomo().await;
    // let event_types_manager = major
    //     .get_manager_by_id(EVENT_TYPES_MANAGE_ID)
    //     .await
    //     .unwrap();

    // let filter_doc = doc! { ENTITY_REMOVED_FIELD_ID.to_string(): false };
    // let type_ids: Vec<String> = match event_types_manager
    //     .get_entities_by_filter(&Some(filter_doc))
    //     .await
    // {
    //     Ok(r) => r
    //         .iter()
    //         .map(|d| d.get_str(ID_FIELD_ID.to_string()).unwrap().to_string())
    //         .collect(),
    //     Err(_e) => {
    //         vec![]
    //     }
    // };

    // for type_id in type_ids {
    //     // 初始化事件事件类型接收端和
    //     let dispachers_arc = get_dispatchers_map();
    //     let _dispatchers_map = dispachers_arc.write();

    //     match get_dispatcher(&type_id) {
    //         Some(_r) => (),
    //         None => {
    //             log::error!("{}: {}-{}", t!("初始化事件服务分发"), type_id, t!("失败"));
    //         }
    //     }
    // }

    // thread::spawn(|| {
    //     let event_localset = get_dispatcher_localset();
    //     let lset = event_localset.read();
    //     lset.run_until(future::pending::<()>());
    // })
    // .join();

    Ok(())
}
