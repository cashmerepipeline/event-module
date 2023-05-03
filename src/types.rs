use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::emitter_delegator::EmitterDelegator;
use crate::protocols::{Event, EventListener, EventEmitter};

// pub type EventTypesMap = BTreeMap<String, EventType>;

// 事件缓存
// pub type EventDeQue = Arc<RwLock<VecDeque<Event>>>;

// event_type_uid:[events]
// pub type EventsQueueMap = BTreeMap<String, EventDeQue>;

pub type EmitterDelegatorsMap = BTreeMap<usize, EmitterDelegator>;

// event_type_uid: [emitters]
pub type EventTypeEmittersMap = BTreeMap<String, Arc<RwLock<EmitterDelegatorsMap>>>;

pub type ListenersMap = BTreeMap<usize, EventListener>;
// event_type_uid: [listeners]
pub type EventTypeListenersMap = BTreeMap<String, Arc<RwLock<ListenersMap>>>;

// 事件负载映射
// pub type EventMessageFieldMap = BTreeMap<String, EventMessageField>;
