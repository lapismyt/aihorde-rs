use crate::enums::RequestErrorCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AihordeError {
    /// Error occurred during HTTP request
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Unexpected HTTP code
    #[error("Unexpected HTTP code ({code}): {message}")]
    UnexpectedHTTPCode { code: u16, message: String },

    /// Error occurred during JSON serialization/deserialization
    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Error occurred during URL parsing
    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    /// API returned an error response with a specific error code
    #[error("API error: {code} - {}", message.as_deref().unwrap_or("No message"))]
    ApiError {
        code: RequestErrorCode,
        message: Option<String>,
    },

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    /// Invalid API key
    #[error("Invalid API key")]
    InvalidApiKey,

    /// Rate limiting error
    #[error("Rate limited: {0}")]
    RateLimited(String),

    /// Timeout error
    #[error("Request timeout: {0}")]
    Timeout(String),

    /// Invalid input provided
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Insufficient permissions
    #[error("Insufficient permissions: {0}")]
    PermissionDenied(String),

    /// Service unavailable
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    /// Unexpected response from server
    #[error("Unexpected response: {0}")]
    UnexpectedResponse(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Unable to parse JSON response
    #[error("Unable to parse JSON response: {0}")]
    JsonParseError(String),

    /// Other errors
    #[error("Other error: {0}")]
    Other(String),
}
