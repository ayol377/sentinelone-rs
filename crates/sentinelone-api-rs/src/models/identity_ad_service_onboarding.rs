//! Models for the `Identity AD Service - Onboarding` tag.
//!
//! APIs for managing AD service onboarding status.
//!
//! Field nullability follows the swagger `2.1` spec: a field is a bare `T`
//! only when it is listed in the schema `required` array *and* not
//! `x-nullable`; otherwise it is `Option<T>` (default-null behaviour). Enum
//! fields are kept as `String` for forward-compatibility, with the allowed
//! values documented in the field doc comment.

use serde::Deserialize;

/// Current onboarding status for the AD service (`OnboardingStatusResponse`).
///
/// Returned (as the `data` payload) by
/// `GET /web/api/v2.1/identity/adservice/api/getOnboardingStatus`.
///
/// The schema declares no `required` fields, so every field is `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingStatus {
    /// Overall onboarding status.
    ///
    /// Allowed values: `COMPLETE`, `INCOMPLETE`. Kept as `String` for
    /// forward-compatibility. Optional/nullable.
    pub status: Option<String>,

    /// List of selected features for the AD service. Optional/nullable.
    pub feature_selected: Option<Vec<String>>,

    /// State of the AD connector.
    ///
    /// Allowed values: `CONFIGURED`, `CONFIG_PENDING`. Kept as `String` for
    /// forward-compatibility. Optional/nullable.
    pub ad_connector: Option<String>,

    /// Domain name associated with the onboarding. Optional/nullable.
    pub domain_name: Option<String>,
}
