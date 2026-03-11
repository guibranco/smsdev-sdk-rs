use reqwest::Error as ReqwestError;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    Http(ReqwestError),
    Serialization(serde_json::Error),
    Unknown,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ApiError::Http(ref err) => write!(f, "HTTP error: {err}"),
            ApiError::Serialization(ref err) => write!(f, "Serialization error: {err}"),
            ApiError::Unknown => write!(f, "Unknown error occurred"),
        }
    }
}

impl From<ReqwestError> for ApiError {
    fn from(err: ReqwestError) -> Self {
        ApiError::Http(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::Serialization(err)
    }
}
