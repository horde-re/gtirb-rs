use super::auxdata::AuxData;
use super::cfg::Cfg;
use super::module::Module;

use std::collections::HashMap;

use node_derive::Node;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Ir {
    pub uuid: Uuid,
    pub modules: Vec<Module>,
    pub aux_data: HashMap<String, AuxData>,
    pub version: u32,
    pub cfg: Option<Cfg>,
}

impl Ir {
    pub fn new() -> Self {
        Ir {
            uuid: Uuid::new_v4(),
            modules: Vec::new(),
            aux_data: HashMap::new(),
            version: 0,
            cfg: None,
        }
    }

    pub fn add_module(&mut self, module: Module) {
        self.modules.push(module);
    }

    pub fn remove_module(&mut self, module: &Module) {
        self.modules.retain(|m| m != module);
    }
}

impl Default for Ir {
    fn default() -> Self {
        Ir::new()
    }
}
