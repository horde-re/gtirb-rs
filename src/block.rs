//! To-do
//! - [ ] Implement the `Block` trait
//! - [ ] Implement the `CodeBlock` struct and Block trait
//! - [ ] Implement the `DataBlock` struct and Block trait
//! - [ ] Implement the `ProxyBlock` struct and Block trait

use node_derive::Node;

use uuid::Uuid;

/// Block represents a base trait for blocks. [`Symbol`] objects may have references to any kind of Block.
// pub trait Block {
//     /// Get the module this node ultimately belongs to
//     fn get_module(&self) -> Option<Module>;

//     /// Get all the symbols that refer to this block
//     fn get_references(&self) -> Vec<Symbol>;
// }

#[derive(Clone, PartialEq)]
pub struct Block {
    pub offset: u64,
    pub value: Option<BlockValue>,
}

#[derive(Clone, PartialEq)]
pub enum BlockValue {
    Code(CodeBlock),
    Data(DataBlock),
}

#[derive(Node, Clone, PartialEq)]
pub struct ProxyBlock {
    pub uuid: Uuid,
}

impl ProxyBlock {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }
}

#[derive(Node, Clone, PartialEq)]
pub struct CodeBlock {
    pub uuid: Uuid,
    pub size: u64,
    pub decode_mode: DecodeMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum DecodeMode {
    AllDefault,
    ArmThumb,
}

#[derive(Node, Clone, PartialEq)]
pub struct DataBlock {
    pub uuid: Uuid,
    pub size: u64,
}
