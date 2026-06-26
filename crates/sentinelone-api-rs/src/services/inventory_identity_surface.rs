//! `Inventory Identity Surface` tag — identity surface asset inventory.
//!
//! Endpoints:
//! - `GET  /web/api/v2.1/xdr/assets/surface/identity`
//! - `POST /web/api/v2.1/xdr/assets/surface/identity/action`
//! - `POST /web/api/v2.1/xdr/assets/surface/identity/available-actions/with-status`
//! - `GET  /web/api/v2.1/xdr/assets/surface/identity/export`

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_identity_surface::*;
use crate::pagination::{Paginated, Response};

/// `Inventory Identity Surface` tag — query, filter, export and act on identity
/// surface inventory assets.
pub struct InventoryIdentitySurfaceService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Comma-join an iterator of string-like values into a single query value, as
/// the SentinelOne API expects for repeated/array query parameters.
fn join_csv<I, S>(values: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    values
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

/// Append a pre-serialized querystring to a path so it can be sent on a POST
/// (the HTTP client's `post` helper does not take a separate query argument;
/// `Url::join` parses the `?...` suffix as the query component).
fn path_with_query(path: &str, query: &impl Serialize) -> String {
    let qs = serde_urlencoded::to_string(query).unwrap_or_default();
    if qs.is_empty() {
        path.to_owned()
    } else {
        format!("{path}?{qs}")
    }
}

/// Query params shared by every `Inventory Identity Surface` endpoint.
///
/// The four endpoints expose the same filter set; the list (`GET .../identity`)
/// and export (`GET .../identity/export`) endpoints additionally accept the
/// paging / sort / format controls (`skip`, `sortBy`, `sortOrder`, `countOnly`,
/// `skipCount`, `limit`, `cursor`, `exportFormat`). Those are ignored by the two
/// `POST` endpoints.
///
/// Every field is optional. Array params are serialized comma-joined; set them
/// via the corresponding builder, which takes an iterator of string-like values.
///
/// Note: `export_format` is only meaningful on the export endpoint, where it is
/// required; set it via [`IdentitySurfaceQuery::export_format`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentitySurfaceQuery {
    /// Free-text filter by tag key (supports multiple values). Array of string.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Array of enum:
    /// `critical`, `high`, `medium`, `low`, `--`.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in). Array of enum: `Infected`,
    /// `Healthy`.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The Group Type. Array of integer (int32).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<String>,
    /// The missing coverage for the asset. Array of enum: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Array of string.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The Object Category. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_category: Option<String>,
    /// The cloud resource ID. Array of string.
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Array of string.
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys (not in). Array of string.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// Tag Keys exists. Array of string.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The region. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The User Principal Name. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Array of
    /// string.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Array of string.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). Array of enum:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Array of string.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,
    /// use `cursor`. Integer (int32). List/export endpoints only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The cloud provider account ID. Array of string.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Array of string.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The Email Address. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    /// On Premises SAM Account Name. Array of string.
    #[serde(rename = "onPremisesSamAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_sam_account_name_contains: Option<String>,
    /// The active coverage for the asset (not in). Array of enum: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// User and cloud tags (not in). Array of string.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in). Array of string.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in). Array of string.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The SAM Account Name. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sam_account_name: Option<String>,
    /// The cloud tags key value (not in). Array of string.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to. Array of enum: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The Forest Name. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forest: Option<String>,
    /// The missing coverage for the asset (not in). Array of enum: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). Array of enum: `Active`, `Inactive`.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The column to sort the results by. Enum: `s1GroupName`,
    /// `ntSecurityDescriptor`, `recycled`, `privileged`, `groupType`,
    /// `accountExpires`, `badPasswordTime`,
    /// `allowedToActOnBehalfOfOtherIdentity`. List/export endpoints only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// On Premises User Principal Name. Array of string.
    #[serde(rename = "onPremisesUserPrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_user_principal_name_contains: Option<String>,
    /// Whether the AD Entity is privileged or not. Array of boolean.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<String>,
    /// Whether the password never expires. Array of boolean.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_never_expire: Option<String>,
    /// The geographical area where cloud resources are hosted. Array of string.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The asset review (not in). Array of enum: `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, ``.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The cloud provider account name. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Whether the AD Entity is deleted or not. Array of boolean.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
    /// Asset Contact Email (not in). Array of string.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// The Email Address. Array of string.
    #[serde(rename = "mail__contains", skip_serializing_if = "Option::is_none")]
    pub mail_contains: Option<String>,
    /// The cloud tags key value. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// List of Account IDs to filter by. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// On Premises Distinguished Name filter. Array of string.
    #[serde(rename = "onPremisesDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_distinguished_name_contains: Option<String>,
    /// The AD Domain Name. Array of string.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in). Array of enum:
    /// `Access Control and Surveillance System`, `Access Point`, `AD Certificate`,
    /// `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`,
    /// `AD DNS Zone`, `AD Domain`.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The LDAP Common Name. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn: Option<String>,
    /// The LDAP Common Name. Array of string.
    #[serde(rename = "cn__contains", skip_serializing_if = "Option::is_none")]
    pub cn_contains: Option<String>,
    /// The LDAP Common Name (not in). Array of string.
    #[serde(rename = "cn__nin", skip_serializing_if = "Option::is_none")]
    pub cn_nin: Option<String>,
    /// On Premises Distinguished Name. Array of string.
    #[serde(rename = "onPremisesDomainName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_domain_name_contains: Option<String>,
    /// The Last time of User Login. String (range, `__between`).
    #[serde(rename = "lastLogonTime__between", skip_serializing_if = "Option::is_none")]
    pub last_logon_time_between: Option<String>,
    /// Asset Contact Email. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Array of enum:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The Object Class (not in). Array of string.
    #[serde(rename = "objectClass__nin", skip_serializing_if = "Option::is_none")]
    pub object_class_nin: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in). Array of enum: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The Forest Name (not in). Array of string.
    #[serde(rename = "forest__nin", skip_serializing_if = "Option::is_none")]
    pub forest_nin: Option<String>,
    /// On Premises Security Identifier filter. Array of string.
    #[serde(rename = "onPremisesSecurityIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_security_identifier_contains: Option<String>,
    /// The ID. Array of string.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values). Array
    /// of string.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type. Array of string.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// Tags. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction. Enum: `asc`, `desc`. List/export endpoints only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The cloud provider organization unit. Array of string.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization. Array of string.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// If true, only the total number of items will be returned, without any of
    /// the actual objects. Boolean. List/export endpoints only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// The SAM Account Name (not in). Array of string.
    #[serde(rename = "samAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub sam_account_name_nin: Option<String>,
    /// The Object SID. Array of string.
    #[serde(rename = "objectSid__contains", skip_serializing_if = "Option::is_none")]
    pub object_sid_contains: Option<String>,
    /// The Group Type (not in). Array of integer (int32).
    #[serde(rename = "groupType__nin", skip_serializing_if = "Option::is_none")]
    pub group_type_nin: Option<String>,
    /// The User Account Control. Array of integer (int32).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_account_control: Option<String>,
    /// Name. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Proxy Addresses. Array of string.
    #[serde(rename = "proxyAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub proxy_addresses_contains: Option<String>,
    /// The Object GUID. Array of string.
    #[serde(rename = "objectGuid__contains", skip_serializing_if = "Option::is_none")]
    pub object_guid_contains: Option<String>,
    /// The name. Array of string.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to. Array of enum: `critical`,
    /// `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in). Array of string.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// Lock-out Time. String (range, `__between`).
    #[serde(rename = "lockOutTime__between", skip_serializing_if = "Option::is_none")]
    pub lock_out_time_between: Option<String>,
    /// If true, the total number of items will not be calculated, which speeds up
    /// execution time. Boolean. List/export endpoints only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// User and cloud tag keys not exists. Array of string.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The Service Principal Name. Array of string.
    #[serde(rename = "servicePrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub service_principal_name_contains: Option<String>,
    /// List of Site IDs to filter by. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. String (range, `__between`).
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Array of enum:
    /// `Access Control and Surveillance System`, `Access Point`, `AD Certificate`,
    /// `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`,
    /// `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory (not in). Array of string.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in). Array of string.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The cloud tags key. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in). Array of enum:
    /// `All`, `Access Key and Secret`, `Access Management`, `Account`,
    /// `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The Service Account. Array of boolean.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
    /// The status alerts of the asset. Array of enum: `Infected`, `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// Given Name. Array of string.
    #[serde(rename = "givenName__contains", skip_serializing_if = "Option::is_none")]
    pub given_name_contains: Option<String>,
    /// The active coverage for the asset. Array of enum: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by. Integer (int32).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Export format. Enum: `csv`, `json`. Required on the export endpoint only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<String>,
    /// The User Principal Name. Array of string.
    #[serde(rename = "userPrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub user_principal_name_contains: Option<String>,
    /// The cloud provider account id. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to. Array of enum: `All`,
    /// `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review. Array of enum: `Not Reviewed`, `Under Analysis`,
    /// `Not Trusted`, `Allowed`, ``.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Other Mails. Array of string.
    #[serde(rename = "otherMails__contains", skip_serializing_if = "Option::is_none")]
    pub other_mails_contains: Option<String>,
    /// Limit number of returned items (1-1000). Integer (int32). List/export
    /// endpoints only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The Distinguished Name. Array of string.
    #[serde(rename = "distinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub distinguished_name_contains: Option<String>,
    /// The Object Class. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_class: Option<String>,
    /// The Forest Name. Array of string.
    #[serde(rename = "forest__contains", skip_serializing_if = "Option::is_none")]
    pub forest_contains: Option<String>,
    /// Bad Password Time. String (range, `__between`).
    #[serde(rename = "badPasswordTime__between", skip_serializing_if = "Option::is_none")]
    pub bad_password_time_between: Option<String>,
    /// Lock-out Time (`last_modified_time`). String (range, `__between`).
    #[serde(rename = "last_modified_time__between", skip_serializing_if = "Option::is_none")]
    pub last_modified_time_between: Option<String>,
    /// The Display Name. Array of string.
    #[serde(rename = "displayName__contains", skip_serializing_if = "Option::is_none")]
    pub display_name_contains: Option<String>,
    /// The ID. Array of string.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account name. Array of string.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset. Array of enum: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The AD Domain Name (not in). Array of string.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Array of
    /// string.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Array of string.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. Array of string.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The Object Category (not in). Array of string.
    #[serde(rename = "objectCategory__nin", skip_serializing_if = "Option::is_none")]
    pub object_category_nin: Option<String>,
    /// The User Account Control (not in). Array of integer (int32).
    #[serde(rename = "userAccountControl__nin", skip_serializing_if = "Option::is_none")]
    pub user_account_control_nin: Option<String>,
    /// The AD Domain Name. Array of string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The User Password Expiry Time. String (range, `__between`).
    #[serde(rename = "userPasswordExpiryTimeComputed__between", skip_serializing_if = "Option::is_none")]
    pub user_password_expiry_time_computed_between: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. String. List/export endpoints only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Whether the Identity Group is enabled or not. Array of boolean.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    /// The User Principal Name (not in). Array of string.
    #[serde(rename = "userPrincipalName__nin", skip_serializing_if = "Option::is_none")]
    pub user_principal_name_nin: Option<String>,
    /// The cloud provider project ID. Array of string.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl IdentitySurfaceQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_contains = Some(join_csv(v)); self }
    /// The criticality that each asset belongs to (not in). Enum: `critical`,
    /// `high`, `medium`, `low`, `--`.
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_criticality_nin = Some(join_csv(v)); self }
    /// The status alerts of the asset (not in). Enum: `Infected`, `Healthy`.
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.infection_status_nin = Some(join_csv(v)); self }
    /// The Group Type. Array of integer (int32).
    pub fn group_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.group_type = Some(join_csv(v)); self }
    /// The missing coverage for the asset.
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.missing_coverage = Some(join_csv(v)); self }
    /// User and cloud tags.
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_value = Some(join_csv(v)); self }
    /// The cloud provider account name (not in).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name_nin = Some(join_csv(v)); self }
    /// The Object Category.
    pub fn object_category<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_category = Some(join_csv(v)); self }
    /// The cloud resource ID.
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_resource_id_contains = Some(join_csv(v)); self }
    /// The cloud provider subscription ID.
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_subscription_id_contains = Some(join_csv(v)); self }
    /// Tag Keys (not in).
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_nin = Some(join_csv(v)); self }
    /// Tag Keys exists.
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_exists = Some(join_csv(v)); self }
    /// The region.
    pub fn region<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region = Some(join_csv(v)); self }
    /// The User Principal Name.
    pub fn user_principal_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.user_principal_name = Some(join_csv(v)); self }
    /// Free-text filter by tag key value (supports multiple values).
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value_contains = Some(join_csv(v)); self }
    /// The cloud tags key (not in).
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_nin = Some(join_csv(v)); self }
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key = Some(join_csv(v)); self }
    /// The risk factors associated with the asset (not in). Enum:
    /// `Unresolved Alerts`, `High Value`.
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.risk_factors_nin = Some(join_csv(v)); self }
    /// Tag Keys not exists.
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_nexists = Some(join_csv(v)); self }
    /// Skip first number of items (0-1000). List/export endpoints only.
    pub fn skip(mut self, v: i64) -> Self { self.skip = Some(v); self }
    /// The cloud provider account ID.
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id_contains = Some(join_csv(v)); self }
    /// Free-text filter by the image name.
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.image_name_contains = Some(join_csv(v)); self }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.group_ids = Some(join_csv(v)); self }
    /// The Email Address.
    pub fn mail<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.mail = Some(join_csv(v)); self }
    /// On Premises SAM Account Name.
    pub fn on_premises_sam_account_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_sam_account_name_contains = Some(join_csv(v)); self }
    /// The active coverage for the asset (not in).
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.active_coverage_nin = Some(join_csv(v)); self }
    /// User and cloud tags (not in).
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_value_nin = Some(join_csv(v)); self }
    /// The region (not in).
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region_nin = Some(join_csv(v)); self }
    /// User and cloud tag keys (not in).
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_nin = Some(join_csv(v)); self }
    /// The SAM Account Name.
    pub fn sam_account_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sam_account_name = Some(join_csv(v)); self }
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value_nin = Some(join_csv(v)); self }
    /// The Surface that each asset belongs to. Enum: `Cloud`, `Identity`,
    /// `Network`, `Endpoint`, `Network Discovery`.
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.surfaces = Some(join_csv(v)); self }
    /// The Forest Name.
    pub fn forest<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.forest = Some(join_csv(v)); self }
    /// The missing coverage for the asset (not in).
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.missing_coverage_nin = Some(join_csv(v)); self }
    /// The status of the asset (not in). Enum: `Active`, `Inactive`.
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_status_nin = Some(join_csv(v)); self }
    /// The column to sort the results by. List/export endpoints only. Enum:
    /// `s1GroupName`, `ntSecurityDescriptor`, `recycled`, `privileged`,
    /// `groupType`, `accountExpires`, `badPasswordTime`,
    /// `allowedToActOnBehalfOfOtherIdentity`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self { self.sort_by = Some(v.into()); self }
    /// On Premises User Principal Name.
    pub fn on_premises_user_principal_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_user_principal_name_contains = Some(join_csv(v)); self }
    /// Whether the AD Entity is privileged or not.
    pub fn privileged<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.privileged = Some(join_csv(v)); self }
    /// Whether the password never expires.
    pub fn password_never_expire<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.password_never_expire = Some(join_csv(v)); self }
    /// The geographical area where cloud resources are hosted.
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region_contains = Some(join_csv(v)); self }
    /// The asset review (not in).
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.device_review_nin = Some(join_csv(v)); self }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name = Some(join_csv(v)); self }
    /// Whether the AD Entity is deleted or not.
    pub fn deleted<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.deleted = Some(join_csv(v)); self }
    /// Asset Contact Email (not in).
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_contact_email_nin = Some(join_csv(v)); self }
    /// The severity of the alert.
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.alert_severity = Some(join_csv(v)); self }
    /// The Email Address.
    pub fn mail_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.mail_contains = Some(join_csv(v)); self }
    /// The cloud tags key value.
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value = Some(join_csv(v)); self }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.account_ids = Some(join_csv(v)); self }
    /// On Premises Distinguished Name.
    pub fn on_premises_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_distinguished_name_contains = Some(join_csv(v)); self }
    /// The AD Domain Name.
    pub fn domain_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.domain_contains = Some(join_csv(v)); self }
    /// The canonical name for the resource type (not in).
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type_nin = Some(join_csv(v)); self }
    /// The LDAP Common Name.
    pub fn cn<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cn = Some(join_csv(v)); self }
    /// The LDAP Common Name (contains).
    pub fn cn_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cn_contains = Some(join_csv(v)); self }
    /// The LDAP Common Name (not in).
    pub fn cn_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cn_nin = Some(join_csv(v)); self }
    /// On Premises Domain Name.
    pub fn on_premises_domain_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_domain_name_contains = Some(join_csv(v)); self }
    /// The Last time of User Login (range, `__between`).
    pub fn last_logon_time_between(mut self, v: impl Into<String>) -> Self { self.last_logon_time_between = Some(v.into()); self }
    /// Asset Contact Email.
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_contact_email = Some(join_csv(v)); self }
    /// The risk factors associated with the asset. Enum: `Unresolved Alerts`,
    /// `High Value`.
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.risk_factors = Some(join_csv(v)); self }
    /// The Object Class (not in).
    pub fn object_class_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_class_nin = Some(join_csv(v)); self }
    /// The environment that the asset exists in.
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_environment = Some(join_csv(v)); self }
    /// The Surface that each asset belongs to (not in).
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.surfaces_nin = Some(join_csv(v)); self }
    /// The Forest Name (not in).
    pub fn forest_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.forest_nin = Some(join_csv(v)); self }
    /// On Premises Security Identifier.
    pub fn on_premises_security_identifier_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_security_identifier_contains = Some(join_csv(v)); self }
    /// The ID (in).
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.id_in = Some(join_csv(v)); self }
    /// Free-text filter by cloud tag key value (supports multiple values).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value_contains = Some(join_csv(v)); self }
    /// The Asset Type.
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type_contains = Some(join_csv(v)); self }
    /// Tags.
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value = Some(join_csv(v)); self }
    /// Sort direction. List/export endpoints only. Enum: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self { self.sort_order = Some(v.into()); self }
    /// The cloud provider organization unit.
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_organization_unit_contains = Some(join_csv(v)); self }
    /// The cloud provider organization.
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_organization_contains = Some(join_csv(v)); self }
    /// If true, only the total number of items will be returned. List/export
    /// endpoints only.
    pub fn count_only(mut self, v: bool) -> Self { self.count_only = Some(v); self }
    /// The SAM Account Name (not in).
    pub fn sam_account_name_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sam_account_name_nin = Some(join_csv(v)); self }
    /// The Object SID.
    pub fn object_sid_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_sid_contains = Some(join_csv(v)); self }
    /// The Group Type (not in). Array of integer (int32).
    pub fn group_type_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.group_type_nin = Some(join_csv(v)); self }
    /// The User Account Control. Array of integer (int32).
    pub fn user_account_control<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.user_account_control = Some(join_csv(v)); self }
    /// Name.
    pub fn names<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.names = Some(join_csv(v)); self }
    /// Proxy Addresses.
    pub fn proxy_addresses_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.proxy_addresses_contains = Some(join_csv(v)); self }
    /// The Object GUID.
    pub fn object_guid_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_guid_contains = Some(join_csv(v)); self }
    /// The name.
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.name_contains = Some(join_csv(v)); self }
    /// The criticality that each asset belongs to. Enum: `critical`, `high`,
    /// `medium`, `low`, `--`.
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_criticality = Some(join_csv(v)); self }
    /// The cloud provider account id (not in).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id_nin = Some(join_csv(v)); self }
    /// Lock-out Time (range, `__between`).
    pub fn lock_out_time_between(mut self, v: impl Into<String>) -> Self { self.lock_out_time_between = Some(v.into()); self }
    /// If true, the total number of items will not be calculated. List/export
    /// endpoints only.
    pub fn skip_count(mut self, v: bool) -> Self { self.skip_count = Some(v); self }
    /// User and cloud tag keys not exists.
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_nexists = Some(join_csv(v)); self }
    /// The Service Principal Name.
    pub fn service_principal_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.service_principal_name_contains = Some(join_csv(v)); self }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.site_ids = Some(join_csv(v)); self }
    /// The Last Seen date and time for the asset (range, `__between`).
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self { self.s1_updated_at_between = Some(v.into()); self }
    /// The canonical name for the resource type.
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type = Some(join_csv(v)); self }
    /// The environment that the asset exists in (not in).
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_environment_nin = Some(join_csv(v)); self }
    /// Name (not in).
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.names_nin = Some(join_csv(v)); self }
    /// The cloud tags key.
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key = Some(join_csv(v)); self }
    /// The sub-category that each resource belongs to (not in).
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sub_category_nin = Some(join_csv(v)); self }
    /// The Service Account.
    pub fn service_account<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.service_account = Some(join_csv(v)); self }
    /// The status alerts of the asset. Enum: `Infected`, `Healthy`.
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.infection_status = Some(join_csv(v)); self }
    /// Given Name.
    pub fn given_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.given_name_contains = Some(join_csv(v)); self }
    /// The active coverage for the asset.
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.active_coverage = Some(join_csv(v)); self }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.counts_for = Some(join_csv(v)); self }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, v: i64) -> Self { self.csv_filter_id = Some(v); self }
    /// Export format. Required on the export endpoint. Enum: `csv`, `json`.
    pub fn export_format(mut self, v: impl Into<String>) -> Self { self.export_format = Some(v.into()); self }
    /// The User Principal Name (contains).
    pub fn user_principal_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.user_principal_name_contains = Some(join_csv(v)); self }
    /// The cloud provider account id.
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id = Some(join_csv(v)); self }
    /// The sub-category that each resource belongs to.
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sub_category = Some(join_csv(v)); self }
    /// The asset review.
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.device_review = Some(join_csv(v)); self }
    /// User and cloud tag keys.
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key = Some(join_csv(v)); self }
    /// Other Mails.
    pub fn other_mails_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.other_mails_contains = Some(join_csv(v)); self }
    /// Limit number of returned items (1-1000). List/export endpoints only.
    pub fn limit(mut self, v: i64) -> Self { self.limit = Some(v); self }
    /// The Distinguished Name.
    pub fn distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.distinguished_name_contains = Some(join_csv(v)); self }
    /// The Object Class.
    pub fn object_class<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_class = Some(join_csv(v)); self }
    /// The Forest Name (contains).
    pub fn forest_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.forest_contains = Some(join_csv(v)); self }
    /// Bad Password Time (range, `__between`).
    pub fn bad_password_time_between(mut self, v: impl Into<String>) -> Self { self.bad_password_time_between = Some(v.into()); self }
    /// Lock-out Time / `last_modified_time` (range, `__between`).
    pub fn last_modified_time_between(mut self, v: impl Into<String>) -> Self { self.last_modified_time_between = Some(v.into()); self }
    /// The Display Name.
    pub fn display_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.display_name_contains = Some(join_csv(v)); self }
    /// The ID (contains).
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.id_contains = Some(join_csv(v)); self }
    /// The cloud provider account name (contains).
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name_contains = Some(join_csv(v)); self }
    /// The status of the asset. Enum: `Active`, `Inactive`.
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_status = Some(join_csv(v)); self }
    /// The AD Domain Name (not in).
    pub fn domain_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.domain_nin = Some(join_csv(v)); self }
    /// Free-text filter by cloud tag key (supports multiple values).
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_contains = Some(join_csv(v)); self }
    /// Tags (not in).
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value_nin = Some(join_csv(v)); self }
    /// User and cloud tag keys exists.
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_exists = Some(join_csv(v)); self }
    /// The Object Category (not in).
    pub fn object_category_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_category_nin = Some(join_csv(v)); self }
    /// The User Account Control (not in). Array of integer (int32).
    pub fn user_account_control_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.user_account_control_nin = Some(join_csv(v)); self }
    /// The AD Domain Name.
    pub fn domain<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.domain = Some(join_csv(v)); self }
    /// The User Password Expiry Time (range, `__between`).
    pub fn user_password_expiry_time_computed_between(mut self, v: impl Into<String>) -> Self { self.user_password_expiry_time_computed_between = Some(v.into()); self }
    /// Cursor position returned by the last request. List/export endpoints only.
    pub fn cursor(mut self, v: impl Into<String>) -> Self { self.cursor = Some(v.into()); self }
    /// Whether the Identity Group is enabled or not.
    pub fn enabled<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.enabled = Some(join_csv(v)); self }
    /// The User Principal Name (not in).
    pub fn user_principal_name_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.user_principal_name_nin = Some(join_csv(v)); self }
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_project_id_contains = Some(join_csv(v)); self }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/surface/identity/action`
/// ("Perform action").
///
/// Spec definition: `IdentityActionPayloadSchema`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityActionBody {
    /// Action name. Required in the spec. Enum (kept as `String`): allowed
    /// values `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`,
    /// `add_note`, `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`,
    /// `clear_tags`.
    pub action_name: String,
    /// List of selected inventory ids (max 5000). Optional. Array of string.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from `select_all` (max 5000). Optional.
    /// Array of string.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl IdentityActionBody {
    /// Construct with the required action name.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self { action_name: action_name.into(), id_in: None, id_nin: None }
    }
    /// List of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.id_in = Some(v.into_iter().map(Into::into).collect()); self
    }
    /// List of inventory ids to exclude from `select_all` (max 5000).
    pub fn id_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.id_nin = Some(v.into_iter().map(Into::into).collect()); self
    }
}

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/surface/identity/available-actions/with-status`
/// ("Available actions").
///
/// Spec definition: `AffectedResourcesSchema`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResourcesBody {
    /// List of selected inventory ids (max 5000). Optional. Array of string.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from `select_all` (max 5000). Optional.
    /// Array of string.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl AffectedResourcesBody {
    /// List of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.id_in = Some(v.into_iter().map(Into::into).collect()); self
    }
    /// List of inventory ids to exclude from `select_all` (max 5000).
    pub fn id_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.id_nin = Some(v.into_iter().map(Into::into).collect()); self
    }
}

impl InventoryIdentitySurfaceService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/surface/identity` — Assets.
    ///
    /// Get identity surface assets. Returns the identity inventory matching the
    /// supplied filters, paginated.
    pub async fn list(
        &self,
        query: &IdentitySurfaceQuery,
    ) -> Result<Paginated<IdentitySurfaceResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/surface/identity", q)
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/surface/identity/action` — Perform action.
    ///
    /// Perform action on selected assets. Filters narrow the target set (query
    /// params); the action and explicit id selection are supplied in the body.
    pub async fn perform_action(
        &self,
        query: &IdentitySurfaceQuery,
        body: &IdentityActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = path_with_query("/web/api/v2.1/xdr/assets/surface/identity/action", query);
        Ok(self.client.http().post(&path, body).await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/surface/identity/available-actions/with-status`
    /// — Available actions.
    ///
    /// Get available actions for the selected assets, each annotated with
    /// whether it is currently disabled and why.
    pub async fn available_actions_with_status(
        &self,
        query: &IdentitySurfaceQuery,
        body: &AffectedResourcesBody,
    ) -> Result<Response<AvailableActionWithStatusResponse>, Error> {
        let path = path_with_query(
            "/web/api/v2.1/xdr/assets/surface/identity/available-actions/with-status",
            query,
        );
        Ok(self.client.http().post(&path, body).await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/surface/identity/export` — Export assets to
    /// CSV or JSON.
    ///
    /// Returns the results for the given inventory filter in a CSV or JSON
    /// format. The `export_format` query param (`csv` or `json`) is required;
    /// set it via [`IdentitySurfaceQuery::export_format`].
    pub async fn export(
        &self,
        query: &IdentitySurfaceQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/surface/identity/export", q)
            .await?)
    }
}
