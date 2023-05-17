use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::protocols::{Event, EventEmitter, EventListener, EventType};

pub type EventTypesMap = BTreeMap<String, Arc<EventType>>;
