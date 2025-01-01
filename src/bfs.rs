use crate::Graph;
use std::collections::VecDeque;

pub(crate) fn bfs(g: &Graph, start_node: usize) -> Vec<Option<usize>> {
    let mut seen = vec![false; g.num_of_nodes()];
    let mut prev_node_list = vec![None; g.num_of_nodes()];
    let mut queue = VecDeque::from([start_node]);

    while let Some(node_id) = queue.pop_front() {
        if !seen[node_id] {
            seen[node_id] = true;
            for neighbor in g.nodes[node_id].get_neighbors() {
                if !seen[neighbor] {
                    prev_node_list[neighbor] = Some(node_id);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    prev_node_list
}
