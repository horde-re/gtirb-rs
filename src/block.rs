//! To-do
//! - [ ] Implement the `Block` trait
//! - [ ] Implement the `CodeBlock` struct and Block trait
//! - [ ] Implement the `DataBlock` struct and Block trait
//! - [ ] Implement the `ProxyBlock` struct and Block trait

use crate::node::Node;

use uuid::{uuid, Uuid};

/// Block represents a base trait for blocks. [`Symbol`] objects may have references to any kind of Block.
// pub trait Block {
//     /// Get the module this node ultimately belongs to
//     fn get_module(&self) -> Option<Module>;

//     /// Get all the symbols that refer to this block
//     fn get_references(&self) -> Vec<Symbol>;
// }

#[derive(Clone, PartialEq)]
pub struct ProxyBlock {
    pub uuid: Vec<u8>,
}
#[derive(Clone, PartialEq)]
pub struct CodeBlock {
    pub uuid: Vec<u8>,

    pub size: u64,

    pub decode_mode: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum DecodeMode {
    AllDefault = 0,
    ArmThumb = 1,
}
impl DecodeMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::AllDefault => "All_Default",
            Self::ArmThumb => "ARM_Thumb",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "All_Default" => Some(Self::AllDefault),
            "ARM_Thumb" => Some(Self::ArmThumb),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq)]
pub struct DataBlock {
    pub uuid: Vec<u8>,

    pub size: u64,
}

#[derive(Clone, PartialEq)]
pub struct Block {
    pub offset: u64,

    pub value: ::core::option::Option<block::Value>,
}
/// Nested message and enum types in `Block`.
pub mod block {
    #[derive(Clone, PartialEq)]
    pub enum Value {
        Code(super::CodeBlock),

        Data(super::DataBlock),
    }
}
#[derive(Clone, PartialEq)]
pub struct ByteInterval {
    pub uuid: Vec<u8>,

    pub blocks: Vec<Block>,

    pub symbolic_expressions: ::std::collections::HashMap<u64, SymbolicExpression>,

    pub has_address: bool,

    pub address: u64,

    pub size: u64,

    pub contents: Vec<u8>,
}
