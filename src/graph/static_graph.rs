//! Represents a graph whose nodes and edges are known before hand

use crate::graph::{Edge, Graph, Node, NodeId};

struct StaticNode {
    edges: Vec<Edge>,
}

impl Node for StaticNode {
    fn neighbors(&self) -> impl Iterator<Item = NodeId> {
        self.edges.iter().map(|e| e.0)
    }
}

struct StaticGraph {
    nodes: Vec<StaticNode>,
}

impl Graph for StaticGraph {
    type NodeType = StaticNode;

    fn get_node(&self, node_id: usize) -> Option<&Self::NodeType> {
        self.nodes.get(node_id)
    }

    fn num_of_nodes(&self) -> Option<usize> {
        Some(self.nodes.len())
    }
}
