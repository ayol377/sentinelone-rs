use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::agent_support_actions::AgentSupportActionResult;
use crate::pagination::Response;

/// `Agent Support Actions` tag.
///
/// Advanced support actions on Agents.
pub struct AgentSupportActionsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Request body for `POST /web/api/v2.1/agents/actions/clear-remote-shell-session`.
///
/// Mirrors `agents.schemas_AgentsActionSchema`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearRemoteShellSessionBody {
    /// Applied filter - only matched Agents will be affected by the requested
    /// action. Leave empty to apply the action on all applicable Agents.
    ///
    /// Required (present in the schema `required` array). This is a large,
    /// deeply nested freeform Agent filter object (site/account/group IDs,
    /// version/OS/scan/threat/cloud filters, free-text `__contains` filters,
    /// timestamp range filters, etc.), so it is represented as
    /// [`serde_json::Value`] for forward-compatibility.
    pub filter: serde_json::Value,
    /// Data.
    ///
    /// Optional / nullable freeform object (`x-nullable`, not required)
    /// -> `Option<serde_json::Value>`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl ClearRemoteShellSessionBody {
    /// Set the Agent filter (required).
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = filter;
        self
    }
    /// Set the optional `data` object.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

impl AgentSupportActionsService<'_> {
    /// `POST /web/api/v2.1/agents/actions/clear-remote-shell-session` — Clear Remote Shell.
    ///
    /// Remote Shell is a powerful way to respond remotely to events on
    /// endpoints. It lets you open full shell capabilities - PowerShell on
    /// Windows and Bash on macOS and Linux.
    ///
    /// For best practices, a Remote Shell session can be terminated in many
    /// ways: from the UI, from Agent timeouts, from endpoint or connections
    /// issues, and so on. If a shell closes at the same time that an Agent goes
    /// offline, Remote Shell status is incorrect on the Management.
    ///
    /// Use this command to clear the "open shell" flags on the Management.
    ///
    /// The IT user role does not have permissions to run this command.
    pub async fn clear_remote_shell_session(
        &self,
        body: &ClearRemoteShellSessionBody,
    ) -> Result<Response<AgentSupportActionResult>, Error> {
        Ok(self
            .client
            .http()
            .post(
                "/web/api/v2.1/agents/actions/clear-remote-shell-session",
                body,
            )
            .await?)
    }
}
