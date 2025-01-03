use crate::tracker::VisitedTracker;
use graph_path::PrevNodeGraphPath;

pub(crate) mod graph_path;
pub(crate) mod static_graph;

pub(crate) type NodeId = usize;
pub(crate) type Weight = f64;

#[derive(PartialEq)]
pub(crate) enum GraphType {
    Directed,
    Undirected,
}

pub(crate) trait Graph<T> {
    type NodeType: Node<T>;
    type Trakcer: VisitedTracker<T>;

    /// Given a node id, return a reference to the concrete node
    fn node(&self, node_id: &T) -> Option<&Self::NodeType>;

    /// Optionally returns the total number of nodes in the graph
    /// for static graph that know the exact count for nodes this
    /// will be Some(value)
    /// for dynamic graphs where the nodes are discovered during
    /// algorithm application, this will be none
    fn num_of_nodes(&self) -> Option<usize>;

    /// Returns the type of graph based on edge type i.e Directed
    /// or Undirected
    fn graph_type(&self) -> &GraphType;

    /// Returns a structure that can be used to nodes that have been
    /// seen (useful during algorihm application)
    fn visited_tracker(&self) -> Self::Trakcer;
}

pub(crate) trait Node<T> {
    /// Returns the neighbors for a given node
    fn neighbors(&self) -> impl Iterator<Item = T>;
}
