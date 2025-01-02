use crate::graph::{Graph, Node};

struct StaticNode {
    edges: Vec<usize>,
}

impl Node for StaticNode {
    fn neighbors(&self) -> &[usize] {
        &self.edges
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
