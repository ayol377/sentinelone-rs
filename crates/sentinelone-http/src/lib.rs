//! Shared async HTTP core for the sentinelone-rs SDK.
//!
//! Auth-scheme-agnostic: the management crate uses [`Auth::ApiToken`], the XDR
//! crate uses [`Auth::Bearer`]. Both go through [`HttpClient`].

use serde::{de::DeserializeOwned, Serialize};
use url::Url;

pub use reqwest::Method;

/// Authorization scheme for outgoing requests.
#[derive(Clone)]
pub enum Auth {
    /// SentinelOne management: `Authorization: ApiToken <token>`.
    ApiToken(String),
    /// SentinelOne XDR / DataSet: `Authorization: Bearer <token>`.
    Bearer(String),
}

impl Auth {
    fn header_value(&self) -> String {
        match self {
            Auth::ApiToken(t) => format!("ApiToken {t}"),
            Auth::Bearer(t) => format!("Bearer {t}"),
        }
    }
}

/// Transport-level error. Higher layers wrap this with API-specific context.
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("invalid base url: {0}")]
    BaseUrl(String),
    #[error("invalid request path: {0}")]
    Path(String),
    #[error("transport error: {0}")]
    Transport(#[source] reqwest::Error),
    #[error("api returned HTTP {status}: {body}")]
    Status { status: u16, body: String },
    #[error("failed to decode response body: {0}")]
    Decode(#[source] reqwest::Error),
}

/// Thin wrapper over `reqwest::Client` that injects auth and a base URL.
///
/// Cloning is cheap — the inner `reqwest::Client` is `Arc`-backed.
#[derive(Clone)]
pub struct HttpClient {
    inner: reqwest::Client,
    base: Url,
    auth: Auth,
}

impl HttpClient {
    /// `base_url` e.g. `https://apse1-2111-mssp.sentinelone.net`.
    pub fn new(base_url: &str, auth: Auth) -> Result<Self, HttpError> {
        let inner = reqwest::Client::builder()
            .user_agent(concat!("sentinelone-rs/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(HttpError::Transport)?;
        Self::with_client(inner, base_url, auth)
    }

    /// Reuse an existing `reqwest::Client` (shared connection pool).
    pub fn with_client(inner: reqwest::Client, base_url: &str, auth: Auth) -> Result<Self, HttpError> {
        let base = Url::parse(base_url).map_err(|e| HttpError::BaseUrl(e.to_string()))?;
        Ok(Self { inner, base, auth })
    }

    pub fn base(&self) -> &Url {
        &self.base
    }

    fn url(&self, path: &str, query: Option<&str>) -> Result<Url, HttpError> {
        let mut u = self.base.join(path).map_err(|e| HttpError::Path(e.to_string()))?;
        if let Some(q) = query {
            u.set_query(Some(q));
        }
        Ok(u)
    }

    /// Core request. `query` is a pre-serialized querystring (e.g. via `serde_urlencoded`).
    pub async fn request_json<B, T>(
        &self,
        method: Method,
        path: &str,
        query: Option<&str>,
        body: Option<&B>,
    ) -> Result<T, HttpError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let url = self.url(path, query)?;
        let mut req = self
            .inner
            .request(method, url)
            .header(reqwest::header::AUTHORIZATION, self.auth.header_value())
            .header(reqwest::header::ACCEPT, "application/json");
        if let Some(b) = body {
            req = req.json(b);
        }
        let resp = req.send().await.map_err(HttpError::Transport)?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(HttpError::Status { status: status.as_u16(), body });
        }
        resp.json::<T>().await.map_err(HttpError::Decode)
    }

    /// GET with optional pre-serialized querystring.
    pub async fn get<T: DeserializeOwned>(&self, path: &str, query: Option<&str>) -> Result<T, HttpError> {
        self.request_json::<(), T>(Method::GET, path, query, None).await
    }

    /// POST a JSON body.
    pub async fn post<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, HttpError> {
        self.request_json::<B, T>(Method::POST, path, None, Some(body)).await
    }

    /// POST a raw (non-JSON) request body with an explicit `Content-Type`.
    ///
    /// Needed by endpoints that take a plain payload rather than JSON, e.g. the
    /// DataSet `/api/uploadLogs` endpoint (raw log text). The response is still
    /// decoded as JSON.
    pub async fn post_raw<T: DeserializeOwned>(
        &self,
        path: &str,
        query: Option<&str>,
        content_type: &str,
        body: impl Into<reqwest::Body>,
    ) -> Result<T, HttpError> {
        let url = self.url(path, query)?;
        let resp = self
            .inner
            .request(Method::POST, url)
            .header(reqwest::header::AUTHORIZATION, self.auth.header_value())
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .body(body)
            .send()
            .await
            .map_err(HttpError::Transport)?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(HttpError::Status { status: status.as_u16(), body });
        }
        resp.json::<T>().await.map_err(HttpError::Decode)
    }
}
