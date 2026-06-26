use sentinelone_http::HttpError;

/// Management API error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] HttpError),

    /// The query matched no resource where exactly one was expected.
    #[error("resource not found")]
    NotFound,

    /// Structured SentinelOne error envelope (`{ "errors": [...] }`).
    #[error("management API error (HTTP {status}): {message}")]
    Api { status: u16, message: String },
}
