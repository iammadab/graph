pub(crate) mod static_graph;

pub(crate) type NodeId = usize;
pub(crate) type Weight = f64;
pub(crate) type Edge = (NodeId, Weight);

#[derive(PartialEq)]
pub(crate) enum GraphType {
    Directed,
    Undirected,
}

pub(crate) trait Graph {
    type NodeType: Node;

    fn node(&self, node_id: usize) -> Option<&Self::NodeType>;
    fn num_of_nodes(&self) -> Option<usize>;
    fn graph_type(&self) -> &GraphType;
}

pub(crate) trait Node {
    fn neighbors(&self) -> impl Iterator<Item = NodeId>;
}
