//! `Inventory Identity Filters` tag — Inventory Identity Resource Filters.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_identity_filters::{
    AutoCompleteResponse, CountFiltersResponse, FreeTextFilterResponse,
};
use crate::pagination::Response;

/// `Inventory Identity Filters` tag.
///
/// Inventory Identity Resource Filters: auto-complete suggestions, filter
/// counts, and free-text filter descriptors for identity-level XDR assets
/// (Active Directory entities, users, groups, and related resources).
pub struct InventoryIdentityFiltersService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for
/// `GET /web/api/v2.1/xdr/assets/identity/filters/autocomplete`.
///
/// All array params are serialized comma-joined, as the API expects. Every
/// field is optional except `text` and `key`, which are required by the spec;
/// set those via [`AutoCompleteQuery::text`] and [`AutoCompleteQuery::key`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteQuery {
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Optional. Allowed
    /// values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in). Optional. Allowed values:
    /// `Infected`, `Healthy`.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The Group Type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<String>,
    /// The missing coverage for the asset. Optional. Allowed values: `CWS`,
    /// `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The Object Category. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_category: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys (not in). Optional.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The region. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The User Principal Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Optional.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). Optional. Allowed
    /// values: `Unresolved Alerts`, `High Value`.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// The cloud provider account ID. Optional.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The Email Address. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    /// Search field key. Required. Allowed values: `cn__contains`,
    /// `displayName__contains`, `distinguishedName__contains`,
    /// `domain__contains`, `objectGuid__contains`, `objectSid__contains`,
    /// `servicePrincipalName__contains`, `forest__contains`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// On Premises SAM Account Name. Optional.
    #[serde(rename = "onPremisesSamAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_sam_account_name_contains: Option<String>,
    /// The active coverage for the asset (not in). Optional. Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data
    /// Classification`, `CNS KSPM`.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in). Optional.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The SAM Account Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sam_account_name: Option<String>,
    /// The cloud tags key value (not in). Optional.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to. Optional. Allowed values: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The Forest Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forest: Option<String>,
    /// The missing coverage for the asset (not in). Optional. Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data
    /// Classification`, `CNS KSPM`.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). Optional. Allowed values: `Active`,
    /// `Inactive`.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// On Premises User Principal Name. Optional.
    #[serde(rename = "onPremisesUserPrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_user_principal_name_contains: Option<String>,
    /// Whether the AD Entity is privileged or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<String>,
    /// Search term text. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Whether the password never expires. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_never_expire: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The asset review (not in). Optional. Allowed values: `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Whether the AD Entity is deleted or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
    /// Asset Contact Email (not in). Optional.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// The Email Address. Optional.
    #[serde(rename = "mail__contains", skip_serializing_if = "Option::is_none")]
    pub mail_contains: Option<String>,
    /// The cloud tags key value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Optional.
    #[serde(rename = "onPremisesDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_distinguished_name_contains: Option<String>,
    /// The AD Domain Name. Optional.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in). Optional. Allowed
    /// values: `Access Control and Surveillance System`, `Access Point`, `AD
    /// Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD
    /// Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The LDAP Common Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn: Option<String>,
    /// The LDAP Common Name. Optional.
    #[serde(rename = "cn__contains", skip_serializing_if = "Option::is_none")]
    pub cn_contains: Option<String>,
    /// The LDAP Common Name (not in). Optional.
    #[serde(rename = "cn__nin", skip_serializing_if = "Option::is_none")]
    pub cn_nin: Option<String>,
    /// On Premises Distinguished Name. Optional.
    #[serde(rename = "onPremisesDomainName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_domain_name_contains: Option<String>,
    /// The Last time of User Login. Optional.
    #[serde(rename = "lastLogonTime__between", skip_serializing_if = "Option::is_none")]
    pub last_logon_time_between: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Optional. Allowed values:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The Object Class (not in). Optional.
    #[serde(rename = "objectClass__nin", skip_serializing_if = "Option::is_none")]
    pub object_class_nin: Option<String>,
    /// The environment that the asset exists in - AWS \. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in). Optional. Allowed values:
    /// `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The Forest Name (not in). Optional.
    #[serde(rename = "forest__nin", skip_serializing_if = "Option::is_none")]
    pub forest_nin: Option<String>,
    /// Optional.
    #[serde(rename = "onPremisesSecurityIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_security_identifier_contains: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    /// Optional.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// Tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// The SAM Account Name (not in). Optional.
    #[serde(rename = "samAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub sam_account_name_nin: Option<String>,
    /// The Object SID. Optional.
    #[serde(rename = "objectSid__contains", skip_serializing_if = "Option::is_none")]
    pub object_sid_contains: Option<String>,
    /// The Group Type (not in). Optional.
    #[serde(rename = "groupType__nin", skip_serializing_if = "Option::is_none")]
    pub group_type_nin: Option<String>,
    /// The User Account Control. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_account_control: Option<String>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Proxy Addresses. Optional.
    #[serde(rename = "proxyAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub proxy_addresses_contains: Option<String>,
    /// The Object GUID. Optional.
    #[serde(rename = "objectGuid__contains", skip_serializing_if = "Option::is_none")]
    pub object_guid_contains: Option<String>,
    /// The name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to. Optional. Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in). Optional.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// Lock-out Time. Optional.
    #[serde(rename = "lockOutTime__between", skip_serializing_if = "Option::is_none")]
    pub lock_out_time_between: Option<String>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The Service Principal Name. Optional.
    #[serde(rename = "servicePrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub service_principal_name_contains: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s_1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Optional. Allowed values:
    /// `Access Control and Surveillance System`, `Access Point`, `AD
    /// Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD
    /// Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS \. Optional.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in). Optional.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in). Optional. Allowed
    /// values: `All`, `Access Key and Secret`, `Access Management`, `Account`,
    /// `Account Group`, `AD Objects`, `Administrative Unit`, `Admission
    /// Controller`.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The Service Account. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
    /// The status alerts of the asset. Optional. Allowed values: `Infected`,
    /// `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// Given Name. Optional.
    #[serde(rename = "givenName__contains", skip_serializing_if = "Option::is_none")]
    pub given_name_contains: Option<String>,
    /// The active coverage for the asset. Optional. Allowed values: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS
    /// KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The User Principal Name. Optional.
    #[serde(rename = "userPrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub user_principal_name_contains: Option<String>,
    /// The cloud provider account id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to. Optional. Allowed values:
    /// `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account
    /// Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review. Optional. Allowed values: `Not Reviewed`, `Under
    /// Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Other Mails. Optional.
    #[serde(rename = "otherMails__contains", skip_serializing_if = "Option::is_none")]
    pub other_mails_contains: Option<String>,
    /// Limit number of returned items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The Distinguished Name. Optional.
    #[serde(rename = "distinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub distinguished_name_contains: Option<String>,
    /// The Object Class. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_class: Option<String>,
    /// The Forest Name. Optional.
    #[serde(rename = "forest__contains", skip_serializing_if = "Option::is_none")]
    pub forest_contains: Option<String>,
    /// Bad Password Time. Optional.
    #[serde(rename = "badPasswordTime__between", skip_serializing_if = "Option::is_none")]
    pub bad_password_time_between: Option<String>,
    /// Lock-out Time. Optional.
    #[serde(rename = "last_modified_time__between", skip_serializing_if = "Option::is_none")]
    pub last_modified_time_between: Option<String>,
    /// The Display Name. Optional.
    #[serde(rename = "displayName__contains", skip_serializing_if = "Option::is_none")]
    pub display_name_contains: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset. Optional. Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The AD Domain Name (not in). Optional.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Optional.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Optional.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The Object Category (not in). Optional.
    #[serde(rename = "objectCategory__nin", skip_serializing_if = "Option::is_none")]
    pub object_category_nin: Option<String>,
    /// The User Account Control (not in). Optional.
    #[serde(rename = "userAccountControl__nin", skip_serializing_if = "Option::is_none")]
    pub user_account_control_nin: Option<String>,
    /// The AD Domain Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The User Password Expiry Time. Optional.
    #[serde(rename = "userPasswordExpiryTimeComputed__between", skip_serializing_if = "Option::is_none")]
    pub user_password_expiry_time_computed_between: Option<String>,
    /// Whether the Identity Group is enabled or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    /// The User Principal Name (not in). Optional.
    #[serde(rename = "userPrincipalName__nin", skip_serializing_if = "Option::is_none")]
    pub user_principal_name_nin: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl AutoCompleteQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to (not in).
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(join_csv(v));
        self
    }
    /// The status alerts of the asset (not in).
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(join_csv(v));
        self
    }
    /// The Group Type.
    pub fn group_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_type = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset.
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(join_csv(v));
        self
    }
    /// User and cloud tags.
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(join_csv(v));
        self
    }
    /// The cloud provider account name (not in).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_nin = Some(join_csv(v));
        self
    }
    /// The Object Category.
    pub fn object_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_category = Some(join_csv(v));
        self
    }
    /// The cloud resource ID.
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider subscription ID.
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id_contains = Some(join_csv(v));
        self
    }
    /// Tag Keys (not in).
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(join_csv(v));
        self
    }
    /// Tag Keys exists.
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(join_csv(v));
        self
    }
    /// The region.
    pub fn region<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(join_csv(v));
        self
    }
    /// The User Principal Name.
    pub fn user_principal_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_principal_name = Some(join_csv(v));
        self
    }
    /// Free-text filter by tag key value (supports multiple values).
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key (not in).
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_nin = Some(join_csv(v));
        self
    }
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(join_csv(v));
        self
    }
    /// The risk factors associated with the asset (not in).
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(join_csv(v));
        self
    }
    /// Tag Keys not exists.
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(join_csv(v));
        self
    }
    /// The cloud provider account ID.
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the image name.
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(join_csv(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// The Email Address.
    pub fn mail<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mail = Some(join_csv(v));
        self
    }
    /// Search field key. Required. See struct field docs for allowed values.
    pub fn key(mut self, v: impl Into<String>) -> Self {
        self.key = Some(v.into());
        self
    }
    /// On Premises SAM Account Name.
    pub fn on_premises_sam_account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.on_premises_sam_account_name_contains = Some(join_csv(v));
        self
    }
    /// The active coverage for the asset (not in).
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tags (not in).
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// The region (not in).
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys (not in).
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(join_csv(v));
        self
    }
    /// The SAM Account Name.
    pub fn sam_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sam_account_name = Some(join_csv(v));
        self
    }
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// The Surface that each asset belongs to.
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(join_csv(v));
        self
    }
    /// The Forest Name.
    pub fn forest<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.forest = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset (not in).
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(join_csv(v));
        self
    }
    /// The status of the asset (not in).
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(join_csv(v));
        self
    }
    /// On Premises User Principal Name.
    pub fn on_premises_user_principal_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.on_premises_user_principal_name_contains = Some(join_csv(v));
        self
    }
    /// Whether the AD Entity is privileged or not.
    pub fn privileged<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.privileged = Some(join_csv(v));
        self
    }
    /// Search term text. Required.
    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.text = Some(v.into());
        self
    }
    /// Whether the password never expires.
    pub fn password_never_expire<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.password_never_expire = Some(join_csv(v));
        self
    }
    /// The geographical area where cloud resources are hosted.
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(join_csv(v));
        self
    }
    /// The asset review (not in).
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(join_csv(v));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name = Some(join_csv(v));
        self
    }
    /// Whether the AD Entity is deleted or not.
    pub fn deleted<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.deleted = Some(join_csv(v));
        self
    }
    /// Asset Contact Email (not in).
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(join_csv(v));
        self
    }
    /// The severity of the alert.
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(join_csv(v));
        self
    }
    /// The Email Address.
    pub fn mail_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mail_contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key value.
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value = Some(join_csv(v));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// on_premises_distinguished_name_contains.
    pub fn on_premises_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.on_premises_distinguished_name_contains = Some(join_csv(v));
        self
    }
    /// The AD Domain Name.
    pub fn domain_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_contains = Some(join_csv(v));
        self
    }
    /// The canonical name for the resource type (not in).
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(join_csv(v));
        self
    }
    /// The LDAP Common Name.
    pub fn cn<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cn = Some(join_csv(v));
        self
    }
    /// The LDAP Common Name.
    pub fn cn_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cn_contains = Some(join_csv(v));
        self
    }
    /// The LDAP Common Name (not in).
    pub fn cn_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cn_nin = Some(join_csv(v));
        self
    }
    /// On Premises Distinguished Name.
    pub fn on_premises_domain_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.on_premises_domain_name_contains = Some(join_csv(v));
        self
    }
    /// The Last time of User Login.
    pub fn last_logon_time_between(mut self, v: impl Into<String>) -> Self {
        self.last_logon_time_between = Some(v.into());
        self
    }
    /// Asset Contact Email.
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(join_csv(v));
        self
    }
    /// The risk factors associated with the asset.
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(join_csv(v));
        self
    }
    /// The Object Class (not in).
    pub fn object_class_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_class_nin = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS \.
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(join_csv(v));
        self
    }
    /// The Surface that each asset belongs to (not in).
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(join_csv(v));
        self
    }
    /// The Forest Name (not in).
    pub fn forest_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.forest_nin = Some(join_csv(v));
        self
    }
    /// on_premises_security_identifier_contains.
    pub fn on_premises_security_identifier_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.on_premises_security_identifier_contains = Some(join_csv(v));
        self
    }
    /// The ID.
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_contains = Some(join_csv(v));
        self
    }
    /// The Asset Type.
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(join_csv(v));
        self
    }
    /// Tags.
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(join_csv(v));
        self
    }
    /// The cloud provider organization unit.
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider organization.
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_contains = Some(join_csv(v));
        self
    }
    /// The SAM Account Name (not in).
    pub fn sam_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sam_account_name_nin = Some(join_csv(v));
        self
    }
    /// The Object SID.
    pub fn object_sid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_sid_contains = Some(join_csv(v));
        self
    }
    /// The Group Type (not in).
    pub fn group_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_type_nin = Some(join_csv(v));
        self
    }
    /// The User Account Control.
    pub fn user_account_control<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_account_control = Some(join_csv(v));
        self
    }
    /// Name.
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(join_csv(v));
        self
    }
    /// Proxy Addresses.
    pub fn proxy_addresses_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_addresses_contains = Some(join_csv(v));
        self
    }
    /// The Object GUID.
    pub fn object_guid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_guid_contains = Some(join_csv(v));
        self
    }
    /// The name.
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to.
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(v));
        self
    }
    /// The cloud provider account id (not in).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_nin = Some(join_csv(v));
        self
    }
    /// Lock-out Time.
    pub fn lock_out_time_between(mut self, v: impl Into<String>) -> Self {
        self.lock_out_time_between = Some(v.into());
        self
    }
    /// User and cloud tag keys not exists.
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(join_csv(v));
        self
    }
    /// The Service Principal Name.
    pub fn service_principal_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.service_principal_name_contains = Some(join_csv(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// The Last Seen date and time for the asset.
    pub fn s_1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s_1_updated_at_between = Some(v.into());
        self
    }
    /// The canonical name for the resource type.
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS \.
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(join_csv(v));
        self
    }
    /// Name (not in).
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(join_csv(v));
        self
    }
    /// The cloud tags key.
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key = Some(join_csv(v));
        self
    }
    /// The sub-category that each resource belongs to (not in).
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(join_csv(v));
        self
    }
    /// The Service Account.
    pub fn service_account<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.service_account = Some(join_csv(v));
        self
    }
    /// The status alerts of the asset.
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(join_csv(v));
        self
    }
    /// Given Name.
    pub fn given_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.given_name_contains = Some(join_csv(v));
        self
    }
    /// The active coverage for the asset.
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(join_csv(v));
        self
    }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(join_csv(v));
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// The User Principal Name.
    pub fn user_principal_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_principal_name_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider account id.
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id = Some(join_csv(v));
        self
    }
    /// The sub-category that each resource belongs to.
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(join_csv(v));
        self
    }
    /// The asset review.
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys.
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(join_csv(v));
        self
    }
    /// Other Mails.
    pub fn other_mails_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.other_mails_contains = Some(join_csv(v));
        self
    }
    /// Limit number of returned items.
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// The Distinguished Name.
    pub fn distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.distinguished_name_contains = Some(join_csv(v));
        self
    }
    /// The Object Class.
    pub fn object_class<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_class = Some(join_csv(v));
        self
    }
    /// The Forest Name.
    pub fn forest_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.forest_contains = Some(join_csv(v));
        self
    }
    /// Bad Password Time.
    pub fn bad_password_time_between(mut self, v: impl Into<String>) -> Self {
        self.bad_password_time_between = Some(v.into());
        self
    }
    /// Lock-out Time.
    pub fn last_modified_time_between(mut self, v: impl Into<String>) -> Self {
        self.last_modified_time_between = Some(v.into());
        self
    }
    /// The Display Name.
    pub fn display_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.display_name_contains = Some(join_csv(v));
        self
    }
    /// The ID.
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(join_csv(v));
        self
    }
    /// The status of the asset.
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(join_csv(v));
        self
    }
    /// The AD Domain Name (not in).
    pub fn domain_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_nin = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key (supports multiple values).
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_contains = Some(join_csv(v));
        self
    }
    /// Tags (not in).
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys exists.
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(join_csv(v));
        self
    }
    /// The Object Category (not in).
    pub fn object_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_category_nin = Some(join_csv(v));
        self
    }
    /// The User Account Control (not in).
    pub fn user_account_control_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_account_control_nin = Some(join_csv(v));
        self
    }
    /// The AD Domain Name.
    pub fn domain<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(join_csv(v));
        self
    }
    /// The User Password Expiry Time.
    pub fn user_password_expiry_time_computed_between(mut self, v: impl Into<String>) -> Self {
        self.user_password_expiry_time_computed_between = Some(v.into());
        self
    }
    /// Whether the Identity Group is enabled or not.
    pub fn enabled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.enabled = Some(join_csv(v));
        self
    }
    /// The User Principal Name (not in).
    pub fn user_principal_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_principal_name_nin = Some(join_csv(v));
        self
    }
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(join_csv(v));
        self
    }
}

/// Query params for `GET /web/api/v2.1/xdr/assets/identity/filters/count`.
///
/// All array params are serialized comma-joined, as the API expects. Every
/// param is optional. This is the same filter surface as
/// [`AutoCompleteQuery`], minus the required `text`/`key` and `limit` fields.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterCountsQuery {
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Optional. Allowed
    /// values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in). Optional. Allowed values:
    /// `Infected`, `Healthy`.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The Group Type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<String>,
    /// The missing coverage for the asset. Optional. Allowed values: `CWS`,
    /// `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The Object Category. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_category: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys (not in). Optional.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The region. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The User Principal Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Optional.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). Optional. Allowed
    /// values: `Unresolved Alerts`, `High Value`.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// The cloud provider account ID. Optional.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The Email Address. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    /// On Premises SAM Account Name. Optional.
    #[serde(rename = "onPremisesSamAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_sam_account_name_contains: Option<String>,
    /// The active coverage for the asset (not in). Optional. Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data
    /// Classification`, `CNS KSPM`.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in). Optional.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The SAM Account Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sam_account_name: Option<String>,
    /// The cloud tags key value (not in). Optional.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to. Optional. Allowed values: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The Forest Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forest: Option<String>,
    /// The missing coverage for the asset (not in). Optional. Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data
    /// Classification`, `CNS KSPM`.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). Optional. Allowed values: `Active`,
    /// `Inactive`.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// On Premises User Principal Name. Optional.
    #[serde(rename = "onPremisesUserPrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_user_principal_name_contains: Option<String>,
    /// Whether the AD Entity is privileged or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<String>,
    /// Whether the password never expires. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_never_expire: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The asset review (not in). Optional. Allowed values: `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Whether the AD Entity is deleted or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
    /// Asset Contact Email (not in). Optional.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// The Email Address. Optional.
    #[serde(rename = "mail__contains", skip_serializing_if = "Option::is_none")]
    pub mail_contains: Option<String>,
    /// The cloud tags key value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Optional.
    #[serde(rename = "onPremisesDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_distinguished_name_contains: Option<String>,
    /// The AD Domain Name. Optional.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in). Optional. Allowed
    /// values: `Access Control and Surveillance System`, `Access Point`, `AD
    /// Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD
    /// Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The LDAP Common Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn: Option<String>,
    /// The LDAP Common Name. Optional.
    #[serde(rename = "cn__contains", skip_serializing_if = "Option::is_none")]
    pub cn_contains: Option<String>,
    /// The LDAP Common Name (not in). Optional.
    #[serde(rename = "cn__nin", skip_serializing_if = "Option::is_none")]
    pub cn_nin: Option<String>,
    /// On Premises Distinguished Name. Optional.
    #[serde(rename = "onPremisesDomainName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_domain_name_contains: Option<String>,
    /// The Last time of User Login. Optional.
    #[serde(rename = "lastLogonTime__between", skip_serializing_if = "Option::is_none")]
    pub last_logon_time_between: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Optional. Allowed values:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The Object Class (not in). Optional.
    #[serde(rename = "objectClass__nin", skip_serializing_if = "Option::is_none")]
    pub object_class_nin: Option<String>,
    /// The environment that the asset exists in - AWS \. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in). Optional. Allowed values:
    /// `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The Forest Name (not in). Optional.
    #[serde(rename = "forest__nin", skip_serializing_if = "Option::is_none")]
    pub forest_nin: Option<String>,
    /// Optional.
    #[serde(rename = "onPremisesSecurityIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_security_identifier_contains: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    /// Optional.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// Tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// The SAM Account Name (not in). Optional.
    #[serde(rename = "samAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub sam_account_name_nin: Option<String>,
    /// The Object SID. Optional.
    #[serde(rename = "objectSid__contains", skip_serializing_if = "Option::is_none")]
    pub object_sid_contains: Option<String>,
    /// The Group Type (not in). Optional.
    #[serde(rename = "groupType__nin", skip_serializing_if = "Option::is_none")]
    pub group_type_nin: Option<String>,
    /// The User Account Control. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_account_control: Option<String>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Proxy Addresses. Optional.
    #[serde(rename = "proxyAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub proxy_addresses_contains: Option<String>,
    /// The Object GUID. Optional.
    #[serde(rename = "objectGuid__contains", skip_serializing_if = "Option::is_none")]
    pub object_guid_contains: Option<String>,
    /// The name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to. Optional. Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in). Optional.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// Lock-out Time. Optional.
    #[serde(rename = "lockOutTime__between", skip_serializing_if = "Option::is_none")]
    pub lock_out_time_between: Option<String>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The Service Principal Name. Optional.
    #[serde(rename = "servicePrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub service_principal_name_contains: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s_1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Optional. Allowed values:
    /// `Access Control and Surveillance System`, `Access Point`, `AD
    /// Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD
    /// Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS \. Optional.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in). Optional.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in). Optional. Allowed
    /// values: `All`, `Access Key and Secret`, `Access Management`, `Account`,
    /// `Account Group`, `AD Objects`, `Administrative Unit`, `Admission
    /// Controller`.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The Service Account. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
    /// The status alerts of the asset. Optional. Allowed values: `Infected`,
    /// `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// Given Name. Optional.
    #[serde(rename = "givenName__contains", skip_serializing_if = "Option::is_none")]
    pub given_name_contains: Option<String>,
    /// The active coverage for the asset. Optional. Allowed values: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS
    /// KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The User Principal Name. Optional.
    #[serde(rename = "userPrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub user_principal_name_contains: Option<String>,
    /// The cloud provider account id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to. Optional. Allowed values:
    /// `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account
    /// Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review. Optional. Allowed values: `Not Reviewed`, `Under
    /// Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Other Mails. Optional.
    #[serde(rename = "otherMails__contains", skip_serializing_if = "Option::is_none")]
    pub other_mails_contains: Option<String>,
    /// The Distinguished Name. Optional.
    #[serde(rename = "distinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub distinguished_name_contains: Option<String>,
    /// The Object Class. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_class: Option<String>,
    /// The Forest Name. Optional.
    #[serde(rename = "forest__contains", skip_serializing_if = "Option::is_none")]
    pub forest_contains: Option<String>,
    /// Bad Password Time. Optional.
    #[serde(rename = "badPasswordTime__between", skip_serializing_if = "Option::is_none")]
    pub bad_password_time_between: Option<String>,
    /// Lock-out Time. Optional.
    #[serde(rename = "last_modified_time__between", skip_serializing_if = "Option::is_none")]
    pub last_modified_time_between: Option<String>,
    /// The Display Name. Optional.
    #[serde(rename = "displayName__contains", skip_serializing_if = "Option::is_none")]
    pub display_name_contains: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset. Optional. Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The AD Domain Name (not in). Optional.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Optional.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Optional.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The Object Category (not in). Optional.
    #[serde(rename = "objectCategory__nin", skip_serializing_if = "Option::is_none")]
    pub object_category_nin: Option<String>,
    /// The User Account Control (not in). Optional.
    #[serde(rename = "userAccountControl__nin", skip_serializing_if = "Option::is_none")]
    pub user_account_control_nin: Option<String>,
    /// The AD Domain Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The User Password Expiry Time. Optional.
    #[serde(rename = "userPasswordExpiryTimeComputed__between", skip_serializing_if = "Option::is_none")]
    pub user_password_expiry_time_computed_between: Option<String>,
    /// Whether the Identity Group is enabled or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    /// The User Principal Name (not in). Optional.
    #[serde(rename = "userPrincipalName__nin", skip_serializing_if = "Option::is_none")]
    pub user_principal_name_nin: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl FilterCountsQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to (not in).
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(join_csv(v));
        self
    }
    /// The status alerts of the asset (not in).
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(join_csv(v));
        self
    }
    /// The Group Type.
    pub fn group_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_type = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset.
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(join_csv(v));
        self
    }
    /// User and cloud tags.
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(join_csv(v));
        self
    }
    /// The cloud provider account name (not in).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_nin = Some(join_csv(v));
        self
    }
    /// The Object Category.
    pub fn object_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_category = Some(join_csv(v));
        self
    }
    /// The cloud resource ID.
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider subscription ID.
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id_contains = Some(join_csv(v));
        self
    }
    /// Tag Keys (not in).
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(join_csv(v));
        self
    }
    /// Tag Keys exists.
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(join_csv(v));
        self
    }
    /// The region.
    pub fn region<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(join_csv(v));
        self
    }
    /// The User Principal Name.
    pub fn user_principal_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_principal_name = Some(join_csv(v));
        self
    }
    /// Free-text filter by tag key value (supports multiple values).
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key (not in).
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_nin = Some(join_csv(v));
        self
    }
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(join_csv(v));
        self
    }
    /// The risk factors associated with the asset (not in).
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(join_csv(v));
        self
    }
    /// Tag Keys not exists.
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(join_csv(v));
        self
    }
    /// The cloud provider account ID.
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the image name.
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(join_csv(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// The Email Address.
    pub fn mail<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mail = Some(join_csv(v));
        self
    }
    /// On Premises SAM Account Name.
    pub fn on_premises_sam_account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.on_premises_sam_account_name_contains = Some(join_csv(v));
        self
    }
    /// The active coverage for the asset (not in).
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tags (not in).
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// The region (not in).
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys (not in).
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(join_csv(v));
        self
    }
    /// The SAM Account Name.
    pub fn sam_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sam_account_name = Some(join_csv(v));
        self
    }
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// The Surface that each asset belongs to.
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(join_csv(v));
        self
    }
    /// The Forest Name.
    pub fn forest<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.forest = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset (not in).
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(join_csv(v));
        self
    }
    /// The status of the asset (not in).
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(join_csv(v));
        self
    }
    /// On Premises User Principal Name.
    pub fn on_premises_user_principal_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.on_premises_user_principal_name_contains = Some(join_csv(v));
        self
    }
    /// Whether the AD Entity is privileged or not.
    pub fn privileged<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.privileged = Some(join_csv(v));
        self
    }
    /// Whether the password never expires.
    pub fn password_never_expire<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.password_never_expire = Some(join_csv(v));
        self
    }
    /// The geographical area where cloud resources are hosted.
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(join_csv(v));
        self
    }
    /// The asset review (not in).
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(join_csv(v));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name = Some(join_csv(v));
        self
    }
    /// Whether the AD Entity is deleted or not.
    pub fn deleted<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.deleted = Some(join_csv(v));
        self
    }
    /// Asset Contact Email (not in).
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(join_csv(v));
        self
    }
    /// The severity of the alert.
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(join_csv(v));
        self
    }
    /// The Email Address.
    pub fn mail_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mail_contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key value.
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value = Some(join_csv(v));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// on_premises_distinguished_name_contains.
    pub fn on_premises_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.on_premises_distinguished_name_contains = Some(join_csv(v));
        self
    }
    /// The AD Domain Name.
    pub fn domain_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_contains = Some(join_csv(v));
        self
    }
    /// The canonical name for the resource type (not in).
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(join_csv(v));
        self
    }
    /// The LDAP Common Name.
    pub fn cn<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cn = Some(join_csv(v));
        self
    }
    /// The LDAP Common Name.
    pub fn cn_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cn_contains = Some(join_csv(v));
        self
    }
    /// The LDAP Common Name (not in).
    pub fn cn_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cn_nin = Some(join_csv(v));
        self
    }
    /// On Premises Distinguished Name.
    pub fn on_premises_domain_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.on_premises_domain_name_contains = Some(join_csv(v));
        self
    }
    /// The Last time of User Login.
    pub fn last_logon_time_between(mut self, v: impl Into<String>) -> Self {
        self.last_logon_time_between = Some(v.into());
        self
    }
    /// Asset Contact Email.
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(join_csv(v));
        self
    }
    /// The risk factors associated with the asset.
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(join_csv(v));
        self
    }
    /// The Object Class (not in).
    pub fn object_class_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_class_nin = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS \.
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(join_csv(v));
        self
    }
    /// The Surface that each asset belongs to (not in).
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(join_csv(v));
        self
    }
    /// The Forest Name (not in).
    pub fn forest_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.forest_nin = Some(join_csv(v));
        self
    }
    /// on_premises_security_identifier_contains.
    pub fn on_premises_security_identifier_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.on_premises_security_identifier_contains = Some(join_csv(v));
        self
    }
    /// The ID.
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_contains = Some(join_csv(v));
        self
    }
    /// The Asset Type.
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(join_csv(v));
        self
    }
    /// Tags.
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(join_csv(v));
        self
    }
    /// The cloud provider organization unit.
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider organization.
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_contains = Some(join_csv(v));
        self
    }
    /// The SAM Account Name (not in).
    pub fn sam_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sam_account_name_nin = Some(join_csv(v));
        self
    }
    /// The Object SID.
    pub fn object_sid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_sid_contains = Some(join_csv(v));
        self
    }
    /// The Group Type (not in).
    pub fn group_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_type_nin = Some(join_csv(v));
        self
    }
    /// The User Account Control.
    pub fn user_account_control<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_account_control = Some(join_csv(v));
        self
    }
    /// Name.
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(join_csv(v));
        self
    }
    /// Proxy Addresses.
    pub fn proxy_addresses_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_addresses_contains = Some(join_csv(v));
        self
    }
    /// The Object GUID.
    pub fn object_guid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_guid_contains = Some(join_csv(v));
        self
    }
    /// The name.
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to.
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(v));
        self
    }
    /// The cloud provider account id (not in).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_nin = Some(join_csv(v));
        self
    }
    /// Lock-out Time.
    pub fn lock_out_time_between(mut self, v: impl Into<String>) -> Self {
        self.lock_out_time_between = Some(v.into());
        self
    }
    /// User and cloud tag keys not exists.
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(join_csv(v));
        self
    }
    /// The Service Principal Name.
    pub fn service_principal_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.service_principal_name_contains = Some(join_csv(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// The Last Seen date and time for the asset.
    pub fn s_1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s_1_updated_at_between = Some(v.into());
        self
    }
    /// The canonical name for the resource type.
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS \.
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(join_csv(v));
        self
    }
    /// Name (not in).
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(join_csv(v));
        self
    }
    /// The cloud tags key.
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key = Some(join_csv(v));
        self
    }
    /// The sub-category that each resource belongs to (not in).
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(join_csv(v));
        self
    }
    /// The Service Account.
    pub fn service_account<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.service_account = Some(join_csv(v));
        self
    }
    /// The status alerts of the asset.
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(join_csv(v));
        self
    }
    /// Given Name.
    pub fn given_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.given_name_contains = Some(join_csv(v));
        self
    }
    /// The active coverage for the asset.
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(join_csv(v));
        self
    }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(join_csv(v));
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// The User Principal Name.
    pub fn user_principal_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_principal_name_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider account id.
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id = Some(join_csv(v));
        self
    }
    /// The sub-category that each resource belongs to.
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(join_csv(v));
        self
    }
    /// The asset review.
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys.
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(join_csv(v));
        self
    }
    /// Other Mails.
    pub fn other_mails_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.other_mails_contains = Some(join_csv(v));
        self
    }
    /// The Distinguished Name.
    pub fn distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.distinguished_name_contains = Some(join_csv(v));
        self
    }
    /// The Object Class.
    pub fn object_class<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_class = Some(join_csv(v));
        self
    }
    /// The Forest Name.
    pub fn forest_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.forest_contains = Some(join_csv(v));
        self
    }
    /// Bad Password Time.
    pub fn bad_password_time_between(mut self, v: impl Into<String>) -> Self {
        self.bad_password_time_between = Some(v.into());
        self
    }
    /// Lock-out Time.
    pub fn last_modified_time_between(mut self, v: impl Into<String>) -> Self {
        self.last_modified_time_between = Some(v.into());
        self
    }
    /// The Display Name.
    pub fn display_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.display_name_contains = Some(join_csv(v));
        self
    }
    /// The ID.
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(join_csv(v));
        self
    }
    /// The status of the asset.
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(join_csv(v));
        self
    }
    /// The AD Domain Name (not in).
    pub fn domain_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_nin = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key (supports multiple values).
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_contains = Some(join_csv(v));
        self
    }
    /// Tags (not in).
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys exists.
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(join_csv(v));
        self
    }
    /// The Object Category (not in).
    pub fn object_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_category_nin = Some(join_csv(v));
        self
    }
    /// The User Account Control (not in).
    pub fn user_account_control_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_account_control_nin = Some(join_csv(v));
        self
    }
    /// The AD Domain Name.
    pub fn domain<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(join_csv(v));
        self
    }
    /// The User Password Expiry Time.
    pub fn user_password_expiry_time_computed_between(mut self, v: impl Into<String>) -> Self {
        self.user_password_expiry_time_computed_between = Some(v.into());
        self
    }
    /// Whether the Identity Group is enabled or not.
    pub fn enabled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.enabled = Some(join_csv(v));
        self
    }
    /// The User Principal Name (not in).
    pub fn user_principal_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_principal_name_nin = Some(join_csv(v));
        self
    }
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(join_csv(v));
        self
    }
}

/// Comma-join an iterator of string-likes into a single CSV query value.
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

impl InventoryIdentityFiltersService<'_> {
    /// Auto Complete.
    ///
    /// Use this command to get values for other fields. When you send this
    /// command with input text and a field name, it returns auto-complete
    /// suggestions for the field.
    ///
    /// `text` and `key` are required by the API; set them via the
    /// [`AutoCompleteQuery`] builder before calling.
    ///
    /// `GET /web/api/v2.1/xdr/assets/identity/filters/autocomplete`
    pub async fn autocomplete(
        &self,
        query: &AutoCompleteQuery,
    ) -> Result<Response<AutoCompleteResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/identity/filters/autocomplete", q)
            .await?)
    }

    /// Filter counts.
    ///
    /// Get filter counts.
    ///
    /// `GET /web/api/v2.1/xdr/assets/identity/filters/count`
    pub async fn count(
        &self,
        query: &FilterCountsQuery,
    ) -> Result<Response<Vec<CountFiltersResponse>>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/identity/filters/count", q)
            .await?)
    }

    /// Free text filters.
    ///
    /// Get free text filters.
    ///
    /// `GET /web/api/v2.1/xdr/assets/identity/filters/free-text`
    pub async fn free_text(&self) -> Result<Response<Vec<FreeTextFilterResponse>>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/identity/filters/free-text", None)
            .await?)
    }
}

