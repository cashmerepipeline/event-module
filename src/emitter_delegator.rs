
use std::sync::mpsc::Sender;

use crate::event_protocol::Event;

#[derive(Debug, Clone)]
pub struct EmitterDelegator {
    pub id: usize,
    pub event_type_id: String,
    pub sender: Sender<Event>,
    pub echo_sender: Sender<Event>
}

