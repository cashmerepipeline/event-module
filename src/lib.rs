#[macro_use]
extern crate rust_i18n;
i18n!("locales");

mod event_types_map;
mod types;
mod emitter_delegators_map;
mod event_echo_wrapper;
mod emitter_delegator;
mod event_services;
mod type_listeners_map;

pub mod service_handles;
pub mod field_ids;
pub mod manage_ids;
pub mod managers;
pub mod protocols;

mod dispatcher;
pub mod type_dispatcher_map;

pub use initialize_event_service::*;
mod initialize_event_service;
