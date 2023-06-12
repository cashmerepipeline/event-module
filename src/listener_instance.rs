use dependencies_sync::tokio::sync::mpsc::Sender;

use crate::event_inner_wrapper::EventInnerWrapper;

pub struct ListenerInstance{
  pub index: u32,
  pub name: String,
  pub sender: Sender<EventInnerWrapper>,
}