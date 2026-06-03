use reqwest::StatusCode;

pub type Result<T> = std::result::Result<T, KitKitError>;

#[derive(Debug, thiserror::Error)]
pub enum KitKitError {
    #[error("api error {status}: {code}: {message}")]
    Api {
        status: StatusCode,
        code: String,
        message: String,
    },

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid base url: {0}")]
    InvalidBaseUrl(String),
}
