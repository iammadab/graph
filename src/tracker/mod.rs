use std::{collections::HashMap, hash::Hash};

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
pub(crate) struct NodeTrackState<T>(bool, Option<T>);

pub(crate) struct StaticTracker {
    state: Vec<NodeTrackState<NodeId>>,
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

pub(crate) struct DynamicTracker<T> {
    state: Vec<NodeTrackState<T>>,
    label_to_id_map: HashMap<T, NodeId>,
}

impl<T: Eq + Hash + Clone> DynamicTracker<T> {
    pub(crate) fn new(initial_state: T) -> Self {
        Self {
            state: vec![],
            label_to_id_map: HashMap::new(),
        }
    }

    fn allocate(&mut self, node_label: &T) -> NodeId {
        self.label_to_id_map
            .insert(node_label.clone(), self.state.len());
        self.state.push(NodeTrackState(false, None));
        self.state.len() - 1
    }
}

impl<T: Eq + Hash> VisitedTracker<T> for DynamicTracker<T> {
    fn has_seen(&self, node_label: &T) -> bool {
        // check if we have stored any information about the node
        // in the label to id map, if we have retrieve and return the seen
        // variable. if we have not stored any information then it hasn't
        // been seen
        self.label_to_id_map
            .get(node_label)
            .map(|node_id| self.state[*node_id].0)
            .unwrap_or(false)
    }

    fn set_seen(&mut self, node_label: &T) {
        // this can come before set_prev and set_prev can also come before
        // first we should try to get the node id
        // if we can then we should set the appropriate parameter
        // if we cannot then we need to allocate space for that node and return
        // the corresponding node id
        // then continue as previously
        todo!()
    }

    fn set_prev(&mut self, node_label: &T, prev_node_label: &T) {
        todo!()
    }

    fn prev_node_list(&self) -> PrevNodeGraphPath {
        todo!()
    }
}
