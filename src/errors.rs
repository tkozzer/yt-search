use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Parsing error: {0}")]
    ParsingError(#[from] ParserError),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Failed to extract ytInitialData")]
    ExtractInitialDataError,

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}
