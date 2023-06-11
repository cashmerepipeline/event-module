use cash_result::OperationResult;


use crate::{event_types_map::get_event_types};

use super::{
    create_dispatch_channel,
};

pub async fn make_dispatch_channels_ready(
    channel_buffer_size: usize,
) -> Result<(), OperationResult> {
    let event_types = get_event_types();
    let event_type_ids: Vec<String> = event_types
        .iter()
        .map(|event_type| event_type.type_id.clone())
        .collect();

    for event_type_id in event_type_ids {
        create_dispatch_channel(&event_type_id, channel_buffer_size);
    }

    Ok(())
}
