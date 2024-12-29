use std::collections::HashMap;

/// Represents a graph edge
struct Edge {
    from: u8,
    to: u8,
    weight: usize,
}

impl Edge {
    fn new(from: u8, to: u8, weight: usize) -> Self {
        Self { from, to, weight }
    }
}

/// Represents a graph node
struct Node {
    index: u8,
    edges: HashMap<u8, Edge>,
}

impl Node {
    fn new(index: u8) -> Self {
        Self {
            index,
            edges: HashMap::new(),
        }
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn get_edge(&self, neighbor: u8) -> Option<&Edge> {
        self.edges.get(&neighbor)
    }

    fn add_edge(&mut self, neighbor: u8, weight: usize) {
        self.edges
            .insert(neighbor, Edge::new(self.index, neighbor, weight));
    }

    fn remove_edge(&mut self, neighbor: u8) {
        self.edges.remove(&neighbor);
    }

    fn get_edge_list(&self) -> Vec<&Edge> {
        self.edges.values().collect()
    }

    fn get_sorted_edge_list(&self) -> Vec<&Edge> {
        let mut sorted_neighbors = self.edges.keys().collect::<Vec<_>>();
        sorted_neighbors.sort();
        sorted_neighbors
            .into_iter()
            .map(|n| &self.edges[n])
            .collect::<Vec<_>>()
    }
}
