//! Service for the `Log Collection` tag.
//!
//! Log Collection operations: manage log collection rules (list, create,
//! update, delete, activate/deactivate), get per-agent-type rule counts and
//! export rules to CSV.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::log_collection::{
    LogCollectionAffectedResultId, LogCollectionAffectedResultIds, LogCollectionAgentTypeCount,
    LogCollectionRule,
};
use crate::pagination::{Paginated, Response};

/// `Log Collection` tag — log collection rule management.
pub struct LogCollectionService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query structs
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/log-collection/agent-type-count`.
///
/// All fields are optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTypeCountQuery {
    /// List of account IDs to filter by. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs to filter by. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of group IDs to filter by. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Return rules from parent scope levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
}

impl AgentTypeCountQuery {
    /// List of account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// List of site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
    /// List of group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Return rules from parent scope levels.
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// Return filters from children scope levels.
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
}

/// Query params for `GET /web/api/v2.1/log-collection/rules` and
/// `GET /web/api/v2.1/log-collection/rules/{agent_type}`.
///
/// All fields are optional. Array params are serialized comma-joined, as the
/// API expects. Enum-valued params are kept as `String` for forward
/// compatibility; allowed values are documented per field.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionRulesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: "150".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values (enum): `id`,
    /// `createdAt`, `createdBy`, `updatedAt`, `updatedBy`, `scopeId`,
    /// `scopeLevel`, `name`. Example: "id".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values (enum): `asc`, `desc`. Example: "asc".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of account IDs to filter by. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs to filter by. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of group IDs to filter by. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// List of IDs to filter by. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Scope. Allowed values (enum): `group`, `site`, `account`, `tenant`.
    /// Example: "group".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// List of scopes to filter by. Allowed item values (enum): `group`,
    /// `site`, `account`, `tenant`. Example: "group".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    /// Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of names to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Free-text filter by rule name (supports multiple).
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Agent type. Allowed values (enum): `windows`, `linux`, `macos`, `k8s`,
    /// `k8sAndContainers`. Example: "windows".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<String>,
    /// List of agent types to filter by. Allowed item values (enum): `windows`,
    /// `linux`, `macos`, `k8s`, `k8sAndContainers`. Example: "windows".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_types: Option<String>,
    /// Configuration type. Allowed values (enum): `collectionPath`, `predicate`.
    /// Example: "collectionPath".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    /// List of configuration types to filter by. Allowed item values (enum):
    /// `collectionPath`, `predicate`. Example: "collectionPath".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_types: Option<String>,
    /// Collection type. Allowed values (enum): `applicationLog`, `flatFileLog`,
    /// `unifiedLogging`, `windowsEventLog`. Example: "applicationLog".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<String>,
    /// List of collection types to filter by. Allowed item values (enum):
    /// `applicationLog`, `flatFileLog`, `unifiedLogging`, `windowsEventLog`.
    /// Example: "applicationLog".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_types: Option<String>,
    /// State.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<bool>,
    /// Include enabled, disabled or both. Example: "True,False".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    /// Free-text filter by rule description (supports multiple).
    #[serde(
        rename = "description__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub description_contains: Option<String>,
    /// Collection path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_path: Option<String>,
    /// List of collection paths to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_paths: Option<String>,
    /// Free-text filter by rule collection path (supports multiple).
    #[serde(
        rename = "collectionPath__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_path_contains: Option<String>,
    /// Exact match filter by collection tags (supports multiple).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_tags: Option<String>,
    /// Free-text filter by rule collection tags (supports multiple).
    #[serde(
        rename = "collectionTags__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_tags_contains: Option<String>,
    /// Exact match filter by excluded tags (supports multiple).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_tags: Option<String>,
    /// Free-text filter by rule exclued tags (supports multiple).
    #[serde(
        rename = "excludedTags__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub excluded_tags_contains: Option<String>,
    /// Free-text filter by rule exclued logs (supports multiple).
    #[serde(
        rename = "excludedLogs__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub excluded_logs_contains: Option<String>,
    /// Free-text filter by rule parser (supports multiple).
    #[serde(rename = "parser__contains", skip_serializing_if = "Option::is_none")]
    pub parser_contains: Option<String>,
    /// Free-text filter by rule author (supports multiple).
    #[serde(
        rename = "createdBy__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by_contains: Option<String>,
    /// Free-text filter by rule last updater (supports multiple).
    #[serde(
        rename = "updatedBy__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_by_contains: Option<String>,
    /// Scope path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_path: Option<String>,
    /// List of scope paths to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_paths: Option<String>,
    /// Free-text filter by scope path (supports multiple).
    #[serde(
        rename = "scopePath__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub scope_path_contains: Option<String>,
    /// Date range for creation time (format:
    /// <from_timestamp>-<to_timestamp>, inclusive). Example:
    /// "1514978890136-1514978650130".
    #[serde(
        rename = "createdAt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_between: Option<String>,
    /// Date range for last update time (format:
    /// <from_timestamp>-<to_timestamp>, inclusive). Example:
    /// "1514978890136-1514978650130".
    #[serde(
        rename = "updatedAt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at_between: Option<String>,
    /// List of IDs to exclude. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_ids: Option<String>,
    /// Free-text filter by rule parser (supports multiple).
    #[serde(
        rename = "logPrefix__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_prefix_contains: Option<String>,
    /// Collection path or channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_path_or_channel: Option<String>,
    /// List of collection paths to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_paths_or_channel: Option<String>,
    /// Free-text filter by rule collection path or channel (supports multiple).
    #[serde(
        rename = "collectionPathOrChannel__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_path_or_channel_contains: Option<String>,
    /// Exact match filter by event Ids (supports multiple).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ids: Option<String>,
    /// Free-text filter by rule eventIds (supports multiple).
    #[serde(rename = "eventIds__contains", skip_serializing_if = "Option::is_none")]
    pub event_ids_contains: Option<String>,
    /// Free-text filter by rule exclued ids (supports multiple).
    #[serde(
        rename = "welExcludedIds__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub wel_excluded_ids_contains: Option<String>,
    /// Return rules from parent scope levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
}

impl LogCollectionRulesQuery {
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// If true, only the total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values (enum): `id`,
    /// `createdAt`, `createdBy`, `updatedAt`, `updatedBy`, `scopeId`,
    /// `scopeLevel`, `name`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values (enum): `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// List of site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
    /// List of group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// List of IDs to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join(v));
        self
    }
    /// Scope. Allowed values (enum): `group`, `site`, `account`, `tenant`.
    pub fn scope(mut self, v: impl Into<String>) -> Self {
        self.scope = Some(v.into());
        self
    }
    /// List of scopes to filter by. Allowed item values (enum): `group`,
    /// `site`, `account`, `tenant`.
    pub fn scopes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scopes = Some(join(v));
        self
    }
    /// Name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// List of names to filter by.
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(join(v));
        self
    }
    /// Free-text filter by rule name (supports multiple).
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join(v));
        self
    }
    /// Agent type. Allowed values (enum): `windows`, `linux`, `macos`, `k8s`,
    /// `k8sAndContainers`.
    pub fn agent_type(mut self, v: impl Into<String>) -> Self {
        self.agent_type = Some(v.into());
        self
    }
    /// List of agent types to filter by. Allowed item values (enum): `windows`,
    /// `linux`, `macos`, `k8s`, `k8sAndContainers`.
    pub fn agent_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_types = Some(join(v));
        self
    }
    /// Configuration type. Allowed values (enum): `collectionPath`, `predicate`.
    pub fn configuration_type(mut self, v: impl Into<String>) -> Self {
        self.configuration_type = Some(v.into());
        self
    }
    /// List of configuration types to filter by. Allowed item values (enum):
    /// `collectionPath`, `predicate`.
    pub fn configuration_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.configuration_types = Some(join(v));
        self
    }
    /// Collection type. Allowed values (enum): `applicationLog`, `flatFileLog`,
    /// `unifiedLogging`, `windowsEventLog`.
    pub fn collection_type(mut self, v: impl Into<String>) -> Self {
        self.collection_type = Some(v.into());
        self
    }
    /// List of collection types to filter by. Allowed item values (enum):
    /// `applicationLog`, `flatFileLog`, `unifiedLogging`, `windowsEventLog`.
    pub fn collection_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_types = Some(join(v));
        self
    }
    /// State.
    pub fn state(mut self, v: bool) -> Self {
        self.state = Some(v);
        self
    }
    /// Include enabled, disabled or both. Example: "True,False".
    pub fn enabled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.enabled = Some(join(v));
        self
    }
    /// Free-text filter by rule description (supports multiple).
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join(v));
        self
    }
    /// Collection path.
    pub fn collection_path(mut self, v: impl Into<String>) -> Self {
        self.collection_path = Some(v.into());
        self
    }
    /// List of collection paths to filter by.
    pub fn collection_paths<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_paths = Some(join(v));
        self
    }
    /// Free-text filter by rule collection path (supports multiple).
    pub fn collection_path_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_path_contains = Some(join(v));
        self
    }
    /// Exact match filter by collection tags (supports multiple).
    pub fn collection_tags<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_tags = Some(join(v));
        self
    }
    /// Free-text filter by rule collection tags (supports multiple).
    pub fn collection_tags_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_tags_contains = Some(join(v));
        self
    }
    /// Exact match filter by excluded tags (supports multiple).
    pub fn excluded_tags<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.excluded_tags = Some(join(v));
        self
    }
    /// Free-text filter by rule exclued tags (supports multiple).
    pub fn excluded_tags_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.excluded_tags_contains = Some(join(v));
        self
    }
    /// Free-text filter by rule exclued logs (supports multiple).
    pub fn excluded_logs_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.excluded_logs_contains = Some(join(v));
        self
    }
    /// Free-text filter by rule parser (supports multiple).
    pub fn parser_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.parser_contains = Some(join(v));
        self
    }
    /// Free-text filter by rule author (supports multiple).
    pub fn created_by_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.created_by_contains = Some(join(v));
        self
    }
    /// Free-text filter by rule last updater (supports multiple).
    pub fn updated_by_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.updated_by_contains = Some(join(v));
        self
    }
    /// Scope path.
    pub fn scope_path(mut self, v: impl Into<String>) -> Self {
        self.scope_path = Some(v.into());
        self
    }
    /// List of scope paths to filter by.
    pub fn scope_paths<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_paths = Some(join(v));
        self
    }
    /// Free-text filter by scope path (supports multiple).
    pub fn scope_path_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_path_contains = Some(join(v));
        self
    }
    /// Date range for creation time (format:
    /// <from_timestamp>-<to_timestamp>, inclusive).
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// Date range for last update time (format:
    /// <from_timestamp>-<to_timestamp>, inclusive).
    pub fn updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.updated_at_between = Some(v.into());
        self
    }
    /// List of IDs to exclude.
    pub fn excluded_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.excluded_ids = Some(join(v));
        self
    }
    /// Free-text filter by rule parser (supports multiple).
    pub fn log_prefix_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.log_prefix_contains = Some(join(v));
        self
    }
    /// Collection path or channel.
    pub fn collection_path_or_channel(mut self, v: impl Into<String>) -> Self {
        self.collection_path_or_channel = Some(v.into());
        self
    }
    /// List of collection paths to filter by.
    pub fn collection_paths_or_channel<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_paths_or_channel = Some(join(v));
        self
    }
    /// Free-text filter by rule collection path or channel (supports multiple).
    pub fn collection_path_or_channel_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_path_or_channel_contains = Some(join(v));
        self
    }
    /// Exact match filter by event Ids (supports multiple).
    pub fn event_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.event_ids = Some(join(v));
        self
    }
    /// Free-text filter by rule eventIds (supports multiple).
    pub fn event_ids_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.event_ids_contains = Some(join(v));
        self
    }
    /// Free-text filter by rule exclued ids (supports multiple).
    pub fn wel_excluded_ids_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.wel_excluded_ids_contains = Some(join(v));
        self
    }
    /// Return rules from parent scope levels.
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// Return filters from children scope levels.
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
}

/// Query params for `GET /web/api/v2.1/log-collection/rules/export` and
/// `GET /web/api/v2.1/log-collection/rules/export/{agent_type}`.
///
/// Same filter set as [`LogCollectionRulesQuery`] minus the pagination and
/// sort params. All fields are optional; array params are serialized
/// comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionRulesExportQuery {
    /// List of account IDs to filter by. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs to filter by. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of group IDs to filter by. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// List of IDs to filter by. Example:
    /// "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Scope. Allowed values (enum): `group`, `site`, `account`, `tenant`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// List of scopes to filter by. Allowed item values (enum): `group`,
    /// `site`, `account`, `tenant`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    /// Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of names to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Free-text filter by rule name (supports multiple).
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Agent type. Allowed values (enum): `windows`, `linux`, `macos`, `k8s`,
    /// `k8sAndContainers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<String>,
    /// List of agent types to filter by. Allowed item values (enum): `windows`,
    /// `linux`, `macos`, `k8s`, `k8sAndContainers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_types: Option<String>,
    /// Configuration type. Allowed values (enum): `collectionPath`, `predicate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    /// List of configuration types to filter by. Allowed item values (enum):
    /// `collectionPath`, `predicate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_types: Option<String>,
    /// Collection type. Allowed values (enum): `applicationLog`, `flatFileLog`,
    /// `unifiedLogging`, `windowsEventLog`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<String>,
    /// List of collection types to filter by. Allowed item values (enum):
    /// `applicationLog`, `flatFileLog`, `unifiedLogging`, `windowsEventLog`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_types: Option<String>,
    /// State.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<bool>,
    /// Include enabled, disabled or both. Example: "True,False".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    /// Free-text filter by rule description (supports multiple).
    #[serde(
        rename = "description__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub description_contains: Option<String>,
    /// Collection path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_path: Option<String>,
    /// List of collection paths to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_paths: Option<String>,
    /// Free-text filter by rule collection path (supports multiple).
    #[serde(
        rename = "collectionPath__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_path_contains: Option<String>,
    /// Exact match filter by collection tags (supports multiple).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_tags: Option<String>,
    /// Free-text filter by rule collection tags (supports multiple).
    #[serde(
        rename = "collectionTags__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_tags_contains: Option<String>,
    /// Exact match filter by excluded tags (supports multiple).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_tags: Option<String>,
    /// Free-text filter by rule exclued tags (supports multiple).
    #[serde(
        rename = "excludedTags__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub excluded_tags_contains: Option<String>,
    /// Free-text filter by rule exclued logs (supports multiple).
    #[serde(
        rename = "excludedLogs__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub excluded_logs_contains: Option<String>,
    /// Free-text filter by rule parser (supports multiple).
    #[serde(rename = "parser__contains", skip_serializing_if = "Option::is_none")]
    pub parser_contains: Option<String>,
    /// Free-text filter by rule author (supports multiple).
    #[serde(
        rename = "createdBy__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by_contains: Option<String>,
    /// Free-text filter by rule last updater (supports multiple).
    #[serde(
        rename = "updatedBy__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_by_contains: Option<String>,
    /// Scope path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_path: Option<String>,
    /// List of scope paths to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_paths: Option<String>,
    /// Free-text filter by scope path (supports multiple).
    #[serde(
        rename = "scopePath__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub scope_path_contains: Option<String>,
    /// Date range for creation time (format:
    /// <from_timestamp>-<to_timestamp>, inclusive).
    #[serde(
        rename = "createdAt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_between: Option<String>,
    /// Date range for last update time (format:
    /// <from_timestamp>-<to_timestamp>, inclusive).
    #[serde(
        rename = "updatedAt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at_between: Option<String>,
    /// List of IDs to exclude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_ids: Option<String>,
    /// Free-text filter by rule parser (supports multiple).
    #[serde(
        rename = "logPrefix__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_prefix_contains: Option<String>,
    /// Collection path or channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_path_or_channel: Option<String>,
    /// List of collection paths to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_paths_or_channel: Option<String>,
    /// Free-text filter by rule collection path or channel (supports multiple).
    #[serde(
        rename = "collectionPathOrChannel__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_path_or_channel_contains: Option<String>,
    /// Exact match filter by event Ids (supports multiple).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ids: Option<String>,
    /// Free-text filter by rule eventIds (supports multiple).
    #[serde(rename = "eventIds__contains", skip_serializing_if = "Option::is_none")]
    pub event_ids_contains: Option<String>,
    /// Free-text filter by rule exclued ids (supports multiple).
    #[serde(
        rename = "welExcludedIds__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub wel_excluded_ids_contains: Option<String>,
    /// Return rules from parent scope levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
}

impl LogCollectionRulesExportQuery {
    /// List of account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// List of site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
    /// List of group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// List of IDs to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join(v));
        self
    }
    /// Scope. Allowed values (enum): `group`, `site`, `account`, `tenant`.
    pub fn scope(mut self, v: impl Into<String>) -> Self {
        self.scope = Some(v.into());
        self
    }
    /// List of scopes to filter by.
    pub fn scopes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scopes = Some(join(v));
        self
    }
    /// Name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// List of names to filter by.
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(join(v));
        self
    }
    /// Free-text filter by rule name (supports multiple).
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join(v));
        self
    }
    /// Agent type. Allowed values (enum): `windows`, `linux`, `macos`, `k8s`,
    /// `k8sAndContainers`.
    pub fn agent_type(mut self, v: impl Into<String>) -> Self {
        self.agent_type = Some(v.into());
        self
    }
    /// List of agent types to filter by.
    pub fn agent_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_types = Some(join(v));
        self
    }
    /// Configuration type. Allowed values (enum): `collectionPath`, `predicate`.
    pub fn configuration_type(mut self, v: impl Into<String>) -> Self {
        self.configuration_type = Some(v.into());
        self
    }
    /// List of configuration types to filter by.
    pub fn configuration_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.configuration_types = Some(join(v));
        self
    }
    /// Collection type. Allowed values (enum): `applicationLog`, `flatFileLog`,
    /// `unifiedLogging`, `windowsEventLog`.
    pub fn collection_type(mut self, v: impl Into<String>) -> Self {
        self.collection_type = Some(v.into());
        self
    }
    /// List of collection types to filter by.
    pub fn collection_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_types = Some(join(v));
        self
    }
    /// State.
    pub fn state(mut self, v: bool) -> Self {
        self.state = Some(v);
        self
    }
    /// Include enabled, disabled or both.
    pub fn enabled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.enabled = Some(join(v));
        self
    }
    /// Free-text filter by rule description (supports multiple).
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join(v));
        self
    }
    /// Collection path.
    pub fn collection_path(mut self, v: impl Into<String>) -> Self {
        self.collection_path = Some(v.into());
        self
    }
    /// List of collection paths to filter by.
    pub fn collection_paths<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_paths = Some(join(v));
        self
    }
    /// Free-text filter by rule collection path (supports multiple).
    pub fn collection_path_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_path_contains = Some(join(v));
        self
    }
    /// Exact match filter by collection tags (supports multiple).
    pub fn collection_tags<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_tags = Some(join(v));
        self
    }
    /// Free-text filter by rule collection tags (supports multiple).
    pub fn collection_tags_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_tags_contains = Some(join(v));
        self
    }
    /// Exact match filter by excluded tags (supports multiple).
    pub fn excluded_tags<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.excluded_tags = Some(join(v));
        self
    }
    /// Free-text filter by rule exclued tags (supports multiple).
    pub fn excluded_tags_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.excluded_tags_contains = Some(join(v));
        self
    }
    /// Free-text filter by rule exclued logs (supports multiple).
    pub fn excluded_logs_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.excluded_logs_contains = Some(join(v));
        self
    }
    /// Free-text filter by rule parser (supports multiple).
    pub fn parser_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.parser_contains = Some(join(v));
        self
    }
    /// Free-text filter by rule author (supports multiple).
    pub fn created_by_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.created_by_contains = Some(join(v));
        self
    }
    /// Free-text filter by rule last updater (supports multiple).
    pub fn updated_by_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.updated_by_contains = Some(join(v));
        self
    }
    /// Scope path.
    pub fn scope_path(mut self, v: impl Into<String>) -> Self {
        self.scope_path = Some(v.into());
        self
    }
    /// List of scope paths to filter by.
    pub fn scope_paths<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_paths = Some(join(v));
        self
    }
    /// Free-text filter by scope path (supports multiple).
    pub fn scope_path_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_path_contains = Some(join(v));
        self
    }
    /// Date range for creation time.
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// Date range for last update time.
    pub fn updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.updated_at_between = Some(v.into());
        self
    }
    /// List of IDs to exclude.
    pub fn excluded_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.excluded_ids = Some(join(v));
        self
    }
    /// Free-text filter by rule parser (supports multiple).
    pub fn log_prefix_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.log_prefix_contains = Some(join(v));
        self
    }
    /// Collection path or channel.
    pub fn collection_path_or_channel(mut self, v: impl Into<String>) -> Self {
        self.collection_path_or_channel = Some(v.into());
        self
    }
    /// List of collection paths to filter by.
    pub fn collection_paths_or_channel<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_paths_or_channel = Some(join(v));
        self
    }
    /// Free-text filter by rule collection path or channel (supports multiple).
    pub fn collection_path_or_channel_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_path_or_channel_contains = Some(join(v));
        self
    }
    /// Exact match filter by event Ids (supports multiple).
    pub fn event_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.event_ids = Some(join(v));
        self
    }
    /// Free-text filter by rule eventIds (supports multiple).
    pub fn event_ids_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.event_ids_contains = Some(join(v));
        self
    }
    /// Free-text filter by rule exclued ids (supports multiple).
    pub fn wel_excluded_ids_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.wel_excluded_ids_contains = Some(join(v));
        self
    }
    /// Return rules from parent scope levels.
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// Return filters from children scope levels.
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
}

// ---------------------------------------------------------------------------
// Body structs
// ---------------------------------------------------------------------------

/// Request body for `DELETE /web/api/v2.1/log-collection/rules`.
///
/// `filter` is required and is a free-form filter object selecting the rules to
/// delete; kept as `serde_json::Value` to mirror the spec's large, optional
/// filter schema verbatim.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteRulesBody {
    /// Filter selecting the rules to delete. Required.
    pub filter: serde_json::Value,
}

/// Request body for `POST /web/api/v2.1/log-collection/rules` (create) and
/// `PUT /web/api/v2.1/log-collection/rules/{rule_id}` (update).
///
/// The spec body is the deeply nested `LogCollectionRulesPostSchema`; the
/// agent-type-specific param objects are kept as `serde_json::Value` to allow
/// freeform construction. Required top-level fields: `agentType`, `name`,
/// `scopeLevel`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleBody {
    /// Scope id. Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i64>,
    /// Scope level. Required. Allowed values (enum): `group`, `site`,
    /// `account`, `global`.
    pub scope_level: String,
    /// Name. Required.
    pub name: String,
    /// Description. Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Agent type. Required. Allowed values (enum): `windows`, `linux`,
    /// `macos`, `k8s`.
    pub agent_type: String,
    /// Linux params (freeform; required field `collectionPath`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_params: Option<serde_json::Value>,
    /// macOS params (freeform; required fields `collectionPath`,
    /// `collectionType`, `configurationType`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macos_params: Option<serde_json::Value>,
    /// Windows params (freeform; required fields `collectionPathOrChannel`,
    /// `collectionType`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_params: Option<serde_json::Value>,
    /// K8s params (freeform; required field `collectionTags`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_params: Option<serde_json::Value>,
}

/// Request body for `POST /web/api/v2.1/log-collection/rules/activation`.
///
/// `data` is required and carries the `activate` flag; `filter` is an optional
/// free-form filter object selecting the rules whose activation state changes.
#[derive(Debug, Clone, Serialize)]
pub struct RulesActivationBody {
    /// Activation data. Required.
    pub data: RulesActivationData,
    /// Filter selecting the rules to affect. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

/// The `data` object of [`RulesActivationBody`].
#[derive(Debug, Clone, Serialize)]
pub struct RulesActivationData {
    /// Activate (`true`) or deactivate (`false`) the selected rules. Required.
    pub activate: bool,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Join an iterator of string-like values into a comma-separated string, as the
/// API expects for array query params.
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

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl LogCollectionService<'_> {
    /// `GET /web/api/v2.1/log-collection/agent-type-count` — Get Agent type
    /// count.
    ///
    /// Get the total number of log collection rules per agent type.
    pub async fn agent_type_count(
        &self,
        query: &AgentTypeCountQuery,
    ) -> Result<Response<LogCollectionAgentTypeCount>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/log-collection/agent-type-count", q)
            .await?)
    }

    /// `DELETE /web/api/v2.1/log-collection/rules` — Delete log collection
    /// rules.
    pub async fn delete_rules(
        &self,
        body: &DeleteRulesBody,
    ) -> Result<Response<LogCollectionAffectedResultIds>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteRulesBody, Response<LogCollectionAffectedResultIds>>(
                Method::DELETE,
                "/web/api/v2.1/log-collection/rules",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/log-collection/rules` — Get Log Collection rules.
    pub async fn list_rules(
        &self,
        query: &LogCollectionRulesQuery,
    ) -> Result<Paginated<LogCollectionRule>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/log-collection/rules", q)
            .await?)
    }

    /// `POST /web/api/v2.1/log-collection/rules` — Create a log collection rule.
    pub async fn create_rule(
        &self,
        body: &RuleBody,
    ) -> Result<Response<LogCollectionAffectedResultId>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/log-collection/rules", body)
            .await?)
    }

    /// `POST /web/api/v2.1/log-collection/rules/activation` — Change activation
    /// status of log collection rules.
    pub async fn rules_activation(
        &self,
        body: &RulesActivationBody,
    ) -> Result<Response<LogCollectionAffectedResultIds>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/log-collection/rules/activation", body)
            .await?)
    }

    /// `GET /web/api/v2.1/log-collection/rules/export` — Export log collection
    /// rules.
    ///
    /// Get a CSV file with all log collection rules according to the passed
    /// filters. The CSV payload is returned wrapped in the standard response
    /// envelope as a raw JSON value.
    pub async fn export_rules(
        &self,
        query: &LogCollectionRulesExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/log-collection/rules/export", q)
            .await?)
    }

    /// `GET /web/api/v2.1/log-collection/rules/export/{agent_type}` — Export log
    /// collection rules.
    ///
    /// Get a CSV file with all log collection rules according to the passed
    /// filters. The CSV payload is returned wrapped in the standard response
    /// envelope as a raw JSON value.
    ///
    /// `agent_type`: Agent type.
    pub async fn export_rules_by_agent_type(
        &self,
        agent_type: impl Into<String>,
        query: &LogCollectionRulesExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/log-collection/rules/export/{}",
            agent_type.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `GET /web/api/v2.1/log-collection/rules/{agent_type}` — Get Log
    /// Collection rules by agent type.
    ///
    /// `agent_type`: Agent type.
    pub async fn list_rules_by_agent_type(
        &self,
        agent_type: impl Into<String>,
        query: &LogCollectionRulesQuery,
    ) -> Result<Paginated<LogCollectionRule>, Error> {
        let path = format!(
            "/web/api/v2.1/log-collection/rules/{}",
            agent_type.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `PUT /web/api/v2.1/log-collection/rules/{rule_id}` — Update a log
    /// collection rule.
    ///
    /// `rule_id`: Rule id.
    pub async fn update_rule(
        &self,
        rule_id: impl Into<String>,
        body: &RuleBody,
    ) -> Result<Response<LogCollectionAffectedResultId>, Error> {
        let path = format!("/web/api/v2.1/log-collection/rules/{}", rule_id.into());
        Ok(self
            .client
            .http()
            .request_json::<RuleBody, Response<LogCollectionAffectedResultId>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }
}
