//! `Mobile Integration` tag — devices, incidents and MSSP / tenant provisioning.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::mobile_integration::{
    CanProvisionTenant, Device, Incident, MsspWithUsers, PartnerKey, TenantWithUsers,
};
use crate::pagination::{Paginated, Response};

/// `Mobile Integration` tag.
///
/// Endpoints for listing managed mobile devices and their incidents, plus
/// MSSP partner / tenant provisioning (partner keys, provision flows).
pub struct MobileIntegrationService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ===========================================================================
// Query params
// ===========================================================================

/// Query params for `GET /web/api/v2.1/mobile-integration/devices`.
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicesQuery {
    /// Include devices only with given privileges. (optional)
    ///
    /// Allowed values: `none`, `rooted`, `jailbroken`. Comma-joined array.
    #[serde(rename = "privileges__in", skip_serializing_if = "Option::is_none")]
    pub privileges_in: Option<String>,
    /// List of Site IDs to filter by. (optional, comma-joined array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Include devices by models that contain text. (optional)
    #[serde(rename = "model__contains", skip_serializing_if = "Option::is_none")]
    pub model_contains: Option<String>,
    /// Include devices by external tracking IDs that contain text. (optional)
    #[serde(rename = "trackingId1__contains", skip_serializing_if = "Option::is_none")]
    pub tracking_id1_contains: Option<String>,
    /// List of Account IDs to filter by. (optional, comma-joined array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Include devices by os version that contain text. (optional)
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// If true, only total number of items will be returned, without any of
    /// the actual objects. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// List of Group IDs to filter by. (optional, comma-joined array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Include devices by users that contain text. (optional)
    #[serde(rename = "user__contains", skip_serializing_if = "Option::is_none")]
    pub user_contains: Option<String>,
    /// Include devices by another external tracking IDs that contain text. (optional)
    #[serde(rename = "trackingId2__contains", skip_serializing_if = "Option::is_none")]
    pub tracking_id2_contains: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Include devices only of given platforms. (optional)
    ///
    /// Allowed values: `android`, `ios`, `chrome_os`. Comma-joined array.
    #[serde(rename = "platform__in", skip_serializing_if = "Option::is_none")]
    pub platform_in: Option<String>,
    /// Indicates a tenant scope request. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Sort direction. (optional)
    ///
    /// Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Include devices with given app versions. (optional, comma-joined array)
    #[serde(rename = "appVersion__in", skip_serializing_if = "Option::is_none")]
    pub app_version_in: Option<String>,
    /// Include devices only with given health state. (optional)
    ///
    /// Allowed values: `normal`, `low`, `medium`, `high`, `critical`,
    /// `not_activated`. Comma-joined array.
    #[serde(rename = "healthState__in", skip_serializing_if = "Option::is_none")]
    pub health_state_in: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Limit number of returned items (1-1000). (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The column to sort the results by. (optional)
    ///
    /// Allowed values: `id`, `appVersion`, `registeredOn`, `lastActiveOn`,
    /// `healthState`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Include devices by device IDs that contain text. (optional)
    #[serde(rename = "deviceId__contains", skip_serializing_if = "Option::is_none")]
    pub device_id_contains: Option<String>,
}

impl DevicesQuery {
    /// Include devices only with given privileges (`none`, `rooted`, `jailbroken`).
    pub fn privileges_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.privileges_in = Some(join(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
    /// Include devices by models that contain text.
    pub fn model_contains(mut self, v: impl Into<String>) -> Self {
        self.model_contains = Some(v.into());
        self
    }
    /// Include devices by external tracking IDs that contain text.
    pub fn tracking_id1_contains(mut self, v: impl Into<String>) -> Self {
        self.tracking_id1_contains = Some(v.into());
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// Include devices by os version that contain text.
    pub fn os_version_contains(mut self, v: impl Into<String>) -> Self {
        self.os_version_contains = Some(v.into());
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join(v));
        self
    }
    /// Include devices by users that contain text.
    pub fn user_contains(mut self, v: impl Into<String>) -> Self {
        self.user_contains = Some(v.into());
        self
    }
    /// Include devices by another external tracking IDs that contain text.
    pub fn tracking_id2_contains(mut self, v: impl Into<String>) -> Self {
        self.tracking_id2_contains = Some(v.into());
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Include devices only of given platforms (`android`, `ios`, `chrome_os`).
    pub fn platform_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.platform_in = Some(join(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Sort direction (`asc`, `desc`).
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Include devices with given app versions.
    pub fn app_version_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.app_version_in = Some(join(v));
        self
    }
    /// Include devices only with given health state.
    pub fn health_state_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.health_state_in = Some(join(v));
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// The column to sort the results by
    /// (`id`, `appVersion`, `registeredOn`, `lastActiveOn`, `healthState`).
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Include devices by device IDs that contain text.
    pub fn device_id_contains(mut self, v: impl Into<String>) -> Self {
        self.device_id_contains = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/mobile-integration/incidents`.
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncidentsQuery {
    /// List of Site IDs to filter by. (optional, comma-joined array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. (optional, comma-joined array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// If true, only total number of items will be returned, without any of
    /// the actual objects. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// List of Group IDs to filter by. (optional, comma-joined array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Include incidents by user email that contain text. (optional)
    #[serde(rename = "user__contains", skip_serializing_if = "Option::is_none")]
    pub user_contains: Option<String>,
    /// Include incidents only of given device ids. (optional, comma-joined array)
    #[serde(rename = "deviceId__in", skip_serializing_if = "Option::is_none")]
    pub device_id_in: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Indicates a tenant scope request. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Sort direction. (optional)
    ///
    /// Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Include incident only of given status actions. (optional)
    ///
    /// Allowed values: `conditional_access`, `user_notified`, `not_mitigated`,
    /// `policy_block`, `no_action_taken`. Comma-joined array.
    #[serde(rename = "statusAction__in", skip_serializing_if = "Option::is_none")]
    pub status_action_in: Option<String>,
    /// Include incident only of given incident statuses. (optional)
    ///
    /// Allowed values: `unresolved`, `in_progress`, `resolved`. Comma-joined array.
    #[serde(rename = "incidentStatus__in", skip_serializing_if = "Option::is_none")]
    pub incident_status_in: Option<String>,
    /// Include incident only of given severities. (optional)
    ///
    /// Allowed values: `low`, `medium`, `high`, `critical`. Comma-joined array.
    #[serde(rename = "severity__in", skip_serializing_if = "Option::is_none")]
    pub severity_in: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Include incident only of given analyst verdicts. (optional)
    ///
    /// Allowed values: `true_positive`, `false_positive`, `suspicious`,
    /// `undefined`. Comma-joined array.
    #[serde(rename = "analystVerdict__in", skip_serializing_if = "Option::is_none")]
    pub analyst_verdict_in: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Limit number of returned items (1-1000). (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Include incident only of given statuses. (optional)
    ///
    /// Allowed values: `not_mitigated`, `mitigated`, `admin_resolved`,
    /// `conditional_access`. Comma-joined array.
    #[serde(rename = "status__in", skip_serializing_if = "Option::is_none")]
    pub status_in: Option<String>,
    /// Include incidents only of given kinds. (optional)
    ///
    /// Allowed values: `threat`, `alert`. Comma-joined array.
    #[serde(rename = "kind__in", skip_serializing_if = "Option::is_none")]
    pub kind_in: Option<String>,
    /// The column to sort the results by. (optional)
    ///
    /// Allowed values: `id`, `reportedTime`, `severity`, `status`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Include incidents by device IDs that contain text. (optional)
    #[serde(rename = "deviceId__contains", skip_serializing_if = "Option::is_none")]
    pub device_id_contains: Option<String>,
}

impl IncidentsQuery {
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join(v));
        self
    }
    /// Include incidents by user email that contain text.
    pub fn user_contains(mut self, v: impl Into<String>) -> Self {
        self.user_contains = Some(v.into());
        self
    }
    /// Include incidents only of given device ids.
    pub fn device_id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_id_in = Some(join(v));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Sort direction (`asc`, `desc`).
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Include incident only of given status actions.
    pub fn status_action_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.status_action_in = Some(join(v));
        self
    }
    /// Include incident only of given incident statuses.
    pub fn incident_status_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.incident_status_in = Some(join(v));
        self
    }
    /// Include incident only of given severities.
    pub fn severity_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.severity_in = Some(join(v));
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Include incident only of given analyst verdicts.
    pub fn analyst_verdict_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.analyst_verdict_in = Some(join(v));
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// Include incident only of given statuses.
    pub fn status_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.status_in = Some(join(v));
        self
    }
    /// Include incidents only of given kinds (`threat`, `alert`).
    pub fn kind_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.kind_in = Some(join(v));
        self
    }
    /// The column to sort the results by
    /// (`id`, `reportedTime`, `severity`, `status`).
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Include incidents by device IDs that contain text.
    pub fn device_id_contains(mut self, v: impl Into<String>) -> Self {
        self.device_id_contains = Some(v.into());
        self
    }
}

/// Scope query params shared by the provisioning GET endpoints:
/// `mssp-provisioning/partner`, `provisioning/can-provision-tenant`,
/// `provisioning/partner-key` and `provisioning/tenant`.
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvisioningScopeQuery {
    /// Indicates a tenant scope request. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// List of Group IDs to filter by. (optional, comma-joined array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Account IDs to filter by. (optional, comma-joined array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. (optional, comma-joined array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl ProvisioningScopeQuery {
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join(v));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
}

// ===========================================================================
// Request bodies
// ===========================================================================

/// Scope filter shared by all provisioning request bodies.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvisioningFilter {
    /// Indicates a tenant scope request. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// List of Group IDs to filter by (1-500 items). (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by (1-500 items). (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (1-500 items). (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
}

/// Admin-user data for the provision-with-user request body.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvisionWithUserData {
    /// User's email address. (required)
    pub admin_email: String,
    /// User's first name. (required)
    pub admin_first_name: String,
    /// User's last name. (required)
    pub admin_last_name: String,
    /// Notification email for sending details. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_notification_email: Option<String>,
}

/// Request body for the provision-with-user endpoints
/// (`POST mssp-provisioning/partner`, `POST provisioning/tenant`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvisionWithUserBody {
    /// Admin user data. (required)
    pub data: ProvisionWithUserData,
    /// Scope filter. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ProvisioningFilter>,
}

/// Partner-key data (client ID + secret) for the partner-key request body.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartnerKeyData {
    /// Partner client ID. (required)
    pub client_id: String,
    /// Partner secret. (required)
    pub secret: String,
}

/// Request body for persisting / updating a partner key
/// (`POST` / `PUT` `provisioning/partner-key`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartnerKeyBody {
    /// Partner key data. (required)
    pub data: PartnerKeyData,
    /// Scope filter. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ProvisioningFilter>,
}

/// Request body for deleting a partner key
/// (`DELETE provisioning/partner-key/{client_id}`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePartnerKeyBody {
    /// Scope filter. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ProvisioningFilter>,
}

// ===========================================================================
// Service methods
// ===========================================================================

impl MobileIntegrationService<'_> {
    /// `GET /web/api/v2.1/mobile-integration/devices` — Devices - Get list of
    /// devices for specific scope.
    ///
    /// Devices - Get list devices for specific scope.
    pub async fn list_devices(&self, query: &DevicesQuery) -> Result<Paginated<Device>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/mobile-integration/devices", q)
            .await?)
    }

    /// `GET /web/api/v2.1/mobile-integration/incidents` — Incidents - Get list
    /// of incidents.
    ///
    /// Incidents - Get list of incidents.
    pub async fn list_incidents(
        &self,
        query: &IncidentsQuery,
    ) -> Result<Paginated<Incident>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/mobile-integration/incidents", q)
            .await?)
    }

    /// `GET /web/api/v2.1/mobile-integration/mssp-provisioning/partner` —
    /// Provision - Get MSSP partner with admin user.
    ///
    /// Gets MSSP partner with the first admin user by scope.
    pub async fn get_mssp_partner(
        &self,
        query: &ProvisioningScopeQuery,
    ) -> Result<Response<MsspWithUsers>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/mobile-integration/mssp-provisioning/partner", q)
            .await?)
    }

    /// `POST /web/api/v2.1/mobile-integration/mssp-provisioning/partner` —
    /// Provision - Provision MSSP partner with admin user.
    ///
    /// Provision a new MSSP partner and create an admin user for the partner
    /// account.
    pub async fn provision_mssp_partner(
        &self,
        body: &ProvisionWithUserBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/mobile-integration/mssp-provisioning/partner", body)
            .await?)
    }

    /// `GET /web/api/v2.1/mobile-integration/provisioning/can-provision-tenant`
    /// — Provision - Check if tenant can be provisioned.
    ///
    /// Checks if tenant can be provisioned by scope.
    pub async fn can_provision_tenant(
        &self,
        query: &ProvisioningScopeQuery,
    ) -> Result<Response<CanProvisionTenant>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/mobile-integration/provisioning/can-provision-tenant",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/mobile-integration/provisioning/partner-key` —
    /// Provision - Get MSSP partner key.
    ///
    /// Gets MSSP partner key by scope.
    pub async fn get_partner_key(
        &self,
        query: &ProvisioningScopeQuery,
    ) -> Result<Response<PartnerKey>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/mobile-integration/provisioning/partner-key", q)
            .await?)
    }

    /// `POST /web/api/v2.1/mobile-integration/provisioning/partner-key` —
    /// Provision - Persist MSSP partner key.
    ///
    /// Persists MSSP partner key - client ID and secret - for future customer
    /// provisioning.
    pub async fn create_partner_key(
        &self,
        body: &PartnerKeyBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/mobile-integration/provisioning/partner-key", body)
            .await?)
    }

    /// `PUT /web/api/v2.1/mobile-integration/provisioning/partner-key` —
    /// Provision - Update MSSP partner key.
    ///
    /// Updates MSSP partner key - client ID and secret - for future customer
    /// provisioning.
    pub async fn update_partner_key(
        &self,
        body: &PartnerKeyBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .request_json(
                Method::PUT,
                "/web/api/v2.1/mobile-integration/provisioning/partner-key",
                None,
                Some(body),
            )
            .await?)
    }

    /// `DELETE /web/api/v2.1/mobile-integration/provisioning/partner-key/{client_id}`
    /// — Deletes MSSP partner key by client ID.
    ///
    /// Provision - Delete MSSP partner key.
    ///
    /// Note: the API returns `204 No Content` on success; the empty body is
    /// surfaced as a decode error by the underlying HTTP layer.
    pub async fn delete_partner_key(
        &self,
        client_id: impl Into<String>,
        body: &DeletePartnerKeyBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/mobile-integration/provisioning/partner-key/{}",
            client_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json(Method::DELETE, &path, None, Some(body))
            .await?)
    }

    /// `GET /web/api/v2.1/mobile-integration/provisioning/tenant` —
    /// Provision - Get tenant with users.
    ///
    /// Gets tenant with users by scope.
    pub async fn get_tenant(
        &self,
        query: &ProvisioningScopeQuery,
    ) -> Result<Response<TenantWithUsers>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/mobile-integration/provisioning/tenant", q)
            .await?)
    }

    /// `POST /web/api/v2.1/mobile-integration/provisioning/tenant` —
    /// Provision - Provision tenant with admin user.
    ///
    /// Provision a new tenant and create an admin user for the tenant account.
    pub async fn provision_tenant(
        &self,
        body: &ProvisionWithUserBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/mobile-integration/provisioning/tenant", body)
            .await?)
    }
}

/// Comma-join an iterator of string-like values for array query params.
fn join<I, S>(v: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    v.into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}
