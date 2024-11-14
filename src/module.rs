#[derive(Clone, PartialEq)]
pub struct Module {
    pub uuid: Vec<u8>,

    pub binary_path: String,

    pub preferred_addr: u64,

    pub rebase_delta: i64,

    pub file_format: i32,

    pub isa: i32,

    pub name: String,

    pub symbols: Vec<Symbol>,

    pub proxies: Vec<ProxyBlock>,

    pub sections: Vec<Section>,

    pub aux_data: ::std::collections::HashMap<String, AuxData>,

    pub entry_point: Vec<u8>,

    pub byte_order: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum FileFormat {
    FormatUndefined = 0,
    Coff = 1,
    Elf = 2,
    Pe = 3,
    IdaProDb32 = 4,
    IdaProDb64 = 5,
    Xcoff = 6,
    Macho = 7,
    Raw = 8,
}
impl FileFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::FormatUndefined => "Format_Undefined",
            Self::Coff => "COFF",
            Self::Elf => "ELF",
            Self::Pe => "PE",
            Self::IdaProDb32 => "IdaProDb32",
            Self::IdaProDb64 => "IdaProDb64",
            Self::Xcoff => "XCOFF",
            Self::Macho => "MACHO",
            Self::Raw => "RAW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Format_Undefined" => Some(Self::FormatUndefined),
            "COFF" => Some(Self::Coff),
            "ELF" => Some(Self::Elf),
            "PE" => Some(Self::Pe),
            "IdaProDb32" => Some(Self::IdaProDb32),
            "IdaProDb64" => Some(Self::IdaProDb64),
            "XCOFF" => Some(Self::Xcoff),
            "MACHO" => Some(Self::Macho),
            "RAW" => Some(Self::Raw),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum Isa {
    Undefined = 0,
    Ia32 = 1,
    Ppc32 = 2,
    X64 = 3,
    Arm = 4,
    ValidButUnsupported = 5,
    Ppc64 = 6,
    Arm64 = 7,
    Mips32 = 8,
    Mips64 = 9,
}
impl Isa {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Undefined => "ISA_Undefined",
            Self::Ia32 => "IA32",
            Self::Ppc32 => "PPC32",
            Self::X64 => "X64",
            Self::Arm => "ARM",
            Self::ValidButUnsupported => "ValidButUnsupported",
            Self::Ppc64 => "PPC64",
            Self::Arm64 => "ARM64",
            Self::Mips32 => "MIPS32",
            Self::Mips64 => "MIPS64",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ISA_Undefined" => Some(Self::Undefined),
            "IA32" => Some(Self::Ia32),
            "PPC32" => Some(Self::Ppc32),
            "X64" => Some(Self::X64),
            "ARM" => Some(Self::Arm),
            "ValidButUnsupported" => Some(Self::ValidButUnsupported),
            "PPC64" => Some(Self::Ppc64),
            "ARM64" => Some(Self::Arm64),
            "MIPS32" => Some(Self::Mips32),
            "MIPS64" => Some(Self::Mips64),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum ByteOrder {
    Undefined = 0,
    BigEndian = 1,
    LittleEndian = 2,
}
impl ByteOrder {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Undefined => "ByteOrder_Undefined",
            Self::BigEndian => "BigEndian",
            Self::LittleEndian => "LittleEndian",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ByteOrder_Undefined" => Some(Self::Undefined),
            "BigEndian" => Some(Self::BigEndian),
            "LittleEndian" => Some(Self::LittleEndian),
            _ => None,
        }
    }
}
