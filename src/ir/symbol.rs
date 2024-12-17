use node_derive::Node;

use uuid::Uuid;

#[derive(Node, Clone, PartialEq)]
pub struct Symbol {
    pub uuid: Uuid,
    pub name: String,
    pub at_end: bool,
    pub optional_payload: Option<OptionalPayload>,
}

#[derive(Clone, PartialEq)]
pub enum OptionalPayload {
    Value(u64),
    ReferentUuid(Vec<u8>),
}
