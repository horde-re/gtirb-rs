//! To-do

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct AuxData {
    pub type_name: String,
    pub data: Vec<u8>,
}

impl AuxData {
    pub fn new(type_name: String, data: Vec<u8>) -> Self {
        Self { type_name, data }
    }
}

impl Default for AuxData {
    fn default() -> Self {
        Self {
            type_name: String::new(),
            data: Vec::new(),
        }
    }
}
