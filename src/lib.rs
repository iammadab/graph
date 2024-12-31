// TODO: remove this
#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    iter::{empty, once},
};

mod clustering;

#[derive(Clone, Debug)]
/// Represents a graph edge
struct Edge {
    from: usize,
    to: usize,
    weight: f64,
}

impl Edge {
    fn new(from: usize, to: usize, weight: f64) -> Self {
        Self { from, to, weight }
    }
}

/// Represents a graph node
#[derive(Clone, Debug)]
pub(crate) struct Node {
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

    fn add_edge(&mut self, neighbor: usize, weight: f64) {
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

#[derive(Clone, Debug)]
/// Represents the full Graph structure
pub(crate) struct Graph {
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

    fn insert_edge(&mut self, from: usize, to: usize, weight: f64) {
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

    /// Generates a subgraph for a target node
    /// if closed the subgraph will contain the target node, its neighbors and all edges between
    /// the relevant nodes
    /// if not closed, same as above but removes the target node and its edges
    fn neighborhood_subgraph(self, target_node: usize, closed: bool) -> Self {
        if !self.undirected {
            panic!("neighborhood_subgraph only implemented for undirected graphs");
        }

        let mut nodes_in_subgraph = self.nodes[target_node].get_neighbors();
        if closed {
            nodes_in_subgraph.insert(target_node);
        }

        // the new graph will have nodes indexed contiguously from 0
        // which might be different from their old label in global graph
        let index_map: HashMap<usize, usize> = nodes_in_subgraph
            .iter()
            .enumerate()
            .map(|(new_index, old_index)| (*old_index, new_index))
            .collect();

        dbg!(&index_map);

        // construct subgraph
        let mut graph = Graph::new(nodes_in_subgraph.len(), true);

        for node in &nodes_in_subgraph {
            for edge in self.nodes[*node].get_edge_list() {
                // only keep edges whose to and from nodes exist in nodes to use
                // we add arbitrary ordering constraints to prevent duplication
                if nodes_in_subgraph.contains(&edge.to) && edge.to > *node {
                    graph.insert_edge(
                        *index_map.get(&edge.to).unwrap(),
                        *index_map.get(&edge.from).unwrap(),
                        edge.weight,
                    );
                }
            }
        }

        graph
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn undirected_graph() -> Graph {
        let mut g = Graph::new(6, true);
        g.insert_edge(0, 1, 1.0);
        g.insert_edge(0, 3, 1.0);
        g.insert_edge(0, 4, 1.0);
        g.insert_edge(1, 4, 1.0);
        g.insert_edge(1, 2, 1.0);
        g.insert_edge(2, 4, 1.0);
        g.insert_edge(2, 5, 1.0);
        g.insert_edge(4, 5, 1.0);
        g
    }

    pub(crate) fn directed_graph() -> Graph {
        let mut g = Graph::new(6, false);
        g.insert_edge(0, 1, 1.0);
        g.insert_edge(0, 3, 1.0);
        g.insert_edge(1, 2, 1.0);
        g.insert_edge(1, 4, 1.0);
        g.insert_edge(4, 0, 1.0);
        g.insert_edge(4, 2, 1.0);
        g.insert_edge(2, 5, 1.0);
        g.insert_edge(2, 2, 1.0);
        g.insert_edge(5, 2, 1.0);
        g.insert_edge(5, 4, 1.0);
        g
    }

    /// Takes the values in the vec as roots of some polynomial
    /// and evaluates it at one point for identity testing
    fn perm(values: Vec<usize>) -> usize {
        values.iter().map(|v| v + 45).product()
    }

    #[test]
    fn test_neighbors() {
        let graph = undirected_graph();
        assert_eq!(
            perm(
                graph.nodes[0]
                    .get_neighbors()
                    .into_iter()
                    .collect::<Vec<_>>()
            ),
            perm(vec![1, 3, 4])
        );
        assert_eq!(
            perm(
                graph.nodes[1]
                    .get_neighbors()
                    .into_iter()
                    .collect::<Vec<_>>()
            ),
            perm(vec![0, 2, 4])
        );
        assert_eq!(
            perm(
                graph.nodes[2]
                    .get_neighbors()
                    .into_iter()
                    .collect::<Vec<_>>()
            ),
            perm(vec![1, 4, 5])
        );
        assert_eq!(
            perm(
                graph.nodes[3]
                    .get_neighbors()
                    .into_iter()
                    .collect::<Vec<_>>()
            ),
            perm(vec![0])
        );
        assert_eq!(
            perm(
                graph.nodes[4]
                    .get_neighbors()
                    .into_iter()
                    .collect::<Vec<_>>()
            ),
            perm(vec![0, 1, 2, 5])
        );
        assert_eq!(
            perm(
                graph.nodes[5]
                    .get_neighbors()
                    .into_iter()
                    .collect::<Vec<_>>()
            ),
            perm(vec![2, 4])
        );
    }

    #[test]
    fn test_degree() {
        let graph = undirected_graph();
        assert_eq!(graph.nodes[0].degree(), 3);
        assert_eq!(graph.nodes[1].degree(), 3);
        assert_eq!(graph.nodes[2].degree(), 3);
        assert_eq!(graph.nodes[3].degree(), 1);
        assert_eq!(graph.nodes[4].degree(), 4);
        assert_eq!(graph.nodes[5].degree(), 2);
    }

    #[test]
    fn test_in_degree() {
        let graph = directed_graph();
        assert_eq!(graph.in_degree(0), 1);
        assert_eq!(graph.in_degree(1), 1);
        assert_eq!(graph.in_degree(2), 4);
        assert_eq!(graph.in_degree(3), 1);
        assert_eq!(graph.in_degree(4), 2);
        assert_eq!(graph.in_degree(5), 1);
    }

    #[test]
    fn test_out_degree() {
        let graph = directed_graph();
        assert_eq!(graph.nodes[0].out_degree(), 2);
        assert_eq!(graph.nodes[1].out_degree(), 2);
        assert_eq!(graph.nodes[2].out_degree(), 2);
        assert_eq!(graph.nodes[3].out_degree(), 0);
        assert_eq!(graph.nodes[4].out_degree(), 2);
        assert_eq!(graph.nodes[5].out_degree(), 2);
    }
}
