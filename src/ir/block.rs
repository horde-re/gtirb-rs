//! To-do

use node_derive::Node;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Block {
    pub offset: u64,
    pub value: Option<BlockValue>,
}

impl Block {
    pub fn new(offset: u64, value: Option<BlockValue>) -> Self {
        Self { offset, value }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum BlockValue {
    Code(CodeBlock),
    Data(DataBlock),
}

impl BlockValue {
    pub fn new_code(uuid: Uuid, size: u64, decode_mode: DecodeMode) -> Self {
        BlockValue::Code(CodeBlock {
            uuid,
            size,
            decode_mode,
        })
    }

    pub fn new_data(uuid: Uuid, size: u64) -> Self {
        BlockValue::Data(DataBlock { uuid, size })
    }
}

#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ProxyBlock {
    pub uuid: Uuid,
}

impl ProxyBlock {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }
}

#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CodeBlock {
    pub uuid: Uuid,
    pub size: u64,
    pub decode_mode: DecodeMode,
}

impl CodeBlock {
    pub fn new(uuid: Uuid, size: u64, decode_mode: DecodeMode) -> Self {
        Self {
            uuid,
            size,
            decode_mode,
        }
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
    pub uuid: Uuid,
    pub size: u64,
}

impl DataBlock {
    pub fn new(uuid: Uuid, size: u64) -> Self {
        Self { uuid, size }
    }
}
