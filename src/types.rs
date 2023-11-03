use std::collections::BTreeMap;
use std::sync::Arc;

use crate::protocols::EventType;

pub type EventTypesMap = BTreeMap<String, Arc<EventType>>;
