//! Ergonomic high-level SDK for SentinelOne.
//!
//! Unifies the Management API and the XDR / Data Lake API behind one
//! [`SentinelOne`] facade. Either or both backends can be configured; calling
//! into an unconfigured backend returns a typed [`Error`], never a panic.
//!
//! ```no_run
//! # async fn run() -> Result<(), sentinelone::Error> {
//! use sentinelone::SentinelOne;
//!
//! let s1 = SentinelOne::builder()
//!     .management("https://apse1-2111-mssp.sentinelone.net", "MGMT_TOKEN")
//!     .xdr("https://xdr.ap1.sentinelone.net", "BEARER_TOKEN")
//!     .build()?;
//!
//! let agent = s1.agent("AGENT_UUID").await?;
//! println!("{:?}", agent.computer_name());
//! agent.disconnect().await?;
//!
//! let rows = s1.power_query("dataset = 'endpoint' | limit 100")
//!     .from("24 hours").to("now")
//!     .run().await?;
//! # let _ = rows;
//! # Ok(())
//! # }
//! ```

use std::sync::Arc;

use sentinelone_api::ManagementClient;
use sentinelone_xdr_api::XdrClient;

pub mod entities;
pub mod error;

pub use entities::agent::Agent;
pub use error::Error;

// Re-export the raw crates for escape-hatch access.
pub use sentinelone_api as api;
pub use sentinelone_xdr_api as xdr_api;

/// The unified SentinelOne client. Cheap to clone (`Arc` inside); entities hold
/// a clone so their methods route through the configured backends.
#[derive(Clone)]
pub struct SentinelOne {
    inner: Arc<Inner>,
}

struct Inner {
    management: Option<ManagementClient>,
    xdr: Option<XdrClient>,
}

impl SentinelOne {
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Borrow the management client, or `Err(ManagementNotConfigured)`.
    pub fn management(&self) -> Result<&ManagementClient, Error> {
        self.inner
            .management
            .as_ref()
            .ok_or(Error::ManagementNotConfigured)
    }

    /// Borrow the XDR client, or `Err(XdrNotConfigured)`.
    pub fn xdr(&self) -> Result<&XdrClient, Error> {
        self.inner.xdr.as_ref().ok_or(Error::XdrNotConfigured)
    }

    /// Fetch an agent by id/uuid (high-level active-record).
    pub async fn agent(&self, id: impl Into<String>) -> Result<Agent, Error> {
        Agent::fetch(self.clone(), id.into()).await
    }

    /// Begin a PowerQuery against the Data Lake. Configuration errors surface on
    /// [`PowerQueryBuilder::run`], not here.
    pub fn power_query(&self, pql: impl Into<String>) -> PowerQueryBuilder {
        PowerQueryBuilder {
            s1: self.clone(),
            pql: pql.into(),
            start: "24 hours".into(),
            end: "now".into(),
        }
    }
}

/// Builder for [`SentinelOne`]. Configure either or both backends.
#[derive(Default)]
pub struct Builder {
    management: Option<(String, String)>,
    xdr: Option<(String, String)>,
}

impl Builder {
    /// Configure the Management API (host + ApiToken).
    pub fn management(mut self, host: impl Into<String>, token: impl Into<String>) -> Self {
        self.management = Some((host.into(), token.into()));
        self
    }

    /// Configure the XDR / Data Lake API (host + Bearer token).
    pub fn xdr(mut self, host: impl Into<String>, token: impl Into<String>) -> Self {
        self.xdr = Some((host.into(), token.into()));
        self
    }

    pub fn build(self) -> Result<SentinelOne, Error> {
        if self.management.is_none() && self.xdr.is_none() {
            return Err(Error::NothingConfigured);
        }
        let management = match self.management {
            Some((h, t)) => Some(ManagementClient::new(&h, t)?),
            None => None,
        };
        let xdr = match self.xdr {
            Some((h, t)) => Some(XdrClient::new(&h, t)?),
            None => None,
        };
        Ok(SentinelOne {
            inner: Arc::new(Inner { management, xdr }),
        })
    }
}

/// Fluent PowerQuery builder. `run` resolves the XDR client (erroring if it was
/// never configured) then executes.
pub struct PowerQueryBuilder {
    s1: SentinelOne,
    pql: String,
    start: String,
    end: String,
}

impl PowerQueryBuilder {
    /// Start of the time window (DataSet syntax: epoch ns, RFC3339, or relative).
    pub fn from(mut self, start: impl Into<String>) -> Self {
        self.start = start.into();
        self
    }

    /// End of the time window.
    pub fn to(mut self, end: impl Into<String>) -> Self {
        self.end = end.into();
        self
    }

    pub async fn run(self) -> Result<sentinelone_xdr_api::PowerQueryResponse, Error> {
        let xdr = self.s1.xdr()?; // null client -> Err(XdrNotConfigured)
        Ok(xdr.power_query(self.pql, self.start, self.end).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_requires_at_least_one_backend() {
        let result = SentinelOne::builder().build();
        assert!(matches!(result, Err(Error::NothingConfigured)));
    }

    #[tokio::test]
    async fn calling_xdr_without_config_errors_not_panics() {
        // Only management configured; no network is touched because xdr()
        // resolution fails before any request is built.
        let s1 = SentinelOne::builder()
            .management("https://example.invalid", "token")
            .build()
            .unwrap();
        let err = s1.power_query("x").run().await.unwrap_err();
        assert!(matches!(err, Error::XdrNotConfigured));
    }
}
