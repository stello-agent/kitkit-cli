use reqwest::{Method, StatusCode};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use url::Url;

use crate::error::{KitKitError, Result};

pub const DEFAULT_BASE_URL: &str = "https://api.kitkit-agent.com";

#[derive(Clone, Debug)]
pub struct KitKitClientConfig {
    pub base_url: Url,
    pub bearer_token: Option<String>,
}

impl KitKitClientConfig {
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        let base_url = Url::parse(base_url.as_ref())?;
        if !base_url.has_host() {
            return Err(KitKitError::InvalidBaseUrl(base_url.to_string()));
        }

        Ok(Self {
            base_url,
            bearer_token: None,
        })
    }

    pub fn with_bearer_token(mut self, bearer_token: impl Into<String>) -> Self {
        self.bearer_token = Some(bearer_token.into());
        self
    }
}

impl Default for KitKitClientConfig {
    fn default() -> Self {
        Self {
            base_url: Url::parse(DEFAULT_BASE_URL).expect("default base url is valid"),
            bearer_token: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KitKitClient {
    base_url: Url,
    bearer_token: Option<String>,
    http: reqwest::Client,
}

impl KitKitClient {
    pub fn new(config: KitKitClientConfig) -> Result<Self> {
        Ok(Self {
            base_url: config.base_url,
            bearer_token: config.bearer_token,
            http: reqwest::Client::new(),
        })
    }

    pub fn with_bearer_token(mut self, bearer_token: impl Into<String>) -> Self {
        self.bearer_token = Some(bearer_token.into());
        self
    }

    pub fn set_bearer_token(&mut self, bearer_token: impl Into<String>) {
        self.bearer_token = Some(bearer_token.into());
    }

    pub fn clear_bearer_token(&mut self) {
        self.bearer_token = None;
    }

    pub(crate) async fn get_json<T>(&self, segments: &[&str]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.request(Method::GET, segments)?.send().await?;
        parse_json(response).await
    }

    pub(crate) async fn post_json<B, T>(&self, segments: &[&str], body: &B) -> Result<T>
    where
        B: serde::Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let response = self
            .request(Method::POST, segments)?
            .json(body)
            .send()
            .await?;
        parse_json(response).await
    }

    #[allow(dead_code)]
    pub(crate) async fn put_json<B, T>(&self, segments: &[&str], body: &B) -> Result<T>
    where
        B: serde::Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let response = self
            .request(Method::PUT, segments)?
            .json(body)
            .send()
            .await?;
        parse_json(response).await
    }

    #[allow(dead_code)]
    pub(crate) async fn delete_empty(&self, segments: &[&str]) -> Result<()> {
        let response = self.request(Method::DELETE, segments)?.send().await?;
        parse_empty(response).await
    }

    fn request(&self, method: Method, segments: &[&str]) -> Result<reqwest::RequestBuilder> {
        let mut request = self.http.request(method, self.url(segments)?);
        if let Some(token) = &self.bearer_token {
            request = request.bearer_auth(token);
        }

        Ok(request)
    }

    fn url(&self, segments: &[&str]) -> Result<Url> {
        let mut url = self.base_url.clone();
        {
            let mut path = url
                .path_segments_mut()
                .map_err(|_| KitKitError::InvalidBaseUrl(self.base_url.to_string()))?;
            path.pop_if_empty();
            path.extend(segments);
        }
        Ok(url)
    }
}

#[derive(Debug, Deserialize)]
struct ApiErrorEnvelope {
    error: Option<ApiErrorBody>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiErrorBody {
    code: Option<String>,
    message: Option<String>,
}

async fn parse_json<T>(response: reqwest::Response) -> Result<T>
where
    T: DeserializeOwned,
{
    let status = response.status();
    let bytes = response.bytes().await?;

    if !status.is_success() {
        return Err(api_error_from_bytes(status, &bytes));
    }

    Ok(serde_json::from_slice(&bytes)?)
}

#[allow(dead_code)]
async fn parse_empty(response: reqwest::Response) -> Result<()> {
    let status = response.status();
    let bytes = response.bytes().await?;

    if !status.is_success() {
        return Err(api_error_from_bytes(status, &bytes));
    }

    Ok(())
}

fn api_error_from_bytes(status: StatusCode, bytes: &[u8]) -> KitKitError {
    let fallback_code = status.as_u16().to_string();
    let fallback_message = String::from_utf8_lossy(bytes).to_string();

    match serde_json::from_slice::<ApiErrorEnvelope>(bytes) {
        Ok(envelope) => {
            let code = envelope
                .error
                .as_ref()
                .and_then(|error| error.code.clone())
                .or(envelope.code)
                .unwrap_or(fallback_code);
            let message = envelope
                .error
                .and_then(|error| error.message)
                .or(envelope.message)
                .unwrap_or(fallback_message);

            KitKitError::Api {
                status,
                code,
                message,
            }
        }
        Err(_) => KitKitError::Api {
            status,
            code: fallback_code,
            message: fallback_message,
        },
    }
}
