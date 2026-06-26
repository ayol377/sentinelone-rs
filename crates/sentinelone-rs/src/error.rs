/// High-level SDK error.
///
/// Calling into an API that was not configured on the builder yields a typed
/// `*NotConfigured` variant rather than a panic.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("management API not configured")]
    ManagementNotConfigured,

    #[error("XDR/datalake API not configured")]
    XdrNotConfigured,

    #[error("at least one of management or xdr must be configured")]
    NothingConfigured,

    #[error(transparent)]
    Management(#[from] sentinelone_api::Error),

    #[error(transparent)]
    Xdr(#[from] sentinelone_xdr_api::Error),
}
