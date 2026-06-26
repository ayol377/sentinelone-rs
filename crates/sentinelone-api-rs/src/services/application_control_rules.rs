//! `Application Control - Rules` tag — Native Application Control rule
//! management APIs.
//!
//! NOTE: unlike most SentinelOne v2.1 endpoints, these NAC endpoints return
//! their payloads "bare" (the response schema is a direct `$ref`, e.g.
//! `NACRule`), not wrapped in the standard `{ data, pagination, errors }`
//! envelope. Methods therefore return the model types directly rather than
//! [`crate::pagination::Paginated`] / [`crate::pagination::Response`].

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::application_control_rules::*;

/// `Application Control - Rules` tag — Native Application Control rule
/// management (create/update/delete rules, query rules, CSV import/export,
/// column metadata and change logs).
pub struct ApplicationControlRulesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ===========================================================================
// Body params
// ===========================================================================

/// Scope selector as JSON. Used as both a standalone delete body and as a
/// nested field of other request bodies.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeSelectorInput {
    /// Scope type. Allowed values: `ACCOUNT`, `SITE`, `GROUP`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_ids: Option<Vec<String>>,
}

impl ScopeSelectorInput {
    /// Set the scope type. Allowed values: `ACCOUNT`, `SITE`, `GROUP`.
    pub fn scope_type(mut self, v: impl Into<String>) -> Self {
        self.scope_type = Some(v.into());
        self
    }
    /// Set the scope IDs.
    pub fn scope_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.scope_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
}

/// Sort directive for a rules query.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonSortInput {
    /// Field name to sort by (e.g. `RULE_NAME`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,
    /// Sort direction. Allowed values: `ASC`, `DESC`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

impl CommonSortInput {
    /// Set the field to sort by (e.g. `RULE_NAME`).
    pub fn by(mut self, v: impl Into<String>) -> Self {
        self.by = Some(v.into());
        self
    }
    /// Set the sort direction. Allowed values: `ASC`, `DESC`.
    pub fn order(mut self, v: impl Into<String>) -> Self {
        self.order = Some(v.into());
        self
    }
}

/// Rules query request. Used by the rules query endpoint and the CSV export
/// endpoint.
///
/// `filters` is a deeply-nested / polymorphic structure
/// ([`CommonFilterInput`](https://) with 14 mutually-exclusive variant
/// sub-objects), so it is kept as freeform [`serde_json::Value`] for
/// forward-compatibility.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRulesRequest {
    /// Scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_selector: Option<ScopeSelectorInput>,
    /// Filters (freeform; each item is a `CommonFilterInput`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<serde_json::Value>>,
    /// Sorting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Vec<CommonSortInput>>,
    /// Include rules from parent scopes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
}

impl GetRulesRequest {
    /// Set the scope selector.
    pub fn scope_selector(mut self, v: ScopeSelectorInput) -> Self {
        self.scope_selector = Some(v);
        self
    }
    /// Set the filters (freeform; each item is a `CommonFilterInput`).
    pub fn filters(mut self, v: Vec<serde_json::Value>) -> Self {
        self.filters = Some(v);
        self
    }
    /// Set the sort directives.
    pub fn sorts(mut self, v: Vec<CommonSortInput>) -> Self {
        self.sorts = Some(v);
        self
    }
    /// Set whether to include rules from parent scopes.
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
}

/// Rule conditions (matching criteria) supplied when creating/updating a rule.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NacRuleConditionsInput {
    /// Publisher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    /// Path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Signer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<String>,
    /// SHA-256 hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    /// Process name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    /// Parent process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_process: Option<String>,
}

/// NAC rule input. Used to create a rule (when `id` is absent) or update an
/// existing one.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NacRuleInput {
    /// Rule ID. Omit to create a new rule; supply to update an existing one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Rule name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeSelectorInput>,
    /// OS types. Each value is one of: `MACOS`, `WINDOWS`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<Vec<String>>,
    /// Whether propagation is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagation: Option<bool>,
    /// Rule conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<NacRuleConditionsInput>,
    /// Exceptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<Vec<NacRuleConditionsInput>>,
    /// Rule behavior. Allowed values: `ALLOW`, `MONITOR`, `BLOCK`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<String>,
}

/// CSV upload input (references a previously uploaded file by operation ID).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NacCsvUploadedFileInput {
    /// File operation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_operation_id: Option<String>,
    /// ETag returned after upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// Scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeSelectorInput>,
}

/// Presigned upload URL request.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NacFileUploadRequestInput {
    /// File name (e.g. `rules.csv`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size_in_bytes: Option<i64>,
    /// Scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeSelectorInput>,
}

// ===========================================================================
// Query params
// ===========================================================================

/// Query params for `GET .../nac/csv/export/status/{fileOperationId}`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCsvExportStatusQuery {
    /// Scope Type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl GetCsvExportStatusQuery {
    /// Set the scope type.
    pub fn scope_type(mut self, v: impl Into<String>) -> Self {
        self.scope_type = Some(v.into());
        self
    }
    /// Set the scope ID.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

/// Query params for `GET .../nac/csv/status/{fileOperationId}`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCsvStatusQuery {
    /// Scope Type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl GetCsvStatusQuery {
    /// Set the scope type.
    pub fn scope_type(mut self, v: impl Into<String>) -> Self {
        self.scope_type = Some(v.into());
        self
    }
    /// Set the scope ID.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

/// Query params for `DELETE .../nac/rules`.
///
/// `ids` (required) is serialized comma-joined, as the API expects for array
/// query params.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRulesQuery {
    /// Rule IDs to delete (comma-joined). Required.
    pub ids: String,
}

impl DeleteRulesQuery {
    /// Build from an iterator of IDs (joined by comma).
    pub fn ids<I, S>(ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = ids
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        Self { ids: joined }
    }
}

/// Query params for `POST .../nac/rules/query` (pagination controls).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRulesQuery {
    /// Pagination page size. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Cursor for pagination. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Pagination direction. Allowed values: `FORWARD`, `BACKWARD`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

impl GetRulesQuery {
    /// Set the pagination page size.
    pub fn page_size(mut self, n: i64) -> Self {
        self.page_size = Some(n);
        self
    }
    /// Set the pagination cursor.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Set the pagination direction. Allowed values: `FORWARD`, `BACKWARD`.
    pub fn direction(mut self, v: impl Into<String>) -> Self {
        self.direction = Some(v.into());
        self
    }
}

/// Query params for `GET .../nac/rules/{id}`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRuleQuery {
    /// Scope Type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl GetRuleQuery {
    /// Set the scope type.
    pub fn scope_type(mut self, v: impl Into<String>) -> Self {
        self.scope_type = Some(v.into());
        self
    }
    /// Set the scope ID.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

/// Query params for `GET .../nac/rules/{id}/changelog`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRuleChangelogQuery {
    /// Scope Type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl GetRuleChangelogQuery {
    /// Set the scope type.
    pub fn scope_type(mut self, v: impl Into<String>) -> Self {
        self.scope_type = Some(v.into());
        self
    }
    /// Set the scope ID.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

// ===========================================================================
// Service methods
// ===========================================================================

impl ApplicationControlRulesService<'_> {
    /// `POST /web/api/v2.1/nac/config/api/v1/nac/csv/export` — Export NAC rules
    /// to CSV.
    ///
    /// Triggers an async CSV export of NAC rules based on filters and sorting.
    pub async fn export_csv(
        &self,
        body: &GetRulesRequest,
    ) -> Result<NacCsvExportRulesStatus, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/nac/config/api/v1/nac/csv/export", body)
            .await?)
    }

    /// `GET /web/api/v2.1/nac/config/api/v1/nac/csv/export/sample` — Download
    /// sample CSV file.
    ///
    /// Returns a presigned URL to download a sample NAC rules CSV file for
    /// reference.
    pub async fn export_csv_sample(&self) -> Result<NacRulesSampleFileMetadata, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/nac/config/api/v1/nac/csv/export/sample", None)
            .await?)
    }

    /// `GET /web/api/v2.1/nac/config/api/v1/nac/csv/export/status/{fileOperationId}`
    /// — Get CSV export status.
    ///
    /// Retrieves the current status of a CSV export request.
    pub async fn csv_export_status(
        &self,
        file_operation_id: impl Into<String>,
        query: &GetCsvExportStatusQuery,
    ) -> Result<NacCsvExportRulesStatus, Error> {
        let path = format!(
            "/web/api/v2.1/nac/config/api/v1/nac/csv/export/status/{}",
            file_operation_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `POST /web/api/v2.1/nac/config/api/v1/nac/csv/import` — Import CSV file
    /// uploaded.
    ///
    /// Imports the CSV file that has been uploaded.
    pub async fn import_csv(
        &self,
        body: &NacCsvUploadedFileInput,
    ) -> Result<NacCsvImportRulesStatus, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/nac/config/api/v1/nac/csv/import", body)
            .await?)
    }

    /// `GET /web/api/v2.1/nac/config/api/v1/nac/csv/status/{fileOperationId}` —
    /// Get CSV processing status.
    ///
    /// Retrieves the current status of a CSV file processing request.
    pub async fn csv_status(
        &self,
        file_operation_id: impl Into<String>,
        query: &GetCsvStatusQuery,
    ) -> Result<NacCsvImportRulesStatus, Error> {
        let path = format!(
            "/web/api/v2.1/nac/config/api/v1/nac/csv/status/{}",
            file_operation_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `POST /web/api/v2.1/nac/config/api/v1/nac/csv/upload-url` — Create
    /// presigned CSV upload URL.
    ///
    /// Generates a presigned URL for uploading a CSV file containing NAC rules.
    pub async fn create_csv_upload_url(
        &self,
        body: &NacFileUploadRequestInput,
    ) -> Result<NacFileUploadPreSignedUrl, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/nac/config/api/v1/nac/csv/upload-url", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/nac/config/api/v1/nac/rules` — Delete NAC rules.
    ///
    /// Deletes multiple NAC rules by their IDs. `ids` (query) is required; the
    /// optional [`ScopeSelectorInput`] body narrows the scope.
    pub async fn delete_rules(
        &self,
        query: &DeleteRulesQuery,
        body: Option<&ScopeSelectorInput>,
    ) -> Result<NacCommonResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<ScopeSelectorInput, NacCommonResponse>(
                Method::DELETE,
                "/web/api/v2.1/nac/config/api/v1/nac/rules",
                q,
                body,
            )
            .await?)
    }

    /// `POST /web/api/v2.1/nac/config/api/v1/nac/rules` — Create or update NAC
    /// rule.
    ///
    /// Creates a new NAC rule if `id` is absent, otherwise updates the existing
    /// rule.
    pub async fn create_or_update_rule(
        &self,
        body: &NacRuleInput,
    ) -> Result<NacCommonResponse, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/nac/config/api/v1/nac/rules", body)
            .await?)
    }

    /// `GET /web/api/v2.1/nac/config/api/v1/nac/rules/metadata/columns` — Get
    /// column metadata.
    ///
    /// Retrieves metadata about rule table columns. Returns a bare array.
    pub async fn rule_column_metadata(&self) -> Result<Vec<ColumnMetadata>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/nac/config/api/v1/nac/rules/metadata/columns", None)
            .await?)
    }

    /// `POST /web/api/v2.1/nac/config/api/v1/nac/rules/query` — Get paginated
    /// NAC rules.
    ///
    /// Retrieves a paginated list of NAC rules with optional filtering and
    /// sorting. Pagination is controlled by [`GetRulesQuery`]; the filter/sort
    /// criteria go in the optional [`GetRulesRequest`] body.
    pub async fn query_rules(
        &self,
        query: &GetRulesQuery,
        body: Option<&GetRulesRequest>,
    ) -> Result<NacRuleConnection, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<GetRulesRequest, NacRuleConnection>(
                Method::POST,
                "/web/api/v2.1/nac/config/api/v1/nac/rules/query",
                q,
                body,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/nac/config/api/v1/nac/rules/{id}` — Get a single NAC
    /// rule.
    ///
    /// Retrieves a single NAC rule by its ID.
    pub async fn get_rule(
        &self,
        id: impl Into<String>,
        query: &GetRuleQuery,
    ) -> Result<NacRule, Error> {
        let path = format!(
            "/web/api/v2.1/nac/config/api/v1/nac/rules/{}",
            id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `PUT /web/api/v2.1/nac/config/api/v1/nac/rules/{id}` — Update NAC rule.
    ///
    /// Updates an existing Native Application Control rule by ID.
    pub async fn update_rule(
        &self,
        id: impl Into<String>,
        body: &NacRuleInput,
    ) -> Result<NacCommonResponse, Error> {
        let path = format!(
            "/web/api/v2.1/nac/config/api/v1/nac/rules/{}",
            id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<NacRuleInput, NacCommonResponse>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/nac/config/api/v1/nac/rules/{id}/changelog` — Get rule
    /// change log.
    ///
    /// Retrieves the change history for a specific rule.
    pub async fn rule_changelog(
        &self,
        id: impl Into<String>,
        query: &GetRuleChangelogQuery,
    ) -> Result<NacRuleChangeLog, Error> {
        let path = format!(
            "/web/api/v2.1/nac/config/api/v1/nac/rules/{}/changelog",
            id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }
}
