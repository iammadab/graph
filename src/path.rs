use crate::Graph;

/// path representation => list of nodes
/// path descrption: go from node i to node i + 1
/// validity constraint is that an edge exists between sliding window sized 2 pair of nodes
pub(crate) fn check_node_path_valid(g: &Graph, path: Vec<usize>) -> bool {
    // empty paths are considered valid
    if path.is_empty() {
        return true;
    }

    // ensure that for every pair, there exists an edge between them
    for i in 1..path.len() {
        if !g.is_edge(path[i - 1], path[i]) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::{path::check_node_path_valid, tests::undirected_graph};

    #[test]
    fn test_node_path_valid() {
        let graph = undirected_graph();
        assert!(check_node_path_valid(&graph, vec![1, 2, 4, 5]));
        assert!(!check_node_path_valid(&graph, vec![2, 5, 4, 3]));
    }
}
