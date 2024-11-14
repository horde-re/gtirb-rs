#[derive(Clone, PartialEq)]
pub struct Symbol {
    pub uuid: Vec<u8>,

    pub name: String,

    pub at_end: bool,

    pub optional_payload: ::core::option::Option<symbol::OptionalPayload>,
}
/// Nested message and enum types in `Symbol`.
pub mod symbol {
    #[derive(Clone, PartialEq)]
    pub enum OptionalPayload {
        Value(u64),

        ReferentUuid(Vec<u8>),
    }
}
