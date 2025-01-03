use crate::graph::graph_path::PrevNodeGraphPath;

pub(crate) trait VisitedTracker<T> {
    /// Use the tracker to determine if a node has been seen
    fn has_seen(&self, node_label: T) -> bool;

    /// Set the seen status of a particular node to true
    fn set_seen(&mut self, node_label: T);

    /// Set the previous_node for a given node to some node_id
    fn set_prev(&mut self, node_label: T, prev_node_label: T);

    /// Converts the tracker state to the prev_node_list path representation
    fn prev_node_list(&self) -> PrevNodeGraphPath;
}
