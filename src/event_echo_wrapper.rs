use tokio::sync::mpsc::Sender;

use crate::protocols::Event;

#[derive(Clone)]
pub struct EventEchoWrapper {
  pub event: Event,
  pub echo_sender: Option<Sender<Event>>,
}