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

/// Generated GTIRB Rust module from the GTIRB protobuf schema.
#[allow(clippy::enum_variant_names)] // Generated code
mod gtirb {
    /// Generated proto Rust module from the GTIRB protobuf schema.
    pub mod proto {
        // This file is generated somewhere in : target/debug/build/gtirb-rs-*/out/gtirb.proto.rs
        include!(concat!(env!("OUT_DIR"), "/gtirb.proto.rs"));
    }
}
