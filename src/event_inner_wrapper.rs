use dependencies_sync::tokio::sync::mpsc::Sender;

use crate::protocols::Event;

#[derive(Clone)]
pub struct EventInnerWrapper {
  pub event: Event,
  pub echo_sender: Option<Sender<Event>>,
}