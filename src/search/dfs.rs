use crate::graph::{Graph, Node, NodeId, VisitedTracker};

pub(crate) fn dfs_stack_path<G: Graph>(graph: &G, start_node: NodeId) -> Vec<Option<NodeId>> {
    let mut stack = vec![start_node];
    let mut visited_tracker = graph.visited_tracker();

    while let Some(node_id) = stack.pop() {
        if !visited_tracker.has_seen(node_id) {
            dbg!(node_id);
            visited_tracker.set_seen(node_id);
            let current_node = graph.node(node_id).unwrap();
            for neighbor in current_node.neighbors() {
                if !visited_tracker.has_seen(neighbor) {
                    visited_tracker.set_prev(neighbor, node_id);
                    stack.push(neighbor);
                }
            }
        }
    }

    visited_tracker.prev_node_list()
}
