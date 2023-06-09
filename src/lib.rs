#[macro_use]
extern crate rust_i18n;
i18n!("locales");

mod event_types_map;
mod types;
mod event_inner_wrapper;
mod event_services;
// mod type_listeners_map;
mod event_type_listeners_map;
mod listener_senders_map;
mod dispatch_queue;
mod dispatch_channels;
mod event_service_configs;

pub mod service_handles;
pub mod field_ids;
pub mod manage_ids;
pub mod managers;
pub mod protocols;

mod dispatcher;
mod type_dispatcher_map;

pub use event_service_configs::*;
pub use initialize_event_service::*;
mod initialize_event_service;
mod dispatch_event;
