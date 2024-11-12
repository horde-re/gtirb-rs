//! This crate provides a Rust API for the GTIRB intermediate representation.
//! It is generated from the GTIRB protobuf schema.

//! Generated GTIRB Rust module from the GTIRB protobuf schema.
mod gtirb {
    //! Generated proto Rust module from the GTIRB protobuf schema.
    pub mod proto {
        include!(concat!(env!("OUT_DIR"), "/gtirb.proto.rs"));
    }
}
