//! Entity models for the `Threat Intelligence` tag.
//!
//! Field optionality follows the spec: a field is bare `T` only when it is in
//! the schema `required` array **and** not `x-nullable`; otherwise `Option<T>`
//! (the "default null" behaviour). Enum-valued fields are kept as `String` for
//! forward-compatibility; allowed values are documented inline.

use serde::Deserialize;

/// A Threat Intelligence indicator of compromise (IOC).
///
/// Returned by `GET /web/api/v2.1/threat-intelligence/iocs` and
/// `POST /web/api/v2.1/threat-intelligence/iocs[/stix]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatIntelligenceIndicator {
    /// The source of the identified Threat Intelligence indicator.
    /// Required.
    pub source: String,
    /// The type of the Threat Intelligence indicator.
    /// Required. Allowed values: `DNS`, `IPV4`, `IPV6`, `MD5`, `SHA1`,
    /// `SHA256`, `URL`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The value of the Threat Intelligence indicator. Required.
    pub value: String,
    /// External reference associated with the Threat Intelligence indicator.
    /// Optional/nullable.
    #[serde(default)]
    pub reference: Option<Vec<Option<String>>>,
    /// Description of the Threat Intelligence indicator. Optional/nullable.
    #[serde(default)]
    pub description: Option<String>,
    /// The date from which the indicator will no longer be monitored
    /// (date-time). Optional/nullable.
    #[serde(default)]
    pub valid_until: Option<String>,
    /// The comparison method used by SentinelOne to trigger the event.
    /// Optional/nullable. Allowed values: `EQUALS`.
    #[serde(default)]
    pub method: Option<String>,
    /// The Account id under which the Site is defined; `null` if it is Global
    /// or Account. Optional/nullable.
    #[serde(default)]
    pub parent_scope_id: Option<String>,
    /// The time at which the indicator was last updated in SentinelOne DB
    /// (date-time). Optional/nullable.
    #[serde(default)]
    pub updated_at: Option<String>,
    /// The detection pattern for this Indicator (expressed as a STIX Pattern).
    /// Optional/nullable.
    #[serde(default)]
    pub pattern: Option<String>,
    /// Intrusion sets. Optional/nullable.
    #[serde(default)]
    pub intrusion_sets: Option<Vec<String>>,
    /// Campaign names. Optional/nullable.
    #[serde(default)]
    pub campaign_names: Option<Vec<String>>,
    /// The time at which the Threat Intelligence indicator was uploaded to
    /// SentinelOne DB (date-time). Optional/nullable.
    #[serde(default)]
    pub upload_time: Option<String>,
    /// The relative level of risk associated with the Threat Intelligence
    /// indicator. An integer between 0 and 100, inclusive. Optional/nullable.
    #[serde(default)]
    pub original_risk_score: Option<i64>,
    /// Unique Id of the Threat Intelligence indicator. Optional/nullable.
    #[serde(default)]
    pub uuid: Option<String>,
    /// The Group/Site/Account id depending on the Scope; `null` if it is
    /// Global. Optional/nullable.
    #[serde(default)]
    pub scope_id: Option<String>,
    /// Malware names. Optional/nullable.
    #[serde(default)]
    pub malware_names: Option<Vec<String>>,
    /// The time at which the Threat Intelligence indicator was originally
    /// created, as indicated by the TI source (date-time). Optional/nullable.
    #[serde(default)]
    pub creation_time: Option<String>,
    /// Characterize the pattern language that the indicator pattern is
    /// expressed in. Optional/nullable.
    #[serde(default)]
    pub pattern_type: Option<String>,
    /// Mitre tactic. Optional/nullable. Present only on the GET response.
    #[serde(default)]
    pub mitre_tactic: Option<Vec<String>>,
    /// Threat actors. Optional/nullable.
    #[serde(default)]
    pub threat_actors: Option<Vec<String>>,
    /// Unique ID of the uploaded Threat Intelligence indicators batch.
    /// Optional/nullable.
    #[serde(default)]
    pub batch_id: Option<String>,
    /// Labels. Optional/nullable.
    #[serde(default)]
    pub labels: Option<Vec<String>>,
    /// The categories of the Threat Intelligence indicator, e.g. the malware
    /// type associated with the IOC. Optional/nullable.
    #[serde(default)]
    pub category: Option<Vec<Option<String>>>,
    /// The metadata of the Threat Intelligence indicator. Optional/nullable.
    #[serde(default)]
    pub metadata: Option<String>,
    /// Scope of the IoC. Optional/nullable. Allowed values: `global`, `site`,
    /// `account`, `group`.
    #[serde(default)]
    pub scope: Option<String>,
    /// The potential impact of the Threat Intelligence indicator. Designed to
    /// work based on OCSF format for scores 0-7. Optional/nullable.
    #[serde(default)]
    pub severity: Option<i64>,
    /// The unique identifier of the indicator as provided by the Threat
    /// Intelligence source. Optional/nullable.
    #[serde(default)]
    pub external_id: Option<String>,
    /// Threat Intelligence indicator name. Optional/nullable.
    #[serde(default)]
    pub name: Option<String>,
    /// Threat actor types. Optional/nullable.
    #[serde(default)]
    pub threat_actor_types: Option<Vec<String>>,
    /// The user that uploaded the Threat Intelligence indicator.
    /// Optional/nullable.
    #[serde(default)]
    pub creator: Option<String>,
}

/// Threat Intelligence user config.
///
/// Returned by `GET` / `POST` `/web/api/v2.1/threat-intelligence/user-config`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatIntelligenceUserConfig {
    /// The time at which the user config was created in SentinelOne DB
    /// (date-time). Required.
    pub created_at: String,
    /// The time at which the user config was last updated in SentinelOne DB
    /// (date-time). Required.
    pub updated_at: String,
    /// The flag to disable Threat Intelligence RetroHunt feature.
    /// Optional/nullable.
    #[serde(default)]
    pub disable_rh: Option<bool>,
    /// The fields to be excluded that would cause a threat to be created.
    /// Optional/nullable.
    #[serde(default)]
    pub threat_exclude_fields: Option<Vec<Option<String>>>,
    /// User defined description of the user config. Optional/nullable.
    #[serde(default)]
    pub description: Option<String>,
    /// The minimum score for a threat to be created. Optional/nullable.
    #[serde(default)]
    pub threat_min_score: Option<i64>,
    /// The group/site/account id depending on the `scopeLevel`.
    /// Optional/nullable.
    #[serde(default)]
    pub scope_id: Option<String>,
    /// An IOC value to be excluded from Threat Intelligence Indicator and
    /// consequently Threat creation. Optional/nullable.
    #[serde(default)]
    pub exclude_tii: Option<Vec<Option<String>>>,
    /// The flag to enable XDR matching feature. Optional/nullable.
    #[serde(default)]
    pub enable_xdr_matching: Option<bool>,
    /// The flag to disable Threat Intelligence Indicator based Threat creation
    /// for the entire account. Optional/nullable.
    #[serde(default)]
    pub disable_threat: Option<bool>,
    /// Scope level of the user config. Optional/nullable. Allowed values:
    /// `global`, `site`, `account`, `group`.
    #[serde(default)]
    pub scope_level: Option<String>,
}

/// Number of entities affected by a delete operation.
///
/// `data` payload of the `DELETE` IOC and user-config endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatIntelligenceAffected {
    /// Number of entities affected by the requested operation.
    /// Optional/nullable.
    #[serde(default)]
    pub affected: Option<i64>,
}
