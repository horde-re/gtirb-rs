//! Intermediate Representation (IR) for the binary analysis.

pub mod address;
pub mod auxdata;
pub mod block;
pub mod byte_interval;
pub mod cfg;
#[allow(clippy::module_inception)]
pub mod ir;
pub mod module;
pub mod node;
pub mod section;
pub mod symbol;
pub mod symbolic_expression;
