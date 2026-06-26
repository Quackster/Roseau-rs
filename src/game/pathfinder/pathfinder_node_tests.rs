use super::*;

#[test]
fn node_defaults_match_java_node_state() {
    let node = PathfinderNode::new(Position::new(1, 2, 0.0));

    assert_eq!(node.position(), Position::new(1, 2, 0.0));
    assert_eq!(node.cost(), i32::MAX);
    assert!(!node.is_in_open());
    assert!(!node.is_in_closed());
    assert!(node.next_node().is_none());
}

#[test]
fn compares_nodes_by_position() {
    let a = PathfinderNode::new(Position::new(1, 2, 0.0));
    let b = PathfinderNode::new(Position::new(1, 2, 5.0));

    assert!(a.equals_position(&b));
}
