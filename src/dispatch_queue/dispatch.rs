use crate::dispatch_queue::event_type_queue_map::DispatchQueue;
use crate::event_services::get_event_runtime;
use crate::event_type_listeners_map::get_event_type_listener_map;
use crate::listener_senders_map::{get_listener_sender_map, remove_listener_senders};
use dependencies_sync::log::{warn, info};
use dependencies_sync::parking_lot::lock_api::RwLock;
use dependencies_sync::parking_lot::RawRwLock;
use std::sync::Arc;

use crate::dispatch_event::dispatch_event;

pub fn dispatch(type_id: &String, dispatch_queue_arc: Arc<RwLock<RawRwLock, DispatchQueue>>) {
    let mut dispatch_queue = dispatch_queue_arc.write();
    while let Some(event_echo_wrapper) = dispatch_queue.pop_front() {
        dispatch_event(type_id.to_owned(), event_echo_wrapper);
    }
}


