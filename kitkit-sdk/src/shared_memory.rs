use crate::{KitKitClient, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SharedMemoryEntry {
    pub slug: String,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListSharedMemoryResponse {
    pub entries: Vec<SharedMemoryEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpsertSharedMemoryRequest {
    pub slug: String,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpsertSharedMemoryResponse {
    pub entry: SharedMemoryEntry,
}

pub async fn list(
    client: &KitKitClient,
    space_id: impl AsRef<str>,
) -> Result<ListSharedMemoryResponse> {
    client
        .get_json(&["spaces", space_id.as_ref(), "shared-memory"])
        .await
}

pub async fn upsert(
    client: &KitKitClient,
    space_id: impl AsRef<str>,
    request: UpsertSharedMemoryRequest,
) -> Result<UpsertSharedMemoryResponse> {
    client
        .post_json(&["spaces", space_id.as_ref(), "shared-memory"], &request)
        .await
}

pub async fn delete(
    client: &KitKitClient,
    space_id: impl AsRef<str>,
    slug: impl AsRef<str>,
) -> Result<()> {
    client
        .delete_empty(&["spaces", space_id.as_ref(), "shared-memory", slug.as_ref()])
        .await
}
