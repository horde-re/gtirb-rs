use super::byte_interval::ByteInterval;

use node_derive::Node;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Section {
    pub uuid: Uuid,
    pub name: String,
    pub byte_intervals: Vec<ByteInterval>,
    pub section_flags: Vec<SectionFlag>,
}

impl Section {
    pub fn new(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            byte_intervals: Vec::new(),
            section_flags: Vec::new(),
        }
    }

    pub fn add_byte_interval(&mut self, byte_interval: ByteInterval) {
        self.byte_intervals.push(byte_interval);
    }

    pub fn remove_byte_interval(&mut self, byte_interval: &ByteInterval) {
        self.byte_intervals.retain(|b| b != byte_interval);
    }

    pub fn add_section_flag(&mut self, section_flag: SectionFlag) {
        self.section_flags.push(section_flag);
    }

    pub fn remove_section_flag(&mut self, section_flag: &SectionFlag) {
        self.section_flags.retain(|f| f != section_flag);
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
#[repr(i32)]
pub enum SectionFlag {
    #[default]
    Undefined,
    Readable,
    Writable,
    Executable,
    Loaded,
    Initialized,
    ThreadLocal,
}
