use serde::{Deserialize, Serialize};

use crate::client::KitKitClient;
use crate::error::Result;

#[derive(Clone, Debug, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub nickname: String,
    pub role: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub user: AuthUser,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    pub id: String,
    pub email: String,
    pub nickname: String,
    pub role: String,
    pub has_tutorial_note: bool,
    pub created_at: String,
}

pub async fn login(client: &KitKitClient, request: LoginRequest) -> Result<LoginResponse> {
    client.post_json(&["auth", "login"], &request).await
}

pub async fn refresh_token(
    client: &KitKitClient,
    request: RefreshTokenRequest,
) -> Result<RefreshTokenResponse> {
    client.post_json(&["auth", "refresh"], &request).await
}

pub async fn me(client: &KitKitClient) -> Result<MeResponse> {
    client.get_json(&["auth", "me"]).await
}
