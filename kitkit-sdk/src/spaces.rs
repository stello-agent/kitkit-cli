pub use crate::sessions::SessionStatus;
use crate::{KitKitClient, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSpacesResponse {
    pub data: Vec<SpaceListItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceListItem {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub id: String,
    pub label: String,
    pub description: String,
    pub pinned_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionTreeNode {
    pub id: String,
    pub label: String,
    pub status: SessionStatus,
    pub turn_count: u64,
    #[serde(default)]
    pub children: Vec<SessionTreeNode>,
    pub source_session_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ForkContext {
    None,
    Inherit,
    Compress,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForkSessionRequest {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consolidate_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork_compress_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ForkContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_vars: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForkSessionResponse {
    pub id: String,
    pub parent_id: Option<String>,
    #[serde(default)]
    pub children: Vec<String>,
    #[serde(default)]
    pub refs: Vec<String>,
    pub depth: u64,
    pub index: u64,
    pub label: String,
}

pub async fn list(client: &KitKitClient) -> Result<ListSpacesResponse> {
    client.get_json(&["spaces"]).await
}

pub async fn get(client: &KitKitClient, space_id: impl AsRef<str>) -> Result<Space> {
    client.get_json(&["spaces", space_id.as_ref()]).await
}

pub async fn topology(client: &KitKitClient, space_id: impl AsRef<str>) -> Result<SessionTreeNode> {
    client
        .get_json(&["spaces", space_id.as_ref(), "topology"])
        .await
}

pub async fn fork_session(
    client: &KitKitClient,
    space_id: impl AsRef<str>,
    source_session_id: impl AsRef<str>,
    request: &ForkSessionRequest,
) -> Result<ForkSessionResponse> {
    client
        .post_json(
            &[
                "spaces",
                space_id.as_ref(),
                "sessions",
                source_session_id.as_ref(),
                "fork",
            ],
            &request,
        )
        .await
}
