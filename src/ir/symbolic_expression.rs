use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct SymStackConst {
    pub offset: i32,
    pub symbol_uuid: Uuid,
}

#[derive(Clone, PartialEq)]
pub struct SymAddrConst {
    pub offset: i64,
    pub symbol_uuid: Uuid,
}

#[derive(Clone, PartialEq)]
pub struct SymAddrAddr {
    pub scale: i64,
    pub offset: i64,
    pub symbol1_uuid: Uuid,
    pub symbol2_uuid: Uuid,
}

#[derive(Clone, PartialEq)]
pub struct SymbolicExpression {
    pub attribute_flags: Vec<i32>,
    pub value: Option<SymbolicExpressionValue>,
}

#[derive(Clone, PartialEq)]
pub enum SymbolicExpressionValue {
    AddrConst(SymAddrConst),
    AddrAddr(SymAddrAddr),
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
