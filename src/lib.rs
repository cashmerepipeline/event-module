#[macro_use]
extern crate rust_i18n;
i18n!("locales");

// mod event_types_map;
// mod event_queue;
// mod event_queue_map;
// mod listeners_map;
// mod event;
// mod event_type;
// mod event_echo_type;
mod types;
// mod listener;
// mod dispatcher;
// mod emitter;
mod emitter_delegators_map;
mod event_echo_wrapper;
// pub mod event_service;

// pub use event_type::*;
// pub use event::*;
// pub use event_echo_type::*;
mod dispatch_local_set;
mod emitter_delegator;

pub mod service_handles;

pub mod event_protocol;
pub mod field_ids;
pub mod manage_ids;

mod dispatcher;
pub mod dispatchers_map;

pub use initialize_event_service::*;
mod initialize_event_service;

// #[cfg(test)]
// mod tests {
//     use crate::{Event, EventEchoType};
//     use crate::event_service::{register_emitter, get_emitter, register_event_type, listen_event_type};

//     #[test]
//     fn register_event_type_test() {
//         let id = register_event_type("test".to_string(), Default::default(), "".to_string());
//         let events_vec = crate::event_types_map::get_event_types();

//         println!("{}", id);
//         println!("{:?}", events_vec[0]);

//         let test_type = &events_vec[0];
//         let listener = test_type.subscribe();

//         assert!(events_vec.len() > 0);
//     }

//     #[test]
//     fn regitster_event_emitter_test(){
//         let event_type_name = "test".to_string();
//         let test_emitter_name = "test_emitter".to_string();

//         let id = register_event_type(event_type_name.clone(), Default::default(), "".to_string());
//         let emitter_id = register_emitter(&id, &test_emitter_name).unwrap();

//         let emitter = get_emitter(&id, &emitter_id);

//         println!("{}", id);
//         println!("{}", emitter_id);
//         println!("{:?}", emitter);

//         assert!(emitter.is_some())
//     }

//     #[test]
//     fn listen_event_type_test(){
//         let event_type_name = "test".to_string();
//         let listener_name = "test_listener".to_string();

//         let event_type_id = register_event_type(event_type_name.clone(), Default::default(), "".to_string());

//         let listener = listen_event_type(&event_type_id, &listener_name);

//         assert!(listener.is_ok());
//     }

//     #[tokio::test]
//     async fn send_event_test(){
//         let event_type_name = "test".to_string();
//         let emitter_name = "test_emitter".to_string();
//         let listener_name = "test_listener".to_string();

//         let event_type_id = register_event_type(event_type_name.clone(), Default::default(), "".to_string());
//         let emitter_id = register_emitter(&event_type_id, &emitter_name).unwrap();

//         let emitter = get_emitter(&event_type_id, &emitter_id);
//         let listener = listen_event_type(&event_type_id, &listener_name);
//         assert!(emitter.is_some());
//         assert!(listener.is_ok());

//         let event = Event{
//             type_uid: event_type_id.clone(),
//             emitter_uid: emitter_id.clone(),
//             serial_number: 1,
//             timestamp: 0,
//             out_time: 1,
//             echo_type: EventEchoType::NoEcho,
//             messages: vec![]
//         };

//         let sended = emitter.unwrap().emit_event(event.clone()).await;

//         let reciver_arc = listener.unwrap().receiver;
//         let mut receiver = reciver_arc.write();
//         let recieved = receiver.recv().await;

//         assert!(sended.is_ok());
//         assert!(recieved.is_ok());
//         assert_eq!(recieved.unwrap(), event);
//     }
// }
