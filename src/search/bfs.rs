use std::collections::VecDeque;

use crate::graph::{graph_path::PrevNodeGraphPath, Graph, Node, VisitedTracker};

pub(crate) fn bfs<G: Graph>(graph: &G, start_node: usize) -> PrevNodeGraphPath {
    let mut visited_tracker = graph.visited_tracker();

    // init queue and set start_node to seen
    let mut queue = VecDeque::from([start_node]);
    visited_tracker.set_seen(start_node);

    while let Some(node_id) = queue.pop_front() {
        for neighbor in graph.node(node_id).unwrap().neighbors() {
            if !visited_tracker.has_seen(neighbor) {
                visited_tracker.set_seen(neighbor);
                visited_tracker.set_prev(neighbor, node_id);
                queue.push_back(neighbor);
            }
        }
    }

    visited_tracker.prev_node_list()
}
