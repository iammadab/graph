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
pub fn dfs_recursive_basic<F>(g: &Graph, start_node: usize, seen: &mut Vec<bool>, apply_fn: &mut F)
where
    F: FnMut(usize, usize),
{
    seen[start_node] = true;
    let current_node = &g.nodes[start_node];
    for neighbor in current_node.get_neighbors() {
        if !seen[neighbor] {
            apply_fn(start_node, neighbor);
            dfs_recursive_basic(g, neighbor, seen, apply_fn);
        }
    }
}

/// Performs dfs and records path information in prev_node representation
/// requires a starting node
pub(crate) fn dfs_recursive_path(g: &Graph, start_node: usize) -> Vec<Option<usize>> {
    let mut prev_node_list = vec![None; g.num_of_nodes()];
    let update_list = |prev_node, new_node| {
        prev_node_list[new_node] = Some(prev_node);
    };
    dfs_basic(g, start_node, update_list);
    prev_node_list
}

/// Performs dfs and records path information in prev_node representation
/// doesn't require a starting node
/// this will behave exactly like dfs_recursive_path if there is no partition in the graph
/// but once we have disconnected chunks, basic will only be able to identify 1 partition
/// (based on the starting node)
/// all will be able to identify all partitions, indexes with None, represents starting elements
/// from different partitions
pub(crate) fn dfs_recursive_path_all(g: &Graph) -> Vec<Option<usize>> {
    let mut prev_node_list = vec![None; g.num_of_nodes()];
    let update_list = |prev_node, new_node| {
        prev_node_list[new_node] = Some(prev_node);
    };
    dfs_basic_all(g, update_list);
    prev_node_list
}

pub(crate) fn dfs_stack_path(g: &Graph, start_node: usize) -> Vec<Option<usize>> {
    let mut stack = vec![start_node];
    let mut prev_node_list = vec![None; g.num_of_nodes()];
    let mut seen = vec![false; g.num_of_nodes()];

    while let Some(node_id) = stack.pop() {
        if !seen[node_id] {
            seen[node_id] = true;
            let current_node = &g.nodes[node_id];
            for neighbor in current_node.get_neighbors() {
                if !seen[neighbor] {
                    prev_node_list[neighbor] = Some(node_id);
                    stack.push(neighbor);
                }
            }
        }
    }

    prev_node_list
}

/// Partitions a graph into components, all nodes in a component can reach one another
/// if there is not path between node a and node b then node a will belong in a different
/// component from node b
pub(crate) fn dfs_connected_components(g: &Graph) -> Vec<usize> {
    let mut component_list = vec![None; g.num_of_nodes()];
    let mut curr_comp = 0;

    for node_id in 0..g.num_of_nodes() {
        // if the current node is not visited then dfs
        // all nodes that can be reached from it belongs in the same component
        if component_list[node_id].is_some() {
            continue;
        }
        dfs_recursive_connected_components(g, node_id, &mut component_list, curr_comp);
        // increment the curr component, any unvisited node belongs in a different component
        curr_comp += 1;
    }

    component_list.into_iter().map(|v| v.unwrap()).collect()
}

fn dfs_recursive_connected_components(
    g: &Graph,
    node_id: usize,
    component_list: &mut [Option<usize>],
    curr_component: usize,
) {
    component_list[node_id] = Some(curr_component);
    for neighbor in g.nodes[node_id].get_neighbors() {
        if component_list[neighbor].is_none() {
            dfs_recursive_connected_components(g, neighbor, component_list, curr_component);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dfs::{dfs_connected_components, dfs_recursive_path_all, dfs_stack_path},
        path::check_previous_node_list_valid,
        tests::{disconnected_undirected_graph, ten_node_undirected_graph},
    };

    #[test]
    fn test_dfs_recursive_path_generation() {
        let graph = ten_node_undirected_graph();
        assert!(check_previous_node_list_valid(
            &graph,
            &dfs_recursive_path_all(&graph)
        ));
    }

    #[test]
    fn test_dfs_stack_path_generation() {
        let graph = ten_node_undirected_graph();
        assert!(check_previous_node_list_valid(
            &graph,
            &dfs_stack_path(&graph, 0)
        ));
    }

    #[test]
    fn test_connected_components() {
        let graph = disconnected_undirected_graph();
        assert_eq!(
            dfs_connected_components(&graph),
            vec![0, 0, 0, 1, 0, 2, 2, 1]
        );
    }
}
