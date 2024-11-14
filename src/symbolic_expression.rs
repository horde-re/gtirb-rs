#[derive(Clone, PartialEq)]
pub struct SymStackConst {
    pub offset: i32,

    pub symbol_uuid: Vec<u8>,
}
#[derive(Clone, PartialEq)]
pub struct SymAddrConst {
    pub offset: i64,

    pub symbol_uuid: Vec<u8>,
}
#[derive(Clone, PartialEq)]
pub struct SymAddrAddr {
    pub scale: i64,

    pub offset: i64,

    pub symbol1_uuid: Vec<u8>,

    pub symbol2_uuid: Vec<u8>,
}
#[derive(Clone, PartialEq)]
pub struct SymbolicExpression {
    pub attribute_flags: Vec<i32>,

    pub value: ::core::option::Option<symbolic_expression::Value>,
}
/// Nested message and enum types in `SymbolicExpression`.
pub mod symbolic_expression {
    #[derive(Clone, PartialEq)]
    pub enum Value {
        AddrConst(super::SymAddrConst),

        AddrAddr(super::SymAddrAddr),
    }
}
/// NOTE:
/// We do not generalize or otherwise unify relocation attributes across
/// architectures and instead prefer an explicit mapping of attributes names
/// to the labels used in the assembly of each architecture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum SymAttribute {
    /// ELF
    Got = 0,
    Gotpc = 1,
    Gotoff = 2,
    Gotrel = 3,
    Plt = 4,
    Pltoff = 5,
    Pcrel = 6,
    Secrel = 7,
    Tls = 8,
    Tlsgd = 9,
    Tlsld = 10,
    Tlsldm = 11,
    Tlscall = 12,
    Tlsdesc = 13,
    Tprel = 14,
    Tpoff = 15,
    Dtprel = 16,
    Dtpoff = 17,
    Ntpoff = 18,
    Dtpmod = 19,
    Page = 20,
    Pageoff = 21,
    Call = 22,
    Lo = 23,
    Hi = 24,
    Higher = 25,
    Highest = 26,
    /// X86
    Gotntpoff = 1000,
    Indntpoff = 1001,
    /// ARM
    G0 = 2001,
    G1 = 2002,
    G2 = 2003,
    G3 = 2004,
    Upper16 = 2005,
    Lower16 = 2006,
    Lo12 = 2007,
    Lo15 = 2008,
    Lo14 = 2009,
    Hi12 = 2010,
    Hi21 = 2011,
    S = 2012,
    Pg = 2013,
    Nc = 2014,
    Abs = 2015,
    Prel = 2016,
    Prel31 = 2017,
    Target1 = 2018,
    Target2 = 2019,
    Sbrel = 2020,
    Tlsldo = 2021,
    /// MIPS
    Hi16 = 3000,
    Lo16 = 3001,
    Gprel = 3002,
    Disp = 3003,
    Ofst = 3004,
    /// PPC
    H = 4000,
    L = 4001,
    Ha = 4002,
    High = 4003,
    Higha = 4004,
    Highera = 4005,
    Highesta = 4006,
    Tocbase = 4007,
    Toc = 4008,
    Notoc = 4009,
}
impl SymAttribute {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Got => "GOT",
            Self::Gotpc => "GOTPC",
            Self::Gotoff => "GOTOFF",
            Self::Gotrel => "GOTREL",
            Self::Plt => "PLT",
            Self::Pltoff => "PLTOFF",
            Self::Pcrel => "PCREL",
            Self::Secrel => "SECREL",
            Self::Tls => "TLS",
            Self::Tlsgd => "TLSGD",
            Self::Tlsld => "TLSLD",
            Self::Tlsldm => "TLSLDM",
            Self::Tlscall => "TLSCALL",
            Self::Tlsdesc => "TLSDESC",
            Self::Tprel => "TPREL",
            Self::Tpoff => "TPOFF",
            Self::Dtprel => "DTPREL",
            Self::Dtpoff => "DTPOFF",
            Self::Ntpoff => "NTPOFF",
            Self::Dtpmod => "DTPMOD",
            Self::Page => "PAGE",
            Self::Pageoff => "PAGEOFF",
            Self::Call => "CALL",
            Self::Lo => "LO",
            Self::Hi => "HI",
            Self::Higher => "HIGHER",
            Self::Highest => "HIGHEST",
            Self::Gotntpoff => "GOTNTPOFF",
            Self::Indntpoff => "INDNTPOFF",
            Self::G0 => "G0",
            Self::G1 => "G1",
            Self::G2 => "G2",
            Self::G3 => "G3",
            Self::Upper16 => "UPPER16",
            Self::Lower16 => "LOWER16",
            Self::Lo12 => "LO12",
            Self::Lo15 => "LO15",
            Self::Lo14 => "LO14",
            Self::Hi12 => "HI12",
            Self::Hi21 => "HI21",
            Self::S => "S",
            Self::Pg => "PG",
            Self::Nc => "NC",
            Self::Abs => "ABS",
            Self::Prel => "PREL",
            Self::Prel31 => "PREL31",
            Self::Target1 => "TARGET1",
            Self::Target2 => "TARGET2",
            Self::Sbrel => "SBREL",
            Self::Tlsldo => "TLSLDO",
            Self::Hi16 => "HI16",
            Self::Lo16 => "LO16",
            Self::Gprel => "GPREL",
            Self::Disp => "DISP",
            Self::Ofst => "OFST",
            Self::H => "H",
            Self::L => "L",
            Self::Ha => "HA",
            Self::High => "HIGH",
            Self::Higha => "HIGHA",
            Self::Highera => "HIGHERA",
            Self::Highesta => "HIGHESTA",
            Self::Tocbase => "TOCBASE",
            Self::Toc => "TOC",
            Self::Notoc => "NOTOC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GOT" => Some(Self::Got),
            "GOTPC" => Some(Self::Gotpc),
            "GOTOFF" => Some(Self::Gotoff),
            "GOTREL" => Some(Self::Gotrel),
            "PLT" => Some(Self::Plt),
            "PLTOFF" => Some(Self::Pltoff),
            "PCREL" => Some(Self::Pcrel),
            "SECREL" => Some(Self::Secrel),
            "TLS" => Some(Self::Tls),
            "TLSGD" => Some(Self::Tlsgd),
            "TLSLD" => Some(Self::Tlsld),
            "TLSLDM" => Some(Self::Tlsldm),
            "TLSCALL" => Some(Self::Tlscall),
            "TLSDESC" => Some(Self::Tlsdesc),
            "TPREL" => Some(Self::Tprel),
            "TPOFF" => Some(Self::Tpoff),
            "DTPREL" => Some(Self::Dtprel),
            "DTPOFF" => Some(Self::Dtpoff),
            "NTPOFF" => Some(Self::Ntpoff),
            "DTPMOD" => Some(Self::Dtpmod),
            "PAGE" => Some(Self::Page),
            "PAGEOFF" => Some(Self::Pageoff),
            "CALL" => Some(Self::Call),
            "LO" => Some(Self::Lo),
            "HI" => Some(Self::Hi),
            "HIGHER" => Some(Self::Higher),
            "HIGHEST" => Some(Self::Highest),
            "GOTNTPOFF" => Some(Self::Gotntpoff),
            "INDNTPOFF" => Some(Self::Indntpoff),
            "G0" => Some(Self::G0),
            "G1" => Some(Self::G1),
            "G2" => Some(Self::G2),
            "G3" => Some(Self::G3),
            "UPPER16" => Some(Self::Upper16),
            "LOWER16" => Some(Self::Lower16),
            "LO12" => Some(Self::Lo12),
            "LO15" => Some(Self::Lo15),
            "LO14" => Some(Self::Lo14),
            "HI12" => Some(Self::Hi12),
            "HI21" => Some(Self::Hi21),
            "S" => Some(Self::S),
            "PG" => Some(Self::Pg),
            "NC" => Some(Self::Nc),
            "ABS" => Some(Self::Abs),
            "PREL" => Some(Self::Prel),
            "PREL31" => Some(Self::Prel31),
            "TARGET1" => Some(Self::Target1),
            "TARGET2" => Some(Self::Target2),
            "SBREL" => Some(Self::Sbrel),
            "TLSLDO" => Some(Self::Tlsldo),
            "HI16" => Some(Self::Hi16),
            "LO16" => Some(Self::Lo16),
            "GPREL" => Some(Self::Gprel),
            "DISP" => Some(Self::Disp),
            "OFST" => Some(Self::Ofst),
            "H" => Some(Self::H),
            "L" => Some(Self::L),
            "HA" => Some(Self::Ha),
            "HIGH" => Some(Self::High),
            "HIGHA" => Some(Self::Higha),
            "HIGHERA" => Some(Self::Highera),
            "HIGHESTA" => Some(Self::Highesta),
            "TOCBASE" => Some(Self::Tocbase),
            "TOC" => Some(Self::Toc),
            "NOTOC" => Some(Self::Notoc),
            _ => None,
        }
    }
}
