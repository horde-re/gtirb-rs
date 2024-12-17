use super::auxdata::AuxData;
use super::block::ProxyBlock;
use super::section::Section;
use super::symbol::Symbol;

use node_derive::Node;

use std::collections::HashMap;

use uuid::Uuid;

#[derive(Node, Clone, PartialEq)]
pub struct Module {
    pub uuid: Uuid,
    pub binary_path: String,
    pub preferred_addr: u64,
    pub rebase_delta: i64,
    pub file_format: i32,
    pub isa: Isa,
    pub name: String,
    pub symbols: Vec<Symbol>,
    pub proxies: Vec<ProxyBlock>,
    pub sections: Vec<Section>,
    pub aux_data: HashMap<String, AuxData>,
    pub entry_point: Vec<u8>,
    pub byte_order: ByteOrder,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum FileFormat {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum Isa {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum ByteOrder {
    Undefined,
    BigEndian,
    LittleEndian,
}
