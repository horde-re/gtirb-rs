use node_derive::Node;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Symbol {
    pub uuid: Uuid,
    pub name: String,
    pub at_end: bool,
    pub payload: Option<Payload>,
}

impl Symbol {
    pub fn new(name: String, at_end: bool, payload: Option<Payload>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            at_end,
            payload,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Payload {
    Value(u64),
    ReferentUuid(Uuid),
}

impl Payload {
    pub fn new_value(value: u64) -> Self {
        Payload::Value(value)
    }

    pub fn new_referent_uuid(referent_uuid: Uuid) -> Self {
        Payload::ReferentUuid(referent_uuid)
    }
}
