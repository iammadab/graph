//! Represents a graph whose nodes and edges are known before hand
use std::collections::BTreeMap;

use crate::graph::{Graph, GraphType, Node, NodeId, NodeTrackState, Weight};

use super::VisitedTracker;

pub(crate) struct StaticNode {
    index: usize,
    edges: BTreeMap<NodeId, Weight>,
}

impl Node for StaticNode {
    fn neighbors(&self) -> impl Iterator<Item = &NodeId> {
        self.edges.keys().into_iter()
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

impl Graph for StaticGraph {
    type NodeType = StaticNode;
    type Trakcer = StaticTracker;

    fn node(&self, node_id: usize) -> Option<&Self::NodeType> {
        self.nodes.get(node_id)
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

pub(crate) struct StaticTracker {
    state: Vec<NodeTrackState>,
}

impl StaticTracker {
    fn new(node_count: usize) -> Self {
        Self {
            state: vec![NodeTrackState(false, None); node_count],
        }
    }
}

impl VisitedTracker for StaticTracker {
    fn has_seen(&self, node_id: NodeId) -> bool {
        self.state[node_id].0
    }

    fn set_seen(&mut self, node_id: NodeId) {
        self.state[node_id].0 = true;
    }

    fn set_prev(&mut self, node_id: NodeId, prev_node_id: NodeId) {
        self.state[node_id].1 = Some(prev_node_id);
    }

    fn prev_node_list(&self) -> Vec<Option<NodeId>> {
        self.state.iter().map(|v| v.1).collect()
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
