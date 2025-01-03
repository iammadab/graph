use crate::graph::{graph_path::PrevNodeGraphPath, NodeId};

pub(crate) trait VisitedTracker<T> {
    /// Use the tracker to determine if a node has been seen
    fn has_seen(&self, node_label: &T) -> bool;

    /// Set the seen status of a particular node to true
    fn set_seen(&mut self, node_label: &T);

    /// Set the previous_node for a given node to some node_id
    fn set_prev(&mut self, node_label: &T, prev_node_label: &T);

    /// Converts the tracker state to the prev_node_list path representation
    fn prev_node_list(&self) -> PrevNodeGraphPath;
}

/// Holds search information about a given node
/// (seen, previous_node)
#[derive(Clone)]
pub(crate) struct NodeTrackState(bool, Option<NodeId>);

pub(crate) struct StaticTracker {
    state: Vec<NodeTrackState>,
}

impl StaticTracker {
    pub(crate) fn new(node_count: usize) -> Self {
        Self {
            state: vec![NodeTrackState(false, None); node_count],
        }
    }
}

impl VisitedTracker<NodeId> for StaticTracker {
    fn has_seen(&self, node_id: &NodeId) -> bool {
        self.state[*node_id].0
    }

    fn set_seen(&mut self, node_id: &NodeId) {
        self.state[*node_id].0 = true;
    }

    fn set_prev(&mut self, node_id: &NodeId, prev_node_id: &NodeId) {
        self.state[*node_id].1 = Some(*prev_node_id);
    }

    fn prev_node_list(&self) -> Vec<Option<NodeId>> {
        self.state.iter().map(|v| v.1).collect()
    }
}
