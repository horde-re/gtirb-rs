//! Tests for the `node_derive` macro.

use gtirb_rs::ir::block::ProxyBlock;
use gtirb_rs::ir::node::{Node, NodeType};

use impls::impls;
use uuid::Uuid;

/// Test that the `ProxyBlock` struct implements the `Node` trait.
#[test]
fn proxyblock_has_node_trait() {
    assert!(impls!(ProxyBlock: Node));
}

/// Test that we can access `Node` methods on a `ProxyBlock`.
#[test]
fn create_proxyblock() {
    let uuid = Uuid::new_v4();
    let block = ProxyBlock::new(uuid);

    assert_eq!(block.get_uuid(), &uuid);
    assert_eq!(block.get_type(), NodeType::ProxyBlock);
}
