//! This crate provides a Rust API for the GTIRB intermediate representation.
//! It is generated from the GTIRB protobuf schema.

pub mod auxdata;
pub mod block;
pub mod byte_interval;
pub mod cfg;
pub mod ir;
pub mod module;
pub mod node;
pub mod offset;
pub mod section;
pub mod symbol;
pub mod symbolic_expression;
