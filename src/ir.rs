use crate::auxdata::AuxData;
use crate::cfg::Cfg;
use crate::module::Module;

use std::collections::HashMap;

use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct Ir {
    pub uuid: Uuid,
    pub modules: Vec<Module>,
    pub aux_data: HashMap<String, AuxData>,
    pub version: u32,
    pub cfg: Option<Cfg>,
}
