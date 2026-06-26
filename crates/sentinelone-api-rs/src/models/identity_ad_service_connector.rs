//! Models for the `Identity AD Service - Connector` tag.
//!
//! APIs for managing AD connector information and operations.
//!
//! Every endpoint in this tag returns the SentinelOne `GenericRestResponse`
//! envelope, whose `data` member is freeform in the OpenAPI spec
//! (`{ "data": {} }`). Because the spec does not pin down a concrete shape for
//! the payload, the service methods deserialize `data` as
//! [`serde_json::Value`]. This module therefore only carries the typed request
//! body shapes that the spec *does* define.

use serde::Serialize;

/// Request body for
/// `POST /web/api/v2.1/identity/adservice/api/isIDREnabledOnEndpoint`.
///
/// Spec definition: `RequestBodyWrapperAgentIDRStatusInput`. The wrapper nests
/// the actual input under an `input` member.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBodyWrapperAgentIdrStatusInput {
    /// The agent IDR status input payload.
    ///
    /// Optional / nullable: not present in any schema `required` array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<AgentIdrStatusInput>,
}

impl RequestBodyWrapperAgentIdrStatusInput {
    /// Set the nested `input` payload.
    pub fn input(mut self, input: AgentIdrStatusInput) -> Self {
        self.input = Some(input);
        self
    }
}

/// Inner input payload for the IDR-status check.
///
/// Spec definition: `AgentIDRStatusInput`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentIdrStatusInput {
    /// Agent UUID.
    ///
    /// Type: string. Optional / nullable: not present in any schema `required`
    /// array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// Request ID.
    ///
    /// Type: string. Optional / nullable: not present in any schema `required`
    /// array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl AgentIdrStatusInput {
    /// Set the agent UUID.
    pub fn agent_uuid(mut self, v: impl Into<String>) -> Self {
        self.agent_uuid = Some(v.into());
        self
    }
    /// Set the request ID.
    pub fn request_id(mut self, v: impl Into<String>) -> Self {
        self.request_id = Some(v.into());
        self
    }
}
