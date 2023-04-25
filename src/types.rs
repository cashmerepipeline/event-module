use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::emitter::Emitter;
use crate::event::Event;
use crate::event_message::EventMessageField;
use crate::event_type::EventType;
use crate::listener::Listener;

pub type EventTypesMap = BTreeMap<String, EventType>;

// 事件缓存
pub type EventDeQue = Arc<RwLock<VecDeque<Event>>>;

// event_type_uid:[events]
pub type EventsQueueMap = BTreeMap<String, EventDeQue>;

pub type EmittersMap = BTreeMap<String, Emitter>;
// event_type_uid: [emitters]
pub type EventTypeEmittersMap = BTreeMap<String, Arc<RwLock<EmittersMap>>>;

pub type ListenersMap = BTreeMap<String, Listener>;
// event_type_uid: [listeners]
pub type EventTypeListenersMap = BTreeMap<String, Arc<RwLock<ListenersMap>>>;

/// 事件负载映射
pub type EventMessageFieldMap = BTreeMap<String, EventMessageField>;
