use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use kitkit_sdk::auth::{AuthUser, MeResponse};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredAuth {
    pub base_url: String,
    pub user: StoredUser,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredUser {
    pub id: String,
    pub email: String,
    pub nickname: String,
    pub role: String,
}

impl From<AuthUser> for StoredUser {
    fn from(user: AuthUser) -> Self {
        Self {
            id: user.id,
            email: user.email,
            nickname: user.nickname,
            role: user.role,
        }
    }
}

impl From<MeResponse> for StoredUser {
    fn from(user: MeResponse) -> Self {
        Self {
            id: user.id,
            email: user.email,
            nickname: user.nickname,
            role: user.role,
        }
    }
}

pub fn load_auth() -> Result<Option<StoredAuth>> {
    let path = auth_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let bytes = fs::read(&path).with_context(|| format!("read auth cache {}", path.display()))?;
    Ok(Some(serde_json::from_slice(&bytes)?))
}

pub fn save_auth(auth: &StoredAuth) -> Result<()> {
    let path = auth_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let bytes = serde_json::to_vec_pretty(auth)?;
    let mut options = OpenOptions::new();
    options.create(true).truncate(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }

    let mut file = options.open(&path)?;
    file.write_all(&bytes)?;
    file.write_all(b"\n")?;
    Ok(())
}

pub fn remove_auth() -> Result<()> {
    let path = auth_path()?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn auth_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("com", "kitkit-agent", "kitkit-cli")
        .ok_or_else(|| anyhow!("unable to determine a platform config directory"))?;
    Ok(dirs.config_dir().join("auth.json"))
}
