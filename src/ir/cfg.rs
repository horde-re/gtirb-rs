use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Cfg {
    pub vertices: Vec<Vec<u8>>,
    pub edges: Vec<Edge>,
}

impl Cfg {
    pub fn new(vertices: Vec<Vec<u8>>, edges: Vec<Edge>) -> Self {
        Self { vertices, edges }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Edge {
    pub source_uuid: Uuid,
    pub target_uuid: Uuid,
    pub label: Option<EdgeLabel>,
}

impl Edge {
    pub fn new(source_uuid: Uuid, target_uuid: Uuid, label: Option<EdgeLabel>) -> Self {
        Self {
            source_uuid,
            target_uuid,
            label,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct EdgeLabel {
    pub conditional: bool,
    pub direct: bool,
    pub r#type: EdgeType,
}

impl EdgeLabel {
    pub fn new(conditional: bool, direct: bool, r#type: EdgeType) -> Self {
        Self {
            conditional,
            direct,
            r#type,
        }
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
#[repr(i32)]
pub enum EdgeType {
    #[default]
    Branch,
    Call,
    Fallthrough,
    Return,
    Syscall,
    Sysret,
}
