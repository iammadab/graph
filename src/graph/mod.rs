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
    type Trakcer: VisitedTracker;

    fn node(&self, node_id: usize) -> Option<&Self::NodeType>;
    fn num_of_nodes(&self) -> Option<usize>;
    fn graph_type(&self) -> &GraphType;
    fn visited_tracker(&self) -> Self::Trakcer;
}

pub(crate) trait Node {
    fn neighbors(&self) -> impl Iterator<Item = NodeId>;
}

#[derive(Clone)]
pub(crate) struct NodeTrackState(bool, Option<NodeId>);

pub(crate) trait VisitedTracker {
    fn has_seen(&self, node_id: NodeId) -> bool;
    fn set_seen(&mut self, node_id: NodeId);
    fn set_prev(&mut self, node_id: NodeId, prev_node_id: NodeId);
    fn prev_node_list(&self) -> Vec<Option<NodeId>>;
}
