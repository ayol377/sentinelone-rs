//! Service for the `Inventory Workstation` tag
//! (Inventory Workstation Resource Filters).

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_workstation::{
    AutoCompleteResponse, AvailableActionWithStatusResponse, CountFiltersResponse,
    FreeTextFilterResponse, Workstation,
};
use crate::pagination::{Paginated, Response};

/// `Inventory Workstation` tag - Inventory Workstation Resource Filters.
///
/// Exposes the workstation asset inventory: list/query assets (GET and POST),
/// perform bulk actions, discover available actions, export results, and read
/// the filter metadata (autocomplete, counts, free-text).
pub struct InventoryWorkstationService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Joins an iterator of string-like values into a comma-separated string,
/// as the SentinelOne API expects for array query parameters.
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

/// Query params for `GET /web/api/v2.1/xdr/assets/workstation` ("Assets") and
/// `GET /web/api/v2.1/xdr/assets/workstation/export` ("Export assets to CSV or
/// JSON").
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects. Enum params are kept as `String` for forward-compatibility; the
/// allowed values are documented per field.
///
/// Note: the export endpoint additionally requires `export_format`, which is a
/// dedicated function argument rather than a field on this struct.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkstationQuery {
    /// Free-text filter by tag key (supports multiple values) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The agent operational state (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentOperationalState__nin", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state_nin: Option<String>,
    /// The criticality that each asset belongs to (not in) Optional.
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// Legacy Identity Policy Name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "legacyIdentityPolicyName", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The missing coverage for the asset Optional.
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "missingCoverage", skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The agent console migration status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentConsoleMigrationStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status_nin: Option<String>,
    /// Tag Keys (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// Tag Keys exists Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CPU Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "cpu__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// The memory of the device in human readable format Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "memoryReadable", skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// Tag Keys Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKey", skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The agent VSS protection status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssProtectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The risk factors associated with the asset (not in) Optional.
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor". Optional.
    #[serde(rename = "skip", skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// AD machine or its groups Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdMachine__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_contains: Option<String>,
    /// Is AD Connector Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// The agent location (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentLocation__nin", skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// Free-text filter by the image name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The agent missing permissions Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentMissingPermissions", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The operating system of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The SDL connectivity last active Optional.
    #[serde(rename = "agentDvConnectivityLastUpdatedDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The subnets Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The ranger tags key Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagsKey", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name_nin: Option<String>,
    /// The connection status between the agent and the SDL service Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDvConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The active coverage for the asset (not in) Optional.
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The agent console connectivity Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentConsoleConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentIdrConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// User and cloud tags (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The network name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "networkName", skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// Whether the agent can configure network quarantine (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentConfigurableNetworkQuarantine__nin", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// User and cloud tag keys (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The manufacturer of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer_nin: Option<String>,
    /// The Surface that each asset belongs to Optional.
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "surfaces", skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentPendingActions", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// The operating system name and version of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in) Optional.
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The serial number Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The status of the asset (not in) Optional.
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The last active date Optional.
    #[serde(rename = "lastActiveDt__between", skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// The column to sort the results by. Optional.
    ///
    /// Allowed values: `s1GroupName`, `cpu`, `legacyIdentityPolicyName`, `previousOsType`, `previousOsVersion`, `agentFirewallStatus`, `s1UpdatedAt`, `memoryReadable`, `s1ManagementId`, `isAdConnector`, `os`, `agentLocationAwareness`, `agentDetectionState`, `agentCustomerIdentifier`, `agentDvConnectivity`, `agentConsoleConnectivity`, `agentIdrConnectivity`, `agentUpToDate`, `networkName`, `s1SiteId`, `s1SiteName`, `serialNumber`, `detectedFromSite`, `agentInstallerType`, `identityAdUserDistinguishedName`, `eppUnsupportedUnknown`, `category`, `s1GroupId`, `agentLastLoggedInUser`, `ipAddress`, `agentK8sPod`, `agentDecommissioned`, `lastActiveDt`, `s1OnboardedAccountId`, `assetContactEmail`, `s1ScopeType`, `agentSubscribeOnDt`, `assetEnvironment`, `previousDeviceFunction`, `osFamily`, `s1AccountId`, `adsEnabled`, `id`, `s1AccountName`, `agentUninstalled`, `s1ScopeLevel`, `memory`, `agentId`, `s1OnboardedAccountName`, `s1OnboardedScopeLevel`, `isDcServer`, `agentOperationalState`, `name`, `agentNetworkStatus`, `agentVssServiceStatus`, `assetCriticality`, `s1ScopePath`, `osVersion`, `agentPendingUninstall`, `agentAntiTamperingStatus`, `s1OnboardedGroupName`, `identityAdMachineDistinguishedName`, `lastRebootDt`, `s1OnboardedSiteName`, `s1ScopeId`, `agentHasLocalConfig`, `agentPendingUpgrade`, `manufacturer`, `agentK8sNamespace`, `agentVssRollbackStatus`, `resourceType`, `agentUuid`, `agentOperationalStateExpirationTimeDt`, `coreCount`, `infectionStatus`, `s1OnboardedSiteId`, `agentVssLastSnapshotDt`, `agentDvConnectivityLastUpdatedDt`, `subCategory`, `s1OnboardedScopeId`, `agentRangerStatus`, `deviceReview`, `s1OnboardedScopePath`, `agentConsoleMigrationStatus`, `architecture`, `agentVssProtectionStatus`, `assetStatus`, `agentConfigurableNetworkQuarantine`, `agentRangerVersion`, `agentHealthStatus`, `agentFullDiskScanDt`, `s1OnboardedGroupId`, `agentAgentVersion`, `firstSeenDt`, `domain`, `lastUpdateDt`, `agentDiskEncryption`, `osNameVersion`.
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The site from which the device was detected Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "detectedFromSite", skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// Any AD string Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAd__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// The agent installer type Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentInstallerType", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent disk metrics volume type Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDiskMetricsVolumeType", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The legacy identity policy name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "legacy_identity_policy_name__contains", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// The agent supported or unknown state Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "eppUnsupportedUnknown", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The agent health status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentHealthStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_health_status_nin: Option<String>,
    /// The agent network scanner version (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentRangerVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version_nin: Option<String>,
    /// The asset review (not in) Optional.
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple values) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value_contains: Option<String>,
    /// Asset Contact Email (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__lte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The severity of the alert Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "alertSeverity", skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The serial number (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "serialNumber__nin", skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// Whether the agent is decommissioned Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDecommissioned", skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent subscribe time Optional.
    #[serde(rename = "agentSubscribeOnDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// The domain Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The OS names and versions Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The canonical name for the resource type (not in) Optional.
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The customer identifier Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentCustomerIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub agent_customer_identifier_contains: Option<String>,
    /// The operating system of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// The manufacturer Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// Asset Contact Email Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetContactEmail", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "udpPorts", skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset Optional.
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "riskFactors", skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The agent full disk scan date Optional.
    #[serde(rename = "agentFullDiskScanDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// The agent anti tampering status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentAntiTamperingStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetEnvironment", skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osFamily", skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// AD user or their groups Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdUser__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_contains: Option<String>,
    /// The Surface that each asset belongs to (not in) Optional.
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// ADS Enabled Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "adsEnabled", skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// The ranger tags key (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_nin: Option<String>,
    /// The agent location Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentLocation", skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent pending actions (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentPendingActions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions_nin: Option<String>,
    /// The ID Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Whether the agent is uninstalled Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentUninstalled", skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// The Asset Type Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// The UUID Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentUuid__contains", skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,
    /// Tags Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction Optional.
    ///
    /// Allowed values: `asc`, `desc`.
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The ranger tags key value Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// Is DC Server Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "isDcServer", skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// The location awareness Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentLocationAwareness__contains", skip_serializing_if = "Option::is_none")]
    pub agent_location_awareness_contains: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects. Optional.
    #[serde(rename = "countOnly", skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// AD user DN Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdUserDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// The operating system family of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The agent operational state Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentOperationalState", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent network status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentNetworkStatus", skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// Name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The agent VSS service status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssServiceStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The MAC addresses Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items. Optional.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The agent network scanner status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentRangerStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status_nin: Option<String>,
    /// The site from which the device was detected (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site_nin: Option<String>,
    /// The name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The name of the application installed on a workstation or a server Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "applicationName", skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// Whether the agent is pending uninstall Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// The first seen date Optional.
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The last update date Optional.
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt_between: Option<String>,
    /// The agent anti tampering status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentAntiTamperingStatus", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The agent version (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentAgentVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_nin: Option<String>,
    /// The operating system version of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osVersion", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The criticality that each asset belongs to Optional.
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetCriticality", skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up execution time. Optional.
    #[serde(rename = "skipCount", skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The discovery methods (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods_nin: Option<String>,
    /// The agent VSS service status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssServiceStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent VSS last snapshot date Optional.
    #[serde(rename = "agentVssLastSnapshotDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// Whether the agent has local configuration Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentHasLocalConfig", skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// User and cloud tag keys not exists Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The agent missing permissions (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentMissingPermissions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions_nin: Option<String>,
    /// Whether the agent is pending upgrade Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentPendingUpgrade", skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The manufacturer of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The agent VSS rollback status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssRollbackStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// List of Site IDs to filter by Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent installer type (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentInstallerType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type_nin: Option<String>,
    /// The Last Seen date and time for the asset Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type Optional.
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Match by the agent UUID Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentUuid", skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The number of cores Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "coreCount", skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in) Optional.
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The discovery methods Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "discoveryMethods", skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset Optional.
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "infectionStatus", skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tcpPorts", skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset Optional.
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "activeCoverage", skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "countsFor", skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by Optional.
    #[serde(rename = "csvFilterId", skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_contains: Option<String>,
    /// The ranger tags key value (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value_nin: Option<String>,
    /// The architecture of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// AD machine DN Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdMachineDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// Live update ID Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentS1AgentLiveUpdatesVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// The agent network status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentNetworkStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_network_status_nin: Option<String>,
    /// The sub-category that each resource belongs to Optional.
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "subCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The agent network scanner status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentRangerStatus", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The OS versions Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review Optional.
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "deviceReview", skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKey", skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The serial number Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number_contains: Option<String>,
    /// The hostnames Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// AD machine groups Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdMachineMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// Limit number of returned items (1-1000) Optional.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The agent VSS rollback status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssRollbackStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent console migration status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentConsoleMigrationStatus", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The architecture of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// AD user groups Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdUserMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_membership_contains: Option<String>,
    /// The connection status between the agent and the SDL service (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDvConnectivity__nin", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The agent VSS protection status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssProtectionStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
    /// The ID Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The internal IPs Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The status of the asset Optional.
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetStatus", skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown_nin: Option<String>,
    /// Whether the agent can configure network quarantine Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentConfigurableNetworkQuarantine", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// The agent health status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentHealthStatus", skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent network scanner version Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentRangerVersion", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent disk metrics volume type (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDiskMetricsVolumeType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The domain of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// The last logged in user Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentLastLoggedInUser__contains", skip_serializing_if = "Option::is_none")]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// Tags (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The agent version Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentAgentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__gte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent version Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentAgentVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_contains: Option<String>,
    /// The domain of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes Optional.
    #[serde(rename = "agentVssVolumesDiffAreaFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The agent disk encryption Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDiskEncryption", skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The status alerts of the asset (not in) Optional.
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The operating system name and version of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osNameVersion", skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl WorkstationQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(join_csv(v));
        self
    }
    /// The agent operational state (not in).
    pub fn agent_operational_state_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state_nin = Some(join_csv(v));
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
    /// Legacy Identity Policy Name.
    pub fn legacy_identity_policy_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name = Some(join_csv(v));
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
    /// The agent console migration status (not in).
    pub fn agent_console_migration_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status_nin = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_disk_metrics_free_percentage_between = Some(v.into());
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
    /// The CPU.
    pub fn cpu_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_contains = Some(join_csv(v));
        self
    }
    /// The memory of the device in human readable format.
    pub fn memory_readable<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(join_csv(v));
        self
    }
    /// The IP addresses.
    pub fn ip_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address_contains = Some(join_csv(v));
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
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(join_csv(v));
        self
    }
    /// The agent VSS protection status (not in).
    pub fn agent_vss_protection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status_nin = Some(join_csv(v));
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
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor".
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// AD machine or its groups.
    pub fn identity_ad_machine_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_contains = Some(join_csv(v));
        self
    }
    /// Is AD Connector.
    pub fn is_ad_connector<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(join_csv(v));
        self
    }
    /// The agent location (not in).
    pub fn agent_location_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_nin = Some(join_csv(v));
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
    /// The agent missing permissions.
    pub fn agent_missing_permissions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions = Some(join_csv(v));
        self
    }
    /// The operating system of the device.
    pub fn os<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(join_csv(v));
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
    /// The SDL connectivity last active.
    pub fn agent_dv_connectivity_last_updated_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_dv_connectivity_last_updated_dt_between = Some(v.into());
        self
    }
    /// The subnets.
    pub fn subnets_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets_contains = Some(join_csv(v));
        self
    }
    /// The ranger tags key.
    pub fn ranger_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key = Some(join_csv(v));
        self
    }
    /// The network name (not in).
    pub fn network_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name_nin = Some(join_csv(v));
        self
    }
    /// The connection status between the agent and the SDL service.
    pub fn agent_dv_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity = Some(join_csv(v));
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
    /// The agent console connectivity.
    pub fn agent_console_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_connectivity = Some(join_csv(v));
        self
    }
    /// The agent Idr connectivity.
    pub fn agent_idr_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_idr_connectivity = Some(join_csv(v));
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
    /// The network name.
    pub fn network_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name = Some(join_csv(v));
        self
    }
    /// Whether the agent can configure network quarantine (not in).
    pub fn agent_configurable_network_quarantine_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine_nin = Some(join_csv(v));
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
    /// The gateway IPs.
    pub fn gateway_ips_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips_contains = Some(join_csv(v));
        self
    }
    /// The manufacturer of the device (not in).
    pub fn manufacturer_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_nin = Some(join_csv(v));
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
    /// The agent pending actions.
    pub fn agent_pending_actions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions = Some(join_csv(v));
        self
    }
    /// The operating system name and version of the device (not in).
    pub fn os_name_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_nin = Some(join_csv(v));
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
    /// The serial number.
    pub fn serial_number<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number = Some(join_csv(v));
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
    /// The last active date.
    pub fn last_active_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_active_dt_between = Some(v.into());
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// The site from which the device was detected.
    pub fn detected_from_site<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site = Some(join_csv(v));
        self
    }
    /// Any AD string.
    pub fn identity_ad_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_contains = Some(join_csv(v));
        self
    }
    /// The agent installer type.
    pub fn agent_installer_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type = Some(join_csv(v));
        self
    }
    /// The agent disk metrics volume type.
    pub fn agent_disk_metrics_volume_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type = Some(join_csv(v));
        self
    }
    /// The legacy identity policy name.
    pub fn legacy_identity_policy_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name_contains = Some(join_csv(v));
        self
    }
    /// The agent supported or unknown state.
    pub fn epp_unsupported_unknown<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown = Some(join_csv(v));
        self
    }
    /// The agent health status (not in).
    pub fn agent_health_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status_nin = Some(join_csv(v));
        self
    }
    /// The agent network scanner version (not in).
    pub fn agent_ranger_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version_nin = Some(join_csv(v));
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
    /// Free-text filter by Ranger tag key value (supports multiple values).
    pub fn ranger_tag_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_value_contains = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_lte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_lte = Some(v);
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
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// The serial number (not in).
    pub fn serial_number_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent is decommissioned.
    pub fn agent_decommissioned<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_decommissioned = Some(join_csv(v));
        self
    }
    /// The agent subscribe time.
    pub fn agent_subscribe_on_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_subscribe_on_dt_between = Some(v.into());
        self
    }
    /// The domain.
    pub fn domain_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_contains = Some(join_csv(v));
        self
    }
    /// The OS names and versions.
    pub fn os_name_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_contains = Some(join_csv(v));
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
    /// The gateway MACs.
    pub fn gateway_macs_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs_contains = Some(join_csv(v));
        self
    }
    /// The customer identifier.
    pub fn agent_customer_identifier_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_customer_identifier_contains = Some(join_csv(v));
        self
    }
    /// The operating system of the device (not in).
    pub fn os_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_nin = Some(join_csv(v));
        self
    }
    /// The manufacturer.
    pub fn manufacturer_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_contains = Some(join_csv(v));
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
    /// The UDP ports.
    pub fn udp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports = Some(join_csv(v));
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
    /// The agent full disk scan date.
    pub fn agent_full_disk_scan_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_full_disk_scan_dt_between = Some(v.into());
        self
    }
    /// The agent anti tampering status (not in).
    pub fn agent_anti_tampering_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status_nin = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory.
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(join_csv(v));
        self
    }
    /// The operating system family of the device.
    pub fn os_family<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(join_csv(v));
        self
    }
    /// AD user or their groups.
    pub fn identity_ad_user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_contains = Some(join_csv(v));
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
    /// ADS Enabled.
    pub fn ads_enabled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ads_enabled = Some(join_csv(v));
        self
    }
    /// The ranger tags key (not in).
    pub fn ranger_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_nin = Some(join_csv(v));
        self
    }
    /// The agent location.
    pub fn agent_location<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location = Some(join_csv(v));
        self
    }
    /// The agent pending actions (not in).
    pub fn agent_pending_actions_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions_nin = Some(join_csv(v));
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
    /// Whether the agent is uninstalled.
    pub fn agent_uninstalled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uninstalled = Some(join_csv(v));
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
    /// The memory of the device in human readable format (not in).
    pub fn memory_readable_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable_nin = Some(join_csv(v));
        self
    }
    /// The UUID.
    pub fn agent_uuid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid_contains = Some(join_csv(v));
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
    /// Sort direction.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// The ranger tags key value.
    pub fn ranger_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value = Some(join_csv(v));
        self
    }
    /// Is DC Server.
    pub fn is_dc_server<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_dc_server = Some(join_csv(v));
        self
    }
    /// The location awareness.
    pub fn agent_location_awareness_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_awareness_contains = Some(join_csv(v));
        self
    }
    /// If true, only total number of items will be returned, without any of the actual objects.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// AD user DN.
    pub fn identity_ad_user_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_distinguished_name_contains = Some(join_csv(v));
        self
    }
    /// The operating system family of the device (not in).
    pub fn os_family_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family_nin = Some(join_csv(v));
        self
    }
    /// The agent operational state.
    pub fn agent_operational_state<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state = Some(join_csv(v));
        self
    }
    /// The agent network status.
    pub fn agent_network_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status = Some(join_csv(v));
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
    /// The agent VSS service status.
    pub fn agent_vss_service_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status = Some(join_csv(v));
        self
    }
    /// The MAC addresses.
    pub fn mac_addresses_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses_contains = Some(join_csv(v));
        self
    }
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// The agent network scanner status (not in).
    pub fn agent_ranger_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status_nin = Some(join_csv(v));
        self
    }
    /// The site from which the device was detected (not in).
    pub fn detected_from_site_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site_nin = Some(join_csv(v));
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
    /// The name of the application installed on a workstation or a server.
    pub fn application_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_name = Some(join_csv(v));
        self
    }
    /// Whether the agent is pending uninstall.
    pub fn agent_pending_uninstall<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_uninstall = Some(join_csv(v));
        self
    }
    /// The first seen date.
    pub fn first_seen_dt_between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt_between = Some(v.into());
        self
    }
    /// The last update date.
    pub fn last_update_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_update_dt_between = Some(v.into());
        self
    }
    /// The agent anti tampering status.
    pub fn agent_anti_tampering_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status = Some(join_csv(v));
        self
    }
    /// The agent version (not in).
    pub fn agent_agent_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_nin = Some(join_csv(v));
        self
    }
    /// The operating system version of the device.
    pub fn os_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(join_csv(v));
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
    /// If true, total number of items will not be calculated, which speeds up execution time.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The discovery methods (not in).
    pub fn discovery_methods_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods_nin = Some(join_csv(v));
        self
    }
    /// The agent VSS service status (not in).
    pub fn agent_vss_service_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status_nin = Some(join_csv(v));
        self
    }
    /// The agent VSS last snapshot date.
    pub fn agent_vss_last_snapshot_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_last_snapshot_dt_between = Some(v.into());
        self
    }
    /// Whether the agent has local configuration.
    pub fn agent_has_local_config<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_has_local_config = Some(join_csv(v));
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
    /// The agent missing permissions (not in).
    pub fn agent_missing_permissions_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent is pending upgrade.
    pub fn agent_pending_upgrade<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_upgrade = Some(join_csv(v));
        self
    }
    /// The manufacturer of the device.
    pub fn manufacturer<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer = Some(join_csv(v));
        self
    }
    /// The agent VSS rollback status.
    pub fn agent_vss_rollback_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status = Some(join_csv(v));
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
    /// The agent installer type (not in).
    pub fn agent_installer_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type_nin = Some(join_csv(v));
        self
    }
    /// The Last Seen date and time for the asset.
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
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
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in).
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
    /// Match by the agent UUID.
    pub fn agent_uuid<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid = Some(join_csv(v));
        self
    }
    /// The number of cores.
    pub fn core_count<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(join_csv(v));
        self
    }
    /// The operating system version of the device (not in).
    pub fn os_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_nin = Some(join_csv(v));
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
    /// The discovery methods.
    pub fn discovery_methods<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods = Some(join_csv(v));
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
    /// The TCP ports.
    pub fn tcp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports = Some(join_csv(v));
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
    /// The number of cores (not in).
    pub fn core_count_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count_nin = Some(join_csv(v));
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// Free-text filter by Ranger tag key (supports multiple values).
    pub fn ranger_tag_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_contains = Some(join_csv(v));
        self
    }
    /// The ranger tags key value (not in).
    pub fn ranger_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// The architecture of the device (not in).
    pub fn architecture_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture_nin = Some(join_csv(v));
        self
    }
    /// AD machine DN.
    pub fn identity_ad_machine_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_distinguished_name_contains = Some(join_csv(v));
        self
    }
    /// Live update ID.
    pub fn agent_s1_agent_live_updates_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_s1_agent_live_updates_version_contains = Some(join_csv(v));
        self
    }
    /// The agent network status (not in).
    pub fn agent_network_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status_nin = Some(join_csv(v));
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
    /// The agent network scanner status.
    pub fn agent_ranger_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status = Some(join_csv(v));
        self
    }
    /// The OS versions.
    pub fn os_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(join_csv(v));
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
    /// The serial number.
    pub fn serial_number_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_contains = Some(join_csv(v));
        self
    }
    /// The hostnames.
    pub fn hostnames_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(join_csv(v));
        self
    }
    /// AD machine groups.
    pub fn identity_ad_machine_membership_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_membership_contains = Some(join_csv(v));
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// The agent VSS rollback status (not in).
    pub fn agent_vss_rollback_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status_nin = Some(join_csv(v));
        self
    }
    /// The agent console migration status.
    pub fn agent_console_migration_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status = Some(join_csv(v));
        self
    }
    /// The architecture of the device.
    pub fn architecture<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(join_csv(v));
        self
    }
    /// AD user groups.
    pub fn identity_ad_user_membership_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_membership_contains = Some(join_csv(v));
        self
    }
    /// The connection status between the agent and the SDL service (not in).
    pub fn agent_dv_connectivity_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity_nin = Some(join_csv(v));
        self
    }
    /// The agent VSS protection status.
    pub fn agent_vss_protection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status = Some(join_csv(v));
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
    /// The internal IPs.
    pub fn internal_ips_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips_contains = Some(join_csv(v));
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
    /// The agent supported or unknown state (not in).
    pub fn epp_unsupported_unknown_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent can configure network quarantine.
    pub fn agent_configurable_network_quarantine<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine = Some(join_csv(v));
        self
    }
    /// The agent health status.
    pub fn agent_health_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status = Some(join_csv(v));
        self
    }
    /// The agent network scanner version.
    pub fn agent_ranger_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version = Some(join_csv(v));
        self
    }
    /// The agent disk metrics volume type (not in).
    pub fn agent_disk_metrics_volume_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type_nin = Some(join_csv(v));
        self
    }
    /// The domain of the device (not in).
    pub fn domain_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_nin = Some(join_csv(v));
        self
    }
    /// The last logged in user.
    pub fn agent_last_logged_in_user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_last_logged_in_user_contains = Some(join_csv(v));
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
    /// The agent version.
    pub fn agent_agent_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_gte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_gte = Some(v);
        self
    }
    /// The agent version.
    pub fn agent_agent_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_contains = Some(join_csv(v));
        self
    }
    /// The domain of the device.
    pub fn domain<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(join_csv(v));
        self
    }
    /// The agent free VSS volume percentage on any of the volumes.
    pub fn agent_vss_volumes_diff_area_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_volumes_diff_area_free_percentage_between = Some(v.into());
        self
    }
    /// The agent disk encryption.
    pub fn agent_disk_encryption<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_encryption = Some(join_csv(v));
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
    /// The operating system name and version of the device.
    pub fn os_name_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(join_csv(v));
        self
    }
}

/// Query params for the filter-scoped workstation endpoints:
/// `POST /web/api/v2.1/xdr/assets/workstation/action` ("Perform action"),
/// `POST /web/api/v2.1/xdr/assets/workstation/available-actions/with-status`
/// ("Available actions"),
/// `GET /web/api/v2.1/xdr/assets/workstation/filters/count` ("Filter counts"),
/// and `GET /web/api/v2.1/xdr/assets/workstation/filters/autocomplete`
/// ("Auto Complete").
///
/// This is the full asset filter set without the list-only pagination/sort
/// params (`skip`, `cursor`, `limit`, `countOnly`, `skipCount`, `sortBy`,
/// `sortOrder`). Every field is optional; array params are comma-joined.
///
/// Note: the autocomplete endpoint additionally requires `key` and `text`
/// (and accepts an optional `limit`), which are dedicated function arguments.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkstationActionQuery {
    /// Free-text filter by tag key (supports multiple values) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The agent operational state (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentOperationalState__nin", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state_nin: Option<String>,
    /// The criticality that each asset belongs to (not in) Optional.
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// Legacy Identity Policy Name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "legacyIdentityPolicyName", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The missing coverage for the asset Optional.
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "missingCoverage", skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The agent console migration status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentConsoleMigrationStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status_nin: Option<String>,
    /// Tag Keys (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// Tag Keys exists Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CPU Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "cpu__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// The memory of the device in human readable format Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "memoryReadable", skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// Tag Keys Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKey", skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The agent VSS protection status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssProtectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The risk factors associated with the asset (not in) Optional.
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// AD machine or its groups Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdMachine__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_contains: Option<String>,
    /// Is AD Connector Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// The agent location (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentLocation__nin", skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// Free-text filter by the image name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The agent missing permissions Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentMissingPermissions", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The operating system of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The SDL connectivity last active Optional.
    #[serde(rename = "agentDvConnectivityLastUpdatedDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The subnets Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The ranger tags key Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagsKey", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name_nin: Option<String>,
    /// The connection status between the agent and the SDL service Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDvConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The active coverage for the asset (not in) Optional.
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The agent console connectivity Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentConsoleConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentIdrConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// User and cloud tags (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The network name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "networkName", skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// Whether the agent can configure network quarantine (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentConfigurableNetworkQuarantine__nin", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// User and cloud tag keys (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The manufacturer of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer_nin: Option<String>,
    /// The Surface that each asset belongs to Optional.
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "surfaces", skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentPendingActions", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// The operating system name and version of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in) Optional.
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The serial number Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The status of the asset (not in) Optional.
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The last active date Optional.
    #[serde(rename = "lastActiveDt__between", skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// The site from which the device was detected Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "detectedFromSite", skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// Any AD string Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAd__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// The agent installer type Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentInstallerType", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent disk metrics volume type Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDiskMetricsVolumeType", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The legacy identity policy name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "legacy_identity_policy_name__contains", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// The agent supported or unknown state Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "eppUnsupportedUnknown", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The agent health status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentHealthStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_health_status_nin: Option<String>,
    /// The agent network scanner version (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentRangerVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version_nin: Option<String>,
    /// The asset review (not in) Optional.
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple values) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value_contains: Option<String>,
    /// Asset Contact Email (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__lte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The severity of the alert Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "alertSeverity", skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The serial number (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "serialNumber__nin", skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// Whether the agent is decommissioned Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDecommissioned", skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent subscribe time Optional.
    #[serde(rename = "agentSubscribeOnDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// The domain Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The OS names and versions Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The canonical name for the resource type (not in) Optional.
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The customer identifier Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentCustomerIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub agent_customer_identifier_contains: Option<String>,
    /// The operating system of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// The manufacturer Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// Asset Contact Email Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetContactEmail", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "udpPorts", skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset Optional.
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "riskFactors", skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The agent full disk scan date Optional.
    #[serde(rename = "agentFullDiskScanDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// The agent anti tampering status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentAntiTamperingStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetEnvironment", skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osFamily", skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// AD user or their groups Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdUser__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_contains: Option<String>,
    /// The Surface that each asset belongs to (not in) Optional.
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// ADS Enabled Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "adsEnabled", skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// The ranger tags key (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_nin: Option<String>,
    /// The agent location Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentLocation", skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent pending actions (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentPendingActions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions_nin: Option<String>,
    /// The ID Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Whether the agent is uninstalled Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentUninstalled", skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// The Asset Type Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// The UUID Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentUuid__contains", skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,
    /// Tags Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The ranger tags key value Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// Is DC Server Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "isDcServer", skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// The location awareness Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentLocationAwareness__contains", skip_serializing_if = "Option::is_none")]
    pub agent_location_awareness_contains: Option<String>,
    /// AD user DN Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdUserDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// The operating system family of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The agent operational state Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentOperationalState", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent network status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentNetworkStatus", skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// Name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The agent VSS service status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssServiceStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The MAC addresses Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The agent network scanner status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentRangerStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status_nin: Option<String>,
    /// The site from which the device was detected (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site_nin: Option<String>,
    /// The name Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The name of the application installed on a workstation or a server Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "applicationName", skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// Whether the agent is pending uninstall Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// The first seen date Optional.
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The last update date Optional.
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt_between: Option<String>,
    /// The agent anti tampering status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentAntiTamperingStatus", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The agent version (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentAgentVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_nin: Option<String>,
    /// The operating system version of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osVersion", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The criticality that each asset belongs to Optional.
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetCriticality", skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The discovery methods (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods_nin: Option<String>,
    /// The agent VSS service status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssServiceStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent VSS last snapshot date Optional.
    #[serde(rename = "agentVssLastSnapshotDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// Whether the agent has local configuration Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentHasLocalConfig", skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// User and cloud tag keys not exists Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The agent missing permissions (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentMissingPermissions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions_nin: Option<String>,
    /// Whether the agent is pending upgrade Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentPendingUpgrade", skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The manufacturer of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The agent VSS rollback status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssRollbackStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// List of Site IDs to filter by Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent installer type (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentInstallerType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type_nin: Option<String>,
    /// The Last Seen date and time for the asset Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type Optional.
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Match by the agent UUID Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentUuid", skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The number of cores Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "coreCount", skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in) Optional.
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The discovery methods Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "discoveryMethods", skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset Optional.
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "infectionStatus", skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tcpPorts", skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset Optional.
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "activeCoverage", skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "countsFor", skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by Optional.
    #[serde(rename = "csvFilterId", skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_contains: Option<String>,
    /// The ranger tags key value (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value_nin: Option<String>,
    /// The architecture of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// AD machine DN Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdMachineDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// Live update ID Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentS1AgentLiveUpdatesVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// The agent network status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentNetworkStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_network_status_nin: Option<String>,
    /// The sub-category that each resource belongs to Optional.
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "subCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The agent network scanner status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentRangerStatus", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The OS versions Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review Optional.
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "deviceReview", skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKey", skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The serial number Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number_contains: Option<String>,
    /// The hostnames Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// AD machine groups Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdMachineMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// The agent VSS rollback status (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssRollbackStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent console migration status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentConsoleMigrationStatus", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The architecture of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// AD user groups Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "identityAdUserMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_membership_contains: Option<String>,
    /// The connection status between the agent and the SDL service (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDvConnectivity__nin", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The agent VSS protection status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentVssProtectionStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
    /// The ID Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The internal IPs Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The status of the asset Optional.
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "assetStatus", skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown_nin: Option<String>,
    /// Whether the agent can configure network quarantine Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentConfigurableNetworkQuarantine", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// The agent health status Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentHealthStatus", skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent network scanner version Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentRangerVersion", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent disk metrics volume type (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDiskMetricsVolumeType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The domain of the device (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// The last logged in user Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentLastLoggedInUser__contains", skip_serializing_if = "Option::is_none")]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// Tags (not in) Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The agent version Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentAgentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__gte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent version Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentAgentVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_contains: Option<String>,
    /// The domain of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes Optional.
    #[serde(rename = "agentVssVolumesDiffAreaFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The agent disk encryption Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "agentDiskEncryption", skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The status alerts of the asset (not in) Optional.
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The operating system name and version of the device Optional.
    ///
    /// Array param: serialized comma-joined.
    #[serde(rename = "osNameVersion", skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl WorkstationActionQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(join_csv(v));
        self
    }
    /// The agent operational state (not in).
    pub fn agent_operational_state_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state_nin = Some(join_csv(v));
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
    /// Legacy Identity Policy Name.
    pub fn legacy_identity_policy_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name = Some(join_csv(v));
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
    /// The agent console migration status (not in).
    pub fn agent_console_migration_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status_nin = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_disk_metrics_free_percentage_between = Some(v.into());
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
    /// The CPU.
    pub fn cpu_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_contains = Some(join_csv(v));
        self
    }
    /// The memory of the device in human readable format.
    pub fn memory_readable<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(join_csv(v));
        self
    }
    /// The IP addresses.
    pub fn ip_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address_contains = Some(join_csv(v));
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
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(join_csv(v));
        self
    }
    /// The agent VSS protection status (not in).
    pub fn agent_vss_protection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status_nin = Some(join_csv(v));
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
    /// AD machine or its groups.
    pub fn identity_ad_machine_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_contains = Some(join_csv(v));
        self
    }
    /// Is AD Connector.
    pub fn is_ad_connector<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(join_csv(v));
        self
    }
    /// The agent location (not in).
    pub fn agent_location_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_nin = Some(join_csv(v));
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
    /// The agent missing permissions.
    pub fn agent_missing_permissions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions = Some(join_csv(v));
        self
    }
    /// The operating system of the device.
    pub fn os<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(join_csv(v));
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
    /// The SDL connectivity last active.
    pub fn agent_dv_connectivity_last_updated_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_dv_connectivity_last_updated_dt_between = Some(v.into());
        self
    }
    /// The subnets.
    pub fn subnets_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets_contains = Some(join_csv(v));
        self
    }
    /// The ranger tags key.
    pub fn ranger_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key = Some(join_csv(v));
        self
    }
    /// The network name (not in).
    pub fn network_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name_nin = Some(join_csv(v));
        self
    }
    /// The connection status between the agent and the SDL service.
    pub fn agent_dv_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity = Some(join_csv(v));
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
    /// The agent console connectivity.
    pub fn agent_console_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_connectivity = Some(join_csv(v));
        self
    }
    /// The agent Idr connectivity.
    pub fn agent_idr_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_idr_connectivity = Some(join_csv(v));
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
    /// The network name.
    pub fn network_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name = Some(join_csv(v));
        self
    }
    /// Whether the agent can configure network quarantine (not in).
    pub fn agent_configurable_network_quarantine_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine_nin = Some(join_csv(v));
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
    /// The gateway IPs.
    pub fn gateway_ips_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips_contains = Some(join_csv(v));
        self
    }
    /// The manufacturer of the device (not in).
    pub fn manufacturer_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_nin = Some(join_csv(v));
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
    /// The agent pending actions.
    pub fn agent_pending_actions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions = Some(join_csv(v));
        self
    }
    /// The operating system name and version of the device (not in).
    pub fn os_name_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_nin = Some(join_csv(v));
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
    /// The serial number.
    pub fn serial_number<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number = Some(join_csv(v));
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
    /// The last active date.
    pub fn last_active_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_active_dt_between = Some(v.into());
        self
    }
    /// The site from which the device was detected.
    pub fn detected_from_site<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site = Some(join_csv(v));
        self
    }
    /// Any AD string.
    pub fn identity_ad_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_contains = Some(join_csv(v));
        self
    }
    /// The agent installer type.
    pub fn agent_installer_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type = Some(join_csv(v));
        self
    }
    /// The agent disk metrics volume type.
    pub fn agent_disk_metrics_volume_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type = Some(join_csv(v));
        self
    }
    /// The legacy identity policy name.
    pub fn legacy_identity_policy_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name_contains = Some(join_csv(v));
        self
    }
    /// The agent supported or unknown state.
    pub fn epp_unsupported_unknown<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown = Some(join_csv(v));
        self
    }
    /// The agent health status (not in).
    pub fn agent_health_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status_nin = Some(join_csv(v));
        self
    }
    /// The agent network scanner version (not in).
    pub fn agent_ranger_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version_nin = Some(join_csv(v));
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
    /// Free-text filter by Ranger tag key value (supports multiple values).
    pub fn ranger_tag_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_value_contains = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_lte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_lte = Some(v);
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
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// The serial number (not in).
    pub fn serial_number_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent is decommissioned.
    pub fn agent_decommissioned<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_decommissioned = Some(join_csv(v));
        self
    }
    /// The agent subscribe time.
    pub fn agent_subscribe_on_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_subscribe_on_dt_between = Some(v.into());
        self
    }
    /// The domain.
    pub fn domain_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_contains = Some(join_csv(v));
        self
    }
    /// The OS names and versions.
    pub fn os_name_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_contains = Some(join_csv(v));
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
    /// The gateway MACs.
    pub fn gateway_macs_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs_contains = Some(join_csv(v));
        self
    }
    /// The customer identifier.
    pub fn agent_customer_identifier_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_customer_identifier_contains = Some(join_csv(v));
        self
    }
    /// The operating system of the device (not in).
    pub fn os_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_nin = Some(join_csv(v));
        self
    }
    /// The manufacturer.
    pub fn manufacturer_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_contains = Some(join_csv(v));
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
    /// The UDP ports.
    pub fn udp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports = Some(join_csv(v));
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
    /// The agent full disk scan date.
    pub fn agent_full_disk_scan_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_full_disk_scan_dt_between = Some(v.into());
        self
    }
    /// The agent anti tampering status (not in).
    pub fn agent_anti_tampering_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status_nin = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory.
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(join_csv(v));
        self
    }
    /// The operating system family of the device.
    pub fn os_family<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(join_csv(v));
        self
    }
    /// AD user or their groups.
    pub fn identity_ad_user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_contains = Some(join_csv(v));
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
    /// ADS Enabled.
    pub fn ads_enabled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ads_enabled = Some(join_csv(v));
        self
    }
    /// The ranger tags key (not in).
    pub fn ranger_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_nin = Some(join_csv(v));
        self
    }
    /// The agent location.
    pub fn agent_location<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location = Some(join_csv(v));
        self
    }
    /// The agent pending actions (not in).
    pub fn agent_pending_actions_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions_nin = Some(join_csv(v));
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
    /// Whether the agent is uninstalled.
    pub fn agent_uninstalled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uninstalled = Some(join_csv(v));
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
    /// The memory of the device in human readable format (not in).
    pub fn memory_readable_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable_nin = Some(join_csv(v));
        self
    }
    /// The UUID.
    pub fn agent_uuid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid_contains = Some(join_csv(v));
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
    /// The ranger tags key value.
    pub fn ranger_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value = Some(join_csv(v));
        self
    }
    /// Is DC Server.
    pub fn is_dc_server<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_dc_server = Some(join_csv(v));
        self
    }
    /// The location awareness.
    pub fn agent_location_awareness_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_awareness_contains = Some(join_csv(v));
        self
    }
    /// AD user DN.
    pub fn identity_ad_user_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_distinguished_name_contains = Some(join_csv(v));
        self
    }
    /// The operating system family of the device (not in).
    pub fn os_family_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family_nin = Some(join_csv(v));
        self
    }
    /// The agent operational state.
    pub fn agent_operational_state<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state = Some(join_csv(v));
        self
    }
    /// The agent network status.
    pub fn agent_network_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status = Some(join_csv(v));
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
    /// The agent VSS service status.
    pub fn agent_vss_service_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status = Some(join_csv(v));
        self
    }
    /// The MAC addresses.
    pub fn mac_addresses_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses_contains = Some(join_csv(v));
        self
    }
    /// The agent network scanner status (not in).
    pub fn agent_ranger_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status_nin = Some(join_csv(v));
        self
    }
    /// The site from which the device was detected (not in).
    pub fn detected_from_site_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site_nin = Some(join_csv(v));
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
    /// The name of the application installed on a workstation or a server.
    pub fn application_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_name = Some(join_csv(v));
        self
    }
    /// Whether the agent is pending uninstall.
    pub fn agent_pending_uninstall<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_uninstall = Some(join_csv(v));
        self
    }
    /// The first seen date.
    pub fn first_seen_dt_between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt_between = Some(v.into());
        self
    }
    /// The last update date.
    pub fn last_update_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_update_dt_between = Some(v.into());
        self
    }
    /// The agent anti tampering status.
    pub fn agent_anti_tampering_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status = Some(join_csv(v));
        self
    }
    /// The agent version (not in).
    pub fn agent_agent_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_nin = Some(join_csv(v));
        self
    }
    /// The operating system version of the device.
    pub fn os_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(join_csv(v));
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
    /// The discovery methods (not in).
    pub fn discovery_methods_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods_nin = Some(join_csv(v));
        self
    }
    /// The agent VSS service status (not in).
    pub fn agent_vss_service_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status_nin = Some(join_csv(v));
        self
    }
    /// The agent VSS last snapshot date.
    pub fn agent_vss_last_snapshot_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_last_snapshot_dt_between = Some(v.into());
        self
    }
    /// Whether the agent has local configuration.
    pub fn agent_has_local_config<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_has_local_config = Some(join_csv(v));
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
    /// The agent missing permissions (not in).
    pub fn agent_missing_permissions_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent is pending upgrade.
    pub fn agent_pending_upgrade<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_upgrade = Some(join_csv(v));
        self
    }
    /// The manufacturer of the device.
    pub fn manufacturer<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer = Some(join_csv(v));
        self
    }
    /// The agent VSS rollback status.
    pub fn agent_vss_rollback_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status = Some(join_csv(v));
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
    /// The agent installer type (not in).
    pub fn agent_installer_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type_nin = Some(join_csv(v));
        self
    }
    /// The Last Seen date and time for the asset.
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
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
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in).
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
    /// Match by the agent UUID.
    pub fn agent_uuid<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid = Some(join_csv(v));
        self
    }
    /// The number of cores.
    pub fn core_count<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(join_csv(v));
        self
    }
    /// The operating system version of the device (not in).
    pub fn os_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_nin = Some(join_csv(v));
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
    /// The discovery methods.
    pub fn discovery_methods<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods = Some(join_csv(v));
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
    /// The TCP ports.
    pub fn tcp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports = Some(join_csv(v));
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
    /// The number of cores (not in).
    pub fn core_count_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count_nin = Some(join_csv(v));
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// Free-text filter by Ranger tag key (supports multiple values).
    pub fn ranger_tag_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_contains = Some(join_csv(v));
        self
    }
    /// The ranger tags key value (not in).
    pub fn ranger_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// The architecture of the device (not in).
    pub fn architecture_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture_nin = Some(join_csv(v));
        self
    }
    /// AD machine DN.
    pub fn identity_ad_machine_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_distinguished_name_contains = Some(join_csv(v));
        self
    }
    /// Live update ID.
    pub fn agent_s1_agent_live_updates_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_s1_agent_live_updates_version_contains = Some(join_csv(v));
        self
    }
    /// The agent network status (not in).
    pub fn agent_network_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status_nin = Some(join_csv(v));
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
    /// The agent network scanner status.
    pub fn agent_ranger_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status = Some(join_csv(v));
        self
    }
    /// The OS versions.
    pub fn os_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(join_csv(v));
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
    /// The serial number.
    pub fn serial_number_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_contains = Some(join_csv(v));
        self
    }
    /// The hostnames.
    pub fn hostnames_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(join_csv(v));
        self
    }
    /// AD machine groups.
    pub fn identity_ad_machine_membership_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_membership_contains = Some(join_csv(v));
        self
    }
    /// The agent VSS rollback status (not in).
    pub fn agent_vss_rollback_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status_nin = Some(join_csv(v));
        self
    }
    /// The agent console migration status.
    pub fn agent_console_migration_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status = Some(join_csv(v));
        self
    }
    /// The architecture of the device.
    pub fn architecture<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(join_csv(v));
        self
    }
    /// AD user groups.
    pub fn identity_ad_user_membership_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_membership_contains = Some(join_csv(v));
        self
    }
    /// The connection status between the agent and the SDL service (not in).
    pub fn agent_dv_connectivity_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity_nin = Some(join_csv(v));
        self
    }
    /// The agent VSS protection status.
    pub fn agent_vss_protection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status = Some(join_csv(v));
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
    /// The internal IPs.
    pub fn internal_ips_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips_contains = Some(join_csv(v));
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
    /// The agent supported or unknown state (not in).
    pub fn epp_unsupported_unknown_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent can configure network quarantine.
    pub fn agent_configurable_network_quarantine<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine = Some(join_csv(v));
        self
    }
    /// The agent health status.
    pub fn agent_health_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status = Some(join_csv(v));
        self
    }
    /// The agent network scanner version.
    pub fn agent_ranger_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version = Some(join_csv(v));
        self
    }
    /// The agent disk metrics volume type (not in).
    pub fn agent_disk_metrics_volume_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type_nin = Some(join_csv(v));
        self
    }
    /// The domain of the device (not in).
    pub fn domain_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_nin = Some(join_csv(v));
        self
    }
    /// The last logged in user.
    pub fn agent_last_logged_in_user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_last_logged_in_user_contains = Some(join_csv(v));
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
    /// The agent version.
    pub fn agent_agent_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_gte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_gte = Some(v);
        self
    }
    /// The agent version.
    pub fn agent_agent_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_contains = Some(join_csv(v));
        self
    }
    /// The domain of the device.
    pub fn domain<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(join_csv(v));
        self
    }
    /// The agent free VSS volume percentage on any of the volumes.
    pub fn agent_vss_volumes_diff_area_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_volumes_diff_area_free_percentage_between = Some(v.into());
        self
    }
    /// The agent disk encryption.
    pub fn agent_disk_encryption<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_encryption = Some(join_csv(v));
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
    /// The operating system name and version of the device.
    pub fn os_name_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(join_csv(v));
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/workstation` ("Assets using
/// POST"). All optional; serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkstationPostQuery {
    /// List of Account IDs to filter by. Optional. Array param: comma-joined.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Optional. Array param: comma-joined.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Optional. Array param: comma-joined.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl WorkstationPostQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
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
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/workstation`
/// (`WorkstationViewInputSchema`).
///
/// `filter` is required by the spec; `data` is an optional/nullable open object.
#[derive(Debug, Default, Serialize)]
pub struct WorkstationViewBody {
    /// Filter. Required. Free-form object (`PaginatedWorkstationFilter`); pass
    /// the desired filter fields as a JSON object.
    pub filter: serde_json::Value,
    /// Data. Optional/nullable. Open object (`EmptyStrict`) in the spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl WorkstationViewBody {
    /// Construct a body from the required `filter` object.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { filter, data: None }
    }
    /// Set the optional `data` object.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/workstation/action`
/// (`WorkstationActionPayloadSchema`).
///
/// `action_name` is required by the spec.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkstationActionBody {
    /// Action name. Required.
    ///
    /// Allowed values (enum, kept as `String` for forward-compat):
    /// `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`,
    /// `add_note`, `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`,
    /// `clear_tags`.
    pub action_name: String,
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000). Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl WorkstationActionBody {
    /// Construct an action body from the required `action_name`.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self { action_name: action_name.into(), id_in: None, id_nin: None }
    }
    /// List of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from select_all (max 5000).
    pub fn id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(v.into_iter().map(Into::into).collect());
        self
    }
}

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/workstation/available-actions/with-status`
/// (`AffectedResourcesSchema`). All fields optional.
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
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from select_all (max 5000).
    pub fn id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(v.into_iter().map(Into::into).collect());
        self
    }
}

impl InventoryWorkstationService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/workstation` - Assets.
    ///
    /// Get inventory workstation assets.
    pub async fn list(
        &self,
        query: &WorkstationQuery,
    ) -> Result<Paginated<Workstation>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/workstation", q)
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/workstation` - Assets using POST.
    ///
    /// POST API to get workstation assets.
    pub async fn list_post(
        &self,
        query: &WorkstationPostQuery,
        body: &WorkstationViewBody,
    ) -> Result<Paginated<Workstation>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<WorkstationViewBody, Paginated<Workstation>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/workstation",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/workstation/action` - Perform action.
    ///
    /// Perform action on selected assets.
    pub async fn perform_action(
        &self,
        query: &WorkstationActionQuery,
        body: &WorkstationActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<WorkstationActionBody, Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/workstation/action",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/workstation/available-actions/with-status`
    /// - Available actions.
    ///
    /// Get inventory workstation available actions.
    pub async fn available_actions(
        &self,
        query: &WorkstationActionQuery,
        body: &AvailableActionsBody,
    ) -> Result<Response<AvailableActionWithStatusResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<AvailableActionsBody, Response<AvailableActionWithStatusResponse>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/workstation/available-actions/with-status",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/workstation/export` - Export assets to CSV
    /// or JSON.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format.
    ///
    /// `export_format`: Export format. Required (query). Allowed values: `csv`,
    /// `json`.
    pub async fn export(
        &self,
        export_format: impl Into<String>,
        query: &WorkstationQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let mut qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let ef = serde_urlencoded::to_string([("exportFormat", export_format.into())])
            .unwrap_or_default();
        if qs.is_empty() {
            qs = ef;
        } else {
            qs.push('&');
            qs.push_str(&ef);
        }
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/workstation/export", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/workstation/filters/autocomplete` - Auto
    /// Complete.
    ///
    /// Use this command to get values for other fields. When you send this
    /// command with input text and a field name, it returns auto-complete
    /// suggestions for the field.
    ///
    /// `key`: Search field key. Required (query). Allowed values:
    /// `resourceType__contains`, `id__contains`, `name__contains`,
    /// `tagsKey__contains`, `tagsKeyValue__contains`, `domain__contains`,
    /// `gatewayMacs__contains`, `gatewayIps__contains`.
    ///
    /// `text`: Search term text. Required (query).
    ///
    /// `limit`: Optional limit on the number of returned items (query).
    pub async fn autocomplete(
        &self,
        key: impl Into<String>,
        text: impl Into<String>,
        limit: Option<i64>,
        query: &WorkstationActionQuery,
    ) -> Result<Response<AutoCompleteResponse>, Error> {
        let mut qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let mut required: Vec<(String, String)> =
            vec![("key".to_owned(), key.into()), ("text".to_owned(), text.into())];
        if let Some(l) = limit {
            required.push(("limit".to_owned(), l.to_string()));
        }
        let extra = serde_urlencoded::to_string(&required).unwrap_or_default();
        if qs.is_empty() {
            qs = extra;
        } else {
            qs.push('&');
            qs.push_str(&extra);
        }
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/workstation/filters/autocomplete", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/workstation/filters/count` - Filter counts.
    ///
    /// Get workstation filter counts.
    pub async fn filter_counts(
        &self,
        query: &WorkstationActionQuery,
    ) -> Result<Response<Vec<CountFiltersResponse>>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/workstation/filters/count", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/workstation/filters/free-text` - Free text
    /// filters.
    ///
    /// Get Workstation free text filters.
    pub async fn free_text_filters(
        &self,
    ) -> Result<Response<Vec<FreeTextFilterResponse>>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/workstation/filters/free-text", None)
            .await?)
    }
}

