use super::auxdata::AuxData;
use super::block::ProxyBlock;
use super::section::Section;
use super::symbol::Symbol;

use node_derive::Node;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Node, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Module {
    pub uuid: Uuid,
    pub binary_path: PathBuf,
    pub preferred_addr: u64,
    pub rebase_delta: i64,
    pub file_format: FileFormat,
    pub isa: Isa,
    pub name: String,
    pub symbols: Vec<Symbol>,
    pub proxies: Vec<ProxyBlock>,
    pub sections: Vec<Section>,
    pub aux_data: HashMap<String, AuxData>,
    pub entry_point: Vec<u8>,
    pub byte_order: ByteOrder,
}

impl Module {
    pub fn new() -> Self {
        Module {
            uuid: Uuid::new_v4(),
            binary_path: PathBuf::new(),
            preferred_addr: 0,
            rebase_delta: 0,
            file_format: FileFormat::FormatUndefined,
            isa: Isa::Undefined,
            name: String::new(),
            symbols: Vec::new(),
            proxies: Vec::new(),
            sections: Vec::new(),
            aux_data: HashMap::new(),
            entry_point: Vec::new(),
            byte_order: ByteOrder::Undefined,
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn remove_symbol(&mut self, symbol: &Symbol) {
        self.symbols.retain(|s| s != symbol);
    }

    pub fn add_proxy(&mut self, proxy: ProxyBlock) {
        self.proxies.push(proxy);
    }

    pub fn remove_proxy(&mut self, proxy: &ProxyBlock) {
        self.proxies.retain(|p| p != proxy);
    }

    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    pub fn remove_section(&mut self, section: &Section) {
        self.sections.retain(|s| s != section);
    }
}

impl Default for Module {
    fn default() -> Self {
        Module::new()
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
#[repr(i32)]
pub enum FileFormat {
    #[default]
    FormatUndefined,
    Coff,
    Elf,
    Pe,
    IdaProDb32,
    IdaProDb64,
    Xcoff,
    Macho,
    Raw,
}

impl FileFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "coff" => FileFormat::Coff,
            "elf" => FileFormat::Elf,
            "pe" => FileFormat::Pe,
            "idaprodb32" => FileFormat::IdaProDb32,
            "idaprodb64" => FileFormat::IdaProDb64,
            "xcoff" => FileFormat::Xcoff,
            "macho" => FileFormat::Macho,
            "raw" => FileFormat::Raw,
            _ => FileFormat::FormatUndefined,
        }
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
#[repr(i32)]
pub enum Isa {
    #[default]
    Undefined,
    Ia32,
    Ppc32,
    X64,
    Arm,
    ValidButUnsupported,
    Ppc64,
    Arm64,
    Mips32,
    Mips64,
}

impl Isa {
    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "ia32" => Isa::Ia32,
            "ppc32" => Isa::Ppc32,
            "x64" => Isa::X64,
            "arm" => Isa::Arm,
            "validbutunsupported" => Isa::ValidButUnsupported,
            "ppc64" => Isa::Ppc64,
            "arm64" => Isa::Arm64,
            "mips32" => Isa::Mips32,
            "mips64" => Isa::Mips64,
            _ => Isa::Undefined,
        }
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
#[repr(i32)]
pub enum ByteOrder {
    #[default]
    Undefined,
    BigEndian,
    LittleEndian,
}
