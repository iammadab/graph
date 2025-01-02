use super::{Graph, NodeId};

pub(crate) type PrevNodeGraphPath = Vec<Option<NodeId>>;
pub(crate) type NodeGraphPath = Vec<NodeId>;
