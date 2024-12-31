use core::f64;

use crate::Graph;

/// Measures how degree of connection between a given nodes neighbors
/// 1 -> neigbors are fully connected
/// 0 -> no connection between neighbors
fn clustering_coefficient(g: &Graph, node: usize) -> f64 {
    let neighbors = g.nodes[node].get_neighbors();
    let total_possible_connections = (neighbors.len() * (neighbors.len() - 1)) / 2;

    // count the number of actual connections

    // Rough
    // for each neighbor, get their edge list and check if their destination
    // node also exists in the neighbor list
    // some constraints
    // if we have nodes u and v, both in the neighbor list for n
    // then we'd encounter u -> v and v -> u
    // they are the same, hence we should only count them as 1
    // the book chooses to only count when u < v which guarantees only picking 1
    // but what if we have a directed graph??
    // if it is directed, then we don't need to exclude
    // TODO: test the effect of this algorithm on directed graphs

    let mut count = 0;
    for neighbor in &neighbors {
        for edge in g.nodes[*neighbor].get_edge_list() {
            // TODO: explain this condition
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
