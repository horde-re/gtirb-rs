#[derive(Clone, PartialEq)]
pub struct Ir {
    pub uuid: Vec<u8>,

    pub modules: Vec<Module>,

    pub aux_data: ::std::collections::HashMap<String, AuxData>,

    pub version: u32,

    pub cfg: ::core::option::Option<Cfg>,
}
