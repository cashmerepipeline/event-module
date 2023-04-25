use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventType {
    pub type_id: u32,
    pub name: String,
    pub has_echo: bool,
    pub description: String,
}

impl EventType {
}
