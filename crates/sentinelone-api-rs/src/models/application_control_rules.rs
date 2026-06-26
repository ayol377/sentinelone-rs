//! Models for the `Application Control - Rules` tag (Native Application
//! Control rule management).
//!
//! Field nullability mirrors `swagger_2_1.json`: none of these definitions
//! declare a `required` array, so every field is `Option<T>` ("default null").
//! Enum-typed fields are kept as `String` for forward-compatibility; the
//! allowed values are documented inline.

use serde::Deserialize;

/// Scope information attached to a NAC rule.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacScopeInfo {
    /// Scope ID.
    pub scope_id: Option<String>,
    /// Scope level. Allowed values: `ACCOUNT`, `SITE`, `GROUP`.
    pub scope_level: Option<String>,
    /// Scope name.
    pub scope_name: Option<String>,
    /// Scope path.
    pub scope_path: Option<String>,
}

/// A single set of rule conditions (matching criteria) for a NAC rule.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacRuleConditions {
    /// Publisher.
    pub publisher: Option<String>,
    /// Path.
    pub path: Option<String>,
    /// Signer.
    pub signer: Option<String>,
    /// SHA-256 hash.
    pub sha256: Option<String>,
    /// Process name.
    pub process: Option<String>,
    /// Parent process.
    pub parent_process: Option<String>,
    /// Application version.
    pub application_version: Option<String>,
}

/// Native Application Control rule.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacRule {
    /// Rule ID.
    pub id: Option<String>,
    /// Rule name.
    pub rule_name: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Scope.
    pub scope: Option<NacScopeInfo>,
    /// OS types. Each value is one of: `MACOS`, `WINDOWS`.
    pub os_type: Option<Vec<String>>,
    /// Rule conditions.
    pub parameters: Option<NacRuleConditions>,
    /// Timestamp when the rule was created (ISO-8601, e.g. `2024-04-03T12:00:00Z`).
    pub created_at: Option<String>,
    /// Created by.
    pub created_by: Option<String>,
    /// Whether propagation is enabled.
    pub propagation: Option<bool>,
    /// Rule behavior. Allowed values: `ALLOW`, `MONITOR`, `BLOCK`.
    pub behavior: Option<String>,
    /// Exceptions.
    pub exceptions: Option<Vec<NacRuleConditions>>,
}

/// Validation error returned for an invalid request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GqlValidationError {
    /// Error code for the validation issue (e.g. `RULE_NAME_REQUIRED`).
    pub code: Option<String>,
    /// Validation error message.
    pub message: Option<String>,
    /// The raw input value that caused the validation error.
    pub value: Option<String>,
}

/// Common response envelope for NAC mutations (create/update/delete).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacCommonResponse {
    /// Whether the request was successful.
    pub success: Option<bool>,
    /// Resource ID.
    pub id: Option<String>,
    /// Status code.
    pub status_code: Option<i64>,
    /// Status message.
    pub status_message: Option<String>,
    /// Validation errors.
    pub validation_errors: Option<Vec<GqlValidationError>>,
}

/// Cursor-based pagination metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    /// Start cursor.
    pub start_cursor: Option<String>,
    /// End cursor.
    pub end_cursor: Option<String>,
    /// Whether there is a next page.
    pub has_next_page: Option<bool>,
    /// Whether there is a previous page.
    pub has_previous_page: Option<bool>,
}

/// A rule together with its pagination cursor.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacRuleEdge {
    /// Rule.
    pub node: Option<NacRule>,
    /// Cursor.
    pub cursor: Option<String>,
}

/// Paginated NAC rules connection (returned by the query endpoint).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacRuleConnection {
    /// Page info.
    pub page_info: Option<PageInfo>,
    /// Rule list with cursors.
    pub edges: Option<Vec<NacRuleEdge>>,
    /// Total count.
    pub total_count: Option<i64>,
}

/// Presigned upload URL response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacFileUploadPreSignedUrl {
    /// File operation ID.
    pub file_operation_id: Option<String>,
    /// Presigned upload URL.
    pub url: Option<String>,
    /// Expiry time in epoch milliseconds.
    pub expires_at: Option<i64>,
}

/// Status of a CSV import operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacCsvImportRulesStatus {
    /// File operation ID.
    pub file_operation_id: Option<String>,
    /// Status. Allowed values: `IN_PROGRESS`, `SUCCESS`, `FAILED`.
    pub status: Option<String>,
    /// Status message.
    pub status_message: Option<String>,
}

/// Status of a CSV export operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacCsvExportRulesStatus {
    /// File operation ID.
    pub file_operation_id: Option<String>,
    /// Status. Allowed values: `IN_PROGRESS`, `SUCCESS`, `FAILED`.
    pub status: Option<String>,
    /// Expiry time in epoch milliseconds.
    pub expires_at: Option<i64>,
    /// Download URL (populated once the export succeeds).
    pub download_url: Option<String>,
    /// Status message.
    pub status_message: Option<String>,
}

/// A single change-log entry for a rule.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLog {
    /// Version.
    pub version: Option<i64>,
    /// Updated by.
    pub updated_by: Option<String>,
    /// Timestamp when the rule was updated (ISO-8601, e.g. `2024-04-03T12:00:00Z`).
    pub updated_at: Option<String>,
}

/// Change history for a single rule.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacRuleChangeLog {
    /// Rule ID.
    pub rule_id: Option<i64>,
    /// Created by.
    pub created_by: Option<String>,
    /// Timestamp when the rule was created (ISO-8601, e.g. `2024-04-03T12:00:00Z`).
    pub created_at: Option<String>,
    /// List of change logs.
    pub change_log: Option<Vec<ChangeLog>>,
}

/// Metadata describing a rule table column.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnMetadata {
    /// Enum values for this column, if applicable (e.g. `ALLOW`, `BLOCK`, `MONITOR`).
    pub enum_values: Option<Vec<String>>,
    /// Field name (e.g. `RULE_NAME`).
    pub field_id: Option<String>,
    /// Supported filter types. Each value is one of: `BOOLEAN_EQUAL`,
    /// `BOOLEAN_IN`, `DATE_RANGE`, `FULLTEXT`, `INT_EQUAL`, `INT_IN`,
    /// `INT_RANGE`, `LONG_EQUAL`, `LONG_IN`, `LONG_RANGE`, `STRING_EQUAL`,
    /// `STRING_IN`.
    pub filter_types: Option<Vec<String>>,
    /// Whether grouping is supported.
    pub groupable: Option<bool>,
    /// Whether sorting is supported.
    pub sortable: Option<bool>,
}

/// Metadata for the sample NAC rules CSV file.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacRulesSampleFileMetadata {
    /// Presigned URL to download the sample file.
    pub url: Option<String>,
    /// Expiry time in epoch milliseconds.
    pub expires_at: Option<i64>,
}
