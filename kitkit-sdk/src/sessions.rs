use crate::{KitKitClient, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Active,
    Archived,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSessionDigestResponse {
    pub digest: SessionDigest,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionDigest {
    pub id: String,
    pub label: String,
    pub status: SessionStatus,
    pub memory: Option<String>,
    pub insight: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PutInsightRequest {
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutInsightResponse {
    pub ok: bool,
    pub session_id: String,
    pub label: String,
    pub content_length: u64,
}

pub async fn digest(
    client: &KitKitClient,
    space_id: impl AsRef<str>,
    session_id: impl AsRef<str>,
) -> Result<GetSessionDigestResponse> {
    client
        .get_json(&[
            "spaces",
            space_id.as_ref(),
            "sessions",
            session_id.as_ref(),
            "digest",
        ])
        .await
}

pub async fn put_insight(
    client: &KitKitClient,
    space_id: impl AsRef<str>,
    session_id: impl AsRef<str>,
    request: PutInsightRequest,
) -> Result<PutInsightResponse> {
    client
        .put_json(
            &[
                "spaces",
                space_id.as_ref(),
                "sessions",
                session_id.as_ref(),
                "insight",
            ],
            &request,
        )
        .await
}
