use super::auxdata::AuxData;
use super::cfg::Cfg;
use super::module::Module;

use std::collections::HashMap;

use node_derive::Node;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A complete internal representation consisting of Modules
#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Ir {
    uuid: Uuid,
    modules: Vec<Module>,
    aux_data: HashMap<String, AuxData>,
    version: u32,
    cfg: Option<Cfg>,
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

    pub fn modules(&self) -> &Vec<Module> {
        &self.modules
    }

    pub fn aux_data(&self) -> &HashMap<String, AuxData> {
        &self.aux_data
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn cfg(&self) -> &Option<Cfg> {
        &self.cfg
    }

    pub fn add_module(&mut self, module: Module) {
        self.modules.push(module);
    }

    pub fn remove_module(&mut self, module: &Module) {
        self.modules.retain(|m| m != module);
    }

    pub fn add_aux_data(&mut self, key: String, aux_data: AuxData) {
        self.aux_data.insert(key, aux_data);
    }

    pub fn remove_aux_data(&mut self, key: &str) {
        self.aux_data.remove(key);
    }
}
