//! Models for the `Cloud Funnel` tag (Cloud Funnel Control Plane).
//!
//! Source of truth: `swagger_2_1.json` definitions
//! `v2_1.cloud_funnel.schemas_*ResponseSchema_200`. Each response wraps the
//! entity under `.data`; these structs model that inner `data` object and are
//! returned via [`crate::pagination::Response`].
//!
//! Fidelity note: a field is a bare `T` only when it appears in the schema's
//! `required` array (and is not `x-nullable`); every other field is modelled
//! as `Option<T>` (the spec's "default null" behaviour). Enum-typed fields are
//! `String` for forward-compatibility, with allowed values documented inline.

use serde::Deserialize;

/// AWS assume role external ID.
///
/// Spec definition: `v2_1.cloud_funnel.schemas_GetAssumeRoleExternalIdResponseSchema_200.data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssumeRoleExternalId {
    /// The AWS assume role external id.
    ///
    /// Required + not `x-nullable` -> bare `String`.
    pub assume_role_external_id: String,
}

/// Estimator size-of-events result.
///
/// Spec definition: `v2_1.cloud_funnel.schemas_GetEstimatorResponseSchema_200.data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Estimator {
    /// Is estimator query status is completed.
    ///
    /// Required + not `x-nullable` -> bare `bool`.
    pub is_completed: bool,
    /// Estimation of uncompressed size. (string)
    ///
    /// Optional/nullable -> `Option`.
    pub uncompressed_bytes: Option<String>,
    /// Estimation of compressed size. (string)
    ///
    /// Optional/nullable -> `Option`.
    pub compressed_bytes: Option<String>,
    /// Estimation of events number. (string, to avoid rounding)
    ///
    /// Optional/nullable -> `Option`.
    pub matching_events: Option<String>,
    /// Error message in case the estimator query failed.
    ///
    /// Optional/nullable -> `Option`.
    pub error: Option<String>,
}

/// Newly created estimator ID.
///
/// Spec definition: `v2_1.cloud_funnel.schemas_InitEstimatorResponseSchema_200.data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitEstimator {
    /// Estimator query id.
    ///
    /// Required + not `x-nullable` -> bare `String`.
    pub estimator_id: String,
    /// Error message in case the estimator query is invalid.
    ///
    /// Optional/nullable -> `Option`.
    pub error: Option<String>,
}

/// Cloud funnel onboarding rule details.
///
/// Spec definition: `v2_1.cloud_funnel.schemas_OnboardingResponseSchema_200.data`.
/// Returned by both the `GET` and `POST` onboarding endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Onboarding {
    /// log-archive-rule id, default for accounts: `cloud-funnel`.
    ///
    /// Optional/nullable -> `Option`.
    pub id: Option<String>,
    /// Bucket url.
    ///
    /// Required + not `x-nullable` -> bare `String`.
    pub bucket_url: String,
    /// Syql query to validate.
    ///
    /// Optional/nullable -> `Option`.
    pub query: Option<String>,
    /// Disable events stream.
    ///
    /// Required + not `x-nullable` -> bare `bool`.
    pub disable_stream: bool,
    /// Is inheriting global setting.
    ///
    /// Optional/nullable -> `Option`.
    pub is_inheriting: Option<bool>,
    /// Is global onboarding exists in table.
    ///
    /// Optional/nullable -> `Option`.
    pub global_onboarding_exists: Option<bool>,
    /// Cloud provider, default is `aws`.
    ///
    /// Enum-like (forward-compat `String`). Optional/nullable -> `Option`.
    pub cloud_provider: Option<String>,
    /// Error message in case the bucket permissions is invalid.
    ///
    /// Optional/nullable -> `Option`.
    pub error: Option<String>,
    /// The AWS assume role external id.
    ///
    /// Optional/nullable -> `Option`.
    pub assume_role_external_id: Option<String>,
    /// The AWS role to assume when using assume role functionality. Only
    /// applicable if `cloud_provider` is `s3`.
    ///
    /// Optional/nullable -> `Option`.
    pub role_to_assume: Option<String>,
    /// If set to true, activates the AWS AssumeRole functionality for accessing
    /// S3 buckets or other associated resources. Only applicable if
    /// `cloud_provider` is `s3`.
    ///
    /// Optional/nullable -> `Option`.
    pub use_assume_role: Option<bool>,
    /// List of desired fields to be included in the output. If not specified,
    /// all fields are included.
    ///
    /// Optional/nullable -> `Option`.
    pub desired_fields: Option<Vec<String>>,
    /// For site scope, is account onboarding exists.
    ///
    /// Optional/nullable -> `Option`.
    pub account_onboarding_exists: Option<bool>,
    /// Access key id, for S3 compatible storages (e.g. R2).
    ///
    /// Optional/nullable -> `Option`.
    pub access_key_id: Option<String>,
    /// Secret access key, for S3 compatible storages (e.g. R2).
    ///
    /// Optional/nullable -> `Option`.
    pub secret_access_key: Option<String>,
}

/// Result of deleting a cloud funnel onboarding rule.
///
/// Spec definition: `v2_1.cloud_funnel.schemas_OnboardingDeleteResponseSchema_200.data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingDelete {
    /// Error message in case the bucket permissions is invalid.
    ///
    /// Required + not `x-nullable` -> bare `String`.
    pub error: String,
}

/// Result of validating bucket permissions.
///
/// Spec definition: `v2_1.cloud_funnel.schemas_BucketValidationResponseSchema_200.data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketValidation {
    /// Bucket permissions is valid or invalid.
    ///
    /// Required + not `x-nullable` -> bare `bool`.
    pub is_valid: bool,
    /// Error message in case the bucket permissions is invalid.
    ///
    /// Optional/nullable -> `Option`.
    pub error: Option<String>,
}

/// Result of validating a Cloud Funnel onboarding query.
///
/// Spec definition: `v2_1.cloud_funnel.schemas_QueryValidationResponseSchema_200.data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryValidation {
    /// Query is valid or invalid.
    ///
    /// Required + not `x-nullable` -> bare `bool`.
    pub is_valid: bool,
    /// Error message in case the query is invalid.
    ///
    /// Required + not `x-nullable` -> bare `String`.
    pub error: String,
}
