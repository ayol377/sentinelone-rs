//! Models for the `marketplace` tag (Singularity Marketplace).
//!
//! Field nullability follows the SentinelOne spec exactly: a field is a bare
//! `T` only when it is listed in the schema `required` array *and* is not
//! `x-nullable`; every other field is `Option<T>` (default-null behaviour).
//! Enum-typed fields are kept as `String` for forward-compatibility, with the
//! allowed values documented in their doc comment.

use serde::Deserialize;

/// Optional capability annotation attached to a Marketplace application or
/// catalog entry.
///
/// Both fields are required in the schema.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketplaceCapabilityAnnotation {
    /// The capability name to annotate. Allowed values: `automation`,
    /// `enrichment`, `ingestion`, `sandboxing`, `alertsIngestion`,
    /// `findingsIngestion`, `purpleAI`.
    pub name: String,
    /// Status annotation for this capability. Allowed values: `deprecated`,
    /// `comingSoon`, `new`, `legacy`.
    pub status: String,
}

/// Number of entities affected by a mutation.
///
/// Returned (wrapped in `data`) by the install / update-config / delete /
/// enable-or-disable endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Affected {
    /// Number of entities affected by the requested operation. Required.
    pub affected: i64,
}

/// A scope-specific installation of a Marketplace application.
///
/// Embedded in [`ApplicationView::scopes`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationScope {
    /// Account name. Optional/nullable.
    pub account: Option<String>,
    /// Account ID. Optional/nullable.
    pub account_id: Option<String>,
    /// Alert message. Required.
    pub alert_message: String,
    /// Application instance name. Required.
    pub application_instance_name: String,
    /// Creation timestamp (date-time string). Required.
    pub created_at: String,
    /// Creator name. Required.
    pub creator: String,
    /// Creator ID. Required.
    pub creator_id: String,
    /// Desired status. Required. Allowed values: `tokenfetched`, `installed`,
    /// `tokenrefetched`, `active`, `deactivated`, `disabled`, `deleted`.
    pub desired_status: String,
    /// Most recent error log entry. Optional/nullable.
    pub error_log: Option<ApplicationLogResponse>,
    /// Group name. Optional/nullable.
    pub group: Option<String>,
    /// Group ID. Optional/nullable.
    pub group_id: Option<String>,
    /// Whether the installation currently has an alert. Required.
    pub has_alert: bool,
    /// Installation ID. Required.
    pub id: String,
    /// Last entity creation timestamp (date-time string). Optional/nullable.
    pub last_entity_created_at: Option<String>,
    /// Modifier name. Optional/nullable.
    pub modifier: Option<String>,
    /// Modifier ID. Optional/nullable.
    pub modifier_id: Option<String>,
    /// Retry-until timestamp (date-time string). Optional/nullable.
    pub retry_until: Option<String>,
    /// Scope ID. Required.
    pub scope_id: String,
    /// Scope level. Required. Allowed values: `group`, `site`, `account`,
    /// `global`.
    pub scope_level: String,
    /// Site name. Optional/nullable.
    pub site: Option<String>,
    /// Site ID. Optional/nullable.
    pub site_id: Option<String>,
    /// Current status. Required. Allowed values: `draft`, `fetchingtoken`,
    /// `tokenfetched`, `tokenfetchingfailed`, `installing`, `installed`,
    /// `installationfailed`, `refetchingtoken`, `tokenrefetched`, `active`,
    /// `activating`, `activationfailed`, `deactivated`, `deactivating`,
    /// `deactivationfailed`, `disabled`, `disabling`, `deleted`, `deleting`,
    /// `deletionfailed`, `error`, `unrecognized`.
    pub status: String,
    /// Last update timestamp (date-time string). Optional/nullable.
    pub updated_at: Option<String>,
}

/// An installed Marketplace application, grouped across scopes.
///
/// Returned (in the `data` array) by
/// `GET /web/api/v2.1/singularity-marketplace/applications`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationView {
    /// Application catalog ID. Required.
    pub application_catalog_id: String,
    /// Whether the catalog entry is hidden. Required.
    pub application_catalog_is_hidden: bool,
    /// Whether the application currently has an alert. Required.
    pub has_alert: bool,
    /// Icon (URL or encoded image). Required.
    pub icon: String,
    /// Last installation timestamp (date-time string). Required.
    pub last_installed_at: String,
    /// Application name. Required.
    pub name: String,
    /// Per-scope installations of this application. Required.
    pub scopes: Vec<ApplicationScope>,
    /// Optional annotations for specific capabilities with status information.
    /// Optional/nullable.
    pub capability_annotations: Option<Vec<MarketplaceCapabilityAnnotation>>,
}

/// Filter descriptor on a catalog application.
///
/// Both fields are required in the schema.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationCatalogFilter {
    /// Filter type. Required. Allowed values: `JsonPath`, `JsonSchema`.
    #[serde(rename = "type")]
    pub filter_type: String,
    /// Filter value. Required.
    pub value: String,
}

/// A Marketplace Application Catalog entry.
///
/// Returned (in the `data` array) by
/// `GET /web/api/v2.1/singularity-marketplace/applications-catalog`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationCatalogView {
    /// Available plugins. Optional/nullable.
    pub available_plugins: Option<String>,
    /// Category name. Required.
    pub category: String,
    /// Category ID. Required.
    pub category_id: String,
    /// Creation timestamp (date-time string). Optional/nullable.
    pub created_at: Option<String>,
    /// Deletion timestamp (date-time string). Optional/nullable.
    pub deleted_at: Option<String>,
    /// Description. Required.
    pub description: String,
    /// External URL. Optional/nullable.
    pub external_url: Option<String>,
    /// Filter descriptor. Optional/nullable.
    pub filter: Option<ApplicationCatalogFilter>,
    /// Icon (URL or encoded image). Optional/nullable.
    pub icon: Option<String>,
    /// Catalog entry ID. Required.
    pub id: String,
    /// Whether the application is installed. Required.
    pub installed: bool,
    /// Catalog key. Required.
    pub key: String,
    /// Application name. Required.
    pub name: String,
    /// OAuth URL. Optional/nullable.
    pub oauth_url: Option<String>,
    /// Retry policy (seconds). Optional/nullable.
    pub retry_policy: i64,
    /// Short summary. Required.
    pub summary: String,
    /// Toggle state. Required.
    pub toggle_state: bool,
    /// Application type. Required.
    #[serde(rename = "type")]
    pub application_type: String,
    /// Last update timestamp (date-time string). Optional/nullable.
    pub updated_at: Option<String>,
    /// View policy. Required.
    pub view_policy: String,
    /// Optional annotations for specific capabilities with status information.
    /// Optional/nullable.
    pub capability_annotations: Option<Vec<MarketplaceCapabilityAnnotation>>,
}

/// A single configuration field descriptor for a (catalog or installed)
/// application.
///
/// `defaultValue` and `placeHolder` are freeform JSON objects in the spec and
/// are kept as [`serde_json::Value`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationSchema {
    /// Default value (freeform JSON object). Optional/nullable.
    pub default_value: Option<serde_json::Value>,
    /// Expression controlling when this field is enabled. Optional/nullable.
    pub enabled_if: Option<String>,
    /// Allowed values for this field. Optional/nullable.
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
    /// Field ID. Required.
    pub id: String,
    /// Human-readable label. Optional/nullable.
    pub label: Option<String>,
    /// Placeholder (freeform JSON object). Optional/nullable.
    pub place_holder: Option<serde_json::Value>,
    /// Whether this field is required. Required.
    pub required: bool,
    /// Field type. Required.
    #[serde(rename = "type")]
    pub field_type: String,
    /// Current/initial value. Optional/nullable.
    pub value: Option<String>,
}

/// Configuration field set of a catalog application.
///
/// Returned (wrapped in `data`) by
/// `GET /web/api/v2.1/singularity-marketplace/applications-catalog/{applicationCatalogId}/config`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationSchemaFields {
    /// Configuration field descriptors. Required.
    pub fields: Vec<ConfigurationSchema>,
}

/// An installed application together with its configuration fields.
///
/// Returned (wrapped in `data`) by
/// `GET /web/api/v2.1/singularity-marketplace/applications/{applicationId}/config`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationWithConfigurationFields {
    /// Account name. Optional/nullable.
    pub account: Option<String>,
    /// Account ID. Optional/nullable.
    pub account_id: Option<String>,
    /// Alert message. Required.
    pub alert_message: String,
    /// Application instance name. Required.
    pub application_instance_name: String,
    /// Creation timestamp (date-time string). Required.
    pub created_at: String,
    /// Creator name. Required.
    pub creator: String,
    /// Creator ID. Required.
    pub creator_id: String,
    /// Desired status. Required. Allowed values: `tokenfetched`, `installed`,
    /// `tokenrefetched`, `active`, `deactivated`, `disabled`, `deleted`.
    pub desired_status: String,
    /// Most recent error log entry. Optional/nullable.
    pub error_log: Option<ApplicationLogResponse>,
    /// Configuration field descriptors. Required.
    pub fields: Vec<ConfigurationSchema>,
    /// Group name. Optional/nullable.
    pub group: Option<String>,
    /// Group ID. Optional/nullable.
    pub group_id: Option<String>,
    /// Whether the installation currently has an alert. Required.
    pub has_alert: bool,
    /// Installation ID. Required.
    pub id: String,
    /// Last entity creation timestamp (date-time string). Optional/nullable.
    pub last_entity_created_at: Option<String>,
    /// Modifier name. Optional/nullable.
    pub modifier: Option<String>,
    /// Modifier ID. Optional/nullable.
    pub modifier_id: Option<String>,
    /// Retry-until timestamp (date-time string). Optional/nullable.
    pub retry_until: Option<String>,
    /// Scope ID. Required.
    pub scope_id: String,
    /// Scope level. Required. Allowed values: `group`, `site`, `account`,
    /// `global`.
    pub scope_level: String,
    /// Site name. Optional/nullable.
    pub site: Option<String>,
    /// Site ID. Optional/nullable.
    pub site_id: Option<String>,
    /// Current status. Required. Allowed values: `draft`, `fetchingtoken`,
    /// `tokenfetched`, `tokenfetchingfailed`, `installing`, `installed`,
    /// `installationfailed`, `refetchingtoken`, `tokenrefetched`, `active`,
    /// `activating`, `activationfailed`, `deactivated`, `deactivating`,
    /// `deactivationfailed`, `disabled`, `disabling`, `deleted`, `deleting`,
    /// `deletionfailed`, `error`, `unrecognized`.
    pub status: String,
    /// Last update timestamp (date-time string). Optional/nullable.
    pub updated_at: Option<String>,
    /// Optional annotations for specific capabilities with status information.
    /// Optional/nullable.
    pub capability_annotations: Option<Vec<MarketplaceCapabilityAnnotation>>,
}

/// A single application invocation log record.
///
/// Returned (in the `data` array) by
/// `GET /web/api/v2.1/singularity-marketplace/applications/{id}/log`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationLogResponse {
    /// Application ID. Required.
    pub application_id: String,
    /// Date of application invocation start, i.e. when the log record was
    /// created (date-time string). Required.
    pub created_at: String,
    /// HTTP status response code of the application invocation. Optional/nullable.
    pub http_status: Option<i64>,
    /// Result of the application invocation. Optional/nullable.
    pub message: Option<String>,
    /// Status. Required. Allowed values: `Created` (started), `Success`
    /// (finished successfully), `Warning`, `Failure` (final attempt failed),
    /// `Retry` (failed, will be retried).
    pub status: String,
    /// Title of the log record. Optional/nullable.
    pub title: Option<String>,
    /// Tracing ID, useful for searching system logs related to this
    /// invocation. Optional/nullable.
    pub trace_id: Option<String>,
}
