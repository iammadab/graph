use std::collections::VecDeque;

use crate::graph::{graph_path::PrevNodeGraphPath, Graph, Node};
use crate::tracker::VisitedTracker;

pub(crate) fn bfs<T: Clone, G: Graph<T>>(graph: &G, start_node: T) -> PrevNodeGraphPath {
    let mut visited_tracker = graph.visited_tracker();

    // init queue and set start_node to seen
    visited_tracker.set_seen(&start_node);
    let mut queue = VecDeque::from([start_node]);

    while let Some(node_id) = queue.pop_front() {
        for neighbor in graph.node(&node_id).unwrap().neighbors() {
            if !visited_tracker.has_seen(neighbor) {
                visited_tracker.set_seen(neighbor);
                visited_tracker.set_prev(neighbor, &node_id);
                queue.push_back(neighbor.clone());
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
    fn test_bfs() {
        let graph = ten_node_undirected_graph();
        assert_eq!(
            prev_node_graph_path_to_isize_vec(&bfs(&graph, 0)),
            vec![-1, 0, 1, 2, 2, 0, 5, 0, 5, 8]
        );
    }
}
