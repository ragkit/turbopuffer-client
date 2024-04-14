use serde_json::Value;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum Error {
  #[error("Request Error: {0}")]
  RequestError(String),

  #[error("Recieved non JSON Response")]
  NonJsonResponse,

  /// When trying to parse the JSON response into a well-formed struct there
  /// was an error. The raw JSON value that was the response is stored in
  /// this error.
  #[error("Invalid Response")]
  InvalidResponse(Value),
}

impl From<reqwest::Error> for Error {
  fn from(value: reqwest::Error) -> Self {
    Error::RequestError(value.to_string())
  }
}

pub(crate) fn non_json(err: reqwest::Error) -> Error {
  tracing::error!("[non_json] {}", err);
  Error::NonJsonResponse
}

pub(crate) fn invalid_response(
  err: serde_json::Error,
  fallback: serde_json::Value,
) -> Error {
  tracing::error!("[invalid_response] {}", err);
  Error::InvalidResponse(fallback)
}
