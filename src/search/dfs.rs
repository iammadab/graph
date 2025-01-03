use crate::graph::{graph_path::PrevNodeGraphPath, Graph, Node, NodeId};
use crate::tracker::VisitedTracker;

pub(crate) fn dfs<T: Clone, G: Graph<T>>(graph: &G, start_node: T) -> PrevNodeGraphPath {
    let mut stack = vec![start_node];
    let mut visited_tracker = graph.visited_tracker();

    while let Some(node_id) = stack.pop() {
        if !visited_tracker.has_seen(&node_id) {
            visited_tracker.set_seen(&node_id);
            let current_node = graph.node(&node_id).unwrap();
            for neighbor in current_node.neighbors() {
                if !visited_tracker.has_seen(neighbor) {
                    visited_tracker.set_prev(neighbor, &node_id);
                    stack.push(neighbor.clone());
                }
            }
        }
    }

    visited_tracker.prev_node_list()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{
        graph_path::prev_node_graph_path_to_isize_vec,
        static_graph::tests::ten_node_undirected_graph,
    };

    #[test]
    fn test_dfs() {
        let graph = ten_node_undirected_graph();
        assert_eq!(
            prev_node_graph_path_to_isize_vec(&dfs(&graph, 0)),
            vec![-1, 2, 4, 2, 9, 2, 5, 0, 7, 8],
        );
    }
}
