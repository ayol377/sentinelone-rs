use serde::{Deserialize, Serialize};

use crate::client::ManagementClient;
use crate::error::Error;
use crate::pagination::Response;

/// `Agent Actions` tag — bulk actions to apply on agents.
///
/// Every action is a `POST` that takes a [`AgentFilter`] (which agents to act
/// on) and, for many actions, an action-specific `data` payload. Most actions
/// return the number of affected agents; a few return a richer object.
pub struct AgentActionsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Target filter for agent actions.
///
/// Only agents that match the filter are affected by the requested action.
/// Leave the filter empty (e.g. [`AgentFilter::default`]) to apply the action
/// to all applicable agents. This is a useful subset of the full common agents
/// filter; all fields are optional and are omitted from the request when unset.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentFilter {
    /// Included Agent IDs. Example: `["225494730938493804"]`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// Excluded Agent IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids_nin: Option<Vec<String>>,
    /// A free-text search term; matches applicable attributes (sub-string
    /// match). Example: `"Linux"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// A list of included Agent UUIDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuids: Option<Vec<String>>,
    /// Agent's universally unique identifier. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// Computer name (exact). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// Free-text filter by computer name (supports multiple values). Optional.
    #[serde(
        rename = "computerName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub computer_name_contains: Option<Vec<String>>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by (alternative, larger limit). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_group_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (alternative, larger limit). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_site_ids: Option<Vec<String>>,
    /// Included OS types. Example: `["windows", "linux", "macos"]`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<Vec<String>>,
    /// Included machine types. Example: `["laptop", "desktop", "server"]`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_types: Option<Vec<String>>,
    /// Included scan statuses. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_statuses: Option<Vec<String>>,
    /// Include only active agents. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<Vec<bool>>,
    /// Include only agents with updated software. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_up_to_date: Option<Vec<bool>>,
    /// Include only agents with at least one active threat. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infected: Option<bool>,
    /// Include active, decommissioned, or both. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_decommissioned: Option<Vec<bool>>,
    /// Include installed, uninstalled, or both. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_uninstalled: Option<Vec<bool>>,
    /// Include all agents matching this saved filter (filter ID). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<String>,
}

impl AgentFilter {
    /// Build a filter targeting the given Agent IDs.
    pub fn ids<I, S>(ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            ids: Some(ids.into_iter().map(Into::into).collect()),
            ..Default::default()
        }
    }

    /// Set excluded Agent IDs.
    pub fn ids_nin<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.ids_nin = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// Set a free-text search term.
    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.query = Some(q.into());
        self
    }

    /// Set the list of included Agent UUIDs.
    pub fn uuids<I, S>(mut self, uuids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.uuids = Some(uuids.into_iter().map(Into::into).collect());
        self
    }

    /// Set a single Agent UUID.
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// Set the exact computer name.
    pub fn computer_name(mut self, name: impl Into<String>) -> Self {
        self.computer_name = Some(name.into());
        self
    }

    /// Set the free-text computer-name (substring) filter values.
    pub fn computer_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.computer_name_contains = Some(values.into_iter().map(Into::into).collect());
        self
    }

    /// Set the list of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// Set the list of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// Set the list of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// Set the list of Group IDs to filter by (alternative, larger limit).
    pub fn filtered_group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.filtered_group_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// Set the list of Site IDs to filter by (alternative, larger limit).
    pub fn filtered_site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.filtered_site_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// Set the included OS types. Example: `["windows", "linux", "macos"]`.
    pub fn os_types<I, S>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.os_types = Some(types.into_iter().map(Into::into).collect());
        self
    }

    /// Set the included machine types.
    pub fn machine_types<I, S>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.machine_types = Some(types.into_iter().map(Into::into).collect());
        self
    }

    /// Set the included scan statuses.
    pub fn scan_statuses<I, S>(mut self, statuses: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.scan_statuses = Some(statuses.into_iter().map(Into::into).collect());
        self
    }

    /// Include only active agents.
    pub fn is_active(mut self, values: impl IntoIterator<Item = bool>) -> Self {
        self.is_active = Some(values.into_iter().collect());
        self
    }

    /// Include only agents with updated software.
    pub fn is_up_to_date(mut self, values: impl IntoIterator<Item = bool>) -> Self {
        self.is_up_to_date = Some(values.into_iter().collect());
        self
    }

    /// Include only agents with at least one active threat.
    pub fn infected(mut self, infected: bool) -> Self {
        self.infected = Some(infected);
        self
    }

    /// Include active, decommissioned, or both.
    pub fn is_decommissioned(mut self, values: impl IntoIterator<Item = bool>) -> Self {
        self.is_decommissioned = Some(values.into_iter().collect());
        self
    }

    /// Include installed, uninstalled, or both.
    pub fn is_uninstalled(mut self, values: impl IntoIterator<Item = bool>) -> Self {
        self.is_uninstalled = Some(values.into_iter().collect());
        self
    }

    /// Include all agents matching this saved filter (filter ID).
    pub fn filter_id(mut self, id: impl Into<String>) -> Self {
        self.filter_id = Some(id.into());
        self
    }
}

/// Request body for actions that take only a filter (no `data`).
#[derive(Serialize)]
struct ActionBody<'a> {
    filter: &'a AgentFilter,
}

/// Request body for actions that take a filter and an action-specific `data`.
#[derive(Serialize)]
struct ActionDataBody<'a, D: Serialize> {
    filter: &'a AgentFilter,
    data: D,
}

/// Request body for actions addressed by path param that take only `data`
/// (used by `fetch-files`, which targets a single agent via the URL).
#[derive(Serialize)]
struct DataBody<D: Serialize> {
    data: D,
}

/// `data` for `approve-stateless-upgrade`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveStatelessUpgradeData {
    /// Number of days users are authorized to upgrade agents. Required.
    pub expiration: i64,
}

impl ApproveStatelessUpgradeData {
    /// Build the payload with the authorization period, in days.
    pub fn new(expiration: i64) -> Self {
        Self { expiration }
    }
}

/// `data` for `broadcast`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastData {
    /// Message to broadcast to agents (must be 140 characters or fewer). Required.
    pub message: String,
}

impl BroadcastData {
    /// Build the payload with the message to broadcast.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// `data` for `disable-agent`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisableAgentData {
    /// Reboot the endpoint. Required.
    pub should_reboot: bool,
    /// Agents will be re-enabled after this timestamp. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// Timezone for the expiration timestamp. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_timezone: Option<String>,
}

impl DisableAgentData {
    /// Build the payload, setting whether the endpoint should reboot.
    pub fn new(should_reboot: bool) -> Self {
        Self {
            should_reboot,
            expiration: None,
            expiration_timezone: None,
        }
    }
}

/// `data` for `enable-agent`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableAgentData {
    /// Reboot the endpoint. Required.
    pub should_reboot: bool,
}

impl EnableAgentData {
    /// Build the payload, setting whether the endpoint should reboot.
    pub fn new(should_reboot: bool) -> Self {
        Self { should_reboot }
    }
}

/// `data` for `fetch-firewall-rules`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchFirewallRulesData {
    /// Desired firewall configuration state. Optional.
    ///
    /// Allowed values: `initial` (configuration that existed before Agent
    /// installation; requires `native` format), `current`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Desired firewall configuration format. Optional.
    ///
    /// Allowed values: `native`, `asset`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// `data` for `fetch-logs`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchLogsData {
    /// Fetch Agent logs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_logs: Option<bool>,
    /// Fetch customer-facing logs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_facing_logs: Option<bool>,
    /// Actively fetch logs from the relevant platform (windows/mac/linux). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_logs: Option<bool>,
}

/// `data` for `firewall-logging`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallLoggingData {
    /// Report blocking activity to management. Required.
    pub report_mgmt: bool,
    /// Report blocking activity to log. Required.
    pub report_log: bool,
}

impl FirewallLoggingData {
    /// Build the payload, choosing the reporting destinations.
    pub fn new(report_mgmt: bool, report_log: bool) -> Self {
        Self {
            report_mgmt,
            report_log,
        }
    }
}

/// `data` for `local-upgrade-authorization`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalUpgradeAuthorizationData {
    /// Agent approval expiration timestamp. Required.
    pub agent_authorization: String,
}

impl LocalUpgradeAuthorizationData {
    /// Build the payload with the approval expiration timestamp.
    pub fn new(agent_authorization: impl Into<String>) -> Self {
        Self {
            agent_authorization: agent_authorization.into(),
        }
    }
}

/// A single tag operation for `manage-tags`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointTag {
    /// Tag ID. Example: `"225494730938493804"`. Required.
    pub tag_id: String,
    /// Operation to perform on the tag. Required.
    ///
    /// Allowed values: `add`, `remove`, `override`.
    pub operation: String,
}

impl EndpointTag {
    /// Build a tag operation from a tag ID and an operation
    /// (`add`, `remove`, or `override`).
    pub fn new(tag_id: impl Into<String>, operation: impl Into<String>) -> Self {
        Self {
            tag_id: tag_id.into(),
            operation: operation.into(),
        }
    }
}

/// `data` for `move-to-console`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveToConsoleData {
    /// Site token of the site to which the agent is to be moved. This is a
    /// base-64 string taken from the target site's `registrationToken`. Required.
    pub token: String,
}

impl MoveToConsoleData {
    /// Build the payload with the target site token.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

/// `data` for `move-to-site`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveToSiteData {
    /// Target site ID. Required.
    pub target_site_id: String,
}

impl MoveToSiteData {
    /// Build the payload with the target site ID.
    pub fn new(target_site_id: impl Into<String>) -> Self {
        Self {
            target_site_id: target_site_id.into(),
        }
    }
}

/// `data` for `set-config`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetConfigData {
    /// Configuration overrides to apply, as a freeform JSON object. Required.
    ///
    /// Get the JSON settings from the Agent Configuration or the SentinelCtl
    /// knowledge base.
    pub config: serde_json::Value,
}

impl SetConfigData {
    /// Build the payload with the configuration override object.
    pub fn new(config: serde_json::Value) -> Self {
        Self { config }
    }
}

/// `data` for `set-external-id`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExternalIdData {
    /// New external ID (Customer Identifier) for the agent. Required.
    pub external_id: String,
}

impl SetExternalIdData {
    /// Build the payload with the new external ID.
    pub fn new(external_id: impl Into<String>) -> Self {
        Self {
            external_id: external_id.into(),
        }
    }
}

/// `data` for `start-profiling`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartProfilingData {
    /// Profiling will be disabled after this many seconds. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// Scope from which a generic password is taken, for `start-remote-shell`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordFromScope {
    /// User scope. Required.
    ///
    /// Allowed values: `tenant`, `account`, `site`.
    pub scope_level: String,
    /// String representation of the scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

/// `data` for `start-remote-shell`.
///
/// Remote Shell is a powerful, full shell on the endpoint. This payload is part
/// of a security-sensitive action; see [`AgentActionsService::start_remote_shell`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartRemoteShellData {
    /// Number of columns of the console shell. Required.
    pub columns: i64,
    /// Number of rows of the console shell. Required.
    pub rows: i64,
    /// Password used to zip the shell history file at the end of the session.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_password: Option<String>,
    /// Used to specify execution where a generic password is used. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_from_scope: Option<PasswordFromScope>,
    /// [DEPRECATED] The 2FA code to authenticate the user. Use only if
    /// `remote_shell_risky_actions` requires it. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_code: Option<String>,
}

impl StartRemoteShellData {
    /// Build the payload with the console dimensions (columns, rows).
    pub fn new(columns: i64, rows: i64) -> Self {
        Self {
            columns,
            rows,
            history_password: None,
            password_from_scope: None,
            two_fa_code: None,
        }
    }
}

/// `data` for `terminate-remote-shell`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateRemoteShellData {
    /// The channel the user is closing. Required.
    pub channel_id: String,
}

impl TerminateRemoteShellData {
    /// Build the payload with the channel ID to terminate.
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
        }
    }
}

/// `data` for `update-software`.
///
/// The API requires `package_type`, `os_type`, and `file_name` (or an
/// equivalent package locator) in practice, even though the schema marks them
/// optional.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSoftwareData {
    /// Upgrade according to the schedule in the agent upgrade configuration. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_scheduled: Option<bool>,
    /// Allow or disallow downgrading the Agent version. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_downgrade: Option<bool>,
    /// Upgrade with a given uploaded package, located by its filename. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Upgrade from a local path on the endpoint. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Filter by a specific OS type; can be combined with `file_name` or `path`.
    /// Optional.
    ///
    /// Allowed values: `linux`, `macos`, `windows_legacy`, `windows`,
    /// `threat_detection_netapp`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Upgrade with a given uploaded package, located by its ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    /// Package type. Optional.
    ///
    /// Allowed values: `Agent`, `Ranger`, `AgentAndRanger`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    /// Ignore conflicts that may arise when upgrading an Agent that has an
    /// active Upgrade Policy. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_conflicts: Option<bool>,
}

/// `data` for `fetch-files`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchFilesData {
    /// List of files to fetch (absolute paths, up to 10 files). Required.
    pub files: Vec<String>,
    /// File encryption password used to open the archive of downloaded files.
    /// Must be 10 or more characters with a mix of upper and lower case letters,
    /// numbers, and symbols. Required.
    pub password: String,
}

impl FetchFilesData {
    /// Build the payload from the file paths and the archive password.
    pub fn new<I, S>(files: I, password: impl Into<String>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            files: files.into_iter().map(Into::into).collect(),
            password: password.into(),
        }
    }
}

/// Response envelope `data` for actions that report a count of affected agents.
#[derive(Debug, Deserialize)]
struct Affected {
    #[serde(default)]
    affected: i64,
}

/// A single reason returned per agent by `reset-passphrase`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPassphraseReason {
    /// Machine-readable reason code.
    pub code: String,
    /// Human-readable reason message.
    pub message: String,
    /// Optional additional details, if provided.
    #[serde(default)]
    pub details: Option<serde_json::Value>,
}

/// Per-agent result returned by `reset-passphrase`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPassphraseResult {
    /// The agent the result refers to.
    pub agent_id: String,
    /// Whether a reset was attempted for this agent.
    pub attempted: bool,
    /// Per-agent status of the reset.
    pub status: String,
    /// Reasons explaining the result (e.g. eligibility failures).
    #[serde(default)]
    pub reasons: Vec<ResetPassphraseReason>,
}

/// Response `data` for `reset-passphrase`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPassphrasesResult {
    /// Per-agent results.
    #[serde(default)]
    pub results: Vec<ResetPassphraseResult>,
    /// Freeform summary of the operation.
    #[serde(default)]
    pub summary: serde_json::Value,
}

/// A single opened channel returned by `start-remote-shell`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteShellChannel {
    /// Name of the channel used to communicate with the agent.
    #[serde(default)]
    pub channel_id: Option<String>,
    /// Agent that matched the filter.
    #[serde(default)]
    pub agent_id: Option<String>,
    /// Whether the agent supports the terminal resize capability.
    #[serde(default)]
    pub supports_resize: Option<bool>,
}

/// Response `data` for `fetch-files`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchFilesResult {
    /// Indicates a successful operation.
    #[serde(default)]
    pub success: Option<bool>,
}

impl AgentActionsService<'_> {
    /// Issue an action and return the number of affected agents.
    async fn post_affected<B>(&self, path: &str, body: &B) -> Result<i64, Error>
    where
        B: Serialize,
    {
        let resp: Response<Affected> = self.client.http().post(path, body).await?;
        Ok(resp.data.affected)
    }

    /// `POST /web/api/v2.1/agents/actions/abort-scan` — Abort Scan.
    ///
    /// Immediately stop a Full Disk Scan on all agents that match the filter.
    /// Returns the number of affected agents.
    pub async fn abort_scan(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/abort-scan",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/approve-stateless-upgrade` — Approve
    /// Stateless Upgrades.
    ///
    /// Approve stateless upgrade for agents. Returns the number of affected agents.
    pub async fn approve_stateless_upgrade(
        &self,
        filter: &AgentFilter,
        data: &ApproveStatelessUpgradeData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/approve-stateless-upgrade",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/approve-uninstall` — Approve Uninstall.
    ///
    /// Approve pending uninstall requests for all agents that match the filter.
    /// Returns the number of affected agents.
    pub async fn approve_uninstall(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/approve-uninstall",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/broadcast` — Broadcast Message.
    ///
    /// Send a message that users can see on Windows and macOS endpoints that
    /// match the filter (not supported on Linux). The message must be 140
    /// characters or fewer. Returns the number of affected agents.
    pub async fn broadcast(
        &self,
        filter: &AgentFilter,
        data: &BroadcastData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/broadcast",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/can-start-remote-shell` — Can run
    /// Remote Shell.
    ///
    /// Check whether the calling user has permission to run Remote Shell on the
    /// agents that match the filter. Returns the number of affected agents.
    pub async fn can_start_remote_shell(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/can-start-remote-shell",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/connect` — Connect to Network.
    ///
    /// Reconnect to the network all endpoints that match the filter (the inverse
    /// of `disconnect`). Returns the number of affected agents.
    pub async fn connect(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/connect",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/decommission` — Decommission.
    ///
    /// Remove the matching agents from the Management Console. They are
    /// recommissioned when the agent next communicates with the Management.
    /// Returns the number of affected agents.
    ///
    /// Note: this is a destructive/sensitive action.
    pub async fn decommission(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/decommission",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/disable-agent` — Disable Agent.
    ///
    /// Disable agents that match the filter. Disabled agents run with a minimal
    /// footprint and do not detect or mitigate threats, but maintain
    /// connectivity with the Management Console. The `data` parameter is
    /// mandatory. Returns the number of affected agents.
    pub async fn disable_agent(
        &self,
        filter: &AgentFilter,
        data: &DisableAgentData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/disable-agent",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/disconnect` — Disconnect from Network.
    ///
    /// Isolate (quarantine) from the network the endpoints that match the
    /// filter; the agent can still communicate with the Management. Returns the
    /// number of affected agents.
    ///
    /// Note: this is a destructive/sensitive action.
    pub async fn disconnect(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/disconnect",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/enable-agent` — Enable Agent.
    ///
    /// Enable disabled agents that match the filter. The `data` parameter is
    /// mandatory. Returns the number of affected agents.
    pub async fn enable_agent(
        &self,
        filter: &AgentFilter,
        data: &EnableAgentData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/enable-agent",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/fetch-firewall-rules` — Fetch Firewall
    /// Rules.
    ///
    /// Make the matching agents fetch the latest Firewall Control rules. Returns
    /// the number of affected agents.
    pub async fn fetch_firewall_rules(
        &self,
        filter: &AgentFilter,
        data: &FetchFirewallRulesData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/fetch-firewall-rules",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/fetch-installed-apps` — Get
    /// Applications.
    ///
    /// Make the matching agents update the data of the applications installed on
    /// the endpoint (Application Risk Management). Returns the number of affected
    /// agents.
    pub async fn fetch_installed_apps(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/fetch-installed-apps",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/fetch-logs` — Fetch Logs.
    ///
    /// Collect the Agent and Endpoint logs from agents that match the filter and
    /// upload them to the Management. Returns the number of affected agents.
    pub async fn fetch_logs(
        &self,
        filter: &AgentFilter,
        data: &FetchLogsData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/fetch-logs",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/firewall-logging` — Fetch Firewall Logs.
    ///
    /// Enable Firewall Control event logging for agents that match the filter.
    /// Returns the number of affected agents.
    pub async fn firewall_logging(
        &self,
        filter: &AgentFilter,
        data: &FirewallLoggingData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/firewall-logging",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/initiate-scan` — Initiate Scan.
    ///
    /// Run a Full Disk Scan on agents that match the filter. Returns the number
    /// of affected agents.
    pub async fn initiate_scan(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/initiate-scan",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/local-upgrade-authorization` — Edit
    /// local upgrade/downgrade Site authorization.
    ///
    /// Edit when authorization of local upgrades/downgrades expires. Returns the
    /// number of affected agents.
    pub async fn local_upgrade_authorization(
        &self,
        filter: &AgentFilter,
        data: &LocalUpgradeAuthorizationData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/local-upgrade-authorization",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/manage-tags` — Manage endpoint tags:
    /// add, remove, override.
    ///
    /// Apply the given tag operations to the endpoints that match the filter.
    /// Returns the number of affected agents.
    pub async fn manage_tags(
        &self,
        filter: &AgentFilter,
        tags: &[EndpointTag],
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/manage-tags",
            &ActionDataBody { filter, data: tags },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/mark-up-to-date` — Mark as up-to-date.
    ///
    /// Manually mark agents that match the filter as up-to-date. Not available
    /// to users with the SOC role. Returns the number of affected agents.
    pub async fn mark_up_to_date(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/mark-up-to-date",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/move-to-console` — Move to Console.
    ///
    /// Move the matching agents to a target Console, Account, and Site, given
    /// the target Site token. Returns the number of affected agents.
    pub async fn move_to_console(
        &self,
        filter: &AgentFilter,
        data: &MoveToConsoleData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/move-to-console",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/move-to-site` — Move between Sites.
    ///
    /// Move the matching agents from one Site to a different Site. Requires
    /// Account or Global level access. Returns the number of affected agents.
    pub async fn move_to_site(
        &self,
        filter: &AgentFilter,
        data: &MoveToSiteData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/move-to-site",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/randomize-uuid` — Randomize UUID.
    ///
    /// Assign a new UUID to agents that match the filter. Run only when
    /// instructed by SentinelOne Support. Returns the number of affected agents.
    ///
    /// Note: this is a destructive/sensitive action — historical threat and Deep
    /// Visibility data is disassociated from the agent.
    pub async fn randomize_uuid(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/randomize-uuid",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/ranger-disable` — Disable Network
    /// Discovery.
    ///
    /// Disable Network Discovery (Ranger) on the agents that match the filter.
    /// Returns the number of affected agents.
    pub async fn ranger_disable(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/ranger-disable",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/ranger-enable` — Enable Network
    /// Discovery.
    ///
    /// Enable Network Discovery (Ranger) on the agents that match the filter.
    /// Returns the number of affected agents.
    pub async fn ranger_enable(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/ranger-enable",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/reject-uninstall` — Reject uninstall.
    ///
    /// Reject uninstall requests for all agents that match the filter. Returns
    /// the number of affected agents.
    pub async fn reject_uninstall(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/reject-uninstall",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/reset-local-config` — Reset Local Config.
    ///
    /// Clear SentinelCtl changes from all agents that match the filter (some
    /// settings are preserved). Returns the number of affected agents.
    pub async fn reset_local_config(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/reset-local-config",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/reset-passphrase` — Reset Passphrases.
    ///
    /// Initiate an agent passphrase reset for agents that match the filter. The
    /// action performs eligibility checks and returns per-agent results.
    ///
    /// Note: this is a security-sensitive action — historical passphrase records
    /// are kept in the Management and associated with the agent.
    pub async fn reset_passphrase(
        &self,
        filter: &AgentFilter,
    ) -> Result<Response<ResetPassphrasesResult>, Error> {
        Ok(self
            .client
            .http()
            .post(
                "/web/api/v2.1/agents/actions/reset-passphrase",
                &ActionBody { filter },
            )
            .await?)
    }

    /// `POST /web/api/v2.1/agents/actions/restart-machine` — Restart.
    ///
    /// Restart endpoints that have an agent installed and that match the filter.
    /// Returns the number of affected agents.
    ///
    /// Note: this is a destructive/sensitive action — consider sending a
    /// `broadcast` message to users before restarting their computers.
    pub async fn restart_machine(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/restart-machine",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/set-config` — Set Persistent
    /// Configuration Overrides.
    ///
    /// Apply persistent configuration overrides to agents that match the filter.
    /// Requires Global permissions or Support. Returns the number of affected
    /// agents.
    pub async fn set_config(
        &self,
        filter: &AgentFilter,
        data: &SetConfigData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/set-config",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/set-external-id` — Set External ID.
    ///
    /// Set a Customer Identifier on all agents that match the filter. Returns the
    /// number of affected agents.
    pub async fn set_external_id(
        &self,
        filter: &AgentFilter,
        data: &SetExternalIdData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/set-external-id",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/shutdown` — Shutdown.
    ///
    /// Shut down all endpoints that match the filter. Offline endpoints cannot be
    /// shut down. Returns the number of affected agents.
    ///
    /// Note: this is a destructive/sensitive action — for infected endpoints
    /// prefer `disconnect` over `shutdown`.
    pub async fn shutdown(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/shutdown",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/start-profiling` — Start Remote
    /// Profiling.
    ///
    /// Start remote profiling on agents that match the filter. Returns the number
    /// of affected agents.
    pub async fn start_profiling(
        &self,
        filter: &AgentFilter,
        data: &StartProfilingData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/start-profiling",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/start-remote-shell` — Start Remote Shell.
    ///
    /// Open a Remote Shell session to agents that match the filter. The `data`
    /// parameter is mandatory. Returns the opened channels.
    ///
    /// Note: this is a security-sensitive action. Remote Shell is a full shell on
    /// the endpoint; SentinelOne recommends against driving it from the API. It
    /// requires elevated permissions, 2FA, and a Control SKU. Prefer targeting a
    /// specific endpoint by UUID.
    pub async fn start_remote_shell(
        &self,
        filter: &AgentFilter,
        data: &StartRemoteShellData,
    ) -> Result<Response<Vec<RemoteShellChannel>>, Error> {
        Ok(self
            .client
            .http()
            .post(
                "/web/api/v2.1/agents/actions/start-remote-shell",
                &ActionDataBody { filter, data },
            )
            .await?)
    }

    /// `POST /web/api/v2.1/agents/actions/stop-profiling` — Stop Remote Profiling.
    ///
    /// Stop remote profiling on agents that match the filter. Returns the number
    /// of affected agents.
    pub async fn stop_profiling(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/stop-profiling",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/terminate-remote-shell` — Terminate
    /// Remote Shell.
    ///
    /// Terminate a Remote Shell session immediately on agents that match the
    /// filter. Returns the number of affected agents.
    pub async fn terminate_remote_shell(
        &self,
        filter: &AgentFilter,
        data: &TerminateRemoteShellData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/terminate-remote-shell",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/uninstall` — Uninstall.
    ///
    /// Uninstall agents that match the filter. Returns the number of affected
    /// agents.
    ///
    /// Note: this is a destructive/sensitive action — reboot endpoints after
    /// uninstall to remove all remnants of the agent.
    pub async fn uninstall(&self, filter: &AgentFilter) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/uninstall",
            &ActionBody { filter },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/actions/update-software` — Update Software.
    ///
    /// Update the Agent version on endpoints that match the filter. In practice
    /// `package_type`, `os_type`, and `file_name` are required. Returns the
    /// number of affected agents.
    pub async fn update_software(
        &self,
        filter: &AgentFilter,
        data: &UpdateSoftwareData,
    ) -> Result<i64, Error> {
        self.post_affected(
            "/web/api/v2.1/agents/actions/update-software",
            &ActionDataBody { filter, data },
        )
        .await
    }

    /// `POST /web/api/v2.1/agents/{agent_id}/actions/fetch-files` — Fetch Files.
    ///
    /// Fetch files (up to 10 MB per command) from a single endpoint, identified
    /// by `agent_id`, and upload them to the Management as a password-protected
    /// archive. Returns a success indicator.
    pub async fn fetch_files(
        &self,
        agent_id: &str,
        data: &FetchFilesData,
    ) -> Result<Response<FetchFilesResult>, Error> {
        let path = format!("/web/api/v2.1/agents/{agent_id}/actions/fetch-files");
        Ok(self
            .client
            .http()
            .post(&path, &DataBody { data })
            .await?)
    }
}
