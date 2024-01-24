use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

mod event_inner_wrapper;
mod event_services;
mod event_types_map;
mod types;
// mod type_listeners_map;
mod event_service_configs;
mod event_type_listeners_map;
mod listener_instances_map;

pub mod ids_codes;
pub mod managers;
pub mod protocols;
pub mod service_handles;

mod dispatcher;
mod type_dispatcher_map;

pub use event_service_configs::*;
pub use initialize_event_service::*;
mod dispatch_event;
mod initialize_event_service;
mod listener_instance;

#[cfg(feature = "use_channel_dispatch")]
mod dispatch_channels;

#[cfg(feature = "use_queue_dispatch")]
mod dispatch_queue;

#[cfg(all(feature = "use_channel_dispatch", feature = "use_queue_dispatch"))]
compile_error!("feature \"use_channel_dispatch\" and feature \"use_queue_dispatch\" cannot be enabled at the same time");
