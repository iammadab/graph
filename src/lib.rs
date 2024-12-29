use std::collections::HashMap;

/// Represents a graph edge
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
}

impl Edge {
    fn new(from: usize, to: usize, weight: usize) -> Self {
        Self { from, to, weight }
    }
}

/// Represents a graph node
struct Node {
    index: usize,
    edges: HashMap<usize, Edge>,
}

impl Node {
    fn new(index: usize) -> Self {
        Self {
            index,
            edges: HashMap::new(),
        }
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn get_edge(&self, neighbor: usize) -> Option<&Edge> {
        self.edges.get(&neighbor)
    }

    fn add_edge(&mut self, neighbor: usize, weight: usize) {
        self.edges
            .insert(neighbor, Edge::new(self.index, neighbor, weight));
    }

    fn remove_edge(&mut self, neighbor: usize) {
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

/// Represents the full Graph structure
struct Graph {
    nodes: Vec<Node>,
    undirected: bool,
}

impl Graph {
    fn new(num_of_nodes: usize, undirected: bool) -> Self {
        Self {
            nodes: (0..num_of_nodes).map(Node::new).collect(),
            undirected,
        }
    }

    fn num_of_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn get_edge(&self, from: usize, to: usize) -> Option<&Edge> {
        if from >= self.num_of_nodes() || to >= self.num_of_nodes() {
            panic!("invalid edge id");
        }

        self.nodes[from].get_edge(to)
    }

    fn is_edge(&self, from: usize, to: usize) -> bool {
        self.get_edge(from, to).is_some()
    }

    fn make_edge_list(&self) -> Vec<&Edge> {
        self.nodes
            .iter()
            .flat_map(|node| node.get_edge_list())
            .collect()
    }
}
