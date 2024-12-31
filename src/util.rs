use core::f64;

use crate::Graph;

/// Measures how degree of connection between a given nodes neighbors
/// 1 -> neigbors are fully connected
/// 0 -> no connection between neighbors
fn clustering_coefficient(g: &Graph, node: usize) -> f64 {
    // the following algorithm only works for undirected graphs
    if !g.undirected {
        panic!("current implementation of clustering coefficient only accepts undirected graphs");
    }

    let neighbors = g.nodes[node].get_neighbors();
    let total_possible_connections = (neighbors.len() * (neighbors.len() - 1)) / 2;

    // count actual connections
    let mut count = 0;
    for neighbor in &neighbors {
        for edge in g.nodes[*neighbor].get_edge_list() {
            // for every connected neighbor u and v
            // they will appear in this loop twice
            // as (u -> v) and as (v -> u)
            // since we only need 1 for the count we can constraint
            // by some arbirary ordering e.g. u > v
            if edge.to > *neighbor && neighbors.contains(&edge.to) {
                count += 1;
            }
        }
    }

    // prevent dividing by 0
    if total_possible_connections == 0 {
        return 0.0;
    }

    count as f64 / total_possible_connections as f64
}

/// Computes the average local clustering coefficient for all nodes in the graph
fn average_clustering_coefficient(g: &Graph) -> f64 {
    let total: f64 = (0..g.num_of_nodes())
        .map(|node_id| clustering_coefficient(g, node_id))
        .sum();

    // prevent dividing by 0
    if g.num_of_nodes() == 0 {
        return 0.0;
    }

    total / g.num_of_nodes() as f64
}

#[cfg(test)]
mod tests {
    use crate::tests::{directed_graph, undirected_graph};

    use super::clustering_coefficient;

    #[test]
    fn test_clustering_coefficient_undirected() {
        let g = undirected_graph();
        assert_eq!(clustering_coefficient(&g, 0), 1.0 / 3.0);
        assert_eq!(clustering_coefficient(&g, 1), 2.0 / 3.0);
        assert_eq!(clustering_coefficient(&g, 2), 2.0 / 3.0);
        assert_eq!(clustering_coefficient(&g, 3), 0.0);
        assert_eq!(clustering_coefficient(&g, 4), 1.0 / 2.0);
        assert_eq!(clustering_coefficient(&g, 5), 1.0);
    }
}
