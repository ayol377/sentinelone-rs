use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::cloud_funnel::{
    AssumeRoleExternalId, BucketValidation, Estimator, InitEstimator, Onboarding, OnboardingDelete,
    QueryValidation,
};
use crate::pagination::Response;

/// `Cloud Funnel` tag — Cloud Funnel Control Plane.
///
/// Manage Cloud Funnel onboarding rules (the configuration that streams events
/// to an external bucket), validate bucket permissions and SyQL queries, fetch
/// the AWS assume-role external ID, and estimate the size of events in a bucket
/// via the estimator workflow (`POST` to create an estimator ID, then `GET` to
/// read the estimate).
pub struct CloudFunnelService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query param structs
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/cloud-funnel/assume-role-external-id`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssumeRoleExternalIdQuery {
    /// Account id. Example: `"225494730938493804"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Site id. Example: `"225494730938493804"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}

impl GetAssumeRoleExternalIdQuery {
    /// Account id. Example: `"225494730938493804"`.
    pub fn account_id(mut self, v: impl Into<String>) -> Self {
        self.account_id = Some(v.into());
        self
    }
    /// Site id. Example: `"225494730938493804"`.
    pub fn site_id(mut self, v: impl Into<String>) -> Self {
        self.site_id = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/cloud-funnel/estimator`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEstimatorQuery {
    /// Estimator query id.
    ///
    /// Required=yes. Kept as `Option` for builder ergonomics, but must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimator_id: Option<String>,
}

impl GetEstimatorQuery {
    /// Estimator query id. Required for this endpoint.
    pub fn estimator_id(mut self, v: impl Into<String>) -> Self {
        self.estimator_id = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/cloud-funnel/onboarding`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOnboardingQuery {
    /// Account id. Example: `"225494730938493804"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Site id. Example: `"225494730938493804"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}

impl GetOnboardingQuery {
    /// Account id. Example: `"225494730938493804"`.
    pub fn account_id(mut self, v: impl Into<String>) -> Self {
        self.account_id = Some(v.into());
        self
    }
    /// Site id. Example: `"225494730938493804"`.
    pub fn site_id(mut self, v: impl Into<String>) -> Self {
        self.site_id = Some(v.into());
        self
    }
}

// ---------------------------------------------------------------------------
// Body structs
//
// Every Cloud Funnel body is wrapped in a top-level `data` object (the schema's
// only top-level `required` field). Each `*Body` therefore carries a single
// `data: *Data` field, and the inner `*Data` struct's field nullability follows
// that nested object's own `required` array.
// ---------------------------------------------------------------------------

/// Request body for `POST /web/api/v2.1/cloud-funnel/estimator` (Create
/// Estimator ID).
///
/// Spec definition: `v2_1.cloud_funnel.schemas_InitEstimatorSchema`.
#[derive(Debug, Default, Serialize)]
pub struct InitEstimatorBody {
    /// Data. Required top-level wrapper.
    pub data: InitEstimatorData,
}

/// Inner `data` object for [`InitEstimatorBody`].
///
/// The inner object declares no `required` fields, so every field is optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitEstimatorData {
    /// Account ids (max 5000). Example: `["225494730938493804"]`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Query.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// List of desired fields to be included in the output. If not specified,
    /// all fields are included.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_fields: Option<Vec<String>>,
    /// Site ids.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
}

impl InitEstimatorBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: InitEstimatorData) -> Self {
        Self { data }
    }
}

impl InitEstimatorData {
    /// Account ids (max 5000).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Query.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// List of desired fields to be included in the output.
    pub fn desired_fields<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.desired_fields = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Site ids.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
}

/// Request body for `DELETE /web/api/v2.1/cloud-funnel/onboarding` (Delete
/// cloud funnel rule).
///
/// Spec definition: `v2_1.cloud_funnel.schemas_OnboardingDeleteSchema`.
#[derive(Debug, Default, Serialize)]
pub struct OnboardingDeleteBody {
    /// Data. Required top-level wrapper.
    pub data: OnboardingDeleteData,
}

/// Inner `data` object for [`OnboardingDeleteBody`].
///
/// The inner object declares no `required` fields, so every field is optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingDeleteData {
    /// Account id. Example: `"225494730938493804"`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Site ids.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
}

impl OnboardingDeleteBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: OnboardingDeleteData) -> Self {
        Self { data }
    }
}

impl OnboardingDeleteData {
    /// Account id. Example: `"225494730938493804"`.
    pub fn account_id(mut self, v: impl Into<String>) -> Self {
        self.account_id = Some(v.into());
        self
    }
    /// Site ids.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
}

/// Request body for `POST /web/api/v2.1/cloud-funnel/onboarding` (Post
/// onboarding cloud funnel).
///
/// Spec definition: `v2_1.cloud_funnel.schemas_OnboardingPostSchema`.
#[derive(Debug, Default, Serialize)]
pub struct OnboardingPostBody {
    /// Data. Required top-level wrapper.
    pub data: OnboardingPostData,
}

/// Inner `data` object for [`OnboardingPostBody`].
///
/// The inner object declares no `required` fields, so every field is optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingPostData {
    /// Account id. Example: `"225494730938493804"`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Bucket url (min length 1).
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_url: Option<String>,
    /// Disable events stream.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stream: Option<bool>,
    /// Is inheriting global setting.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inheriting: Option<bool>,
    /// Syql query to validate.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Cloud provider, default is `aws`.
    ///
    /// Enum-like (forward-compat `String`). Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// If set to true, activates the AWS AssumeRole functionality for accessing
    /// S3 buckets or other associated resources. Only applicable if
    /// `cloud_provider` is `s3`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_assume_role: Option<bool>,
    /// The aws role to assume when using assume role functionality. Only
    /// applicable if `cloud_provider` is `s3`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_to_assume: Option<String>,
    /// List of desired fields to be included in the output. If not specified,
    /// all fields are included.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_fields: Option<Vec<String>>,
    /// Site ids.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// Access key id, for S3 compatible storages (e.g. R2).
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// Secret access key, for S3 compatible storages (e.g. R2).
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
}

impl OnboardingPostBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: OnboardingPostData) -> Self {
        Self { data }
    }
}

impl OnboardingPostData {
    /// Account id. Example: `"225494730938493804"`.
    pub fn account_id(mut self, v: impl Into<String>) -> Self {
        self.account_id = Some(v.into());
        self
    }
    /// Bucket url (min length 1).
    pub fn bucket_url(mut self, v: impl Into<String>) -> Self {
        self.bucket_url = Some(v.into());
        self
    }
    /// Disable events stream.
    pub fn disable_stream(mut self, v: bool) -> Self {
        self.disable_stream = Some(v);
        self
    }
    /// Is inheriting global setting.
    pub fn is_inheriting(mut self, v: bool) -> Self {
        self.is_inheriting = Some(v);
        self
    }
    /// Syql query to validate.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Cloud provider, default is `aws`.
    pub fn cloud_provider(mut self, v: impl Into<String>) -> Self {
        self.cloud_provider = Some(v.into());
        self
    }
    /// Activate the AWS AssumeRole functionality. Only applicable if
    /// `cloud_provider` is `s3`.
    pub fn use_assume_role(mut self, v: bool) -> Self {
        self.use_assume_role = Some(v);
        self
    }
    /// The aws role to assume when using assume role functionality.
    pub fn role_to_assume(mut self, v: impl Into<String>) -> Self {
        self.role_to_assume = Some(v.into());
        self
    }
    /// List of desired fields to be included in the output.
    pub fn desired_fields<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.desired_fields = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Site ids.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Access key id, for S3 compatible storages (e.g. R2).
    pub fn access_key_id(mut self, v: impl Into<String>) -> Self {
        self.access_key_id = Some(v.into());
        self
    }
    /// Secret access key, for S3 compatible storages (e.g. R2).
    pub fn secret_access_key(mut self, v: impl Into<String>) -> Self {
        self.secret_access_key = Some(v.into());
        self
    }
}

/// Request body for `POST /web/api/v2.1/cloud-funnel/validate-bucket-permissions`
/// (Validate Bucket).
///
/// Spec definition: `v2_1.cloud_funnel.schemas_BucketValidationSchema`.
#[derive(Debug, Default, Serialize)]
pub struct BucketValidationBody {
    /// Data. Required top-level wrapper.
    pub data: BucketValidationData,
}

/// Inner `data` object for [`BucketValidationBody`].
///
/// `bucketUrl` is the only `required` field of this inner object; the rest are
/// optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketValidationData {
    /// Bucket url to validate permissions for (min length 1).
    ///
    /// Required (inner `data.required`) -> bare `String`.
    pub bucket_url: String,
    /// Account id. Example: `"225494730938493804"`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Cloud provider, default is `aws`.
    ///
    /// Enum-like (forward-compat `String`). Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// If set to true, activates the AWS AssumeRole functionality for accessing
    /// S3 buckets or other associated resources. Only applicable if
    /// `cloud_provider` is `s3`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_assume_role: Option<bool>,
    /// The aws role to assume when using assume role functionality. Only
    /// applicable if `cloud_provider` is `s3`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_to_assume: Option<String>,
    /// Site id. Example: `"225494730938493804"`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// Access key id, for S3 compatible storages (e.g. R2).
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// Secret access key, for S3 compatible storages (e.g. R2).
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
}

impl BucketValidationBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: BucketValidationData) -> Self {
        Self { data }
    }
}

impl BucketValidationData {
    /// New inner `data` with the required `bucketUrl`.
    pub fn new(bucket_url: impl Into<String>) -> Self {
        Self {
            bucket_url: bucket_url.into(),
            ..Default::default()
        }
    }
    /// Account id. Example: `"225494730938493804"`.
    pub fn account_id(mut self, v: impl Into<String>) -> Self {
        self.account_id = Some(v.into());
        self
    }
    /// Cloud provider, default is `aws`.
    pub fn cloud_provider(mut self, v: impl Into<String>) -> Self {
        self.cloud_provider = Some(v.into());
        self
    }
    /// Activate the AWS AssumeRole functionality. Only applicable if
    /// `cloud_provider` is `s3`.
    pub fn use_assume_role(mut self, v: bool) -> Self {
        self.use_assume_role = Some(v);
        self
    }
    /// The aws role to assume when using assume role functionality.
    pub fn role_to_assume(mut self, v: impl Into<String>) -> Self {
        self.role_to_assume = Some(v.into());
        self
    }
    /// Site id. Example: `"225494730938493804"`.
    pub fn site_id(mut self, v: impl Into<String>) -> Self {
        self.site_id = Some(v.into());
        self
    }
    /// Access key id, for S3 compatible storages (e.g. R2).
    pub fn access_key_id(mut self, v: impl Into<String>) -> Self {
        self.access_key_id = Some(v.into());
        self
    }
    /// Secret access key, for S3 compatible storages (e.g. R2).
    pub fn secret_access_key(mut self, v: impl Into<String>) -> Self {
        self.secret_access_key = Some(v.into());
        self
    }
}

/// Request body for `POST /web/api/v2.1/cloud-funnel/validate-query` (Validate
/// Query).
///
/// Spec definition: `v2_1.cloud_funnel.schemas_QueryValidationSchema`.
#[derive(Debug, Default, Serialize)]
pub struct QueryValidationBody {
    /// Data. Required top-level wrapper.
    pub data: QueryValidationData,
}

/// Inner `data` object for [`QueryValidationBody`].
///
/// `query` is the only field and it is `required`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryValidationData {
    /// Query to validate (min length 1).
    ///
    /// Required (inner `data.required`) -> bare `String`.
    pub query: String,
}

impl QueryValidationBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: QueryValidationData) -> Self {
        Self { data }
    }
}

impl QueryValidationData {
    /// New inner `data` with the required `query`.
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl CloudFunnelService<'_> {
    /// `GET /web/api/v2.1/cloud-funnel/assume-role-external-id` — Get AWS
    /// assume role external ID.
    ///
    /// Get the AWS assume role external ID.
    pub async fn get_assume_role_external_id(
        &self,
        query: &GetAssumeRoleExternalIdQuery,
    ) -> Result<Response<AssumeRoleExternalId>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/cloud-funnel/assume-role-external-id", q)
            .await?)
    }

    /// `GET /web/api/v2.1/cloud-funnel/estimator` — Get estimate size of
    /// events.
    ///
    /// Get estimate size of events in the bucket. You need the estimator ID
    /// which can be generated by running the API: "Create Estimator ID".
    pub async fn get_estimator(
        &self,
        query: &GetEstimatorQuery,
    ) -> Result<Response<Estimator>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/cloud-funnel/estimator", q)
            .await?)
    }

    /// `POST /web/api/v2.1/cloud-funnel/estimator` — Create Estimator ID.
    ///
    /// Create estimator ID. This is needed to run the API "Get Estimate Size
    /// Of Events".
    pub async fn create_estimator(
        &self,
        body: &InitEstimatorBody,
    ) -> Result<Response<InitEstimator>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/cloud-funnel/estimator", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/cloud-funnel/onboarding` — Delete cloud funnel
    /// rule.
    ///
    /// Deletes cloud funnel onboarding rule.
    pub async fn delete_onboarding(
        &self,
        body: &OnboardingDeleteBody,
    ) -> Result<Response<OnboardingDelete>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<OnboardingDeleteBody, Response<OnboardingDelete>>(
                Method::DELETE,
                "/web/api/v2.1/cloud-funnel/onboarding",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/cloud-funnel/onboarding` — Get cloud funnel rule.
    ///
    /// Gets cloud funnel onboarding rule details.
    pub async fn get_onboarding(
        &self,
        query: &GetOnboardingQuery,
    ) -> Result<Response<Onboarding>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/cloud-funnel/onboarding", q)
            .await?)
    }

    /// `POST /web/api/v2.1/cloud-funnel/onboarding` — Post onboarding cloud
    /// funnel.
    ///
    /// Post onboarding cloud funnel rule.
    pub async fn post_onboarding(
        &self,
        body: &OnboardingPostBody,
    ) -> Result<Response<Onboarding>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/cloud-funnel/onboarding", body)
            .await?)
    }

    /// `POST /web/api/v2.1/cloud-funnel/validate-bucket-permissions` — Validate
    /// Bucket.
    ///
    /// Validates bucket permissions.
    pub async fn validate_bucket_permissions(
        &self,
        body: &BucketValidationBody,
    ) -> Result<Response<BucketValidation>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/cloud-funnel/validate-bucket-permissions", body)
            .await?)
    }

    /// `POST /web/api/v2.1/cloud-funnel/validate-query` — Validate Query.
    ///
    /// Verifies that a query is valid before using it as filter for a Cloud
    /// Funnel onboarding.
    pub async fn validate_query(
        &self,
        body: &QueryValidationBody,
    ) -> Result<Response<QueryValidation>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/cloud-funnel/validate-query", body)
            .await?)
    }
}
