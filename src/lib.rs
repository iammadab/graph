use std::collections::{HashMap, HashSet};

#[derive(Clone)]
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
#[derive(Clone)]
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

    fn get_neighbors(&self) -> HashSet<usize> {
        self.edges.values().map(|edge| edge.to).collect()
    }

    /// Assumes directed graph, returns edges that have this node as the from node
    fn get_out_neighbors(&self) -> HashSet<usize> {
        self.get_neighbors()
    }

    /// Represents the number of edges connected to a node
    fn degree(&self) -> usize {
        self.edges.len()
    }

    /// Represents the number of outgoing edges from a node
    fn out_degree(&self) -> usize {
        self.degree()
    }
}

#[derive(Clone)]
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

    fn insert_edge(&mut self, from: usize, to: usize, weight: usize) {
        self.nodes[from].add_edge(to, weight);
        if self.undirected {
            self.nodes[to].add_edge(from, weight);
        }
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        self.nodes[from].remove_edge(to);
        if self.undirected {
            self.nodes[to].remove_edge(from);
        }
    }

    fn insert_node(&mut self) -> &Node {
        let new_node = Node::new(self.num_of_nodes());
        self.nodes.push(new_node);
        &self.nodes[self.num_of_nodes() - 1]
    }

    /// Returns the list of all nodes that point to the target node
    /// Assumes directed graph
    fn get_in_neighbors(&self, target_node: usize) -> HashSet<usize> {
        self.nodes
            .iter()
            .filter(|node| {
                for edge in node.get_edge_list() {
                    if edge.to == target_node {
                        return true;
                    }
                }
                false
            })
            .map(|node| node.index)
            .collect()
    }

    /// Returns the total number of incoming edges to a node
    fn in_degree(&self, target_node: usize) -> usize {
        self.get_in_neighbors(target_node).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn directed_graph() -> Graph {
        let mut g = Graph::new(5, false);
        g.insert_edge(0, 1, 1);
        g.insert_edge(0, 3, 1);
        g.insert_edge(0, 4, 3);
        g.insert_edge(1, 2, 2);
        g.insert_edge(1, 4, 1);
        g.insert_edge(3, 4, 3);
        g.insert_edge(4, 2, 3);
        g.insert_edge(4, 3, 3);
        g.insert_edge(0, 1, 1);
        g
    }
}
