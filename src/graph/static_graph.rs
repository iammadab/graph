//! Represents a graph whose nodes and edges are known before hand
use crate::tracker::StaticTracker;
use std::collections::BTreeMap;

use crate::graph::{Graph, GraphType, Node, NodeId, Weight};

pub(crate) struct StaticNode {
    index: usize,
    edges: BTreeMap<NodeId, Weight>,
}

impl Node<NodeId> for StaticNode {
    fn neighbors(&self) -> impl Iterator<Item = &NodeId> {
        self.edges.keys()
    }
}

impl StaticNode {
    fn new(index: usize) -> Self {
        Self {
            index,
            edges: BTreeMap::new(),
        }
    }

    fn add_edge(&mut self, neighbor: NodeId, weight: Weight) {
        self.edges.insert(neighbor, weight);
    }
}

pub(crate) struct StaticGraph {
    nodes: Vec<StaticNode>,
    graph_type: GraphType,
}

impl Graph<NodeId> for StaticGraph {
    type NodeType = StaticNode;
    type Trakcer = StaticTracker;

    fn node(&self, node_id: &NodeId) -> Option<&Self::NodeType> {
        self.nodes.get(*node_id)
    }

    fn num_of_nodes(&self) -> Option<usize> {
        Some(self.nodes.len())
    }

    fn graph_type(&self) -> &GraphType {
        &self.graph_type
    }

    fn visited_tracker(&self) -> Self::Trakcer {
        Self::Trakcer::new(self.nodes.len())
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn undirected_graph() -> StaticGraph {
        let mut g = StaticGraph::new(6, GraphType::Undirected);
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

    pub(crate) fn directed_graph() -> StaticGraph {
        let mut g = StaticGraph::new(6, GraphType::Directed);
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

    pub(crate) fn weighted_directed_graph() -> StaticGraph {
        let mut g = StaticGraph::new(6, GraphType::Directed);
        g.insert_edge(0, 1, 5.0);
        g.insert_edge(0, 3, 1.0);
        g.insert_edge(0, 4, 2.5);
        g.insert_edge(1, 0, 4.0);
        g.insert_edge(1, 2, 1.0);
        g.insert_edge(2, 1, 3.0);
        g.insert_edge(3, 4, 3.0);
        g.insert_edge(4, 1, 1.0);
        g.insert_edge(4, 2, 5.0);
        g.insert_edge(4, 3, 1.0);
        g.insert_edge(4, 5, 2.0);
        g.insert_edge(5, 2, 1.0);
        g.insert_edge(5, 4, 1.0);
        g
    }

    pub(crate) fn ten_node_undirected_graph() -> StaticGraph {
        let mut g = StaticGraph::new(10, GraphType::Undirected);
        g.insert_edge(0, 1, 0.0);
        g.insert_edge(0, 5, 0.0);
        g.insert_edge(0, 7, 0.0);
        g.insert_edge(1, 2, 0.0);
        g.insert_edge(2, 3, 0.0);
        g.insert_edge(2, 5, 0.0);
        g.insert_edge(2, 4, 0.0);
        g.insert_edge(4, 9, 0.0);
        g.insert_edge(5, 6, 0.0);
        g.insert_edge(5, 8, 0.0);
        g.insert_edge(6, 8, 0.0);
        g.insert_edge(7, 8, 0.0);
        g.insert_edge(8, 9, 0.0);
        g
    }

    pub(crate) fn disconnected_undirected_graph() -> StaticGraph {
        let mut g = StaticGraph::new(8, GraphType::Undirected);
        g.insert_edge(0, 1, 0.0);
        g.insert_edge(0, 4, 0.0);
        g.insert_edge(1, 2, 0.0);
        g.insert_edge(3, 7, 0.0);
        g.insert_edge(5, 6, 0.0);
        g
    }
}
