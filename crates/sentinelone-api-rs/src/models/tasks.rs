//! Models for the `Tasks` tag (task-related configuration operations).
//!
//! Response entities are envelope-stripped: the SentinelOne `{ data, errors }`
//! / `{ data, pagination, errors }` wrappers are handled by
//! [`crate::pagination::Response`] / [`crate::pagination::Paginated`]; the
//! structs below model the inner `data`.

use serde::Deserialize;

/// Task configuration of a scope.
///
/// Inner `data` of `tasks.schemas_ResponseTaskSchema_200`, returned by
/// `GET /web/api/v2.1/tasks-configuration` and `PUT /web/api/v2.1/tasks-configuration`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskConfiguration {
    /// Inherit parent's scope Max Concurrent configuration. Required.
    pub inherit_parent_concurrency_config: bool,
    /// Inherit parent's scope Maintenance windows configuration. Required.
    pub inherit_parent_maintenance_config: bool,
    /// Max concurrent. Required.
    pub max_concurrent: i64,
    /// Timezone gmt. Required.
    pub timezone_gmt: String,
    /// Stores the maintenance time for each day. Required.
    ///
    /// Freeform object keyed by day; values are arbitrary objects.
    pub maintenance_windows_by_day: serde_json::Value,
    /// Defines task's type and priority.
    ///
    /// Enum: `agents_upgrade`, `agent_version_change`, `auto_deploy`,
    /// `script_execution`, `cis_scan`, `gad`, `forensics_collection`.
    /// Optional/nullable.
    #[serde(default)]
    pub task_type: Option<String>,
    /// Scope's parent max concurrent limit, must not exceed. Optional/nullable.
    #[serde(default)]
    pub parent_max_concurrent: Option<i64>,
    /// Timestamp of last concurrency configuration update (ISO-8601 string).
    /// Optional/nullable.
    #[serde(default)]
    pub concurrency_config_updated_at: Option<String>,
    /// Timestamp of last maintenance configuration update (ISO-8601 string).
    /// Optional/nullable.
    #[serde(default)]
    pub maintenance_config_updated_at: Option<String>,
    /// User name of last updated maintenance configuration. Optional/nullable.
    #[serde(default)]
    pub maintenance_config_updated_by: Option<String>,
    /// User name of last updated concurrency configuration. Optional/nullable.
    #[serde(default)]
    pub concurrency_config_updated_by: Option<String>,
    /// Flexible maintenance window policy (returned when set). Freeform object.
    /// Optional/nullable.
    #[serde(default)]
    pub policy_payload: Option<serde_json::Value>,
}

/// Task configuration in flexible maintenance window format.
///
/// Inner `data` of `tasks.schemas_FlexibleMaintenanceResponseSchema_200`,
/// returned by `GET /web/api/v2.1/tasks-configuration/flexible` and
/// `PUT /web/api/v2.1/tasks-configuration/flexible`. The schema marks no field
/// required, so all fields are optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleTaskConfiguration {
    /// Task type.
    ///
    /// Enum: `agents_upgrade`, `agent_version_change`, `auto_deploy`,
    /// `script_execution`, `cis_scan`, `gad`, `forensics_collection`.
    /// Optional/nullable.
    #[serde(default)]
    pub task_type: Option<String>,
    /// Inherit parent concurrency config. Optional/nullable.
    #[serde(default)]
    pub inherit_parent_concurrency_config: Option<bool>,
    /// Inherit parent maintenance config. Optional/nullable.
    #[serde(default)]
    pub inherit_parent_maintenance_config: Option<bool>,
    /// Max concurrent. Optional/nullable.
    #[serde(default)]
    pub max_concurrent: Option<i64>,
    /// Parent max concurrent. Optional/nullable.
    #[serde(default)]
    pub parent_max_concurrent: Option<i64>,
    /// Concurrency config updated at (ISO-8601 string). Optional/nullable.
    #[serde(default)]
    pub concurrency_config_updated_at: Option<String>,
    /// Concurrency config updated by. Optional/nullable.
    #[serde(default)]
    pub concurrency_config_updated_by: Option<String>,
    /// Maintenance config updated at (ISO-8601 string). Optional/nullable.
    #[serde(default)]
    pub maintenance_config_updated_at: Option<String>,
    /// Maintenance config updated by. Optional/nullable.
    #[serde(default)]
    pub maintenance_config_updated_by: Option<String>,
    /// Timezone. Optional/nullable.
    #[serde(default)]
    pub timezone: Option<String>,
    /// Frequency. Optional/nullable.
    #[serde(default)]
    pub frequency: Option<String>,
    /// Interval. Optional/nullable.
    #[serde(default)]
    pub interval: Option<i64>,
    /// Starts on (ISO-8601 string). Optional/nullable.
    #[serde(default)]
    pub starts_on: Option<String>,
    /// End. Freeform value (untyped in the spec). Optional/nullable.
    #[serde(default)]
    pub end: Option<serde_json::Value>,
    /// Active. Optional/nullable.
    #[serde(default)]
    pub active: Option<bool>,
    /// Rules. Freeform value (untyped in the spec). Optional/nullable.
    #[serde(default)]
    pub rules: Option<serde_json::Value>,
}

/// Boolean flag response.
///
/// Inner `data` of `tasks.schemas_BooleanSchema_200`, returned by
/// `GET /web/api/v2.1/tasks-configuration/has-explicit-subscope`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskBooleanFlag {
    /// Returns boolean value for the request. Optional/nullable.
    #[serde(default)]
    pub flag: Option<bool>,
}

/// A child scope entry with a local (non-inherited) task configuration.
///
/// Item of the `data` array in `tasks.schemas_SubscopeFullHierarchySchema_many_200`,
/// returned by `GET /web/api/v2.1/tasks-configuration/explicit-subscopes`. The
/// schema marks no field required, so all fields are optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSubscope {
    /// Account id. Optional/nullable.
    #[serde(default)]
    pub account_id: Option<String>,
    /// Account name. Optional/nullable.
    #[serde(default)]
    pub account_name: Option<String>,
    /// Site id. Optional/nullable.
    #[serde(default)]
    pub site_id: Option<String>,
    /// Site name. Optional/nullable.
    #[serde(default)]
    pub site_name: Option<String>,
    /// Group id. Optional/nullable.
    #[serde(default)]
    pub group_id: Option<String>,
    /// Group name. Optional/nullable.
    #[serde(default)]
    pub group_name: Option<String>,
}
