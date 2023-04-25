use std::sync::Arc;
use parking_lot::RwLock;
use tokio::sync::broadcast::Receiver;

use super::event::Event;

#[derive(Debug, Clone)]
pub struct Listener {
    pub uid: String,
    pub name: String,
    pub event_type_uid: String,
    pub receiver: Arc<RwLock<Receiver<Event>>>,
}
