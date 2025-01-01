pub(crate) trait Graph<'a> {
    type NodeType: Node;

    fn get_node(node_id: usize) -> Option<&'a Self::NodeType>;
}

pub(crate) trait Node {
    fn neighbors(&self) -> Vec<usize>;
}
