pub(crate) mod static_graph;

// TODO: add documentation
pub(crate) trait Graph {
    type NodeType: Node;

    fn get_node(&self, node_id: usize) -> Option<&Self::NodeType>;
    fn num_of_nodes(&self) -> Option<usize>;
}

pub(crate) trait Node {
    fn neighbors(&self) -> &[usize];
}
