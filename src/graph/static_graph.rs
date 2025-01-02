//! Represents a graph whose nodes and edges are known before hand
use crate::graph::{Edge, Graph, GraphType, Node, NodeId, Weight};

struct StaticNode {
    index: usize,
    edges: Vec<Edge>,
}

impl Node for StaticNode {
    fn neighbors(&self) -> impl Iterator<Item = NodeId> {
        self.edges.iter().map(|e| e.0)
    }
}

impl StaticNode {
    fn new(index: usize) -> Self {
        Self {
            index,
            edges: vec![],
        }
    }

    fn add_edge(&mut self, neighbor: NodeId, weight: Weight) {
        self.edges.push((neighbor, weight))
    }
}

struct StaticGraph {
    nodes: Vec<StaticNode>,
    graph_type: GraphType,
}

impl Graph for StaticGraph {
    type NodeType = StaticNode;

    fn node(&self, node_id: usize) -> Option<&Self::NodeType> {
        self.nodes.get(node_id)
    }

    fn num_of_nodes(&self) -> Option<usize> {
        Some(self.nodes.len())
    }

    fn graph_type(&self) -> &GraphType {
        &self.graph_type
    }
}

impl StaticGraph {
    fn new(node_count: usize, graph_type: GraphType) -> Self {
        Self {
            nodes: (0..node_count).map(StaticNode::new).collect(),
            graph_type,
        }
    }

    fn insert_edge(&mut self, from: NodeId, to: NodeId, weight: Weight) {
        self.nodes[from].add_edge(to, weight);
        if self.graph_type == GraphType::Undirected {
            self.nodes[to].add_edge(from, weight);
        }
    }
}
