use thiserror::Error;

/// All errors that can be produced by the SmsDev SDK.
#[derive(Debug, Error)]
pub enum SmsDevError {
    /// An HTTP-level error returned by `reqwest`.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// The API returned a response with `situacao = "ERROR"`.
    #[error("API error (code {code}): {description}")]
    Api {
        /// Numeric error code returned by the API.
        code: String,
        /// Human-readable description returned by the API.
        description: String,
    },

    /// A URL could not be constructed from the provided parameters.
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// JSON serialisation / deserialisation error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// The API returned an unexpected or empty response body.
    #[error("Unexpected response: {0}")]
    UnexpectedResponse(String),
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, SmsDevError>;
