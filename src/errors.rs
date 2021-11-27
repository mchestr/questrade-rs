use serde::{Deserialize, Serialize};
use url::ParseError;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub code: u32,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Ok(T),
    Err(ApiError),
}

#[derive(thiserror::Error, Debug)]
pub enum QuestradeError {
    #[error("{0:?}")]
    ApiError(ApiError),
    #[error("{0:?}")]
    Builder(String),
    #[error("{0}")]
    InternalError(String),
    #[error("{0}")]
    TransportError(String),
}

impl From<reqwest::Error> for QuestradeError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_connect()
            || err.is_redirect()
            || err.is_timeout()
            || err.is_request()
            || err.is_builder()
        {
            Self::TransportError(err.to_string())
        } else {
            Self::InternalError(err.to_string())
        }
    }
}

impl From<ParseError> for QuestradeError {
    fn from(err: ParseError) -> Self {
        Self::InternalError(err.to_string())
    }
}
