use serde::{Serialize, Deserialize};
use crate::cid::Cid;

/// A unique, cryptographically verifiable identifier for any entity in the ecosystem.
pub type EntityId = String;

/// The form or "type" of an entity, which defines its behavior and capabilities.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum EntityForm {
    Worker, Automata, Agent, Node,
}

/// The security context in which an action is performed.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ExecutionContext {
    SovereignSystem, SovereignUser, RemoteSystem, RemoteUser,
}

/// The type of a sovereign node, defining its role in the network.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum NodeType {
    HomeNode, PrivateNode, FirmNode, PublicNode,
}

/// A node's public-facing profile, used for discovery and identification.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodeProfile {
    pub node_id: EntityId,
    pub node_type: NodeType,
    pub display_name: String,
    pub pub_pgm_cid: Cid,
}
