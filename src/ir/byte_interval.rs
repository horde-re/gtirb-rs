use std::collections::HashMap;

use super::address::Address;
use super::block::Block;
use super::symbolic_expression::SymbolicExpression;

use node_derive::Node;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct ByteInterval {
    uuid: Uuid,
    blocks: Vec<Block>,
    symbolic_expressions: HashMap<u64, SymbolicExpression>,
    has_address: bool,
    address: Address,
    size: u64,
    content: Vec<u8>,
}

impl ByteInterval {
    #[allow(unused_variables)]
    pub fn new(address: Option<Address>, size: u64, init_size: Option<u64>) -> Self {
        // TODO: Implement the rest of the function
        // use init_size
        ByteInterval {
            uuid: Uuid::new_v4(),
            blocks: Vec::new(),
            symbolic_expressions: HashMap::new(),
            has_address: false,
            address: address.unwrap_or(0),
            size,
            content: Vec::new(),
        }
    }

    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    pub fn symbolic_expressions(&self) -> &HashMap<u64, SymbolicExpression> {
        &self.symbolic_expressions
    }

    pub fn has_address(&self) -> bool {
        self.has_address
    }

    pub fn address(&self) -> Address {
        self.address
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn content(&self) -> &Vec<u8> {
        &self.content
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
