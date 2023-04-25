use tokio::sync;
use tokio::sync::broadcast::error::SendError;
use crate::event_service::get_event_types;
use crate::event_types_map::get_event_type;

use super::event::Event;

#[derive(Debug, PartialEq, Clone)]
pub struct Emitter {
    pub uid: String,
    pub event_type_uid: String,
    pub name: String,
}

impl Emitter {
    pub async fn emit_event(
        &self,
        event: Event,
    ) -> Result<usize, SendError<Event>> {
        // self.event_sender.send(event).await
        let event_type = get_event_type(&self.event_type_uid).unwrap();
        let sender = event_type.get_sender();

        sender.send(event)
    }
}
