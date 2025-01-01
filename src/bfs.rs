use crate::Graph;
use std::collections::VecDeque;

pub(crate) fn bfs(g: &Graph, start_node: usize) -> Vec<Option<usize>> {
    let mut seen = vec![false; g.num_of_nodes()];
    let mut prev_node_list = vec![None; g.num_of_nodes()];
    let mut queue = VecDeque::from([start_node]);

    seen[start_node] = true;

    while let Some(node_id) = queue.pop_front() {
        for neighbor in g.nodes[node_id].get_neighbors() {
            if !seen[neighbor] {
                seen[neighbor] = true;
                prev_node_list[neighbor] = Some(node_id);
                queue.push_back(neighbor);
            }
        }
    }

    prev_node_list
}

#[cfg(test)]
mod tests {
    use crate::bfs::{self, bfs};
    use crate::tests::ten_node_undirected_graph;

    #[test]
    fn test_bfs() {
        let g = ten_node_undirected_graph();
        assert_eq!(
            bfs(&g, 0)
                .into_iter()
                .map(|v| v.map(|v| v as isize).unwrap_or(-1))
                .collect::<Vec<_>>(),
            vec![-1, 0, 1, 2, 2, 0, 5, 0, 5, 8]
        );
    }
}
