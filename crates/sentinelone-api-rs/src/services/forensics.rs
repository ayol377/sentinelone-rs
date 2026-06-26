use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::forensics::{
    ApplicationConnectionsData, ApplicationForensicsData, ApplicationForensicsDetailsData,
};
use crate::pagination::Response;

/// `Forensics` tag.
///
/// Forensics APIs.
///
/// **All endpoints in this service are DEPRECATED.**
pub struct ForensicsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for
/// `GET /web/api/v2.1/applications/{application_id}/forensics`.
///
/// Every field is optional. Array params are serialized comma-joined (set them
/// with the iterator-taking builder methods).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationForensicsQuery {
    /// List of Site IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl ApplicationForensicsQuery {
    /// List of Site IDs to filter by (comma-joined). Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by (comma-joined). Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by (comma-joined). Optional.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/applications/{application_id}/forensics/connections`.
///
/// Every field is optional. Array params are serialized comma-joined (set them
/// with the iterator-taking builder methods).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationConnectionsQuery {
    /// List of Site IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Country code. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

impl ApplicationConnectionsQuery {
    /// List of Site IDs to filter by (comma-joined). Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by (comma-joined). Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by (comma-joined). Optional.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// Country code. Optional.
    pub fn country_code(mut self, code: impl Into<String>) -> Self {
        self.country_code = Some(code.into());
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/applications/{application_id}/forensics/details`.
///
/// Every field is optional. Array params are serialized comma-joined (set them
/// with the iterator-taking builder methods).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationForensicsDetailsQuery {
    /// List of Site IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl ApplicationForensicsDetailsQuery {
    /// List of Site IDs to filter by (comma-joined). Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by (comma-joined). Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by (comma-joined). Optional.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/applications/{application_id}/forensics/export/{export_format}`.
///
/// Every field is optional. Array params are serialized comma-joined (set them
/// with the iterator-taking builder methods).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationForensicsExportQuery {
    /// List of Site IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl ApplicationForensicsExportQuery {
    /// List of Site IDs to filter by (comma-joined). Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by (comma-joined). Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by (comma-joined). Optional.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
}

/// Join an iterator of string-likes into a comma-separated value, as the API
/// expects for array query params.
fn join_csv<I, S>(values: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    values
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

impl ForensicsService<'_> {
    /// `GET /web/api/v2.1/applications/{application_id}/forensics` —
    /// Application Forensics.
    ///
    /// DEPRECATED.
    pub async fn application_forensics(
        &self,
        application_id: impl Into<String>,
        query: &ApplicationForensicsQuery,
    ) -> Result<Response<ApplicationForensicsData>, Error> {
        let path = format!(
            "/web/api/v2.1/applications/{}/forensics",
            application_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `GET /web/api/v2.1/applications/{application_id}/forensics/connections` —
    /// Application Connections.
    ///
    /// [DEPRECATED] Returns an empty array.
    pub async fn application_connections(
        &self,
        application_id: impl Into<String>,
        query: &ApplicationConnectionsQuery,
    ) -> Result<Response<ApplicationConnectionsData>, Error> {
        let path = format!(
            "/web/api/v2.1/applications/{}/forensics/connections",
            application_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `GET /web/api/v2.1/applications/{application_id}/forensics/details` —
    /// Application Forensics - Detailed.
    ///
    /// [DEPRECATED] Returns an empty array.
    pub async fn application_forensics_details(
        &self,
        application_id: impl Into<String>,
        query: &ApplicationForensicsDetailsQuery,
    ) -> Result<Response<ApplicationForensicsDetailsData>, Error> {
        let path = format!(
            "/web/api/v2.1/applications/{}/forensics/details",
            application_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `GET /web/api/v2.1/applications/{application_id}/forensics/export/{export_format}` —
    /// Export Application.
    ///
    /// [DEPRECATED] Returns an empty array.
    ///
    /// `export_format` is a path enum; allowed values (documented for
    /// forward-compat; passed as `String`): `csv`, `json`.
    pub async fn export_application(
        &self,
        application_id: impl Into<String>,
        export_format: impl Into<String>,
        query: &ApplicationForensicsExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/applications/{}/forensics/export/{}",
            application_id.into(),
            export_format.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }
}
