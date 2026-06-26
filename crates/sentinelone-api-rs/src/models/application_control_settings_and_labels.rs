//! Models for the `Application Control - Settings and Labels` tag.
//!
//! Native Application Control (NAC) settings management and labels entities.
//!
//! NOTE: Unlike most Management API endpoints, the NAC config endpoints return
//! their payloads *directly* (no `{ data: ... }` envelope), so these models are
//! deserialized straight from the HTTP response body rather than via
//! [`crate::pagination::Response`] / [`crate::pagination::Paginated`].

use serde::Deserialize;

/// NAC settings (`#/definitions/NACSettings`).
///
/// All fields are optional/nullable: none are in a schema `required` array.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacSettings {
    /// Default behavior.
    ///
    /// Allowed values: `ALLOW`, `MONITOR`, `BLOCK`. Optional/nullable.
    pub fallback_behavior: Option<String>,
    /// Whether NAC is enabled. Optional/nullable.
    pub enable_application_control: Option<bool>,
    /// Whether settings are inherited. Optional/nullable.
    pub inherit_application_control: Option<bool>,
}

/// Rule label (`#/definitions/NACLabel`).
///
/// All fields are optional/nullable: none are in a schema `required` array.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacLabel {
    /// Label ID. Optional/nullable.
    pub id: Option<String>,
    /// Label name (e.g. `productivity`). Optional/nullable.
    pub label_name: Option<String>,
}

/// Common response (`#/definitions/NACCommonResponse`).
///
/// All fields are optional/nullable: none are in a schema `required` array.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NacCommonResponse {
    /// Whether the request was successful. Optional/nullable.
    pub success: Option<bool>,
    /// Resource ID. Optional/nullable.
    pub id: Option<String>,
    /// Status code (e.g. `200`). Optional/nullable.
    pub status_code: Option<i64>,
    /// Status message. Optional/nullable.
    pub status_message: Option<String>,
    /// Validation errors. Optional/nullable.
    pub validation_errors: Option<Vec<GqlValidationError>>,
}

/// Validation error returned for an invalid request
/// (`#/definitions/GQLValidationError`).
///
/// All fields are optional/nullable: none are in a schema `required` array.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GqlValidationError {
    /// Error code for the validation issue (e.g. `RULE_NAME_REQUIRED`).
    /// Optional/nullable.
    pub code: Option<String>,
    /// Validation error message. Optional/nullable.
    pub message: Option<String>,
    /// The raw input value that caused the validation error (e.g. `signer`).
    /// Optional/nullable.
    pub value: Option<String>,
}
