use std::collections::HashMap;

use crate::block::Block;
use crate::symbolic_expression::SymbolicExpression;

use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct ByteInterval {
    pub uuid: Uuid,
    pub blocks: Vec<Block>,
    pub symbolic_expressions: HashMap<u64, SymbolicExpression>,
    pub has_address: bool,
    pub address: u64,
    pub size: u64,
    pub contents: Vec<u8>,
}
