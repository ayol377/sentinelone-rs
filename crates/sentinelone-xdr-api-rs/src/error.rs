use sentinelone_http::HttpError;

/// XDR / Data Lake API error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] HttpError),

    /// A long-running query failed or was cancelled server-side.
    #[error("query failed: {0}")]
    Query(String),
}
