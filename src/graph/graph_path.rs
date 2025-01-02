use super::{Graph, NodeId};

pub(crate) type PrevNodeGraphPath = Vec<Option<NodeId>>;
pub(crate) type NodeGraphPath = Vec<NodeId>;

pub(crate) fn prev_node_graph_path_to_isize_vec(path: &PrevNodeGraphPath) -> Vec<isize> {
    path.iter()
        .map(|v| (v.map(|v| v as isize)).unwrap_or(-1))
        .collect()
}
