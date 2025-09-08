use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use crate::{
    hlc::Hlc,
    cid::Cid,
    entity::{EntityId, EntityForm, ExecutionContext},
    error::Error,
};

pub type MimeType = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CduType {
    System, Config, Log, Chat, Task, Project, Contract, Procedure, Math, Component, License,
    World, Chapter, Publication,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreatorInfo {
    pub id: EntityId,
    pub form: EntityForm,
    pub context: ExecutionContext,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SourceProvenance {
    pub node_id: EntityId,
    pub cid: Cid,
    pub hlc_id: Hlc,
    pub creator: CreatorInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StandardLicense {
    BadaasV1, Mit, Apache2_0, Gpl3_0, CcBySa4_0,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum LicenseType {
    Standard(StandardLicense),
    Custom(Cid),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ConfidenceMetric {
    Token, Group, Trace,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GenerationInfo {
    pub prompt_cid: Cid,
    pub score: f64,
    pub metric: ConfidenceMetric,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_cid: Option<Cid>,
}

// --- FIX: Removed `Eq` because it contains `GenerationInfo` ---
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// ---
#[serde(rename_all = "camelCase")]
pub struct IntrinsicMetadata {
    pub license: LicenseType,
    pub cdu_type: CduType,
    pub id: Hlc,
    pub lineage_id: Hlc,
    pub causal_links: Vec<Hlc>,
    pub subject: String,
    pub tags: Vec<String>,
    pub creator: CreatorInfo,
    pub content_type: MimeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceProvenance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<GenerationInfo>,
}

// --- FIX: Removed `Eq` because it contains `IntrinsicMetadata` ---
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// ---
#[serde(rename_all = "camelCase")]
pub struct Cdu {
    pub cid: Cid,
    pub content: Vec<u8>,
    pub metadata: IntrinsicMetadata,
    pub provenance_signature: Vec<u8>,
}

impl Cdu {
    /// Calculates the CID for a given content and metadata.
    pub fn calculate_cid(content: &[u8], metadata: &IntrinsicMetadata) -> Result<Cid, Error> {
        let metadata_bytes = serde_json::to_vec(metadata)?;
        let mut hasher = Sha256::new();
        hasher.update(content);
        hasher.update(&metadata_bytes);
        let hash_result = hasher.finalize();
        let cid_str = format!("sha256:{}", STANDARD.encode(hash_result));
        Ok(Cid::new(cid_str))
    }
}
