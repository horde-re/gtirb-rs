//! To-do
//! - [ ] Implement the `Block` trait
//! - [ ] Implement the `CodeBlock` struct and Block trait
//! - [ ] Implement the `DataBlock` struct and Block trait
//! - [ ] Implement the `ProxyBlock` struct and Block trait

use crate::gtirb::proto::{CodeBlock, DataBlock, Module, ProxyBlock, Symbol};

use crate::node::Node;

/// Block represents a base trait for blocks. [`Symbol`] objects may have references to any kind of Block.
pub trait Block {
    type Node: Node;

    // Get the module this node ultimately belongs to
    fn get_module(&self) -> Option<Module>;

    // Get all the symbols that refer to this block
    fn get_references(&self) -> Vec<Symbol>;
}

impl CodeBlock {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CodeBlock::default()
    }
}

impl DataBlock {
    #[allow(dead_code)]
    pub fn new() -> Self {
        DataBlock::default()
    }
}

impl ProxyBlock {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ProxyBlock::default()
    }
}
