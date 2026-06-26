//! `Inventory Identity` tag — Inventory Identity Resources.
//!
//! Covers the five XDR identity-asset endpoints: list (GET/POST), perform
//! action, available actions, and export.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_identity::{AvailableActionWithStatus, Identity};
use crate::pagination::{Paginated, Response};

/// `Inventory Identity` tag — operations on identity inventory assets.
///
/// Wraps the `/web/api/v2.1/xdr/assets/identity` endpoints: list assets
/// (`GET` / `POST`), perform an action on a selection, query available actions,
/// and export to CSV/JSON.
pub struct InventoryIdentityService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Joins an iterator of string-likes by comma (the form array query params use).
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

/// Shared identity-asset filter query parameters.
///
/// These filter fields are common to every Inventory Identity endpoint that
/// accepts them (list, action, available-actions, export). Per-method query
/// structs hold this set plus any method-specific extras (paging, sorting,
/// export format, ...).
///
/// Every field is optional. Array filters are serialized as a single
/// comma-joined string (the form the API expects); use the builder methods,
/// which accept an iterator and join by comma. Scalar `__between` filters take a
/// single string value.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityFilterQuery {
    /// Free-text filter by tag key (supports multiple values). Array param.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Array param.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in). Array param.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The Group Type. Array param (`array<integer(int32)>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<String>,
    /// The missing coverage for the asset. Array param.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Array param.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The Object Category. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_category: Option<String>,
    /// The cloud resource ID. Array param.
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Array param.
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys (not in). Array param.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// Tag Keys exists. Array param.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The region. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The User Principal Name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Array param.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Array param.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). Array param.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Array param.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// The cloud provider account ID. Array param.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Array param.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The Email Address. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    /// On Premises SAM Account Name. Array param.
    #[serde(rename = "onPremisesSamAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_sam_account_name_contains: Option<String>,
    /// The active coverage for the asset (not in). Array param.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// User and cloud tags (not in). Array param.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in). Array param.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in). Array param.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The SAM Account Name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sam_account_name: Option<String>,
    /// The cloud tags key value (not in). Array param.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to. Array param.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The Forest Name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forest: Option<String>,
    /// The missing coverage for the asset (not in). Array param.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). Array param.
    /// Allowed values: `Active`, `Inactive`.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// On Premises User Principal Name. Array param.
    #[serde(rename = "onPremisesUserPrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_user_principal_name_contains: Option<String>,
    /// Whether the AD Entity is privileged or not. Array param (`array<boolean>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<String>,
    /// Whether the password never expires. Array param (`array<boolean>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_never_expire: Option<String>,
    /// The geographical area where cloud resources are hosted. Array param.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The asset review (not in). Array param.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty).
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The cloud provider account name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Whether the AD Entity is deleted or not. Array param (`array<boolean>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
    /// Asset Contact Email (not in). Array param.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// The Email Address. Array param.
    #[serde(rename = "mail__contains", skip_serializing_if = "Option::is_none")]
    pub mail_contains: Option<String>,
    /// The cloud tags key value. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// List of Account IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// On Premises Distinguished Name. Array param.
    #[serde(rename = "onPremisesDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_distinguished_name_contains: Option<String>,
    /// The AD Domain Name. Array param.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in). Array param.
    /// Allowed values include: `Access Control and Surveillance System`,
    /// `Access Point`, `AD Certificate`, `AD Certificate Authority`,
    /// `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The LDAP Common Name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn: Option<String>,
    /// The LDAP Common Name. Array param.
    #[serde(rename = "cn__contains", skip_serializing_if = "Option::is_none")]
    pub cn_contains: Option<String>,
    /// The LDAP Common Name (not in). Array param.
    #[serde(rename = "cn__nin", skip_serializing_if = "Option::is_none")]
    pub cn_nin: Option<String>,
    /// On Premises Domain Name. Array param.
    #[serde(rename = "onPremisesDomainName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_domain_name_contains: Option<String>,
    /// The Last time of User Login. Scalar `__between` filter.
    #[serde(rename = "lastLogonTime__between", skip_serializing_if = "Option::is_none")]
    pub last_logon_time_between: Option<String>,
    /// Asset Contact Email. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Array param.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The Object Class (not in). Array param.
    #[serde(rename = "objectClass__nin", skip_serializing_if = "Option::is_none")]
    pub object_class_nin: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP |
    /// Active Directory. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in). Array param.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The Forest Name (not in). Array param.
    #[serde(rename = "forest__nin", skip_serializing_if = "Option::is_none")]
    pub forest_nin: Option<String>,
    /// On Premises Security Identifier. Array param.
    #[serde(rename = "onPremisesSecurityIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_security_identifier_contains: Option<String>,
    /// The ID. Array param.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values). Array param.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type. Array param.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// Tags. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The cloud provider organization unit. Array param.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization. Array param.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// The SAM Account Name (not in). Array param.
    #[serde(rename = "samAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub sam_account_name_nin: Option<String>,
    /// The Object SID. Array param.
    #[serde(rename = "objectSid__contains", skip_serializing_if = "Option::is_none")]
    pub object_sid_contains: Option<String>,
    /// The Group Type (not in). Array param (`array<integer(int32)>`).
    #[serde(rename = "groupType__nin", skip_serializing_if = "Option::is_none")]
    pub group_type_nin: Option<String>,
    /// The User Account Control. Array param (`array<integer(int32)>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_account_control: Option<String>,
    /// Name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Proxy Addresses. Array param.
    #[serde(rename = "proxyAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub proxy_addresses_contains: Option<String>,
    /// The Object GUID. Array param.
    #[serde(rename = "objectGuid__contains", skip_serializing_if = "Option::is_none")]
    pub object_guid_contains: Option<String>,
    /// The name. Array param.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to. Array param.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in). Array param.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// Lock-out Time. Scalar `__between` filter.
    #[serde(rename = "lockOutTime__between", skip_serializing_if = "Option::is_none")]
    pub lock_out_time_between: Option<String>,
    /// User and cloud tag keys not exists. Array param.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The Service Principal Name. Array param.
    #[serde(rename = "servicePrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub service_principal_name_contains: Option<String>,
    /// List of Site IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. Scalar `__between` filter.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Array param.
    /// Allowed values include: `Access Control and Surveillance System`,
    /// `Access Point`, `AD Certificate`, `AD Certificate Authority`,
    /// `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP |
    /// Active Directory (not in). Array param.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in). Array param.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The cloud tags key. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in). Array param.
    /// Allowed values include: `All`, `Access Key and Secret`,
    /// `Access Management`, `Account`, `Account Group`, `AD Objects`,
    /// `Administrative Unit`, `Admission Controller`.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The Service Account. Array param (`array<boolean>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
    /// The status alerts of the asset. Array param.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// Given Name. Array param.
    #[serde(rename = "givenName__contains", skip_serializing_if = "Option::is_none")]
    pub given_name_contains: Option<String>,
    /// The active coverage for the asset. Array param.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The User Principal Name. Array param.
    #[serde(rename = "userPrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub user_principal_name_contains: Option<String>,
    /// The cloud provider account id. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to. Array param.
    /// Allowed values include: `All`, `Access Key and Secret`,
    /// `Access Management`, `Account`, `Account Group`, `AD Objects`,
    /// `Administrative Unit`, `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review. Array param.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Other Mails. Array param.
    #[serde(rename = "otherMails__contains", skip_serializing_if = "Option::is_none")]
    pub other_mails_contains: Option<String>,
    /// The Distinguished Name. Array param.
    #[serde(rename = "distinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub distinguished_name_contains: Option<String>,
    /// The Object Class. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_class: Option<String>,
    /// The Forest Name. Array param.
    #[serde(rename = "forest__contains", skip_serializing_if = "Option::is_none")]
    pub forest_contains: Option<String>,
    /// Bad Password Time. Scalar `__between` filter.
    #[serde(rename = "badPasswordTime__between", skip_serializing_if = "Option::is_none")]
    pub bad_password_time_between: Option<String>,
    /// Last modified time. Scalar `__between` filter.
    #[serde(rename = "last_modified_time__between", skip_serializing_if = "Option::is_none")]
    pub last_modified_time_between: Option<String>,
    /// The Display Name. Array param.
    #[serde(rename = "displayName__contains", skip_serializing_if = "Option::is_none")]
    pub display_name_contains: Option<String>,
    /// The ID. Array param.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account name. Array param.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset. Array param.
    /// Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The AD Domain Name (not in). Array param.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Array param.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Array param.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. Array param.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The Object Category (not in). Array param.
    #[serde(rename = "objectCategory__nin", skip_serializing_if = "Option::is_none")]
    pub object_category_nin: Option<String>,
    /// The User Account Control (not in). Array param (`array<integer(int32)>`).
    #[serde(rename = "userAccountControl__nin", skip_serializing_if = "Option::is_none")]
    pub user_account_control_nin: Option<String>,
    /// The AD Domain Name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The User Password Expiry Time. Scalar `__between` filter.
    #[serde(rename = "userPasswordExpiryTimeComputed__between", skip_serializing_if = "Option::is_none")]
    pub user_password_expiry_time_computed_between: Option<String>,
    /// Whether the Identity Group is enabled or not. Array param (`array<boolean>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    /// The User Principal Name (not in). Array param.
    #[serde(rename = "userPrincipalName__nin", skip_serializing_if = "Option::is_none")]
    pub user_principal_name_nin: Option<String>,
    /// The cloud provider project ID. Array param.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl IdentityFilterQuery {
    /// The ID (`id__in`). Array param.
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(join_csv(values));
        self
    }
    /// The ID (`id__contains`). Array param.
    pub fn id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(join_csv(values));
        self
    }
    /// Name (`names`). Array param.
    pub fn names<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(join_csv(values));
        self
    }
    /// The name (`name__contains`). Array param.
    pub fn name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(values));
        self
    }
    /// List of Account IDs to filter by (`accountIds`). Array param.
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(values));
        self
    }
    /// List of Site IDs to filter by (`siteIds`). Array param.
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(values));
        self
    }
    /// List of Group IDs to filter by (`groupIds`). Array param.
    pub fn group_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(values));
        self
    }
    /// The criticality that each asset belongs to (`assetCriticality`). Array param.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    pub fn asset_criticality<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(values));
        self
    }
    /// The Surface that each asset belongs to (`surfaces`). Array param.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    pub fn surfaces<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(join_csv(values));
        self
    }
    /// The canonical name for the resource type (`resourceType`). Array param.
    pub fn resource_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(values));
        self
    }
    /// The region (`region`). Array param.
    pub fn region<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(join_csv(values));
        self
    }
    /// The status of the asset (`assetStatus`). Array param.
    /// Allowed values: `Active`, `Inactive`.
    pub fn asset_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(join_csv(values));
        self
    }
    /// The status alerts of the asset (`infectionStatus`). Array param.
    /// Allowed values: `Infected`, `Healthy`.
    pub fn infection_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(join_csv(values));
        self
    }
    /// The active coverage for the asset (`activeCoverage`). Array param.
    pub fn active_coverage<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(join_csv(values));
        self
    }
    /// The missing coverage for the asset (`missingCoverage`). Array param.
    pub fn missing_coverage<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(join_csv(values));
        self
    }
    /// The sub-category that each resource belongs to (`subCategory`). Array param.
    pub fn sub_category<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(join_csv(values));
        self
    }
    /// The risk factors associated with the asset (`riskFactors`). Array param.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    pub fn risk_factors<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(join_csv(values));
        self
    }
    /// The Forest Name (`forest`). Array param.
    pub fn forest<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.forest = Some(join_csv(values));
        self
    }
    /// The AD Domain Name (`domain`). Array param.
    pub fn domain<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(join_csv(values));
        self
    }
    /// The LDAP Common Name (`cn`). Array param.
    pub fn cn<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cn = Some(join_csv(values));
        self
    }
    /// The User Principal Name (`userPrincipalName`). Array param.
    pub fn user_principal_name<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_principal_name = Some(join_csv(values));
        self
    }
    /// The Email Address (`mail`). Array param.
    pub fn mail<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mail = Some(join_csv(values));
        self
    }
    /// The SAM Account Name (`samAccountName`). Array param.
    pub fn sam_account_name<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sam_account_name = Some(join_csv(values));
        self
    }
    /// The Group Type (`groupType`). Array param (`array<integer(int32)>`).
    pub fn group_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_type = Some(join_csv(values));
        self
    }
    /// The User Account Control (`userAccountControl`). Array param
    /// (`array<integer(int32)>`).
    pub fn user_account_control<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_account_control = Some(join_csv(values));
        self
    }
    /// Whether the AD Entity is privileged or not (`privileged`). Array param
    /// (`array<boolean>`); each value should be `true` or `false`.
    pub fn privileged<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.privileged = Some(join_csv(values));
        self
    }
    /// Whether the Identity Group is enabled or not (`enabled`). Array param
    /// (`array<boolean>`); each value should be `true` or `false`.
    pub fn enabled<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.enabled = Some(join_csv(values));
        self
    }
    /// Whether the AD Entity is deleted or not (`deleted`). Array param
    /// (`array<boolean>`); each value should be `true` or `false`.
    pub fn deleted<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.deleted = Some(join_csv(values));
        self
    }
    /// The Service Account (`serviceAccount`). Array param (`array<boolean>`);
    /// each value should be `true` or `false`.
    pub fn service_account<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.service_account = Some(join_csv(values));
        self
    }
    /// Whether the password never expires (`passwordNeverExpire`). Array param
    /// (`array<boolean>`); each value should be `true` or `false`.
    pub fn password_never_expire<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.password_never_expire = Some(join_csv(values));
        self
    }
    /// The columns for which filter count would be returned for (`countsFor`). Array param.
    pub fn counts_for<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(join_csv(values));
        self
    }
    /// The ID of the CSV file to filter by (`csvFilterId`).
    pub fn csv_filter_id(mut self, id: i64) -> Self {
        self.csv_filter_id = Some(id);
        self
    }
    /// The Last Seen date and time for the asset (`s1UpdatedAt__between`).
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    /// The Last time of User Login (`lastLogonTime__between`).
    pub fn last_logon_time_between(mut self, v: impl Into<String>) -> Self {
        self.last_logon_time_between = Some(v.into());
        self
    }
    /// Lock-out Time (`lockOutTime__between`).
    pub fn lock_out_time_between(mut self, v: impl Into<String>) -> Self {
        self.lock_out_time_between = Some(v.into());
        self
    }
    /// Bad Password Time (`badPasswordTime__between`).
    pub fn bad_password_time_between(mut self, v: impl Into<String>) -> Self {
        self.bad_password_time_between = Some(v.into());
        self
    }
    /// Last modified time (`last_modified_time__between`).
    pub fn last_modified_time_between(mut self, v: impl Into<String>) -> Self {
        self.last_modified_time_between = Some(v.into());
        self
    }
    /// The User Password Expiry Time (`userPasswordExpiryTimeComputed__between`).
    pub fn user_password_expiry_time_computed_between(mut self, v: impl Into<String>) -> Self {
        self.user_password_expiry_time_computed_between = Some(v.into());
        self
    }
}

/// List/export-only paging, sorting and count params (serialized separately from
/// the shared filter set; see [`ListAssetsQuery`] and [`ExportAssetsQuery`]).
///
/// `serde_urlencoded` does not support `#[serde(flatten)]`, so these extras and
/// the [`IdentityFilterQuery`] are serialized independently and concatenated.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct PagingExtras {
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_count: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_format: Option<String>,
}

/// Concatenates the (non-empty) urlencoded forms of `extras` and `filter`.
fn build_query_string<E: Serialize>(extras: &E, filter: &IdentityFilterQuery) -> String {
    let mut parts: Vec<String> = Vec::new();
    let extra_qs = serde_urlencoded::to_string(extras).unwrap_or_default();
    if !extra_qs.is_empty() {
        parts.push(extra_qs);
    }
    let filter_qs = serde_urlencoded::to_string(filter).unwrap_or_default();
    if !filter_qs.is_empty() {
        parts.push(filter_qs);
    }
    parts.join("&")
}

/// Query params for `GET /web/api/v2.1/xdr/assets/identity`.
///
/// Wraps the shared [`IdentityFilterQuery`] and adds the list-specific paging,
/// sorting, and count controls. Array filter params are serialized comma-joined.
#[derive(Debug, Default)]
pub struct ListAssetsQuery {
    /// Shared identity-asset filter fields.
    pub filter: IdentityFilterQuery,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    pub cursor: Option<String>,
    /// The column to sort the results by. Optional.
    /// Allowed values: `s1GroupName`, `recycled`, `groupType`, `s1UpdatedAt`,
    /// `objectCategory`, `cloudProviderResourceGroup`, `resultantPso`, `region`.
    pub sort_by: Option<String>,
    /// Sort direction. Optional. Allowed values: `asc`, `desc`.
    pub sort_order: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    pub skip_count: Option<bool>,
}

impl ListAssetsQuery {
    /// Set the shared filter fields.
    pub fn filter(mut self, filter: IdentityFilterQuery) -> Self {
        self.filter = filter;
        self
    }
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
    /// The column to sort the results by. Allowed values: `s1GroupName`,
    /// `recycled`, `groupType`, `s1UpdatedAt`, `objectCategory`,
    /// `cloudProviderResourceGroup`, `resultantPso`, `region`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction (`asc` or `desc`).
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Return only the total number of items.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Skip calculating the total number of items.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }

    /// Build the full querystring (paging extras + the filter set).
    fn to_query_string(&self) -> String {
        let extras = PagingExtras {
            skip: self.skip,
            limit: self.limit,
            cursor: self.cursor.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            count_only: self.count_only,
            skip_count: self.skip_count,
            export_format: None,
        };
        build_query_string(&extras, &self.filter)
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/identity` (Assets using POST).
///
/// This endpoint only accepts the scope filters as query params; the asset
/// filter itself is supplied in the request body ([`AssetsViewBody`]). Array
/// params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsViewQuery {
    /// List of Account IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl AssetsViewQuery {
    /// List of Account IDs to filter by (`accountIds`). Array param.
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(values));
        self
    }
    /// List of Site IDs to filter by (`siteIds`). Array param.
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(values));
        self
    }
    /// List of Group IDs to filter by (`groupIds`). Array param.
    pub fn group_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(values));
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/identity/action`.
///
/// Only the shared [`IdentityFilterQuery`] filter fields are accepted as query
/// params on this endpoint (the action itself is supplied in the body).
#[derive(Debug, Default)]
pub struct PerformActionQuery {
    /// Shared identity-asset filter fields.
    pub filter: IdentityFilterQuery,
}

impl PerformActionQuery {
    /// Set the shared filter fields.
    pub fn filter(mut self, filter: IdentityFilterQuery) -> Self {
        self.filter = filter;
        self
    }
    /// Build the full querystring (the filter set).
    fn to_query_string(&self) -> String {
        serde_urlencoded::to_string(&self.filter).unwrap_or_default()
    }
}

/// Query params for
/// `POST /web/api/v2.1/xdr/assets/identity/available-actions/with-status`.
///
/// Only the shared [`IdentityFilterQuery`] filter fields are accepted as query
/// params on this endpoint.
#[derive(Debug, Default)]
pub struct AvailableActionsQuery {
    /// Shared identity-asset filter fields.
    pub filter: IdentityFilterQuery,
}

impl AvailableActionsQuery {
    /// Set the shared filter fields.
    pub fn filter(mut self, filter: IdentityFilterQuery) -> Self {
        self.filter = filter;
        self
    }
    /// Build the full querystring (the filter set).
    fn to_query_string(&self) -> String {
        serde_urlencoded::to_string(&self.filter).unwrap_or_default()
    }
}

/// Query params for `GET /web/api/v2.1/xdr/assets/identity/export`.
///
/// Wraps the shared [`IdentityFilterQuery`] and adds export-specific
/// paging/sorting controls plus the required `exportFormat`. The only required
/// param is `export_format`; set it via [`ExportAssetsQuery::new`].
#[derive(Debug, Default)]
pub struct ExportAssetsQuery {
    /// Shared identity-asset filter fields.
    pub filter: IdentityFilterQuery,
    /// Export format. Required. Allowed values: `csv`, `json`.
    pub export_format: String,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    pub cursor: Option<String>,
    /// The column to sort the results by. Optional.
    /// Allowed values: `s1GroupName`, `recycled`, `groupType`, `s1UpdatedAt`,
    /// `objectCategory`, `cloudProviderResourceGroup`, `resultantPso`, `region`.
    pub sort_by: Option<String>,
    /// Sort direction. Optional. Allowed values: `asc`, `desc`.
    pub sort_order: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    pub skip_count: Option<bool>,
}

impl ExportAssetsQuery {
    /// Create a new export query with the required export format
    /// (`csv` or `json`).
    pub fn new(export_format: impl Into<String>) -> Self {
        Self {
            filter: IdentityFilterQuery::default(),
            export_format: export_format.into(),
            skip: None,
            limit: None,
            cursor: None,
            sort_by: None,
            sort_order: None,
            count_only: None,
            skip_count: None,
        }
    }
    /// Set the required export format. Allowed values: `csv`, `json`.
    pub fn export_format(mut self, v: impl Into<String>) -> Self {
        self.export_format = v.into();
        self
    }
    /// Set the shared filter fields.
    pub fn filter(mut self, filter: IdentityFilterQuery) -> Self {
        self.filter = filter;
        self
    }
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
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction (`asc` or `desc`).
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Return only the total number of items.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Skip calculating the total number of items.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }

    /// Build the full querystring (`exportFormat` + paging extras + filter set).
    fn to_query_string(&self) -> String {
        let extras = PagingExtras {
            skip: self.skip,
            limit: self.limit,
            cursor: self.cursor.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            count_only: self.count_only,
            skip_count: self.skip_count,
            export_format: Some(self.export_format.clone()),
        };
        build_query_string(&extras, &self.filter)
    }
}

/// Body for `POST /web/api/v2.1/xdr/assets/identity` (`IdentityViewInputSchema`).
///
/// `filter` is required and is a freeform/paginated identity filter object; it
/// is kept as [`serde_json::Value`] because the spec models it as a large,
/// loosely structured filter (`PaginatedIdentityFilter`). `data` is optional and
/// nullable (`EmptyStrict` in the spec).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsViewBody {
    /// Data. Optional / nullable. Freeform object (`EmptyStrict` in the spec).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Filter. Required. Freeform paginated identity filter
    /// (`PaginatedIdentityFilter` in the spec).
    pub filter: serde_json::Value,
}

impl AssetsViewBody {
    /// Create a new view body with the required `filter` object.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { data: None, filter }
    }
    /// Set the optional `data` object.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
    /// Set the required `filter` object.
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = filter;
        self
    }
}

/// Body for `POST /web/api/v2.1/xdr/assets/identity/action`
/// (`IdentityActionPayloadSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformActionBody {
    /// Action name. Required. Allowed values: `export_resource_details`,
    /// `mark_asset_criticality_high`, `mark_asset_criticality_low`,
    /// `clear_asset_criticality`, `mark_asset_criticality_medium`,
    /// `mark_asset_criticality_critical`, `update_asset_contact`,
    /// `clear_asset_contact`, `apply_review`, `add_note`, `manage_tags`,
    /// `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    pub action_name: String,
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000). Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl PerformActionBody {
    /// Create a new action body with the required `actionName`.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self {
            action_name: action_name.into(),
            id_in: None,
            id_nin: None,
        }
    }
    /// List of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from select_all (max 5000).
    pub fn id_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(values.into_iter().map(Into::into).collect());
        self
    }
}

/// Body for
/// `POST /web/api/v2.1/xdr/assets/identity/available-actions/with-status`
/// (`AffectedResourcesSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionsBody {
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000). Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl AvailableActionsBody {
    /// List of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from select_all (max 5000).
    pub fn id_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(values.into_iter().map(Into::into).collect());
        self
    }
}

impl InventoryIdentityService<'_> {
    /// **Assets** — Get assets.
    ///
    /// Get assets.
    ///
    /// `GET /web/api/v2.1/xdr/assets/identity`
    pub async fn list(&self, query: &ListAssetsQuery) -> Result<Paginated<Identity>, Error> {
        let qs = query.to_query_string();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/identity", q)
            .await?)
    }

    /// **Assets using POST** — POST API to get Assets.
    ///
    /// POST API to get Assets.
    ///
    /// `POST /web/api/v2.1/xdr/assets/identity`
    pub async fn list_post(
        &self,
        query: &AssetsViewQuery,
        body: &AssetsViewBody,
    ) -> Result<Paginated<Identity>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let path = if qs.is_empty() {
            "/web/api/v2.1/xdr/assets/identity".to_owned()
        } else {
            format!("/web/api/v2.1/xdr/assets/identity?{qs}")
        };
        Ok(self.client.http().post(&path, body).await?)
    }

    /// **Perform action** — Perform action on selected assets.
    ///
    /// Perform action on selected assets.
    ///
    /// `POST /web/api/v2.1/xdr/assets/identity/action`
    pub async fn perform_action(
        &self,
        query: &PerformActionQuery,
        body: &PerformActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = query.to_query_string();
        let path = if qs.is_empty() {
            "/web/api/v2.1/xdr/assets/identity/action".to_owned()
        } else {
            format!("/web/api/v2.1/xdr/assets/identity/action?{qs}")
        };
        Ok(self.client.http().post(&path, body).await?)
    }

    /// **Available actions** — Get cloud inventory identity available-actions.
    ///
    /// Get cloud inventory identity available-actions.
    ///
    /// `POST /web/api/v2.1/xdr/assets/identity/available-actions/with-status`
    pub async fn available_actions(
        &self,
        query: &AvailableActionsQuery,
        body: &AvailableActionsBody,
    ) -> Result<Response<AvailableActionWithStatus>, Error> {
        let qs = query.to_query_string();
        let path = if qs.is_empty() {
            "/web/api/v2.1/xdr/assets/identity/available-actions/with-status".to_owned()
        } else {
            format!("/web/api/v2.1/xdr/assets/identity/available-actions/with-status?{qs}")
        };
        Ok(self.client.http().post(&path, body).await?)
    }

    /// **Export assets to CSV or JSON** — Returns the results for given
    /// inventory filter in a CSV or JSON format.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format.
    ///
    /// `GET /web/api/v2.1/xdr/assets/identity/export`
    pub async fn export(
        &self,
        query: &ExportAssetsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = query.to_query_string();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/identity/export", q)
            .await?)
    }
}
