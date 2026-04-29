use thiserror::Error;

#[derive(Error, Debug)]
pub enum OAuth2Error {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("URL parsing failed: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("API returned error status: {0}")]
    ApiError(String),

    #[error("Not modified (304) response")]
    NotModified,

    #[error("Client is nil")]
    NilClient,
}

pub type Result<T> = std::result::Result<T, OAuth2Error>;
