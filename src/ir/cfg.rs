use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct Cfg {
    pub vertices: Vec<Vec<u8>>,
    pub edges: Vec<Edge>,
}

#[derive(Clone, PartialEq)]
pub struct Edge {
    pub source_uuid: Uuid,
    pub target_uuid: Uuid,
    pub label: Option<EdgeLabel>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct EdgeLabel {
    pub conditional: bool,
    pub direct: bool,
    pub r#type: EdgeType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum EdgeType {
    Branch,
    Call,
    Fallthrough,
    Return,
    Syscall,
    Sysret,
}
