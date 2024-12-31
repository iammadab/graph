use crate::Graph;

/// path representation => list of nodes
/// path descrption: go from node i to node i + 1
/// validity constraint is that an edge exists between sliding window sized 2 pair of nodes
pub(crate) fn check_node_path_valid(g: &Graph, path: &[usize]) -> bool {
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

/// path representation => list of edges
/// path description: after traversing edge i, traverse edge i + 1
/// validity constraint: each edge must be an actual edge, the destination of edge i should be that
/// start of edge i + 1
pub(crate) fn check_edge_path_valid(g: &Graph, path: &[(usize, usize)]) -> bool {
    // empty paths are considered valid
    if path.is_empty() {
        return true;
    }

    for i in 1..path.len() {
        // validate first edge
        if i == 1 && !g.is_edge(path[i - 1].0, path[i - 1].1) {
            return false;
        }

        if !g.is_edge(path[i].0, path[i].1) {
            return false;
        }

        // ensure destination of previous edge is the start of current edge
        if path[i - 1].1 != path[i].0 {
            return false;
        }
    }

    true
}

/// TODO: add documentation
pub(crate) fn check_previous_node_list_valid(g: &Graph, path: &[Option<usize>]) -> bool {
    // should have an entry for each node in the graph
    if path.len() != g.num_of_nodes() {
        return false;
    }

    // every index in path represents a node
    // the contents of the index represents a node that can get to it
    // hence (content, node) should represent an edge
    for (node_id, maybe_previous_id) in path.iter().enumerate() {
        if let Some(previous_id) = maybe_previous_id {
            if !g.is_edge(*previous_id, node_id) {
                return false;
            }
        }
    }

    true
}

/// Converts from a list of previous nodes to a node list representation for a given destination
/// node
pub(crate) fn node_list_from_prev_node_list(
    prev_node_list: &[Option<usize>],
    destination: usize,
) -> Vec<usize> {
    let mut node_list = vec![];
    let mut current = Some(destination);

    while let Some(curr_index) = current {
        node_list.push(curr_index);
        current = prev_node_list[curr_index]
    }

    node_list.reverse();
    node_list
}

pub(crate) fn path_cost(g: &Graph, path: &[(usize, usize)]) -> f64 {
    // verify the path is valid
    check_edge_path_valid(g, path);

    // add up each edge weight
    path.iter()
        .map(|p| g.get_edge(p.0, p.1).unwrap().weight)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{
        path::{check_edge_path_valid, check_node_path_valid},
        tests::undirected_graph,
    };

    use super::node_list_from_prev_node_list;

    #[test]
    fn test_path_node_list_valid() {
        let graph = undirected_graph();
        assert!(check_node_path_valid(&graph, &vec![1, 2, 4, 5]));
        assert!(!check_node_path_valid(&graph, &vec![2, 5, 4, 3]));
    }

    #[test]
    fn test_path_edge_list_valid() {
        let graph = undirected_graph();
        assert!(check_edge_path_valid(
            &graph,
            &vec![(0, 1), (1, 4), (4, 5), (5, 2)]
        ));
        assert!(!check_edge_path_valid(
            &graph,
            &vec![(0, 1), (1, 3), (4, 5), (5, 2)]
        ));
    }

    #[test]
    fn test_prev_node_list_to_path_node_list() {
        let prev_node_list = vec![-1, 0, 1, 2, 2, 0, 5, 0, 5, 8]
            .into_iter()
            .map(|v| if v == -1 { None } else { Some(v as usize) })
            .collect::<Vec<Option<usize>>>();
        assert_eq!(
            node_list_from_prev_node_list(&prev_node_list, 9),
            vec![0, 5, 8, 9]
        );
    }
}
