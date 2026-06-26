//! Response models for the `Hyperautomation` tag.
//!
//! Field nullability follows the spec: a field is bare `T` only when it is in
//! the schema `required` array and not `x-nullable`; otherwise `Option<T>`
//! (serde defaults a missing value to `None`). Enum-typed fields are kept as
//! `String` for forward compatibility — the allowed values are documented in
//! each field's doc comment. Cross-tag / freeform objects are `serde_json::Value`.

use serde::Deserialize;

/// A user attributed to a workflow (creator/updater).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowAttributionUser {
    /// User id. **Required.**
    pub id: String,
    /// User email. Optional / nullable.
    pub email: Option<String>,
    /// User display name. Optional / nullable.
    pub name: Option<String>,
}

/// Canvas dimensions of a workflow.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    /// Canvas height. Optional / nullable.
    pub height: Option<f64>,
    /// Canvas width. Optional / nullable.
    pub width: Option<f64>,
}

/// A single workflow execution row, as returned by the executions list.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowExecutionInfratable {
    /// Scope id. **Required.**
    pub scope_id: String,
    /// Scope level. Enum: `tenant`, `account`, `site`. **Required.**
    pub scope_level: String,
    /// Management id. **Required.**
    pub mgmt_id: String,
    /// Execution id (uuid). **Required.**
    pub id: String,
    /// Workflow version id (uuid). **Required.**
    pub version_id: String,
    /// Workflow id (uuid). **Required.**
    pub workflow_id: String,
    /// Workflow name. **Required** (nullable in spec). Optional / nullable.
    pub workflow_name: Option<String>,
    /// Workflow description. **Required** (nullable in spec). Optional / nullable.
    pub workflow_description: Option<String>,
    /// Workflow tags. **Required** (nullable in spec). Optional / nullable.
    pub workflow_tags: Option<Vec<String>>,
    /// Number of versions. **Required** (nullable in spec). Optional / nullable.
    pub version_count: Option<i64>,
    /// Trigger type. Enum: `http_trigger`, `scheduled_trigger`, `email_trigger`,
    /// `manual_trigger`, `singularity_response_trigger`, `snippet_trigger`. **Required.**
    pub trigger: String,
    /// Site name. Optional / nullable.
    pub site_name: Option<String>,
    /// Account name. Optional / nullable.
    pub account_name: Option<String>,
    /// Parent scope id. Optional / nullable.
    pub parent_scope_id: Option<String>,
    /// Site state. Enum: `active`, `deleted`, `expired`. Optional / nullable.
    pub site_state: Option<String>,
    /// Account state. Enum: `active`, `deleted`, `expired`. Optional / nullable.
    pub account_state: Option<String>,
    /// Creation timestamp (date-time). Optional / nullable.
    pub created_at: Option<String>,
    /// Update timestamp (date-time). Optional / nullable.
    pub updated_at: Option<String>,
    /// Execution state. Enum: `Running`, `Pending`, `Stuck`, `Completed`, `Error`,
    /// `Waiting`, `Aborted`, `CompletedWithErrors`. Optional / nullable.
    pub state: Option<String>,
    /// Offload state. Enum: `Running`, `PartiallyRunning`, `Completed`,
    /// `PartiallyCompleted`, `Error`, `Waiting`. Optional / nullable.
    pub offload_state: Option<String>,
    /// Execution duration (duration string). Optional / nullable.
    pub duration: Option<String>,
    /// Time saved (number). Optional / nullable.
    pub time_saved: Option<f64>,
    /// Number of executed actions. Optional / nullable.
    pub executed_actions: Option<i64>,
    /// Executed action groups (freeform object). Optional / nullable.
    pub executed_action_groups: Option<serde_json::Value>,
    /// Whether execution output exists. Optional / nullable.
    pub has_execution_output: Option<bool>,
}

/// A workflow execution returned by the manual-trigger endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowExecutionRead {
    /// Scope id. **Required.**
    pub scope_id: String,
    /// Scope level. Enum: `tenant`, `account`, `site`. **Required.**
    pub scope_level: String,
    /// Management id. **Required.**
    pub mgmt_id: String,
    /// Execution id (uuid). **Required.**
    pub id: String,
    /// Workflow version id (uuid). **Required.**
    pub version_id: String,
    /// Workflow id (uuid). **Required.**
    pub workflow_id: String,
    /// Creation timestamp (date-time). Optional / nullable.
    pub created_at: Option<String>,
    /// Update timestamp (date-time). Optional / nullable.
    pub updated_at: Option<String>,
    /// Execution state. Enum: `Running`, `Pending`, `Stuck`, `Completed`, `Error`,
    /// `Waiting`, `Aborted`, `CompletedWithErrors`. Optional / nullable.
    pub state: Option<String>,
    /// Offload state. Enum: `Running`, `PartiallyRunning`, `Completed`,
    /// `PartiallyCompleted`, `Error`, `Waiting`. Optional / nullable.
    pub offload_state: Option<String>,
    /// Execution duration (duration string). Optional / nullable.
    pub duration: Option<String>,
    /// Time saved (number). Optional / nullable.
    pub time_saved: Option<f64>,
    /// Number of executed actions. Optional / nullable.
    pub executed_actions: Option<i64>,
    /// Executed action groups (freeform object). Optional / nullable.
    pub executed_action_groups: Option<serde_json::Value>,
    /// Whether execution output exists. Optional / nullable.
    pub has_execution_output: Option<bool>,
}

/// Enriched per-action error data within an execution response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedActionExecutionErrorData {
    /// Action id. **Required.**
    pub action_id: String,
    /// Action execution name. **Required.**
    pub action_execution_name: String,
    /// Action display name. Optional / nullable.
    pub action_display_name: Option<String>,
    /// Action error message. Optional / nullable.
    pub action_error: Option<String>,
}

/// A workflow execution fetched by id.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicWorkflowExecutionResponse {
    /// Scope id. **Required.**
    pub scope_id: String,
    /// Scope level. Enum: `tenant`, `account`, `site`. **Required.**
    pub scope_level: String,
    /// Management id. **Required.**
    pub mgmt_id: String,
    /// Execution id (uuid). **Required.**
    pub id: String,
    /// Workflow version id (uuid). **Required.**
    pub version_id: String,
    /// Workflow id (uuid). **Required.**
    pub workflow_id: String,
    /// Workflow state. Enum: `active`, `inactive`, `deactivated`, `draft`. **Required.**
    pub workflow_state: String,
    /// Singularity response event type. Enum: `alert`, `incident`,
    /// `misconfiguration`, `vulnerability`, `activity`. **Required** (nullable). Optional / nullable.
    pub singularity_response_event_type: Option<String>,
    /// Singularity response event id. **Required** (nullable). Optional / nullable.
    pub singularity_response_event_id: Option<String>,
    /// Creation timestamp (date-time). Optional / nullable.
    pub created_at: Option<String>,
    /// Update timestamp (date-time). Optional / nullable.
    pub updated_at: Option<String>,
    /// Execution state. Enum: `Running`, `Pending`, `Stuck`, `Completed`, `Error`,
    /// `Waiting`, `Aborted`, `CompletedWithErrors`. Optional / nullable.
    pub state: Option<String>,
    /// Execution duration (duration string). Optional / nullable.
    pub duration: Option<String>,
    /// Time saved (number). Optional / nullable.
    pub time_saved: Option<f64>,
    /// Number of executed actions. Optional / nullable.
    pub executed_actions: Option<i64>,
    /// Whether execution output exists. Optional / nullable.
    pub has_execution_output: Option<bool>,
    /// Per-action error data. Optional / nullable.
    pub error_actions: Option<Vec<EnrichedActionExecutionErrorData>>,
}

/// Raw output of a workflow execution.
#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionFinalResponse {
    /// The execution output (freeform object). **Required.**
    #[serde(rename = "ExecutionOutput")]
    pub execution_output: serde_json::Value,
}

/// A single workflow version record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowVersionData {
    /// Scope id. **Required.**
    pub scope_id: String,
    /// Scope level. Enum: `tenant`, `account`, `site`. **Required.**
    pub scope_level: String,
    /// Management id. **Required.**
    pub mgmt_id: String,
    /// Workflow id (uuid). **Required.**
    pub id: String,
    /// Version id (uuid). **Required.**
    pub version_id: String,
    /// Workflow name. **Required.**
    pub name: String,
    /// Creator user id. **Required.**
    pub created_by: String,
    /// Updater user id. **Required.**
    pub updated_by: String,
    /// User who created the workflow. Optional / nullable.
    pub created_by_user: Option<WorkflowAttributionUser>,
    /// User who last updated the workflow. Optional / nullable.
    pub updated_by_user: Option<WorkflowAttributionUser>,
    /// Creation timestamp (date-time). Optional / nullable.
    pub created_at: Option<String>,
    /// Update timestamp (date-time). Optional / nullable.
    pub updated_at: Option<String>,
    /// Version description. Optional / nullable.
    pub version_description: Option<String>,
    /// Execution timeout (seconds). Optional / nullable.
    pub timeout: Option<i64>,
    /// Daily maximum executions. Optional / nullable.
    pub daily_max_executions: Option<i64>,
    /// Maximum concurrency. Optional / nullable.
    pub max_concurrency: Option<i64>,
    /// Notification recipients. Optional / nullable.
    pub notify_to: Option<Vec<String>>,
    /// Time saved. Optional / nullable.
    pub time_saved: Option<i64>,
    /// Time saved unit. Optional / nullable.
    pub time_saved_unit: Option<String>,
    /// Whether the workflow is a snippet. Optional / nullable.
    pub is_snippet: Option<bool>,
    /// Canvas dimensions. Optional / nullable.
    pub dimensions: Option<Dimensions>,
    /// Workflow description. Optional / nullable.
    pub description: Option<String>,
    /// Activation timestamp (date-time). Optional / nullable.
    pub activated_at: Option<String>,
    /// Number of versions. Optional / nullable.
    pub version_count: Option<i64>,
    /// Workflow state (enum string). Optional / nullable.
    pub state: Option<String>,
    /// Lifecycle state. Enum: `active`, `archived`, `deleted`. Optional / nullable.
    pub lifecycle_state: Option<String>,
    /// Status. Enum: `idle`, `running`. Optional / nullable.
    pub status: Option<String>,
    /// Execution time (date-time). Optional / nullable.
    pub execution_time: Option<String>,
    /// Execution status. Enum: `Running`, `Pending`, `Stuck`, `Completed`, `Error`,
    /// `Waiting`, `Aborted`, `CompletedWithErrors`. Optional / nullable.
    pub execution_status: Option<String>,
}

/// Wrapper response for the workflow-versions list endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicWorkflowVersionData {
    /// The list of versions. **Required.**
    pub versions: Vec<WorkflowVersionData>,
}

/// A workflow read record with scope metadata (the `workflow` field of a
/// [`WorkflowWithActionsMeta`] row).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfratableWorkflowReadScopeMeta {
    /// Scope id. **Required.**
    pub scope_id: String,
    /// Scope level. Enum: `tenant`, `account`, `site`. **Required.**
    pub scope_level: String,
    /// Management id. **Required.**
    pub mgmt_id: String,
    /// Workflow id (uuid). **Required.**
    pub id: String,
    /// Version id (uuid). **Required.**
    pub version_id: String,
    /// Workflow name. **Required.**
    pub name: String,
    /// Creator user id. **Required.**
    pub created_by: String,
    /// Updater user id. **Required.**
    pub updated_by: String,
    /// User who created the workflow. Optional / nullable.
    pub created_by_user: Option<WorkflowAttributionUser>,
    /// User who last updated the workflow. Optional / nullable.
    pub updated_by_user: Option<WorkflowAttributionUser>,
    /// Tags. Optional / nullable.
    pub tags: Option<Vec<String>>,
    /// Site name. Optional / nullable.
    pub site_name: Option<String>,
    /// Account name. Optional / nullable.
    pub account_name: Option<String>,
    /// Parent scope id. Optional / nullable.
    pub parent_scope_id: Option<String>,
    /// Site state. Enum: `active`, `deleted`, `expired`. Optional / nullable.
    pub site_state: Option<String>,
    /// Account state. Enum: `active`, `deleted`, `expired`. Optional / nullable.
    pub account_state: Option<String>,
    /// Creation timestamp (date-time). Optional / nullable.
    pub created_at: Option<String>,
    /// Update timestamp (date-time). Optional / nullable.
    pub updated_at: Option<String>,
    /// Version description. Optional / nullable.
    pub version_description: Option<String>,
    /// Execution timeout (seconds). Optional / nullable.
    pub timeout: Option<i64>,
    /// Daily maximum executions. Optional / nullable.
    pub daily_max_executions: Option<i64>,
    /// Maximum concurrency. Optional / nullable.
    pub max_concurrency: Option<i64>,
    /// Notification recipients. Optional / nullable.
    pub notify_to: Option<Vec<String>>,
    /// Time saved. Optional / nullable.
    pub time_saved: Option<i64>,
    /// Time saved unit. Optional / nullable.
    pub time_saved_unit: Option<String>,
    /// Whether the workflow is a snippet. Optional / nullable.
    pub is_snippet: Option<bool>,
    /// Canvas dimensions. Optional / nullable.
    pub dimensions: Option<Dimensions>,
    /// Workflow description. Optional / nullable.
    pub description: Option<String>,
    /// Activation timestamp (date-time). Optional / nullable.
    pub activated_at: Option<String>,
    /// "Available until" timestamp. Optional / nullable.
    pub available_until: Option<String>,
    /// Number of versions. Optional / nullable.
    pub version_count: Option<i64>,
    /// Workflow state (enum string). Optional / nullable.
    pub state: Option<String>,
    /// Lifecycle state. Enum: `active`, `archived`, `deleted`. Optional / nullable.
    pub lifecycle_state: Option<String>,
    /// Status. Enum: `idle`, `running`. Optional / nullable.
    pub status: Option<String>,
}

/// Lightweight action metadata attached to a workflow row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionReadMeta {
    /// Action id. **Required.**
    pub id: String,
    /// Action type. **Required.**
    #[serde(rename = "type")]
    pub type_: String,
    /// Integration id. Optional / nullable.
    pub integration_id: Option<String>,
}

/// A workflow row (workflow + its actions) returned by the workflows list.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowWithActionsMeta {
    /// Workflow id (uuid). **Required.**
    pub id: String,
    /// The workflow record. **Required.**
    pub workflow: InfratableWorkflowReadScopeMeta,
    /// The workflow's actions. **Required.**
    pub actions: Vec<ActionReadMeta>,
}

/// Exported workflow action wrapper.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowActionImportExportWrapper {
    /// The action payload (freeform object). **Required.**
    pub action: serde_json::Value,
    /// Export id. **Required.**
    pub export_id: String,
    /// Ids this action is connected to. Optional / nullable.
    pub connected_to: Option<serde_json::Value>,
    /// Parent action. Optional / nullable.
    pub parent_action: Option<serde_json::Value>,
}

/// An exported workflow (a single version).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowImportExport {
    /// Workflow name. **Required.**
    pub name: String,
    /// Workflow description. Optional / nullable.
    pub description: Option<String>,
    /// Exported actions. Optional / nullable.
    pub actions: Option<Vec<WorkflowActionImportExportWrapper>>,
}

/// An exported custom action.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomActionExport {
    /// Action type. **Required.**
    #[serde(rename = "type")]
    pub type_: String,
    /// Client data (freeform object). Optional / nullable.
    pub client_data: Option<serde_json::Value>,
    /// Action data (freeform object). Optional / nullable.
    pub data: Option<serde_json::Value>,
    /// Action description. Optional / nullable.
    pub description: Option<String>,
    /// Snippet version id. Optional / nullable.
    pub snippet_version_id: Option<String>,
    /// Snippet workflow id. Optional / nullable.
    pub snippet_workflow_id: Option<String>,
    /// Action status. Optional / nullable.
    pub status: Option<String>,
    /// Tag. Optional / nullable.
    pub tag: Option<String>,
}

/// An exported custom integration with its actions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomIntegrationImportExport {
    /// Vendor name. **Required.**
    pub vendor_name: String,
    /// Product name. **Required.**
    pub product_name: String,
    /// Description. Optional / nullable.
    pub description: Option<String>,
    /// Logo. Optional / nullable.
    pub logo: Option<String>,
    /// Vendor logo. Optional / nullable.
    pub vendor_logo: Option<String>,
    /// Default connection data (freeform object). Optional / nullable.
    pub default_connection_data: Option<serde_json::Value>,
    /// Auth form metadata (freeform object). Optional / nullable.
    pub auth_form_metadata: Option<serde_json::Value>,
    /// Integration state. Enum: `active`, `disabled`. Optional / nullable.
    pub state: Option<String>,
    /// Vendor key. Optional / nullable.
    pub vendor_key: Option<String>,
    /// Product key. Optional / nullable.
    pub product_key: Option<String>,
    /// Exported actions. Optional / nullable.
    pub actions: Option<Vec<CustomActionExport>>,
}

/// A workflow read record with scope metadata, returned by import.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowReadScopeMeta {
    /// Scope id. **Required.**
    pub scope_id: String,
    /// Scope level. Enum: `tenant`, `account`, `site`. **Required.**
    pub scope_level: String,
    /// Management id. **Required.**
    pub mgmt_id: String,
    /// Workflow id (uuid). **Required.**
    pub id: String,
    /// Version id (uuid). **Required.**
    pub version_id: String,
    /// Workflow name. **Required.**
    pub name: String,
    /// Creator user id. **Required.**
    pub created_by: String,
    /// Updater user id. **Required.**
    pub updated_by: String,
    /// User who created the workflow. Optional / nullable.
    pub created_by_user: Option<WorkflowAttributionUser>,
    /// User who last updated the workflow. Optional / nullable.
    pub updated_by_user: Option<WorkflowAttributionUser>,
    /// Tags. Optional / nullable.
    pub tags: Option<Vec<String>>,
    /// Site name. Optional / nullable.
    pub site_name: Option<String>,
    /// Account name. Optional / nullable.
    pub account_name: Option<String>,
    /// Parent scope id. Optional / nullable.
    pub parent_scope_id: Option<String>,
    /// Site state. Enum: `active`, `deleted`, `expired`. Optional / nullable.
    pub site_state: Option<String>,
    /// Account state. Enum: `active`, `deleted`, `expired`. Optional / nullable.
    pub account_state: Option<String>,
    /// Creation timestamp (date-time). Optional / nullable.
    pub created_at: Option<String>,
    /// Update timestamp (date-time). Optional / nullable.
    pub updated_at: Option<String>,
    /// Version description. Optional / nullable.
    pub version_description: Option<String>,
    /// Execution timeout (seconds). Optional / nullable.
    pub timeout: Option<i64>,
    /// Daily maximum executions. Optional / nullable.
    pub daily_max_executions: Option<i64>,
    /// Maximum concurrency. Optional / nullable.
    pub max_concurrency: Option<i64>,
    /// Notification recipients. Optional / nullable.
    pub notify_to: Option<Vec<String>>,
    /// Time saved. Optional / nullable.
    pub time_saved: Option<i64>,
    /// Time saved unit. Optional / nullable.
    pub time_saved_unit: Option<String>,
    /// Whether the workflow is a snippet. Optional / nullable.
    pub is_snippet: Option<bool>,
    /// Canvas dimensions. Optional / nullable.
    pub dimensions: Option<Dimensions>,
    /// Workflow description. Optional / nullable.
    pub description: Option<String>,
    /// Activation timestamp (date-time). Optional / nullable.
    pub activated_at: Option<String>,
    /// Number of versions. Optional / nullable.
    pub version_count: Option<i64>,
    /// Workflow state (enum string). Optional / nullable.
    pub state: Option<String>,
    /// Lifecycle state. Enum: `active`, `archived`, `deleted`. Optional / nullable.
    pub lifecycle_state: Option<String>,
    /// Status. Enum: `idle`, `running`. Optional / nullable.
    pub status: Option<String>,
}

/// A custom integration read record with scope metadata, returned by import.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicIntegrationReadScopeMeta {
    /// Integration id (uuid). **Required.**
    pub id: String,
    /// Vendor name. **Required.**
    pub vendor_name: String,
    /// Product name. **Required.**
    pub product_name: String,
    /// Site name. Optional / nullable.
    pub site_name: Option<String>,
    /// Account name. Optional / nullable.
    pub account_name: Option<String>,
    /// Parent scope id. Optional / nullable.
    pub parent_scope_id: Option<String>,
    /// Site state. Enum: `active`, `deleted`, `expired`. Optional / nullable.
    pub site_state: Option<String>,
    /// Account state. Enum: `active`, `deleted`, `expired`. Optional / nullable.
    pub account_state: Option<String>,
    /// Creation timestamp (date-time). Optional / nullable.
    pub created_at: Option<String>,
    /// Update timestamp (date-time). Optional / nullable.
    pub updated_at: Option<String>,
    /// Scope id. Optional / nullable.
    pub scope_id: Option<String>,
    /// Scope level. Enum: `tenant`, `account`, `site`. Optional / nullable.
    pub scope_level: Option<String>,
    /// Management id. Optional / nullable.
    pub mgmt_id: Option<String>,
    /// Description. Optional / nullable.
    pub description: Option<String>,
    /// Logo. Optional / nullable.
    pub logo: Option<String>,
    /// Default connection data (freeform object). Optional / nullable.
    pub default_connection_data: Option<serde_json::Value>,
    /// Auth form metadata (freeform object). Optional / nullable.
    pub auth_form_metadata: Option<serde_json::Value>,
    /// Integration state. Enum: `active`, `disabled`. Optional / nullable.
    pub state: Option<String>,
    /// Vendor logo. Optional / nullable.
    pub vendor_logo: Option<String>,
    /// Vendor key. Optional / nullable.
    pub vendor_key: Option<String>,
    /// Product key. Optional / nullable.
    pub product_key: Option<String>,
    /// Origin id (uuid). Optional / nullable.
    pub origin_id: Option<String>,
}

/// Error returned by a test action / expression evaluation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestActionExecutionError {
    /// Error text. **Required.**
    pub text: String,
    /// Error values (freeform object). Optional / nullable.
    pub values: Option<serde_json::Value>,
}

/// The result of evaluating a single expression split.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionSplitEvaluationResult {
    /// The split expression segment. **Required** (nullable). Optional / nullable.
    pub split: Option<String>,
    /// The evaluated expression. **Required** (nullable). Optional / nullable.
    pub evaluated_expression: Option<String>,
    /// Per-split error (freeform). **Required** (nullable). Optional / nullable.
    pub error: Option<serde_json::Value>,
    /// The evaluated value (freeform). **Required** (nullable). Optional / nullable.
    pub value: Option<serde_json::Value>,
    /// Whether the result size overflowed. **Required** (nullable). Optional / nullable.
    pub result_size_overflow: Option<bool>,
}

/// The result of an expression evaluation / breakdown.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionEvaluationResult {
    /// Per-split results. **Required** (nullable). Optional / nullable.
    pub results: Option<Vec<ExpressionSplitEvaluationResult>>,
    /// Evaluation error. **Required** (nullable). Optional / nullable.
    pub error: Option<TestActionExecutionError>,
}
