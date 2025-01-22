//! To-do

use node_derive::Node;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Block {
    offset: u64,
    value: Option<BlockValue>,
}

impl Block {
    pub fn new(offset: u64, value: Option<BlockValue>) -> Self {
        Self { offset, value }
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn value(&self) -> &Option<BlockValue> {
        &self.value
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum BlockValue {
    Code(CodeBlock),
    Data(DataBlock),
}

impl BlockValue {
    pub fn new_code(uuid: Uuid, bytes: Vec<u8>, decode_mode: DecodeMode) -> Self {
        BlockValue::Code(CodeBlock {
            uuid,
            bytes,
            decode_mode,
        })
    }

    pub fn new_data(uuid: Uuid, size: u64) -> Self {
        BlockValue::Data(DataBlock { uuid, size })
    }
}

/// A placeholder that serves as the endpoint (source or target)
/// of a CFG edge.
///
/// ProxyBlock objects allow the construction of CFG edges to or from
/// another node. For example, a call to a function in another module
/// may be represented by an edge that originates at the calling
/// CodeBlock and targets a ProxyBlock. Another example would be an
/// edge that represents an indirect jump whose target is not known.
///
/// A ProxyBlock does not represent any instructions and so has neither
/// an address nor a size.
#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct ProxyBlock {
    uuid: Uuid,
}

impl ProxyBlock {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }
}

/// A basic block that contains code.
#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CodeBlock {
    uuid: Uuid,
    bytes: Vec<u8>,
    decode_mode: DecodeMode,
}

impl CodeBlock {
    pub fn new(uuid: Uuid, bytes: Vec<u8>, decode_mode: DecodeMode) -> Self {
        Self {
            uuid,
            bytes,
            decode_mode,
        }
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn decode_mode(&self) -> DecodeMode {
        self.decode_mode
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
#[repr(i32)]
pub enum DecodeMode {
    #[default]
    AllDefault,
    ArmThumb,
}

#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DataBlock {
    uuid: Uuid,
    size: u64,
}

impl DataBlock {
    pub fn new(uuid: Uuid, size: u64) -> Self {
        Self { uuid, size }
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}
