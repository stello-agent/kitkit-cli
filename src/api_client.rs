use crate::auth_store::{StoredAuth, load_auth, save_auth};
use anyhow::{Context, Result, anyhow, bail};
use kitkit_sdk::auth::{self, RefreshTokenRequest, RefreshTokenResponse};
use kitkit_sdk::{KitKitClient, KitKitClientConfig, KitKitError};

pub async fn authenticated_client(base_url: &str) -> Result<KitKitClient> {
    let stored = load_auth()?
        .ok_or_else(|| anyhow!("no stored auth token; run `kitkit-cli auth login` first"))?;
    let stored = refresh_auth_if_needed(base_url, stored).await?;
    client_with_token(base_url, &stored.access_token)
}

pub async fn refresh_auth_if_needed(base_url: &str, mut stored: StoredAuth) -> Result<StoredAuth> {
    if stored.base_url != base_url {
        bail!(
            "stored auth token is for {}, but current base URL is {}; run `kitkit-cli --base-url {} auth login`",
            stored.base_url,
            base_url,
            base_url
        );
    }

    let client = client_with_token(base_url, &stored.access_token)?;
    match auth::me(&client).await {
        Ok(me) => {
            stored.user = me.into();
            save_auth(&stored)?;
            Ok(stored)
        }
        Err(KitKitError::Api { status, .. }) if status.as_u16() == 401 => {
            let anonymous = anonymous_client(base_url)?;
            let refreshed = auth::refresh_token(
                &anonymous,
                RefreshTokenRequest {
                    refresh_token: stored.refresh_token.clone(),
                },
            )
            .await
            .context("refresh token failed; run `kitkit-cli auth login` again")?;
            apply_refresh(base_url, stored, refreshed).await
        }
        Err(err) => Err(err.into()),
    }
}

pub fn anonymous_client(base_url: &str) -> Result<KitKitClient> {
    Ok(KitKitClient::new(KitKitClientConfig::new(base_url)?)?)
}

fn client_with_token(base_url: &str, token: &str) -> Result<KitKitClient> {
    Ok(KitKitClient::new(
        KitKitClientConfig::new(base_url)?.with_bearer_token(token),
    )?)
}

async fn apply_refresh(
    base_url: &str,
    mut stored: StoredAuth,
    refreshed: RefreshTokenResponse,
) -> Result<StoredAuth> {
    stored.access_token = refreshed.access_token;
    stored.refresh_token = refreshed.refresh_token;
    let client = client_with_token(base_url, &stored.access_token)?;
    stored.user = auth::me(&client).await?.into();
    save_auth(&stored)?;
    Ok(stored)
}
