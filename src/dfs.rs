use crate::Graph;

/// Performs dfs starting from a single node
/// since it starts from a single node and there might be disconnected units in the Graph
/// then it is possible that some nodes will not be visited
pub(crate) fn dfs_basic<F>(g: &Graph, start_node: usize, mut apply_fn: F)
where
    F: FnMut(usize, usize),
{
    let mut seen = vec![false; g.num_of_nodes()];
    dfs_recursive_basic(g, start_node, &mut seen, &mut apply_fn);
}

/// Performs depth first search but ensures every node is visited
pub(crate) fn dfs_basic_all<F>(g: &Graph, mut apply_fn: F)
where
    F: FnMut(usize, usize),
{
    let mut seen = vec![false; g.num_of_nodes()];
    for i in 0..g.num_of_nodes() {
        if !seen[i] {
            dfs_recursive_basic(g, i, &mut seen, &mut apply_fn);
        }
    }
}

/// Helper function to recursively apply the dfs core logic
pub(crate) fn dfs_recursive_basic<F>(
    g: &Graph,
    start_node: usize,
    seen: &mut Vec<bool>,
    mut apply_fn: F,
) where
    F: FnMut(usize, usize),
{
    let current_node = &g.nodes[start_node];
    for neighbor in current_node.get_neighbors() {
        if !seen[neighbor] {
            apply_fn(start_node, neighbor);
            dfs_recursive_basic(g, neighbor, seen, &mut apply_fn);
        }
    }
}
