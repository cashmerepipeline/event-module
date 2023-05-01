use bson::doc;
use cash_result::OperationResult;


use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;

use crate::dispatchers_map::{get_dispatchers_map, get_dispatcher};
use crate::{prototols::EventType, manage_ids::EVENT_TYPES_MANAGE_ID};

/// 初始化事件服务
pub async fn initialize_event_service() -> Result<(), OperationResult> {
    let major = majordomo::get_majordomo().await;
    let event_types_manager = major
        .get_manager_by_id(EVENT_TYPES_MANAGE_ID)
        .await
        .unwrap();

    let filter_doc = doc! { ENTITY_REMOVED_FIELD_ID.to_string(): false };
    let event_types: Vec<EventType> = match event_types_manager
        .get_entities_by_filter(&Some(filter_doc))
        .await
    {
        Ok(r) => r
            .iter()
            .map(|d| bson::from_document(d.to_owned()).unwrap())
            .collect::<Vec<EventType>>(),
        Err(_e) => {
            vec![]
        }
    };

    for event_type in event_types {
        // 初始化事件事件类型接收端和
        let dispachers_arc = get_dispatchers_map();
        let _dispatchers_map = dispachers_arc.write();

        match get_dispatcher(&event_type.type_id){
            Some(_r) => (),
            None => {
                log::error!(
                    "{}: {}-{}",
                    t!("初始化事件服务分发"),
                    event_type.type_id,
                    t!("失败")
                );
            }
        }
    }

    Ok(())
}
