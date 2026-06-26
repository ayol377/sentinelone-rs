use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::policies::Policy;
use crate::pagination::Response;

/// `Policies` tag.
///
/// Policies related endpoints. A policy can be read and updated at four scopes:
/// the Global (tenant) scope, and per-Account, per-Site, and per-Group scopes.
/// Best practice for updates is to `GET` the policy first, modify the fields you
/// need, then `PUT` it back.
pub struct PoliciesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Request body for the `PUT .../policy` endpoints (`policies_TenantPolicySchema`).
///
/// The schema wraps the policy fields under a top-level `data` object. `data`
/// is required by the schema; the individual policy fields inside it are all
/// optional, so you typically send only the fields you wish to change (after
/// reading the current policy).
#[derive(Debug, Default, Serialize)]
pub struct PolicyUpdateBody {
    /// Data. The policy fields to apply. Required.
    pub data: PolicyUpdateData,
}

impl PolicyUpdateBody {
    /// Construct an update body from a [`PolicyUpdateData`] payload.
    pub fn new(data: PolicyUpdateData) -> Self {
        Self { data }
    }
}

/// Policy fields for [`PolicyUpdateBody`] (`policies_TenantPolicySchema.data`).
///
/// Every field is optional; only the fields you set are serialized. Enum-typed
/// fields are plain `String` (allowed values documented per field) for
/// forward compatibility.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyUpdateData {
    /// Network quarantine on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_quarantine_on: Option<bool>,
    /// Automatic immune on/off - this value must be true since all policies are
    /// immune by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_immune_on: Option<bool>,
    /// Auto decommission on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_decommission_on: Option<bool>,
    /// True if this is the tenant policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Cloud validation on. (Spec declares no explicit type; defaults to `true`.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_validation_on: Option<serde_json::Value>,
    /// Share data with SentinelOne.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub research_on: Option<bool>,
    /// Default action for auto mitigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_mitigation_action: Option<String>,
    /// Automatic decommission period in days (1-99).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_decommission_days: Option<i64>,
    /// Mitigation modes. Allowed values: `detect`, `protect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_mode: Option<String>,
    /// Timestamp of policy creation (ISO-8601, e.g. `2018-02-27T04:49:26.257525Z`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// \[DEPRECATED\] Show endpoint notification on suspicious. Replaced by
    /// `show_suspicious` in the agent UI section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_notification: Option<bool>,
    /// The engines statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<serde_json::Value>,
    /// Mitigation mode (suspicious). Allowed values: `detect`, `protect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_mode_suspicious: Option<String>,
    /// If true, initiate full disk scan upon first registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_new_agents: Option<bool>,
    /// Anti tampering on/off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_tampering_on: Option<bool>,
    /// Suspicious signed driver blocking on/off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_driver_blocking_on: Option<bool>,
    /// Suspicious unsigned driver blocking on/off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsigned_driver_blocking_on: Option<bool>,
    /// Suspicious driver blocking engine on/off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_blocking: Option<bool>,
    /// Informational alerts on/off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub informational_alerts_on: Option<bool>,
    /// \[DEPRECATED\] Show/hide Agent UI. Moved inside the agent UI section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ui_on: Option<bool>,
    /// True if snapshots are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots_on: Option<bool>,
    /// True if logging is enabled in the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_logging_on: Option<bool>,
    /// Monitor on execute on/off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_on_execute: Option<bool>,
    /// Monitor on write.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_on_write: Option<bool>,
    /// True if IOC is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ioc: Option<bool>,
    /// The IOC attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ioc_attributes: Option<serde_json::Value>,
    /// The DV attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_attributes_per_event_type: Option<serde_json::Value>,
    /// IOC supported for the scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ioc_supported: Option<bool>,
    /// Indicates the parent scope from which this policy is inherited, or `null`
    /// if it is not inherited. Allowed values: `site`, `account`, `global`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<String>,
    /// Time of the last update to the policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The user id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The user that created the policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_full_name: Option<String>,
    /// True if Remote Shell is enabled for the scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_shell: Option<bool>,
    /// Automatic file upload configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_file_upload: Option<serde_json::Value>,
    /// Agent UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ui: Option<serde_json::Value>,
    /// Remote script orchestration upload limits configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_script_orchestration: Option<serde_json::Value>,
    /// FE indication as to how to display DV policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dv_policy_per_event_type: Option<bool>,
    /// Remote ops forensics configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ops_forensics: Option<serde_json::Value>,
    /// Determines if macros should be removed from macro threats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_macros: Option<bool>,
    /// Determines if malicious macros should be mitigated using ML models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_macros_ml: Option<bool>,
    /// Identity update interval in minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_update_interval: Option<i64>,
    /// Identity telemetry report interval in minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_report_interval: Option<i64>,
    /// Identity duplicate command consolidation interval in minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_throttling_interval: Option<i64>,
    /// Endpoint reporting level. Allowed values: `disabled`, `conservative`,
    /// `moderate`, `aggressive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_endpoint_reporting: Option<String>,
    /// Identity module on/off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_on: Option<bool>,
    /// Forensics auto triggering configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forensics_auto_triggering: Option<serde_json::Value>,
    /// Allow Unprotect By Approved Process on/off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unprotect_by_approved_process: Option<bool>,
    /// Identity configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_configuration_settings: Option<serde_json::Value>,
    /// Log collector on/off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_collector_enabled: Option<bool>,
    /// Drift detection delay time in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_detection_delay_time: Option<i64>,
    /// Network Protection Infrastructure on/off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_protection_infra: Option<bool>,
    /// SMB Lateral Movement Mitigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_lateral_movement_mitigation: Option<bool>,
}

impl PoliciesService<'_> {
    /// `GET /web/api/v2.1/accounts/{account_id}/policy` — Account Policy.
    ///
    /// Get the policy for the Account given by ID. To get the ID of an Account,
    /// run "accounts". See also: Get Policy.
    ///
    /// `account_id`: Account ID. You can get the ID from the Get accounts
    /// command. Example: "225494730938493804". Required.
    pub async fn account_policy(
        &self,
        account_id: impl Into<String>,
    ) -> Result<Response<Policy>, Error> {
        let path = format!("/web/api/v2.1/accounts/{}/policy", account_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `PUT /web/api/v2.1/accounts/{account_id}/policy` — Update Account Policy.
    ///
    /// Change the policy for the Account given by ID. Best practice: Get the
    /// policy of the Account before you attempt to change it. See also: Get
    /// Policy.
    ///
    /// `account_id`: Account ID. You can get the ID from the Get accounts
    /// command. Example: "225494730938493804". Required.
    pub async fn update_account_policy(
        &self,
        account_id: impl Into<String>,
        body: &PolicyUpdateBody,
    ) -> Result<Response<Policy>, Error> {
        let path = format!("/web/api/v2.1/accounts/{}/policy", account_id.into());
        Ok(self
            .client
            .http()
            .request_json::<PolicyUpdateBody, Response<Policy>>(Method::PUT, &path, None, Some(body))
            .await?)
    }

    /// `GET /web/api/v2.1/groups/{group_id}/policy` — Group Policy.
    ///
    /// Get the policy of the Group given by ID. To get the ID of a Group, run
    /// "groups". See also: Get Policy.
    ///
    /// `group_id`: Group ID. Example: "225494730938493804". Required.
    pub async fn group_policy(
        &self,
        group_id: impl Into<String>,
    ) -> Result<Response<Policy>, Error> {
        let path = format!("/web/api/v2.1/groups/{}/policy", group_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `PUT /web/api/v2.1/groups/{group_id}/policy` — Update Group Policy.
    ///
    /// Change the policy for the Group given by ID. Best practice: Get the
    /// policy of the Group before you attempt to change it. See also: Get
    /// Policy.
    ///
    /// `group_id`: Group ID. Example: "225494730938493804". Required.
    pub async fn update_group_policy(
        &self,
        group_id: impl Into<String>,
        body: &PolicyUpdateBody,
    ) -> Result<Response<Policy>, Error> {
        let path = format!("/web/api/v2.1/groups/{}/policy", group_id.into());
        Ok(self
            .client
            .http()
            .request_json::<PolicyUpdateBody, Response<Policy>>(Method::PUT, &path, None, Some(body))
            .await?)
    }

    /// `GET /web/api/v2.1/sites/{site_id}/policy` — Site Policy.
    ///
    /// Get the policy of the Site given by ID. To get the ID of a Site, run
    /// "sites". See also: Get Policy.
    ///
    /// `site_id`: Site ID. Example: "225494730938493804". Required.
    pub async fn site_policy(&self, site_id: impl Into<String>) -> Result<Response<Policy>, Error> {
        let path = format!("/web/api/v2.1/sites/{}/policy", site_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `PUT /web/api/v2.1/sites/{site_id}/policy` — Update Site Policy.
    ///
    /// Change the policy for the Site given by ID. Best practice: Get the policy
    /// of the Site before you attempt to change it. See also: Get Policy.
    ///
    /// `site_id`: Site ID. Example: "225494730938493804". Required.
    pub async fn update_site_policy(
        &self,
        site_id: impl Into<String>,
        body: &PolicyUpdateBody,
    ) -> Result<Response<Policy>, Error> {
        let path = format!("/web/api/v2.1/sites/{}/policy", site_id.into());
        Ok(self
            .client
            .http()
            .request_json::<PolicyUpdateBody, Response<Policy>>(Method::PUT, &path, None, Some(body))
            .await?)
    }

    /// `GET /web/api/v2.1/tenant/policy` — Global Policy.
    ///
    /// Get the Global policy. This is the default policy for your deployment.
    /// See also: Get Policy.
    pub async fn global_policy(&self) -> Result<Response<Policy>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/tenant/policy", None)
            .await?)
    }

    /// `PUT /web/api/v2.1/tenant/policy` — Update Global Policy.
    ///
    /// Change the policy of your deployment. Best practice: Get the Global
    /// policy before you attempt to change it. See also: Get Policy. You must be
    /// a Global Admin user to change the Global Policy.
    pub async fn update_global_policy(
        &self,
        body: &PolicyUpdateBody,
    ) -> Result<Response<Policy>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<PolicyUpdateBody, Response<Policy>>(
                Method::PUT,
                "/web/api/v2.1/tenant/policy",
                None,
                Some(body),
            )
            .await?)
    }
}
