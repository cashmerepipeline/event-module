use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::emitter_delegator::EmitterDelegator;
use crate::protocols::{Event, EventEmitter, EventListener, EventType};

pub type EventTypesMap = BTreeMap<String, Arc<EventType>>;

pub type EmitterDelegatorsMap = BTreeMap<usize, EmitterDelegator>;

// event_type_uid: [emitters]
pub type EventTypeEmittersMap = BTreeMap<String, Arc<RwLock<EmitterDelegatorsMap>>>;
