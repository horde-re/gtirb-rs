#[derive(Clone, Copy, PartialEq)]
pub struct EdgeLabel {
    pub conditional: bool,

    pub direct: bool,

    pub r#type: i32,
}
#[derive(Clone, PartialEq)]
pub struct Edge {
    pub source_uuid: Vec<u8>,

    pub target_uuid: Vec<u8>,

    pub label: ::core::option::Option<EdgeLabel>,
}
#[derive(Clone, PartialEq)]
pub struct Cfg {
    pub vertices: Vec<Vec<u8>>,

    pub edges: Vec<Edge>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum EdgeType {
    TypeBranch = 0,
    TypeCall = 1,
    TypeFallthrough = 2,
    TypeReturn = 3,
    TypeSyscall = 4,
    TypeSysret = 5,
}
impl EdgeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::TypeBranch => "Type_Branch",
            Self::TypeCall => "Type_Call",
            Self::TypeFallthrough => "Type_Fallthrough",
            Self::TypeReturn => "Type_Return",
            Self::TypeSyscall => "Type_Syscall",
            Self::TypeSysret => "Type_Sysret",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Type_Branch" => Some(Self::TypeBranch),
            "Type_Call" => Some(Self::TypeCall),
            "Type_Fallthrough" => Some(Self::TypeFallthrough),
            "Type_Return" => Some(Self::TypeReturn),
            "Type_Syscall" => Some(Self::TypeSyscall),
            "Type_Sysret" => Some(Self::TypeSysret),
            _ => None,
        }
    }
}
