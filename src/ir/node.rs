//! Node is the root *class* for many [GTIRB components](https://grammatech.github.io/gtirb/md__c_o_m_p_o_n_e_n_t_s.html).

use serde::{Deserialize, Serialize};

/// Parent trait for many GTIRB components.
pub trait Node {
    type UUID: Eq + Default;

    fn get_uuid(&self) -> &Self::UUID;
    fn get_type(&self) -> NodeType;
}

/// Type alias for a GTIRB node with a UUID.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum NodeType {
    #[default]
    Node,
    CfgNode,
    CodeBlock,
    ProxyBlock,
    DataBlock,
    Ir,
    Module,
    Section,
    Symbol,
    ByteInterval,
}
