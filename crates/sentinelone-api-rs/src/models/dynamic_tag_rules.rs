//! Models for the `Dynamic tag rules` tag.
//!
//! API for managing dynamic tag rules.

use serde::Deserialize;

/// A dynamic tag rule.
///
/// Source schema: `v2_1.inventory.tags.rules.schemas_TagRuleSchema` (also the
/// `data` item of the list response). The schema marks only `conditions` and
/// `name` as required; everything else is optional/nullable and therefore
/// `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagRule {
    /// The conditions of the tag rule. **Required.**
    pub conditions: TagRuleConditions,
    /// The name of the tag rule. **Required.**
    pub name: String,
    /// The site ID of the tag rule. Optional.
    pub site_id: Option<String>,
    /// The management ID of the tag rule. Optional.
    pub mgmt_id: Option<String>,
    /// The list of excluded asset IDs. Optional.
    pub excluded_assets: Option<Vec<String>>,
    /// The scope of the tag rule. Optional.
    pub scopes: Option<TagRuleScopeDetail>,
    /// The ID of the user who last updated the tag rule. Optional.
    pub updated_by_id: Option<String>,
    /// The ID of the user who created the tag rule. Optional.
    pub created_by_id: Option<String>,
    /// The description of the tag rule. Optional/nullable (default `""`).
    pub description: Option<String>,
    /// The list of tags associated with the rule. Optional.
    pub tags: Option<Vec<TagRuleTagDetail>>,
    /// The date and time (ISO-8601) when the tag rule was created. Optional.
    pub created_at: Option<String>,
    /// The ID of the tag rule. Optional.
    pub id: Option<String>,
    /// The account ID of the tag rule. Optional.
    pub account_id: Option<String>,
    /// The status of the tag rule. Allowed values: `enabled`, `disabled`.
    /// Optional.
    pub status: Option<String>,
    /// The email of the user who created the tag rule. Optional.
    pub created_by_email: Option<String>,
    /// The date and time (ISO-8601) when the tag rule was last updated.
    /// Optional.
    pub updated_at: Option<String>,
    /// The email of the user who last updated the tag rule. Optional.
    pub updated_by_email: Option<String>,
}

/// The scope of a tag rule.
///
/// Source schema: `ScopeDetail`. No fields are required.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagRuleScopeDetail {
    /// The type of the scope. Allowed values: `account`, `site`. Optional.
    pub scope_type: Option<String>,
    /// Scope ids. Optional.
    pub scope_ids: Option<TagRuleScopeDetailScopeIds>,
}

/// The scope IDs of a tag rule scope.
///
/// Source schema: `ScopeDetailScopeIds`. No fields are required.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagRuleScopeDetailScopeIds {
    /// The list of account IDs. Optional.
    pub accounts: Option<Vec<String>>,
    /// The list of site IDs. Can be empty. When not empty, the account IDs must
    /// contain only one account ID. Optional.
    pub sites: Option<Vec<String>>,
}

/// The conditions of a tag rule.
///
/// Source schema: `Conditions`. No fields are required.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagRuleConditions {
    /// Properties (1-10 items). Optional.
    pub properties: Option<Vec<TagRuleProperty>>,
    /// The operand to apply to the asset properties. Allowed values: `and`,
    /// `or`. Optional.
    pub operand: Option<String>,
}

/// A single condition property within a tag rule.
///
/// Source schema: `Property`. Only `name` is required.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagRuleProperty {
    /// The name of the asset property. **Required.** Allowed values:
    /// `category`, `resourceType`, `assetName`, `ipAddress`, `os`, `osFamily`,
    /// `osVersion`, `osNameVersion`, `agentVersion`, `applicationName`.
    pub name: String,
    /// The list of values to compare the asset property value with (1-10
    /// items). Optional.
    pub values: Option<Vec<String>>,
    /// The operand to compare the asset property value with. Allowed values:
    /// `equals`, `notEquals`, `contains`, `notContains`, `startsWith`,
    /// `endsWith`. Optional.
    pub operand: Option<String>,
}

/// A tag associated with a tag rule.
///
/// Source schema: `TagDetail`. No fields are required.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagRuleTagDetail {
    /// The value of the tag. Optional.
    pub value: Option<String>,
    /// The scope ID of the tag. Optional.
    pub scope_id: Option<String>,
    /// The ID of the tag. Optional.
    pub id: Option<String>,
    /// The key of the tag. Optional.
    pub key: Option<String>,
    /// The scope level of the tag. Optional.
    pub scope_level: Option<String>,
}

/// Response payload for deleting tag rules.
///
/// Source schema: `DeleteTagRuleResponse` (the `data` of
/// `v2_1.inventory.tags.rules.schemas_DeleteTagRuleResponseSchema`). No fields
/// are required.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTagRuleResponse {
    /// Number of affected tag rules. Optional.
    pub affected: Option<i64>,
}

/// An asset inventory entry matched by a tag rule (the `data` item of the
/// `.../rules/test` response).
///
/// Source schema: `InventoryResponse`. No fields are required, so every field
/// is `Option<T>`. Large/freeform sub-objects and cross-tag references are
/// modeled as [`serde_json::Value`] for forward compatibility.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryResponse {
    /// Id. Optional.
    pub id: Option<String>,
    /// Name. Optional.
    pub name: Option<String>,
    /// Category. Optional.
    pub category: Option<String>,
    /// Sub category. Optional.
    pub sub_category: Option<String>,
    /// Resource type. Optional.
    pub resource_type: Option<String>,
    /// Cpu. Optional.
    pub cpu: Option<String>,
    /// Manufacturer. Optional.
    pub manufacturer: Option<String>,
    /// Serial number. Optional.
    pub serial_number: Option<String>,
    /// Region. Optional.
    pub region: Option<String>,
    /// Network name. Optional.
    pub network_name: Option<String>,
    /// Network names. Optional.
    pub network_names: Option<Vec<String>>,
    /// Asset status. Optional.
    pub asset_status: Option<String>,
    /// Asset criticality. Optional.
    pub asset_criticality: Option<String>,
    /// Asset environment. Optional.
    pub asset_environment: Option<String>,
    /// Asset contact email. Optional.
    pub asset_contact_email: Option<String>,
    /// Infection status. Optional.
    pub infection_status: Option<String>,
    /// Device review. Optional.
    pub device_review: Option<String>,
    /// Device review log. Optional.
    pub device_review_log: Option<Vec<serde_json::Value>>,
    /// Previous device function. Optional.
    pub previous_device_function: Option<String>,
    /// Previous os type. Optional.
    pub previous_os_type: Option<String>,
    /// Previous os version. Optional.
    pub previous_os_version: Option<String>,
    /// Legacy identity policy name. Optional.
    pub legacy_identity_policy_name: Option<String>,
    /// Missing coverage. Optional.
    pub missing_coverage: Option<Vec<String>>,
    /// Active coverage. Optional.
    pub active_coverage: Option<Vec<String>>,
    /// Surfaces. Optional.
    pub surfaces: Option<Vec<String>>,
    /// Risk factors. Optional.
    pub risk_factors: Option<Vec<String>>,
    /// Discovery methods. Optional.
    pub discovery_methods: Option<Vec<String>>,
    /// Tcp ports. Optional.
    pub tcp_ports: Option<Vec<String>>,
    /// Udp ports. Optional.
    pub udp_ports: Option<Vec<String>>,
    /// Tags. Optional. Freeform objects.
    pub tags: Option<Vec<serde_json::Value>>,
    /// Cloud tags. Optional. Freeform objects.
    pub cloud_tags: Option<Vec<serde_json::Value>>,
    /// Ranger tags. Optional.
    pub ranger_tags: Option<Vec<String>>,
    /// Notes. Optional. Cross-tag reference (`NotesResponse`).
    pub notes: Option<Vec<serde_json::Value>>,
    /// Alerts. Optional. Cross-tag reference (`AlertResponse`).
    pub alerts: Option<Vec<serde_json::Value>>,
    /// Alerts count. Optional. Cross-tag reference (`AlertResponse`).
    pub alerts_count: Option<Vec<serde_json::Value>>,
    /// Agent. Optional. Cross-tag reference (`AgentResponse`).
    pub agent: Option<serde_json::Value>,
    /// Identity. Optional. Cross-tag reference (`IdentityResponse`).
    pub identity: Option<serde_json::Value>,
    /// Source json. Optional. Freeform object.
    pub source_json: Option<serde_json::Value>,
    /// Id secondary. Optional.
    pub id_secondary: Option<Vec<String>>,
    /// Is ad connector. Optional.
    pub is_ad_connector: Option<bool>,
    /// Is dc server. Optional.
    pub is_dc_server: Option<bool>,
    /// Ads enabled. Optional.
    pub ads_enabled: Option<bool>,
    /// Epp unsupported unknown. Optional.
    pub epp_unsupported_unknown: Option<String>,
    /// First seen dt. Optional.
    pub first_seen_dt: Option<String>,
    /// Last active dt. Optional.
    pub last_active_dt: Option<String>,
    /// Last reboot dt. Optional.
    pub last_reboot_dt: Option<String>,
    /// Last update dt. Optional.
    pub last_update_dt: Option<String>,
    /// Created time. Optional.
    pub created_time: Option<String>,
    /// Detected from site. Optional.
    pub detected_from_site: Option<String>,
    /// Cloud resource id. Optional.
    pub cloud_resource_id: Option<String>,
    /// Cloud resource uid. Optional.
    pub cloud_resource_uid: Option<String>,
    /// Cloud provider account id. Optional.
    pub cloud_provider_account_id: Option<String>,
    /// Cloud provider account name. Optional.
    pub cloud_provider_account_name: Option<String>,
    /// Cloud provider organization. Optional.
    pub cloud_provider_organization: Option<String>,
    /// Cloud provider organization unit. Optional.
    pub cloud_provider_organization_unit: Option<String>,
    /// Cloud provider organization unit path. Optional.
    pub cloud_provider_organization_unit_path: Option<String>,
    /// Cloud provider project id. Optional.
    pub cloud_provider_project_id: Option<String>,
    /// Cloud provider resource group. Optional.
    pub cloud_provider_resource_group: Option<String>,
    /// Cloud provider subscription id. Optional.
    pub cloud_provider_subscription_id: Option<String>,
    /// Cloud provider url string. Optional.
    pub cloud_provider_url_string: Option<String>,
    /// S1 group id. Optional.
    pub s1_group_id: Option<String>,
    /// S1 group name. Optional.
    pub s1_group_name: Option<String>,
    /// S1 site id. Optional.
    pub s1_site_id: Option<String>,
    /// S1 site name. Optional.
    pub s1_site_name: Option<String>,
    /// S1 account id. Optional.
    pub s1_account_id: Option<String>,
    /// S1 account name. Optional.
    pub s1_account_name: Option<String>,
    /// S1 management id. Optional.
    pub s1_management_id: Option<i64>,
    /// S1 scope type. Optional.
    pub s1_scope_type: Option<i64>,
    /// S1 scope level. Optional.
    pub s1_scope_level: Option<String>,
    /// S1 scope id. Optional.
    pub s1_scope_id: Option<String>,
    /// S1 scope path. Optional.
    pub s1_scope_path: Option<String>,
    /// S1 updated at. Optional.
    pub s1_updated_at: Option<String>,
    /// S1 onboarded account id. Optional.
    pub s1_onboarded_account_id: Option<i64>,
    /// S1 onboarded account name. Optional.
    pub s1_onboarded_account_name: Option<String>,
    /// S1 onboarded site id. Optional.
    pub s1_onboarded_site_id: Option<i64>,
    /// S1 onboarded site name. Optional.
    pub s1_onboarded_site_name: Option<String>,
    /// S1 onboarded group id. Optional.
    pub s1_onboarded_group_id: Option<i64>,
    /// S1 onboarded group name. Optional.
    pub s1_onboarded_group_name: Option<String>,
    /// S1 onboarded scope id. Optional.
    pub s1_onboarded_scope_id: Option<i64>,
    /// S1 onboarded scope level. Optional.
    pub s1_onboarded_scope_level: Option<String>,
    /// S1 onboarded scope path. Optional.
    pub s1_onboarded_scope_path: Option<String>,
}
