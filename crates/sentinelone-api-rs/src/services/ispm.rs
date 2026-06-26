use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::ispm::*;
use crate::pagination::{Paginated, Response};

/// `ISPM` tag — Identity Security Posture Management views and operations.
///
/// Wraps the SentinelOne `ranger-ad` Cloud APIs for Active Directory / Azure AD
/// security posture assessment: querying assessment status, exposures and
/// affected objects, and triggering assessments / acknowledging / skipping
/// exposures.
pub struct IspmService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ===========================================================================
// Query structs
// ===========================================================================

/// Query params for `GET /web/api/v2.1/ranger-ad/assessment-status`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssessmentStatusQuery {
    /// List of site IDs separated by comma. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of account IDs separated by comma. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl AssessmentStatusQuery {
    /// List of site IDs (joined by comma). Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of account IDs (joined by comma). Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `POST /web/api/v2.1/ranger-ad/get-exposures`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExposuresQuery {
    /// Limit number of returned items (1-1000). Example: `10`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `150`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// List of account IDs separated by comma. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl GetExposuresQuery {
    /// Limit number of returned items (1-1000). Optional.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Skip first number of items (0-1000). Optional.
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// List of account IDs (joined by comma). Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (joined by comma). Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `POST /web/api/v2.1/ranger-ad/get-affected-objects`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAffectedObjectsQuery {
    /// Limit number of returned items (1-1000). Example: `10`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `150`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// List of account IDs separated by comma. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl GetAffectedObjectsQuery {
    /// Limit number of returned items (1-1000). Optional.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Skip first number of items (0-1000). Optional.
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// List of account IDs (joined by comma). Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (joined by comma). Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params shared by the `set-ack-status`, `set-skipped-exposures` and
/// `trigger-assessment` ISPM endpoints (`siteIds` / `accountIds`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeQuery {
    /// List of site IDs separated by comma. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of account IDs separated by comma. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl ScopeQuery {
    /// List of site IDs (joined by comma). Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of account IDs (joined by comma). Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
}

fn join_csv<I, S>(ids: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    ids.into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

// ===========================================================================
// Body structs
// ===========================================================================

/// Request body for `POST /web/api/v2.1/ranger-ad/get-exposures`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExposuresBody {
    /// Filtering criteria for exposures. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<GetExposuresFilter>,
}

impl GetExposuresBody {
    /// Filtering criteria for exposures. Optional.
    pub fn filter(mut self, filter: GetExposuresFilter) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Filtering criteria for `get-exposures`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExposuresFilter {
    /// Filter by exposure detection status. Optional.
    ///
    /// Allowed values: `Vulnerable`, `Not_Vulnerable`, `Skipped`,
    /// `In_Progress`, `Pending`, `Mitigated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_status: Option<Vec<String>>,
    /// Filter by specific detection names. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_name: Option<Vec<String>>,
    /// Filter by Active Directory domain names. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<Vec<String>>,
    /// Filter by Active Directory forest names. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forest_name: Option<Vec<String>>,
    /// Filter by exposure severity level. Optional.
    ///
    /// Allowed values: `Critical`, `High`, `Medium`, `Low`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Vec<String>>,
    /// Filter by detection source. Optional.
    ///
    /// Allowed values: `OnPremAD`, `AzureAD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<String>>,
}

/// Request body for `POST /web/api/v2.1/ranger-ad/get-affected-objects`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAffectedObjectsBody {
    /// Filtering criteria for affected objects. Required.
    pub filter: GetAffectedObjectsFilter,
}

/// Filtering criteria for `get-affected-objects`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAffectedObjectsFilter {
    /// Filter by specific detection names. Required (min 1 item).
    pub detection_name: Vec<String>,
    /// Filter by Active Directory domain names. Required (min 1 item).
    pub domain_name: Vec<String>,
    /// Filter by Active Directory forest names. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forest_name: Option<Vec<String>>,
    /// Filter by Active Directory object type. Optional.
    ///
    /// Allowed values: `Computer`, `User`, `Group`, `Container`, `GMSA`, `OU`,
    /// `GPO`, `Domain`, `Domain Controller`, `DNS Zone`,
    /// `ForeignSecurityPrincipals`, `CertificateV1`, `CertificateV2`,
    /// `CertTemplateV1`, `CertTemplateV2`, `CertTemplateV3`, `CertAuthority`,
    /// `Tenant`, `Unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<Vec<String>>,
    /// Filter by detection module source. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_src: Option<Vec<String>>,
    /// Filter by Distinguished Name (DN) of objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dn: Option<Vec<String>>,
    /// Filter by assessment job type. Optional.
    ///
    /// Allowed values: `user`, `scheduled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_job_type: Option<Vec<String>>,
}

/// Request body for `POST /web/api/v2.1/ranger-ad/set-ack-status`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAckStatusBody {
    /// Filtering criteria for exposures to acknowledge. Required.
    pub filter: SetExposuresFilter,
}

/// Request body for `POST /web/api/v2.1/ranger-ad/set-skipped-exposures`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSkippedExposuresBody {
    /// Filtering criteria for exposures to mark as skipped. Required.
    pub filter: SetExposuresFilter,
}

/// Filtering criteria shared by `set-ack-status` and `set-skipped-exposures`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExposuresFilter {
    /// List of forest names to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forest_name: Option<Vec<String>>,
    /// List of detection names. Required (min 1 item).
    pub detection_name: Vec<String>,
    /// List of domain names. Required (min 1 item).
    pub domain_name: Vec<String>,
}

/// Request body for `POST /web/api/v2.1/ranger-ad/trigger-assessment`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerAssessmentBody {
    /// Assessment trigger configuration. Required.
    pub filter: TriggerAssessmentFilter,
}

/// Assessment trigger configuration for `trigger-assessment`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerAssessmentFilter {
    /// Whether to perform a full scan (`true`) or reassess specific exposures
    /// (`false`). Required.
    pub is_full_scan: bool,
    /// List of domain names to scan (used when `isFullScan=true`).
    /// Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<Vec<String>>,
    /// Source type for full scan (OnPrem-AD or Azure-AD). Optional/nullable.
    ///
    /// Allowed values: `AD`, `Azure`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_source: Option<String>,
    /// List of specific exposures to reassess (used when `isFullScan=false`).
    /// Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposure_list: Option<Vec<TriggerAssessmentExposure>>,
}

/// A specific exposure to reassess via `trigger-assessment`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerAssessmentExposure {
    /// Domain name where the exposure exists. Required.
    pub domain_name: String,
    /// Name of the detection to reassess. Required.
    pub detection_name: String,
}

// ===========================================================================
// Service methods
// ===========================================================================

impl IspmService<'_> {
    /// `GET /web/api/v2.1/ranger-ad/assessment-status` — Get Assessment Status.
    ///
    /// Use the below Cloud API to get the status of the AD Assessment status
    /// for that account.
    pub async fn assessment_status(
        &self,
        query: &AssessmentStatusQuery,
    ) -> Result<Response<AssessmentStatus>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/ranger-ad/assessment-status", q)
            .await?)
    }

    /// `POST /web/api/v2.1/ranger-ad/get-exposures` — Get Exposures.
    ///
    /// Use the below Cloud API to get all the exposures based on the selected
    /// filters.
    pub async fn get_exposures(
        &self,
        query: &GetExposuresQuery,
        body: &GetExposuresBody,
    ) -> Result<Paginated<Exposure>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<GetExposuresBody, Paginated<Exposure>>(
                Method::POST,
                "/web/api/v2.1/ranger-ad/get-exposures",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/ranger-ad/get-affected-objects` — Get Affected
    /// Objects.
    ///
    /// Use the below Cloud API to get all the affected objects based on the
    /// selected filters.
    pub async fn get_affected_objects(
        &self,
        query: &GetAffectedObjectsQuery,
        body: &GetAffectedObjectsBody,
    ) -> Result<Paginated<AffectedObject>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<GetAffectedObjectsBody, Paginated<AffectedObject>>(
                Method::POST,
                "/web/api/v2.1/ranger-ad/get-affected-objects",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/ranger-ad/set-ack-status` — Set Acknowledged Status.
    ///
    /// Use the below Cloud API to set acknowledgement status.
    pub async fn set_ack_status(
        &self,
        query: &ScopeQuery,
        body: &SetAckStatusBody,
    ) -> Result<Response<SuccessMessage>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<SetAckStatusBody, Response<SuccessMessage>>(
                Method::POST,
                "/web/api/v2.1/ranger-ad/set-ack-status",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/ranger-ad/set-skipped-exposures` — Set Skipped
    /// Exposures.
    ///
    /// Use the below Cloud API to set the list of exposures to be skipped.
    pub async fn set_skipped_exposures(
        &self,
        query: &ScopeQuery,
        body: &SetSkippedExposuresBody,
    ) -> Result<Response<SuccessMessage>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<SetSkippedExposuresBody, Response<SuccessMessage>>(
                Method::POST,
                "/web/api/v2.1/ranger-ad/set-skipped-exposures",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/ranger-ad/trigger-assessment` — Trigger Assessment.
    ///
    /// Use the below Cloud API to trigger ADAssessment.
    pub async fn trigger_assessment(
        &self,
        query: &ScopeQuery,
        body: &TriggerAssessmentBody,
    ) -> Result<Response<SuccessMessage>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<TriggerAssessmentBody, Response<SuccessMessage>>(
                Method::POST,
                "/web/api/v2.1/ranger-ad/trigger-assessment",
                q,
                Some(body),
            )
            .await?)
    }
}
