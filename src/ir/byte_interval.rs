use std::collections::HashMap;

use super::block::Block;
use super::symbolic_expression::SymbolicExpression;

use node_derive::Node;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ByteInterval {
    pub uuid: Uuid,
    pub blocks: Vec<Block>,
    pub symbolic_expressions: HashMap<u64, SymbolicExpression>,
    pub has_address: bool,
    pub address: u64,
    pub size: u64,
    pub contents: Vec<u8>,
}

impl ByteInterval {
    pub fn new() -> Self {
        ByteInterval {
            uuid: Uuid::new_v4(),
            blocks: Vec::new(),
            symbolic_expressions: HashMap::new(),
            has_address: false,
            address: 0,
            size: 0,
            contents: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn remove_block(&mut self, block: &Block) {
        self.blocks.retain(|b| b != block);
    }

    pub fn add_symbolic_expression(
        &mut self,
        offset: u64,
        symbolic_expression: SymbolicExpression,
    ) {
        self.symbolic_expressions
            .insert(offset, symbolic_expression);
    }

    pub fn remove_symbolic_expression(&mut self, offset: u64) {
        self.symbolic_expressions.remove(&offset);
    }
}

impl Default for ByteInterval {
    fn default() -> Self {
        ByteInterval::new()
    }
}
