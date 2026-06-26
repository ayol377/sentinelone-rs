use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_endpoint_surface::{
    AvailableActionWithStatusResponse, EndpointResponse,
};
use crate::pagination::{Paginated, Response};

/// Service for the `Inventory Endpoint Surface` tag of the SentinelOne
/// Management API.
///
/// Covers the XDR inventory endpoint-surface resources: listing endpoint
/// assets (via `GET` or `POST`), exporting them to CSV/JSON, performing
/// actions on a selection, and querying available actions with status.
pub struct InventoryEndpointSurfaceService<'a> {
    pub(crate) client: &'a ManagementClient,
}


/// Query params for `GET /web/api/v2.1/xdr/assets/surface/endpoint`
/// (Get inventory endpoint assets).
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
pub struct InventoryEndpointSurfaceListQuery {
    /// Free-text filter by tag key (supports multiple values) Optional.
    #[serde(rename = "tagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The agent operational state (not in) Optional.
    #[serde(rename = "agentOperationalState__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state_nin: Option<String>,
    /// The criticality that each asset belongs to (not in) Allowed values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// Legacy Identity Policy Name Optional.
    #[serde(rename = "legacyIdentityPolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The missing coverage for the asset Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "missingCoverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags Optional.
    #[serde(rename = "allTagsKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The agent console migration status (not in) Optional.
    #[serde(rename = "agentConsoleMigrationStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status_nin: Option<String>,
    /// Tag Keys (not in) Optional.
    #[serde(rename = "tagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// Tag Keys exists Optional.
    #[serde(rename = "tagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CPU Optional.
    #[serde(rename = "cpu__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// The memory of the device in human readable format Optional.
    #[serde(rename = "memoryReadable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses Optional.
    #[serde(rename = "ipAddress__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values) Optional.
    #[serde(rename = "tagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// Tag Keys Optional.
    #[serde(rename = "tagsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The agent VSS protection status (not in) Optional.
    #[serde(rename = "agentVssProtectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The risk factors associated with the asset (not in) Allowed values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists Optional.
    #[serde(rename = "tagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor". Optional.
    #[serde(rename = "skip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// AD machine or its groups Optional.
    #[serde(rename = "identityAdMachine__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_contains: Option<String>,
    /// The operating system of the device Optional.
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Is AD Connector Optional.
    #[serde(rename = "isAdConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// Free-text filter by the image name Optional.
    #[serde(rename = "imageName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The agent missing permissions Optional.
    #[serde(rename = "agentMissingPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The agent location (not in) Optional.
    #[serde(rename = "agentLocation__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// List of Group IDs to filter by Optional.
    #[serde(rename = "groupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The SDL connectivity last active Optional.
    #[serde(rename = "agentDvConnectivityLastUpdatedDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The subnets Optional.
    #[serde(rename = "subnets__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The connection status between the agent and the SDL service Optional.
    #[serde(rename = "agentDvConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The active coverage for the asset (not in) Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "activeCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The agent console connectivity Optional.
    #[serde(rename = "agentConsoleConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity Optional.
    #[serde(rename = "agentIdrConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// User and cloud tags (not in) Optional.
    #[serde(rename = "allTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// Whether the agent can configure network quarantine (not in) Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// User and cloud tag keys (not in) Optional.
    #[serde(rename = "allTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs Optional.
    #[serde(rename = "gatewayIps__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The Surface that each asset belongs to Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. Optional.
    #[serde(rename = "surfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions Optional.
    #[serde(rename = "agentPendingActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// The operating system name and version of the device (not in) Optional.
    #[serde(rename = "osNameVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in) Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "missingCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The serial number Optional.
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The status of the asset (not in) Allowed values: `Active`, `Inactive`. Optional.
    #[serde(rename = "assetStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The last active date Optional.
    #[serde(rename = "lastActiveDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// The column to sort the results by. Allowed values: `s1GroupName`, `cpu`, `legacyIdentityPolicyName`, `previousOsType`, `previousOsVersion`, `agentFirewallStatus`, `s1UpdatedAt`, `memoryReadable`, `s1ManagementId`, `isAdConnector`, `os`, `agentLocationAwareness`, `agentDetectionState`, `agentCustomerIdentifier`, `agentDvConnectivity`, `agentConsoleConnectivity`, `agentIdrConnectivity`, `agentUpToDate`, `networkName`, `s1SiteId`, `s1SiteName`, `serialNumber`, `detectedFromSite`, `agentInstallerType`, `identityAdUserDistinguishedName`, `eppUnsupportedUnknown`, `category`, `s1GroupId`, `agentLastLoggedInUser`, `ipAddress`, `agentK8sPod`, `agentDecommissioned`, `lastActiveDt`, `s1OnboardedAccountId`, `assetContactEmail`, `s1ScopeType`, `agentSubscribeOnDt`, `assetEnvironment`, `previousDeviceFunction`, `osFamily`, `s1AccountId`, `adsEnabled`, `id`, `s1AccountName`, `agentUninstalled`, `s1ScopeLevel`, `memory`, `agentId`, `s1OnboardedAccountName`, `s1OnboardedScopeLevel`, `isDcServer`, `agentOperationalState`, `agentNetworkStatus`, `name`, `agentVssServiceStatus`, `agentPendingUninstall`, `agentAntiTamperingStatus`, `assetCriticality`, `s1ScopePath`, `osVersion`, `s1OnboardedGroupName`, `identityAdMachineDistinguishedName`, `lastRebootDt`, `s1OnboardedSiteName`, `s1ScopeId`, `agentHasLocalConfig`, `agentPendingUpgrade`, `manufacturer`, `agentK8sNamespace`, `agentVssRollbackStatus`, `resourceType`, `agentUuid`, `agentOperationalStateExpirationTimeDt`, `coreCount`, `infectionStatus`, `s1OnboardedSiteId`, `agentVssLastSnapshotDt`, `agentDvConnectivityLastUpdatedDt`, `subCategory`, `s1OnboardedScopeId`, `agentRangerStatus`, `deviceReview`, `s1OnboardedScopePath`, `agentConsoleMigrationStatus`, `architecture`, `agentVssProtectionStatus`, `assetStatus`, `agentConfigurableNetworkQuarantine`, `agentRangerVersion`, `agentHealthStatus`, `agentFullDiskScanDt`, `s1OnboardedGroupId`, `agentAgentVersion`, `firstSeenDt`, `domain`, `lastUpdateDt`, `agentDiskEncryption`, `osNameVersion`. Optional.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Any AD string Optional.
    #[serde(rename = "identityAd__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// The agent installer type Optional.
    #[serde(rename = "agentInstallerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent disk metrics volume type Optional.
    #[serde(rename = "agentDiskMetricsVolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The legacy identity policy name Optional.
    #[serde(rename = "legacy_identity_policy_name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// The agent health status (not in) Optional.
    #[serde(rename = "agentHealthStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_status_nin: Option<String>,
    /// The agent network scanner version (not in) Optional.
    #[serde(rename = "agentRangerVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version_nin: Option<String>,
    /// The asset review (not in) Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. Optional.
    #[serde(rename = "deviceReview__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Asset Contact Email (not in) Optional.
    #[serde(rename = "assetContactEmail__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__lte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The severity of the alert Optional.
    #[serde(rename = "alertSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by Optional.
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The serial number (not in) Optional.
    #[serde(rename = "serialNumber__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// Whether the agent is decommissioned Optional.
    #[serde(rename = "agentDecommissioned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent subscribe time Optional.
    #[serde(rename = "agentSubscribeOnDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// The OS names and versions Optional.
    #[serde(rename = "osNameVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The domain Optional.
    #[serde(rename = "domain__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in) Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`. Optional.
    #[serde(rename = "resourceType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs Optional.
    #[serde(rename = "gatewayMacs__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The customer identifier Optional.
    #[serde(rename = "agentCustomerIdentifier__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_customer_identifier_contains: Option<String>,
    /// The operating system of the device (not in) Optional.
    #[serde(rename = "os__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// Asset Contact Email Optional.
    #[serde(rename = "assetContactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset Allowed values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The agent full disk scan date Optional.
    #[serde(rename = "agentFullDiskScanDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// The agent anti tampering status (not in) Optional.
    #[serde(rename = "agentAntiTamperingStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory Optional.
    #[serde(rename = "assetEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device Optional.
    #[serde(rename = "osFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// AD user or their groups Optional.
    #[serde(rename = "identityAdUser__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_contains: Option<String>,
    /// The Surface that each asset belongs to (not in) Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. Optional.
    #[serde(rename = "surfaces__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// ADS Enabled Optional.
    #[serde(rename = "adsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// The agent location Optional.
    #[serde(rename = "agentLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent pending actions (not in) Optional.
    #[serde(rename = "agentPendingActions__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions_nin: Option<String>,
    /// The ID Optional.
    #[serde(rename = "id__in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Whether the agent is uninstalled Optional.
    #[serde(rename = "agentUninstalled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// The Asset Type Optional.
    #[serde(rename = "resourceType__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in) Optional.
    #[serde(rename = "memoryReadable__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// The UUID Optional.
    #[serde(rename = "agentUuid__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,
    /// Tags Optional.
    #[serde(rename = "tagsKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction Allowed values: `asc`, `desc`. Optional.
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Is DC Server Optional.
    #[serde(rename = "isDcServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// The location awareness Optional.
    #[serde(rename = "agentLocationAwareness__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location_awareness_contains: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects. Optional.
    #[serde(rename = "countOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// AD user DN Optional.
    #[serde(rename = "identityAdUserDistinguishedName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items. Optional.
    #[serde(rename = "cursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The agent operational state Optional.
    #[serde(rename = "agentOperationalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent network status Optional.
    #[serde(rename = "agentNetworkStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// Name Optional.
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The agent VSS service status Optional.
    #[serde(rename = "agentVssServiceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The MAC addresses Optional.
    #[serde(rename = "macAddresses__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The agent network scanner status (not in) Optional.
    #[serde(rename = "agentRangerStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status_nin: Option<String>,
    /// The agent version (not in) Optional.
    #[serde(rename = "agentAgentVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_nin: Option<String>,
    /// The name Optional.
    #[serde(rename = "name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The operating system version of the device Optional.
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Whether the agent is pending uninstall Optional.
    #[serde(rename = "agentPendingUninstall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// The agent anti tampering status Optional.
    #[serde(rename = "agentAntiTamperingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The first seen date Optional.
    #[serde(rename = "firstSeenDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The name of the application installed on a workstation or a server Optional.
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// The criticality that each asset belongs to Allowed values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up execution time. Optional.
    #[serde(rename = "skipCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The agent VSS service status (not in) Optional.
    #[serde(rename = "agentVssServiceStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent VSS last snapshot date Optional.
    #[serde(rename = "agentVssLastSnapshotDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// Whether the agent has local configuration Optional.
    #[serde(rename = "agentHasLocalConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// User and cloud tag keys not exists Optional.
    #[serde(rename = "allTagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The agent missing permissions (not in) Optional.
    #[serde(rename = "agentMissingPermissions__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions_nin: Option<String>,
    /// Whether the agent is pending upgrade Optional.
    #[serde(rename = "agentPendingUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The agent VSS rollback status Optional.
    #[serde(rename = "agentVssRollbackStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// List of Site IDs to filter by Optional.
    #[serde(rename = "siteIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent installer type (not in) Optional.
    #[serde(rename = "agentInstallerType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_type_nin: Option<String>,
    /// The Last Seen date and time for the asset Optional.
    #[serde(rename = "s1UpdatedAt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`. Optional.
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in) Optional.
    #[serde(rename = "assetEnvironment__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in) Optional.
    #[serde(rename = "names__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Match by the agent UUID Optional.
    #[serde(rename = "agentUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The number of cores Optional.
    #[serde(rename = "coreCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in) Optional.
    #[serde(rename = "osVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in) Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`. Optional.
    #[serde(rename = "subCategory__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset Allowed values: `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "activeCoverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for Optional.
    #[serde(rename = "countsFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in) Optional.
    #[serde(rename = "coreCount__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by Optional.
    #[serde(rename = "csvFilterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The architecture of the device (not in) Optional.
    #[serde(rename = "architecture__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// AD machine DN Optional.
    #[serde(rename = "identityAdMachineDistinguishedName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// Live update ID Optional.
    #[serde(rename = "agentS1AgentLiveUpdatesVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// The agent network status (not in) Optional.
    #[serde(rename = "agentNetworkStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_status_nin: Option<String>,
    /// The sub-category that each resource belongs to Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`. Optional.
    #[serde(rename = "subCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The agent network scanner status Optional.
    #[serde(rename = "agentRangerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The OS versions Optional.
    #[serde(rename = "osVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. Optional.
    #[serde(rename = "deviceReview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys Optional.
    #[serde(rename = "allTagsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The serial number Optional.
    #[serde(rename = "serialNumber__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number_contains: Option<String>,
    /// The hostnames Optional.
    #[serde(rename = "hostnames__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// AD machine groups Optional.
    #[serde(rename = "identityAdMachineMembership__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// Limit number of returned items (1-1000) Optional.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The agent VSS rollback status (not in) Optional.
    #[serde(rename = "agentVssRollbackStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent console migration status Optional.
    #[serde(rename = "agentConsoleMigrationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The architecture of the device Optional.
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// AD user groups Optional.
    #[serde(rename = "identityAdUserMembership__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_membership_contains: Option<String>,
    /// The connection status between the agent and the SDL service (not in) Optional.
    #[serde(rename = "agentDvConnectivity__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The agent VSS protection status Optional.
    #[serde(rename = "agentVssProtectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
    /// The ID Optional.
    #[serde(rename = "id__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The internal IPs Optional.
    #[serde(rename = "internalIps__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The status of the asset Allowed values: `Active`, `Inactive`. Optional.
    #[serde(rename = "assetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Whether the agent can configure network quarantine Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// The agent health status Optional.
    #[serde(rename = "agentHealthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent network scanner version Optional.
    #[serde(rename = "agentRangerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent disk encryption Optional.
    #[serde(rename = "agentDiskEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The domain of the device (not in) Optional.
    #[serde(rename = "domain__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// The agent disk metrics volume type (not in) Optional.
    #[serde(rename = "agentDiskMetricsVolumeType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The last logged in user Optional.
    #[serde(rename = "agentLastLoggedInUser__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// Tags (not in) Optional.
    #[serde(rename = "tagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The agent version Optional.
    #[serde(rename = "agentAgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists Optional.
    #[serde(rename = "allTagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__gte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent version Optional.
    #[serde(rename = "agentAgentVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_contains: Option<String>,
    /// The domain of the device Optional.
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes Optional.
    #[serde(rename = "agentVssVolumesDiffAreaFreePercentage__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The operating system family of the device (not in) Optional.
    #[serde(rename = "osFamily__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The status alerts of the asset (not in) Allowed values: `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The operating system name and version of the device Optional.
    #[serde(rename = "osNameVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl InventoryEndpointSurfaceListQuery {
    pub fn tags_key_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_operational_state_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_criticality_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn legacy_identity_policy_name<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn missing_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_migration_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_disk_metrics_free_percentage_between = Some(v.into());
        self
    }
    pub fn tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn cpu_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn memory_readable<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn ip_address_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_protection_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn risk_factors_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    pub fn identity_ad_machine_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn is_ad_connector<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn image_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_missing_permissions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity_last_updated_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_dv_connectivity_last_updated_dt_between = Some(v.into());
        self
    }
    pub fn subnets_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn active_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_idr_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_idr_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_configurable_network_quarantine_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn gateway_ips_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn surfaces<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_actions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_name_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn missing_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn last_active_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_active_dt_between = Some(v.into());
        self
    }
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    pub fn identity_ad_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_installer_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_volume_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn legacy_identity_policy_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_health_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn device_review_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_contact_email_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_lte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_lte = Some(v);
        self
    }
    pub fn alert_severity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_decommissioned = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_subscribe_on_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_subscribe_on_dt_between = Some(v.into());
        self
    }
    pub fn os_name_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn resource_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn gateway_macs_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_customer_identifier_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_customer_identifier_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_contact_email<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn risk_factors<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_full_disk_scan_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_full_disk_scan_dt_between = Some(v.into());
        self
    }
    pub fn agent_anti_tampering_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_environment<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_family<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_user_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn surfaces_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn ads_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ads_enabled = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_actions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn id_in<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uninstalled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uninstalled = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn resource_type_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn memory_readable_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uuid_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    pub fn is_dc_server<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_dc_server = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location_awareness_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_awareness_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    pub fn identity_ad_user_distinguished_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_distinguished_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    pub fn agent_operational_state<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_network_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn names<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_service_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn mac_addresses_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_agent_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_uninstall<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_uninstall = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_anti_tampering_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn first_seen_dt_between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt_between = Some(v.into());
        self
    }
    pub fn application_name<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_name = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_criticality<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    pub fn agent_vss_service_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_last_snapshot_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_last_snapshot_dt_between = Some(v.into());
        self
    }
    pub fn agent_has_local_config<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_has_local_config = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_missing_permissions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_upgrade<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_upgrade = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_rollback_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_installer_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    pub fn resource_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_environment_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn names_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uuid<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn core_count<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn sub_category_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn infection_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn active_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn counts_for<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn core_count_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    pub fn architecture_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_machine_distinguished_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_distinguished_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_s1_agent_live_updates_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_s1_agent_live_updates_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_network_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn sub_category<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn device_review<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn hostnames_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_machine_membership_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_membership_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    pub fn agent_vss_rollback_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_migration_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn architecture<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_user_membership_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_membership_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_protection_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn internal_ips_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_configurable_network_quarantine<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_health_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_encryption<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_encryption = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_volume_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_last_logged_in_user_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_last_logged_in_user_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_agent_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_gte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_gte = Some(v);
        self
    }
    pub fn agent_agent_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_volumes_diff_area_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_volumes_diff_area_free_percentage_between = Some(v.into());
        self
    }
    pub fn os_family_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn infection_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_name_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
}


/// Query params for `POST /web/api/v2.1/xdr/assets/surface/endpoint`
/// (Assets using POST). Scope filters only; the full filter goes in the body.
#[derive(Debug, Default, Serialize)]
pub struct InventoryEndpointSurfaceListPostQuery {
    /// List of Account IDs to filter by Optional.
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by Optional.
    #[serde(rename = "siteIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by Optional.
    #[serde(rename = "groupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl InventoryEndpointSurfaceListPostQuery {
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
}


/// Query params for `POST /web/api/v2.1/xdr/assets/surface/endpoint/action`
/// (Perform action on selected assets). Every field is optional.
#[derive(Debug, Default, Serialize)]
pub struct InventoryEndpointSurfaceActionQuery {
    /// Free-text filter by tag key (supports multiple values) Optional.
    #[serde(rename = "tagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The agent operational state (not in) Optional.
    #[serde(rename = "agentOperationalState__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state_nin: Option<String>,
    /// The criticality that each asset belongs to (not in) Allowed values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// Legacy Identity Policy Name Optional.
    #[serde(rename = "legacyIdentityPolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The missing coverage for the asset Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "missingCoverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags Optional.
    #[serde(rename = "allTagsKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The agent console migration status (not in) Optional.
    #[serde(rename = "agentConsoleMigrationStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status_nin: Option<String>,
    /// Tag Keys (not in) Optional.
    #[serde(rename = "tagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// Tag Keys exists Optional.
    #[serde(rename = "tagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CPU Optional.
    #[serde(rename = "cpu__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// The memory of the device in human readable format Optional.
    #[serde(rename = "memoryReadable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses Optional.
    #[serde(rename = "ipAddress__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values) Optional.
    #[serde(rename = "tagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// Tag Keys Optional.
    #[serde(rename = "tagsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The agent VSS protection status (not in) Optional.
    #[serde(rename = "agentVssProtectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The risk factors associated with the asset (not in) Allowed values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists Optional.
    #[serde(rename = "tagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// AD machine or its groups Optional.
    #[serde(rename = "identityAdMachine__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_contains: Option<String>,
    /// The operating system of the device Optional.
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Is AD Connector Optional.
    #[serde(rename = "isAdConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// Free-text filter by the image name Optional.
    #[serde(rename = "imageName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The agent missing permissions Optional.
    #[serde(rename = "agentMissingPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The agent location (not in) Optional.
    #[serde(rename = "agentLocation__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// List of Group IDs to filter by Optional.
    #[serde(rename = "groupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The SDL connectivity last active Optional.
    #[serde(rename = "agentDvConnectivityLastUpdatedDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The subnets Optional.
    #[serde(rename = "subnets__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The connection status between the agent and the SDL service Optional.
    #[serde(rename = "agentDvConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The active coverage for the asset (not in) Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "activeCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The agent console connectivity Optional.
    #[serde(rename = "agentConsoleConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity Optional.
    #[serde(rename = "agentIdrConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// User and cloud tags (not in) Optional.
    #[serde(rename = "allTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// Whether the agent can configure network quarantine (not in) Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// User and cloud tag keys (not in) Optional.
    #[serde(rename = "allTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs Optional.
    #[serde(rename = "gatewayIps__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The Surface that each asset belongs to Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. Optional.
    #[serde(rename = "surfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions Optional.
    #[serde(rename = "agentPendingActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// The operating system name and version of the device (not in) Optional.
    #[serde(rename = "osNameVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in) Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "missingCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The serial number Optional.
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The status of the asset (not in) Allowed values: `Active`, `Inactive`. Optional.
    #[serde(rename = "assetStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The last active date Optional.
    #[serde(rename = "lastActiveDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// Any AD string Optional.
    #[serde(rename = "identityAd__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// The agent installer type Optional.
    #[serde(rename = "agentInstallerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent disk metrics volume type Optional.
    #[serde(rename = "agentDiskMetricsVolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The legacy identity policy name Optional.
    #[serde(rename = "legacy_identity_policy_name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// The agent health status (not in) Optional.
    #[serde(rename = "agentHealthStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_status_nin: Option<String>,
    /// The agent network scanner version (not in) Optional.
    #[serde(rename = "agentRangerVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version_nin: Option<String>,
    /// The asset review (not in) Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. Optional.
    #[serde(rename = "deviceReview__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Asset Contact Email (not in) Optional.
    #[serde(rename = "assetContactEmail__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__lte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The severity of the alert Optional.
    #[serde(rename = "alertSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by Optional.
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The serial number (not in) Optional.
    #[serde(rename = "serialNumber__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// Whether the agent is decommissioned Optional.
    #[serde(rename = "agentDecommissioned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent subscribe time Optional.
    #[serde(rename = "agentSubscribeOnDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// The OS names and versions Optional.
    #[serde(rename = "osNameVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The domain Optional.
    #[serde(rename = "domain__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in) Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`. Optional.
    #[serde(rename = "resourceType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs Optional.
    #[serde(rename = "gatewayMacs__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The customer identifier Optional.
    #[serde(rename = "agentCustomerIdentifier__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_customer_identifier_contains: Option<String>,
    /// The operating system of the device (not in) Optional.
    #[serde(rename = "os__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// Asset Contact Email Optional.
    #[serde(rename = "assetContactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset Allowed values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The agent full disk scan date Optional.
    #[serde(rename = "agentFullDiskScanDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// The agent anti tampering status (not in) Optional.
    #[serde(rename = "agentAntiTamperingStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory Optional.
    #[serde(rename = "assetEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device Optional.
    #[serde(rename = "osFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// AD user or their groups Optional.
    #[serde(rename = "identityAdUser__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_contains: Option<String>,
    /// The Surface that each asset belongs to (not in) Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. Optional.
    #[serde(rename = "surfaces__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// ADS Enabled Optional.
    #[serde(rename = "adsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// The agent location Optional.
    #[serde(rename = "agentLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent pending actions (not in) Optional.
    #[serde(rename = "agentPendingActions__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions_nin: Option<String>,
    /// The ID Optional.
    #[serde(rename = "id__in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Whether the agent is uninstalled Optional.
    #[serde(rename = "agentUninstalled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// The Asset Type Optional.
    #[serde(rename = "resourceType__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in) Optional.
    #[serde(rename = "memoryReadable__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// The UUID Optional.
    #[serde(rename = "agentUuid__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,
    /// Tags Optional.
    #[serde(rename = "tagsKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Is DC Server Optional.
    #[serde(rename = "isDcServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// The location awareness Optional.
    #[serde(rename = "agentLocationAwareness__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location_awareness_contains: Option<String>,
    /// AD user DN Optional.
    #[serde(rename = "identityAdUserDistinguishedName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// The agent operational state Optional.
    #[serde(rename = "agentOperationalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent network status Optional.
    #[serde(rename = "agentNetworkStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// Name Optional.
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The agent VSS service status Optional.
    #[serde(rename = "agentVssServiceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The MAC addresses Optional.
    #[serde(rename = "macAddresses__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The agent network scanner status (not in) Optional.
    #[serde(rename = "agentRangerStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status_nin: Option<String>,
    /// The agent version (not in) Optional.
    #[serde(rename = "agentAgentVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_nin: Option<String>,
    /// The name Optional.
    #[serde(rename = "name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The operating system version of the device Optional.
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Whether the agent is pending uninstall Optional.
    #[serde(rename = "agentPendingUninstall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// The agent anti tampering status Optional.
    #[serde(rename = "agentAntiTamperingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The first seen date Optional.
    #[serde(rename = "firstSeenDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The name of the application installed on a workstation or a server Optional.
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// The criticality that each asset belongs to Allowed values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The agent VSS service status (not in) Optional.
    #[serde(rename = "agentVssServiceStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent VSS last snapshot date Optional.
    #[serde(rename = "agentVssLastSnapshotDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// Whether the agent has local configuration Optional.
    #[serde(rename = "agentHasLocalConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// User and cloud tag keys not exists Optional.
    #[serde(rename = "allTagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The agent missing permissions (not in) Optional.
    #[serde(rename = "agentMissingPermissions__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions_nin: Option<String>,
    /// Whether the agent is pending upgrade Optional.
    #[serde(rename = "agentPendingUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The agent VSS rollback status Optional.
    #[serde(rename = "agentVssRollbackStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// List of Site IDs to filter by Optional.
    #[serde(rename = "siteIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent installer type (not in) Optional.
    #[serde(rename = "agentInstallerType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_type_nin: Option<String>,
    /// The Last Seen date and time for the asset Optional.
    #[serde(rename = "s1UpdatedAt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`. Optional.
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in) Optional.
    #[serde(rename = "assetEnvironment__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in) Optional.
    #[serde(rename = "names__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Match by the agent UUID Optional.
    #[serde(rename = "agentUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The number of cores Optional.
    #[serde(rename = "coreCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in) Optional.
    #[serde(rename = "osVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in) Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`. Optional.
    #[serde(rename = "subCategory__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset Allowed values: `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "activeCoverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for Optional.
    #[serde(rename = "countsFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in) Optional.
    #[serde(rename = "coreCount__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by Optional.
    #[serde(rename = "csvFilterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The architecture of the device (not in) Optional.
    #[serde(rename = "architecture__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// AD machine DN Optional.
    #[serde(rename = "identityAdMachineDistinguishedName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// Live update ID Optional.
    #[serde(rename = "agentS1AgentLiveUpdatesVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// The agent network status (not in) Optional.
    #[serde(rename = "agentNetworkStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_status_nin: Option<String>,
    /// The sub-category that each resource belongs to Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`. Optional.
    #[serde(rename = "subCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The agent network scanner status Optional.
    #[serde(rename = "agentRangerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The OS versions Optional.
    #[serde(rename = "osVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. Optional.
    #[serde(rename = "deviceReview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys Optional.
    #[serde(rename = "allTagsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The serial number Optional.
    #[serde(rename = "serialNumber__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number_contains: Option<String>,
    /// The hostnames Optional.
    #[serde(rename = "hostnames__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// AD machine groups Optional.
    #[serde(rename = "identityAdMachineMembership__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// The agent VSS rollback status (not in) Optional.
    #[serde(rename = "agentVssRollbackStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent console migration status Optional.
    #[serde(rename = "agentConsoleMigrationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The architecture of the device Optional.
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// AD user groups Optional.
    #[serde(rename = "identityAdUserMembership__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_membership_contains: Option<String>,
    /// The connection status between the agent and the SDL service (not in) Optional.
    #[serde(rename = "agentDvConnectivity__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The agent VSS protection status Optional.
    #[serde(rename = "agentVssProtectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
    /// The ID Optional.
    #[serde(rename = "id__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The internal IPs Optional.
    #[serde(rename = "internalIps__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The status of the asset Allowed values: `Active`, `Inactive`. Optional.
    #[serde(rename = "assetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Whether the agent can configure network quarantine Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// The agent health status Optional.
    #[serde(rename = "agentHealthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent network scanner version Optional.
    #[serde(rename = "agentRangerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent disk encryption Optional.
    #[serde(rename = "agentDiskEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The domain of the device (not in) Optional.
    #[serde(rename = "domain__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// The agent disk metrics volume type (not in) Optional.
    #[serde(rename = "agentDiskMetricsVolumeType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The last logged in user Optional.
    #[serde(rename = "agentLastLoggedInUser__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// Tags (not in) Optional.
    #[serde(rename = "tagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The agent version Optional.
    #[serde(rename = "agentAgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists Optional.
    #[serde(rename = "allTagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__gte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent version Optional.
    #[serde(rename = "agentAgentVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_contains: Option<String>,
    /// The domain of the device Optional.
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes Optional.
    #[serde(rename = "agentVssVolumesDiffAreaFreePercentage__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The operating system family of the device (not in) Optional.
    #[serde(rename = "osFamily__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The status alerts of the asset (not in) Allowed values: `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The operating system name and version of the device Optional.
    #[serde(rename = "osNameVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl InventoryEndpointSurfaceActionQuery {
    pub fn tags_key_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_operational_state_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_criticality_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn legacy_identity_policy_name<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn missing_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_migration_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_disk_metrics_free_percentage_between = Some(v.into());
        self
    }
    pub fn tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn cpu_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn memory_readable<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn ip_address_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_protection_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn risk_factors_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_machine_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn is_ad_connector<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn image_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_missing_permissions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity_last_updated_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_dv_connectivity_last_updated_dt_between = Some(v.into());
        self
    }
    pub fn subnets_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn active_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_idr_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_idr_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_configurable_network_quarantine_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn gateway_ips_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn surfaces<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_actions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_name_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn missing_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn last_active_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_active_dt_between = Some(v.into());
        self
    }
    pub fn identity_ad_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_installer_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_volume_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn legacy_identity_policy_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_health_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn device_review_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_contact_email_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_lte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_lte = Some(v);
        self
    }
    pub fn alert_severity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_decommissioned = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_subscribe_on_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_subscribe_on_dt_between = Some(v.into());
        self
    }
    pub fn os_name_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn resource_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn gateway_macs_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_customer_identifier_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_customer_identifier_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_contact_email<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn risk_factors<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_full_disk_scan_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_full_disk_scan_dt_between = Some(v.into());
        self
    }
    pub fn agent_anti_tampering_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_environment<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_family<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_user_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn surfaces_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn ads_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ads_enabled = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_actions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn id_in<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uninstalled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uninstalled = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn resource_type_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn memory_readable_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uuid_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn is_dc_server<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_dc_server = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location_awareness_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_awareness_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_user_distinguished_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_distinguished_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_operational_state<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_network_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn names<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_service_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn mac_addresses_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_agent_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_uninstall<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_uninstall = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_anti_tampering_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn first_seen_dt_between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt_between = Some(v.into());
        self
    }
    pub fn application_name<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_name = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_criticality<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_service_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_last_snapshot_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_last_snapshot_dt_between = Some(v.into());
        self
    }
    pub fn agent_has_local_config<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_has_local_config = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_missing_permissions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_upgrade<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_upgrade = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_rollback_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_installer_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    pub fn resource_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_environment_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn names_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uuid<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn core_count<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn sub_category_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn infection_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn active_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn counts_for<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn core_count_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    pub fn architecture_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_machine_distinguished_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_distinguished_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_s1_agent_live_updates_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_s1_agent_live_updates_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_network_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn sub_category<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn device_review<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn hostnames_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_machine_membership_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_membership_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_rollback_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_migration_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn architecture<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_user_membership_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_membership_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_protection_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn internal_ips_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_configurable_network_quarantine<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_health_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_encryption<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_encryption = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_volume_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_last_logged_in_user_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_last_logged_in_user_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_agent_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_gte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_gte = Some(v);
        self
    }
    pub fn agent_agent_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_volumes_diff_area_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_volumes_diff_area_free_percentage_between = Some(v.into());
        self
    }
    pub fn os_family_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn infection_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_name_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
}


/// Query params for
/// `POST /web/api/v2.1/xdr/assets/surface/endpoint/available-actions/with-status`
/// (Get endpoint available actions). Every field is optional.
#[derive(Debug, Default, Serialize)]
pub struct InventoryEndpointSurfaceAvailableActionsQuery {
    /// Free-text filter by tag key (supports multiple values) Optional.
    #[serde(rename = "tagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The agent operational state (not in) Optional.
    #[serde(rename = "agentOperationalState__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state_nin: Option<String>,
    /// The criticality that each asset belongs to (not in) Allowed values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// Legacy Identity Policy Name Optional.
    #[serde(rename = "legacyIdentityPolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The missing coverage for the asset Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "missingCoverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags Optional.
    #[serde(rename = "allTagsKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The agent console migration status (not in) Optional.
    #[serde(rename = "agentConsoleMigrationStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status_nin: Option<String>,
    /// Tag Keys (not in) Optional.
    #[serde(rename = "tagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// Tag Keys exists Optional.
    #[serde(rename = "tagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CPU Optional.
    #[serde(rename = "cpu__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// The memory of the device in human readable format Optional.
    #[serde(rename = "memoryReadable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses Optional.
    #[serde(rename = "ipAddress__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values) Optional.
    #[serde(rename = "tagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// Tag Keys Optional.
    #[serde(rename = "tagsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The agent VSS protection status (not in) Optional.
    #[serde(rename = "agentVssProtectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The risk factors associated with the asset (not in) Allowed values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists Optional.
    #[serde(rename = "tagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// AD machine or its groups Optional.
    #[serde(rename = "identityAdMachine__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_contains: Option<String>,
    /// The operating system of the device Optional.
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Is AD Connector Optional.
    #[serde(rename = "isAdConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// Free-text filter by the image name Optional.
    #[serde(rename = "imageName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The agent missing permissions Optional.
    #[serde(rename = "agentMissingPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The agent location (not in) Optional.
    #[serde(rename = "agentLocation__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// List of Group IDs to filter by Optional.
    #[serde(rename = "groupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The SDL connectivity last active Optional.
    #[serde(rename = "agentDvConnectivityLastUpdatedDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The subnets Optional.
    #[serde(rename = "subnets__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The connection status between the agent and the SDL service Optional.
    #[serde(rename = "agentDvConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The active coverage for the asset (not in) Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "activeCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The agent console connectivity Optional.
    #[serde(rename = "agentConsoleConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity Optional.
    #[serde(rename = "agentIdrConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// User and cloud tags (not in) Optional.
    #[serde(rename = "allTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// Whether the agent can configure network quarantine (not in) Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// User and cloud tag keys (not in) Optional.
    #[serde(rename = "allTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs Optional.
    #[serde(rename = "gatewayIps__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The Surface that each asset belongs to Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. Optional.
    #[serde(rename = "surfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions Optional.
    #[serde(rename = "agentPendingActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// The operating system name and version of the device (not in) Optional.
    #[serde(rename = "osNameVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in) Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "missingCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The serial number Optional.
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The status of the asset (not in) Allowed values: `Active`, `Inactive`. Optional.
    #[serde(rename = "assetStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The last active date Optional.
    #[serde(rename = "lastActiveDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// Any AD string Optional.
    #[serde(rename = "identityAd__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// The agent installer type Optional.
    #[serde(rename = "agentInstallerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent disk metrics volume type Optional.
    #[serde(rename = "agentDiskMetricsVolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The legacy identity policy name Optional.
    #[serde(rename = "legacy_identity_policy_name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// The agent health status (not in) Optional.
    #[serde(rename = "agentHealthStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_status_nin: Option<String>,
    /// The agent network scanner version (not in) Optional.
    #[serde(rename = "agentRangerVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version_nin: Option<String>,
    /// The asset review (not in) Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. Optional.
    #[serde(rename = "deviceReview__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Asset Contact Email (not in) Optional.
    #[serde(rename = "assetContactEmail__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__lte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The severity of the alert Optional.
    #[serde(rename = "alertSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by Optional.
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The serial number (not in) Optional.
    #[serde(rename = "serialNumber__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// Whether the agent is decommissioned Optional.
    #[serde(rename = "agentDecommissioned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent subscribe time Optional.
    #[serde(rename = "agentSubscribeOnDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// The OS names and versions Optional.
    #[serde(rename = "osNameVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The domain Optional.
    #[serde(rename = "domain__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in) Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`. Optional.
    #[serde(rename = "resourceType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs Optional.
    #[serde(rename = "gatewayMacs__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The customer identifier Optional.
    #[serde(rename = "agentCustomerIdentifier__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_customer_identifier_contains: Option<String>,
    /// The operating system of the device (not in) Optional.
    #[serde(rename = "os__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// Asset Contact Email Optional.
    #[serde(rename = "assetContactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset Allowed values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The agent full disk scan date Optional.
    #[serde(rename = "agentFullDiskScanDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// The agent anti tampering status (not in) Optional.
    #[serde(rename = "agentAntiTamperingStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory Optional.
    #[serde(rename = "assetEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device Optional.
    #[serde(rename = "osFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// AD user or their groups Optional.
    #[serde(rename = "identityAdUser__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_contains: Option<String>,
    /// The Surface that each asset belongs to (not in) Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. Optional.
    #[serde(rename = "surfaces__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// ADS Enabled Optional.
    #[serde(rename = "adsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// The agent location Optional.
    #[serde(rename = "agentLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent pending actions (not in) Optional.
    #[serde(rename = "agentPendingActions__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions_nin: Option<String>,
    /// The ID Optional.
    #[serde(rename = "id__in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Whether the agent is uninstalled Optional.
    #[serde(rename = "agentUninstalled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// The Asset Type Optional.
    #[serde(rename = "resourceType__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in) Optional.
    #[serde(rename = "memoryReadable__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// The UUID Optional.
    #[serde(rename = "agentUuid__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,
    /// Tags Optional.
    #[serde(rename = "tagsKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Is DC Server Optional.
    #[serde(rename = "isDcServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// The location awareness Optional.
    #[serde(rename = "agentLocationAwareness__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location_awareness_contains: Option<String>,
    /// AD user DN Optional.
    #[serde(rename = "identityAdUserDistinguishedName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// The agent operational state Optional.
    #[serde(rename = "agentOperationalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent network status Optional.
    #[serde(rename = "agentNetworkStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// Name Optional.
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The agent VSS service status Optional.
    #[serde(rename = "agentVssServiceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The MAC addresses Optional.
    #[serde(rename = "macAddresses__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The agent network scanner status (not in) Optional.
    #[serde(rename = "agentRangerStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status_nin: Option<String>,
    /// The agent version (not in) Optional.
    #[serde(rename = "agentAgentVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_nin: Option<String>,
    /// The name Optional.
    #[serde(rename = "name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The operating system version of the device Optional.
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Whether the agent is pending uninstall Optional.
    #[serde(rename = "agentPendingUninstall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// The agent anti tampering status Optional.
    #[serde(rename = "agentAntiTamperingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The first seen date Optional.
    #[serde(rename = "firstSeenDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The name of the application installed on a workstation or a server Optional.
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// The criticality that each asset belongs to Allowed values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The agent VSS service status (not in) Optional.
    #[serde(rename = "agentVssServiceStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent VSS last snapshot date Optional.
    #[serde(rename = "agentVssLastSnapshotDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// Whether the agent has local configuration Optional.
    #[serde(rename = "agentHasLocalConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// User and cloud tag keys not exists Optional.
    #[serde(rename = "allTagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The agent missing permissions (not in) Optional.
    #[serde(rename = "agentMissingPermissions__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions_nin: Option<String>,
    /// Whether the agent is pending upgrade Optional.
    #[serde(rename = "agentPendingUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The agent VSS rollback status Optional.
    #[serde(rename = "agentVssRollbackStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// List of Site IDs to filter by Optional.
    #[serde(rename = "siteIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent installer type (not in) Optional.
    #[serde(rename = "agentInstallerType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_type_nin: Option<String>,
    /// The Last Seen date and time for the asset Optional.
    #[serde(rename = "s1UpdatedAt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`. Optional.
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in) Optional.
    #[serde(rename = "assetEnvironment__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in) Optional.
    #[serde(rename = "names__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Match by the agent UUID Optional.
    #[serde(rename = "agentUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The number of cores Optional.
    #[serde(rename = "coreCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in) Optional.
    #[serde(rename = "osVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in) Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`. Optional.
    #[serde(rename = "subCategory__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset Allowed values: `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "activeCoverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for Optional.
    #[serde(rename = "countsFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in) Optional.
    #[serde(rename = "coreCount__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by Optional.
    #[serde(rename = "csvFilterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The architecture of the device (not in) Optional.
    #[serde(rename = "architecture__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// AD machine DN Optional.
    #[serde(rename = "identityAdMachineDistinguishedName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// Live update ID Optional.
    #[serde(rename = "agentS1AgentLiveUpdatesVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// The agent network status (not in) Optional.
    #[serde(rename = "agentNetworkStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_status_nin: Option<String>,
    /// The sub-category that each resource belongs to Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`. Optional.
    #[serde(rename = "subCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The agent network scanner status Optional.
    #[serde(rename = "agentRangerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The OS versions Optional.
    #[serde(rename = "osVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. Optional.
    #[serde(rename = "deviceReview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys Optional.
    #[serde(rename = "allTagsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The serial number Optional.
    #[serde(rename = "serialNumber__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number_contains: Option<String>,
    /// The hostnames Optional.
    #[serde(rename = "hostnames__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// AD machine groups Optional.
    #[serde(rename = "identityAdMachineMembership__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// The agent VSS rollback status (not in) Optional.
    #[serde(rename = "agentVssRollbackStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent console migration status Optional.
    #[serde(rename = "agentConsoleMigrationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The architecture of the device Optional.
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// AD user groups Optional.
    #[serde(rename = "identityAdUserMembership__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_membership_contains: Option<String>,
    /// The connection status between the agent and the SDL service (not in) Optional.
    #[serde(rename = "agentDvConnectivity__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The agent VSS protection status Optional.
    #[serde(rename = "agentVssProtectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
    /// The ID Optional.
    #[serde(rename = "id__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The internal IPs Optional.
    #[serde(rename = "internalIps__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The status of the asset Allowed values: `Active`, `Inactive`. Optional.
    #[serde(rename = "assetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Whether the agent can configure network quarantine Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// The agent health status Optional.
    #[serde(rename = "agentHealthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent network scanner version Optional.
    #[serde(rename = "agentRangerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent disk encryption Optional.
    #[serde(rename = "agentDiskEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The domain of the device (not in) Optional.
    #[serde(rename = "domain__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// The agent disk metrics volume type (not in) Optional.
    #[serde(rename = "agentDiskMetricsVolumeType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The last logged in user Optional.
    #[serde(rename = "agentLastLoggedInUser__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// Tags (not in) Optional.
    #[serde(rename = "tagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The agent version Optional.
    #[serde(rename = "agentAgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists Optional.
    #[serde(rename = "allTagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__gte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent version Optional.
    #[serde(rename = "agentAgentVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_contains: Option<String>,
    /// The domain of the device Optional.
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes Optional.
    #[serde(rename = "agentVssVolumesDiffAreaFreePercentage__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The operating system family of the device (not in) Optional.
    #[serde(rename = "osFamily__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The status alerts of the asset (not in) Allowed values: `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The operating system name and version of the device Optional.
    #[serde(rename = "osNameVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl InventoryEndpointSurfaceAvailableActionsQuery {
    pub fn tags_key_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_operational_state_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_criticality_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn legacy_identity_policy_name<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn missing_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_migration_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_disk_metrics_free_percentage_between = Some(v.into());
        self
    }
    pub fn tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn cpu_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn memory_readable<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn ip_address_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_protection_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn risk_factors_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_machine_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn is_ad_connector<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn image_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_missing_permissions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity_last_updated_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_dv_connectivity_last_updated_dt_between = Some(v.into());
        self
    }
    pub fn subnets_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn active_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_idr_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_idr_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_configurable_network_quarantine_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn gateway_ips_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn surfaces<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_actions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_name_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn missing_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn last_active_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_active_dt_between = Some(v.into());
        self
    }
    pub fn identity_ad_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_installer_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_volume_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn legacy_identity_policy_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_health_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn device_review_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_contact_email_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_lte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_lte = Some(v);
        self
    }
    pub fn alert_severity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_decommissioned = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_subscribe_on_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_subscribe_on_dt_between = Some(v.into());
        self
    }
    pub fn os_name_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn resource_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn gateway_macs_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_customer_identifier_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_customer_identifier_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_contact_email<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn risk_factors<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_full_disk_scan_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_full_disk_scan_dt_between = Some(v.into());
        self
    }
    pub fn agent_anti_tampering_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_environment<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_family<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_user_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn surfaces_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn ads_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ads_enabled = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_actions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn id_in<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uninstalled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uninstalled = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn resource_type_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn memory_readable_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uuid_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn is_dc_server<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_dc_server = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location_awareness_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_awareness_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_user_distinguished_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_distinguished_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_operational_state<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_network_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn names<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_service_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn mac_addresses_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_agent_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_uninstall<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_uninstall = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_anti_tampering_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn first_seen_dt_between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt_between = Some(v.into());
        self
    }
    pub fn application_name<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_name = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_criticality<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_service_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_last_snapshot_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_last_snapshot_dt_between = Some(v.into());
        self
    }
    pub fn agent_has_local_config<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_has_local_config = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_missing_permissions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_upgrade<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_upgrade = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_rollback_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_installer_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    pub fn resource_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_environment_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn names_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uuid<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn core_count<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn sub_category_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn infection_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn active_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn counts_for<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn core_count_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    pub fn architecture_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_machine_distinguished_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_distinguished_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_s1_agent_live_updates_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_s1_agent_live_updates_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_network_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn sub_category<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn device_review<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn hostnames_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_machine_membership_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_membership_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_rollback_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_migration_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn architecture<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_user_membership_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_membership_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_protection_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn internal_ips_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_configurable_network_quarantine<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_health_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_encryption<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_encryption = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_volume_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_last_logged_in_user_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_last_logged_in_user_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_agent_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_gte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_gte = Some(v);
        self
    }
    pub fn agent_agent_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_volumes_diff_area_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_volumes_diff_area_free_percentage_between = Some(v.into());
        self
    }
    pub fn os_family_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn infection_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_name_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
}


/// Query params for
/// `GET /web/api/v2.1/xdr/assets/surface/endpoint/export`
/// (Export assets to CSV or JSON).
///
/// `export_format` is required; construct via [`InventoryEndpointSurfaceExportQuery::new`].
#[derive(Debug, Default, Serialize)]
pub struct InventoryEndpointSurfaceExportQuery {
    /// Free-text filter by tag key (supports multiple values) Optional.
    #[serde(rename = "tagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The agent operational state (not in) Optional.
    #[serde(rename = "agentOperationalState__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state_nin: Option<String>,
    /// The criticality that each asset belongs to (not in) Allowed values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// Legacy Identity Policy Name Optional.
    #[serde(rename = "legacyIdentityPolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The missing coverage for the asset Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "missingCoverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags Optional.
    #[serde(rename = "allTagsKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The agent console migration status (not in) Optional.
    #[serde(rename = "agentConsoleMigrationStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status_nin: Option<String>,
    /// Tag Keys (not in) Optional.
    #[serde(rename = "tagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// Tag Keys exists Optional.
    #[serde(rename = "tagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CPU Optional.
    #[serde(rename = "cpu__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// The memory of the device in human readable format Optional.
    #[serde(rename = "memoryReadable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses Optional.
    #[serde(rename = "ipAddress__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values) Optional.
    #[serde(rename = "tagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// Tag Keys Optional.
    #[serde(rename = "tagsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The agent VSS protection status (not in) Optional.
    #[serde(rename = "agentVssProtectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The risk factors associated with the asset (not in) Allowed values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists Optional.
    #[serde(rename = "tagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor". Optional.
    #[serde(rename = "skip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// AD machine or its groups Optional.
    #[serde(rename = "identityAdMachine__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_contains: Option<String>,
    /// The operating system of the device Optional.
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Is AD Connector Optional.
    #[serde(rename = "isAdConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// Free-text filter by the image name Optional.
    #[serde(rename = "imageName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The agent missing permissions Optional.
    #[serde(rename = "agentMissingPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The agent location (not in) Optional.
    #[serde(rename = "agentLocation__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// List of Group IDs to filter by Optional.
    #[serde(rename = "groupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The SDL connectivity last active Optional.
    #[serde(rename = "agentDvConnectivityLastUpdatedDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The subnets Optional.
    #[serde(rename = "subnets__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The connection status between the agent and the SDL service Optional.
    #[serde(rename = "agentDvConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The active coverage for the asset (not in) Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "activeCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The agent console connectivity Optional.
    #[serde(rename = "agentConsoleConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity Optional.
    #[serde(rename = "agentIdrConnectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// User and cloud tags (not in) Optional.
    #[serde(rename = "allTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// Whether the agent can configure network quarantine (not in) Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// User and cloud tag keys (not in) Optional.
    #[serde(rename = "allTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs Optional.
    #[serde(rename = "gatewayIps__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The Surface that each asset belongs to Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. Optional.
    #[serde(rename = "surfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions Optional.
    #[serde(rename = "agentPendingActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// The operating system name and version of the device (not in) Optional.
    #[serde(rename = "osNameVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in) Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "missingCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The serial number Optional.
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The status of the asset (not in) Allowed values: `Active`, `Inactive`. Optional.
    #[serde(rename = "assetStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The last active date Optional.
    #[serde(rename = "lastActiveDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// The column to sort the results by. Allowed values: `s1GroupName`, `cpu`, `legacyIdentityPolicyName`, `previousOsType`, `previousOsVersion`, `agentFirewallStatus`, `s1UpdatedAt`, `memoryReadable`, `s1ManagementId`, `isAdConnector`, `os`, `agentLocationAwareness`, `agentDetectionState`, `agentCustomerIdentifier`, `agentDvConnectivity`, `agentConsoleConnectivity`, `agentIdrConnectivity`, `agentUpToDate`, `networkName`, `s1SiteId`, `s1SiteName`, `serialNumber`, `detectedFromSite`, `agentInstallerType`, `identityAdUserDistinguishedName`, `eppUnsupportedUnknown`, `category`, `s1GroupId`, `agentLastLoggedInUser`, `ipAddress`, `agentK8sPod`, `agentDecommissioned`, `lastActiveDt`, `s1OnboardedAccountId`, `assetContactEmail`, `s1ScopeType`, `agentSubscribeOnDt`, `assetEnvironment`, `previousDeviceFunction`, `osFamily`, `s1AccountId`, `adsEnabled`, `id`, `s1AccountName`, `agentUninstalled`, `s1ScopeLevel`, `memory`, `agentId`, `s1OnboardedAccountName`, `s1OnboardedScopeLevel`, `isDcServer`, `agentOperationalState`, `agentNetworkStatus`, `name`, `agentVssServiceStatus`, `agentPendingUninstall`, `agentAntiTamperingStatus`, `assetCriticality`, `s1ScopePath`, `osVersion`, `s1OnboardedGroupName`, `identityAdMachineDistinguishedName`, `lastRebootDt`, `s1OnboardedSiteName`, `s1ScopeId`, `agentHasLocalConfig`, `agentPendingUpgrade`, `manufacturer`, `agentK8sNamespace`, `agentVssRollbackStatus`, `resourceType`, `agentUuid`, `agentOperationalStateExpirationTimeDt`, `coreCount`, `infectionStatus`, `s1OnboardedSiteId`, `agentVssLastSnapshotDt`, `agentDvConnectivityLastUpdatedDt`, `subCategory`, `s1OnboardedScopeId`, `agentRangerStatus`, `deviceReview`, `s1OnboardedScopePath`, `agentConsoleMigrationStatus`, `architecture`, `agentVssProtectionStatus`, `assetStatus`, `agentConfigurableNetworkQuarantine`, `agentRangerVersion`, `agentHealthStatus`, `agentFullDiskScanDt`, `s1OnboardedGroupId`, `agentAgentVersion`, `firstSeenDt`, `domain`, `lastUpdateDt`, `agentDiskEncryption`, `osNameVersion`. Optional.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Any AD string Optional.
    #[serde(rename = "identityAd__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// The agent installer type Optional.
    #[serde(rename = "agentInstallerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent disk metrics volume type Optional.
    #[serde(rename = "agentDiskMetricsVolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The legacy identity policy name Optional.
    #[serde(rename = "legacy_identity_policy_name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// The agent health status (not in) Optional.
    #[serde(rename = "agentHealthStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_status_nin: Option<String>,
    /// The agent network scanner version (not in) Optional.
    #[serde(rename = "agentRangerVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version_nin: Option<String>,
    /// The asset review (not in) Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. Optional.
    #[serde(rename = "deviceReview__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Asset Contact Email (not in) Optional.
    #[serde(rename = "assetContactEmail__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__lte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The severity of the alert Optional.
    #[serde(rename = "alertSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by Optional.
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The serial number (not in) Optional.
    #[serde(rename = "serialNumber__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// Whether the agent is decommissioned Optional.
    #[serde(rename = "agentDecommissioned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent subscribe time Optional.
    #[serde(rename = "agentSubscribeOnDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// The OS names and versions Optional.
    #[serde(rename = "osNameVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The domain Optional.
    #[serde(rename = "domain__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in) Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`. Optional.
    #[serde(rename = "resourceType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs Optional.
    #[serde(rename = "gatewayMacs__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The customer identifier Optional.
    #[serde(rename = "agentCustomerIdentifier__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_customer_identifier_contains: Option<String>,
    /// The operating system of the device (not in) Optional.
    #[serde(rename = "os__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// Asset Contact Email Optional.
    #[serde(rename = "assetContactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset Allowed values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The agent full disk scan date Optional.
    #[serde(rename = "agentFullDiskScanDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// The agent anti tampering status (not in) Optional.
    #[serde(rename = "agentAntiTamperingStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory Optional.
    #[serde(rename = "assetEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device Optional.
    #[serde(rename = "osFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// AD user or their groups Optional.
    #[serde(rename = "identityAdUser__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_contains: Option<String>,
    /// The Surface that each asset belongs to (not in) Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. Optional.
    #[serde(rename = "surfaces__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// ADS Enabled Optional.
    #[serde(rename = "adsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// The agent location Optional.
    #[serde(rename = "agentLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent pending actions (not in) Optional.
    #[serde(rename = "agentPendingActions__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions_nin: Option<String>,
    /// The ID Optional.
    #[serde(rename = "id__in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Whether the agent is uninstalled Optional.
    #[serde(rename = "agentUninstalled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// The Asset Type Optional.
    #[serde(rename = "resourceType__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in) Optional.
    #[serde(rename = "memoryReadable__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// The UUID Optional.
    #[serde(rename = "agentUuid__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,
    /// Tags Optional.
    #[serde(rename = "tagsKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction Allowed values: `asc`, `desc`. Optional.
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Is DC Server Optional.
    #[serde(rename = "isDcServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// The location awareness Optional.
    #[serde(rename = "agentLocationAwareness__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location_awareness_contains: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects. Optional.
    #[serde(rename = "countOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// AD user DN Optional.
    #[serde(rename = "identityAdUserDistinguishedName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items. Optional.
    #[serde(rename = "cursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The agent operational state Optional.
    #[serde(rename = "agentOperationalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent network status Optional.
    #[serde(rename = "agentNetworkStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// Name Optional.
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The agent VSS service status Optional.
    #[serde(rename = "agentVssServiceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The MAC addresses Optional.
    #[serde(rename = "macAddresses__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The agent network scanner status (not in) Optional.
    #[serde(rename = "agentRangerStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status_nin: Option<String>,
    /// The agent version (not in) Optional.
    #[serde(rename = "agentAgentVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_nin: Option<String>,
    /// The name Optional.
    #[serde(rename = "name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The operating system version of the device Optional.
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Whether the agent is pending uninstall Optional.
    #[serde(rename = "agentPendingUninstall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// The agent anti tampering status Optional.
    #[serde(rename = "agentAntiTamperingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The first seen date Optional.
    #[serde(rename = "firstSeenDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The name of the application installed on a workstation or a server Optional.
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// The criticality that each asset belongs to Allowed values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up execution time. Optional.
    #[serde(rename = "skipCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The agent VSS service status (not in) Optional.
    #[serde(rename = "agentVssServiceStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent VSS last snapshot date Optional.
    #[serde(rename = "agentVssLastSnapshotDt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// Whether the agent has local configuration Optional.
    #[serde(rename = "agentHasLocalConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// User and cloud tag keys not exists Optional.
    #[serde(rename = "allTagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The agent missing permissions (not in) Optional.
    #[serde(rename = "agentMissingPermissions__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions_nin: Option<String>,
    /// Whether the agent is pending upgrade Optional.
    #[serde(rename = "agentPendingUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The agent VSS rollback status Optional.
    #[serde(rename = "agentVssRollbackStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// List of Site IDs to filter by Optional.
    #[serde(rename = "siteIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent installer type (not in) Optional.
    #[serde(rename = "agentInstallerType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_type_nin: Option<String>,
    /// The Last Seen date and time for the asset Optional.
    #[serde(rename = "s1UpdatedAt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`. Optional.
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in) Optional.
    #[serde(rename = "assetEnvironment__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in) Optional.
    #[serde(rename = "names__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Match by the agent UUID Optional.
    #[serde(rename = "agentUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The number of cores Optional.
    #[serde(rename = "coreCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in) Optional.
    #[serde(rename = "osVersion__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in) Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`. Optional.
    #[serde(rename = "subCategory__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset Allowed values: `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`. Optional.
    #[serde(rename = "activeCoverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for Optional.
    #[serde(rename = "countsFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in) Optional.
    #[serde(rename = "coreCount__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by Optional.
    #[serde(rename = "csvFilterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The architecture of the device (not in) Optional.
    #[serde(rename = "architecture__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// AD machine DN Optional.
    #[serde(rename = "identityAdMachineDistinguishedName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// Export format. Allowed values: `csv`, `json`. Required.
    #[serde(rename = "exportFormat")]
    pub export_format: String,
    /// Live update ID Optional.
    #[serde(rename = "agentS1AgentLiveUpdatesVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// The agent network status (not in) Optional.
    #[serde(rename = "agentNetworkStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_status_nin: Option<String>,
    /// The sub-category that each resource belongs to Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`. Optional.
    #[serde(rename = "subCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The agent network scanner status Optional.
    #[serde(rename = "agentRangerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The OS versions Optional.
    #[serde(rename = "osVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. Optional.
    #[serde(rename = "deviceReview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys Optional.
    #[serde(rename = "allTagsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The serial number Optional.
    #[serde(rename = "serialNumber__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number_contains: Option<String>,
    /// The hostnames Optional.
    #[serde(rename = "hostnames__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// AD machine groups Optional.
    #[serde(rename = "identityAdMachineMembership__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// Limit number of returned items (1-1000) Optional.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The agent VSS rollback status (not in) Optional.
    #[serde(rename = "agentVssRollbackStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent console migration status Optional.
    #[serde(rename = "agentConsoleMigrationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The architecture of the device Optional.
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// AD user groups Optional.
    #[serde(rename = "identityAdUserMembership__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_membership_contains: Option<String>,
    /// The connection status between the agent and the SDL service (not in) Optional.
    #[serde(rename = "agentDvConnectivity__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The agent VSS protection status Optional.
    #[serde(rename = "agentVssProtectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
    /// The ID Optional.
    #[serde(rename = "id__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The internal IPs Optional.
    #[serde(rename = "internalIps__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The status of the asset Allowed values: `Active`, `Inactive`. Optional.
    #[serde(rename = "assetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Whether the agent can configure network quarantine Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// The agent health status Optional.
    #[serde(rename = "agentHealthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent network scanner version Optional.
    #[serde(rename = "agentRangerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent disk encryption Optional.
    #[serde(rename = "agentDiskEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The domain of the device (not in) Optional.
    #[serde(rename = "domain__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// The agent disk metrics volume type (not in) Optional.
    #[serde(rename = "agentDiskMetricsVolumeType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The last logged in user Optional.
    #[serde(rename = "agentLastLoggedInUser__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// Tags (not in) Optional.
    #[serde(rename = "tagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The agent version Optional.
    #[serde(rename = "agentAgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists Optional.
    #[serde(rename = "allTagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The agent free disk percentage on any of the disks Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__gte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent version Optional.
    #[serde(rename = "agentAgentVersion__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_contains: Option<String>,
    /// The domain of the device Optional.
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes Optional.
    #[serde(rename = "agentVssVolumesDiffAreaFreePercentage__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The operating system family of the device (not in) Optional.
    #[serde(rename = "osFamily__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The status alerts of the asset (not in) Allowed values: `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The operating system name and version of the device Optional.
    #[serde(rename = "osNameVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl InventoryEndpointSurfaceExportQuery {
    /// Create a new export query with the required export format.
    ///
    /// `export_format` allowed values: `csv`, `json`.
    pub fn new(export_format: impl Into<String>) -> Self {
        Self {
            export_format: export_format.into(),
            ..Default::default()
        }
    }

    pub fn tags_key_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_operational_state_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_criticality_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn legacy_identity_policy_name<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn missing_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_migration_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_disk_metrics_free_percentage_between = Some(v.into());
        self
    }
    pub fn tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn cpu_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn memory_readable<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn ip_address_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_protection_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn risk_factors_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    pub fn identity_ad_machine_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn is_ad_connector<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn image_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_missing_permissions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity_last_updated_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_dv_connectivity_last_updated_dt_between = Some(v.into());
        self
    }
    pub fn subnets_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn active_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_idr_connectivity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_idr_connectivity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_configurable_network_quarantine_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn gateway_ips_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn surfaces<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_actions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_name_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn missing_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn last_active_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_active_dt_between = Some(v.into());
        self
    }
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    pub fn identity_ad_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_installer_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_volume_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn legacy_identity_policy_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_health_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn device_review_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_contact_email_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_lte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_lte = Some(v);
        self
    }
    pub fn alert_severity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_decommissioned = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_subscribe_on_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_subscribe_on_dt_between = Some(v.into());
        self
    }
    pub fn os_name_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn resource_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn gateway_macs_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_customer_identifier_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_customer_identifier_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_contact_email<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn risk_factors<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_full_disk_scan_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_full_disk_scan_dt_between = Some(v.into());
        self
    }
    pub fn agent_anti_tampering_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_environment<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_family<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_user_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn surfaces_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn ads_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ads_enabled = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_actions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn id_in<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uninstalled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uninstalled = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn resource_type_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn memory_readable_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uuid_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    pub fn is_dc_server<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_dc_server = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_location_awareness_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_awareness_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    pub fn identity_ad_user_distinguished_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_distinguished_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    pub fn agent_operational_state<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_network_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn names<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_service_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn mac_addresses_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_agent_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_uninstall<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_uninstall = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_anti_tampering_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn first_seen_dt_between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt_between = Some(v.into());
        self
    }
    pub fn application_name<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_name = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_criticality<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    pub fn agent_vss_service_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_last_snapshot_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_last_snapshot_dt_between = Some(v.into());
        self
    }
    pub fn agent_has_local_config<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_has_local_config = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_missing_permissions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_pending_upgrade<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_upgrade = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_rollback_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_installer_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    pub fn resource_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_environment_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn names_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_uuid<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn core_count<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn sub_category_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn infection_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn active_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn counts_for<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn core_count_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    pub fn architecture_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_machine_distinguished_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_distinguished_name_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn export_format(mut self, v: impl Into<String>) -> Self {
        self.export_format = v.into();
        self
    }
    pub fn agent_s1_agent_live_updates_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_s1_agent_live_updates_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_network_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn sub_category<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn device_review<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn serial_number_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn hostnames_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_machine_membership_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_membership_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    pub fn agent_vss_rollback_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_console_migration_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn architecture<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn identity_ad_user_membership_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_membership_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_dv_connectivity_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_protection_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn internal_ips_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn asset_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_configurable_network_quarantine<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_health_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_ranger_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_encryption<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_encryption = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_volume_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_last_logged_in_user_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_last_logged_in_user_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_agent_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn all_tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage_gte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_gte = Some(v);
        self
    }
    pub fn agent_agent_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_contains = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn domain<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn agent_vss_volumes_diff_area_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_volumes_diff_area_free_percentage_between = Some(v.into());
        self
    }
    pub fn os_family_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn infection_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
    pub fn os_name_version<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(
            vals.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","),
        );
        self
    }
}


/// Request body for `POST /web/api/v2.1/xdr/assets/surface/endpoint`
/// (definition `EndpointViewInputSchema`).
#[derive(Debug, Default, Serialize)]
pub struct InventoryEndpointSurfaceListPostBody {
    /// Filter (`PaginatedEndpointFilter`). Required. This is a large/freeform
    /// object mirroring the GET query filter set, modeled as `serde_json::Value`.
    pub filter: serde_json::Value,
    /// Data (`EmptyStrict`). Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Request body for `POST /web/api/v2.1/xdr/assets/surface/endpoint/action`
/// (definition `EndpointActionPayloadSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryEndpointSurfaceActionBody {
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

impl InventoryEndpointSurfaceActionBody {
    /// Create a new action body with the required action name.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self {
            action_name: action_name.into(),
            ..Default::default()
        }
    }
    /// Set the list of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set the list of inventory ids to exclude from select_all (max 5000).
    pub fn id_nin<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(ids.into_iter().map(Into::into).collect());
        self
    }
}

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/surface/endpoint/available-actions/with-status`
/// (definition `AffectedResourcesSchema`).
#[derive(Debug, Default, Serialize)]
pub struct InventoryEndpointSurfaceAvailableActionsBody {
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000). Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl InventoryEndpointSurfaceAvailableActionsBody {
    /// Set the list of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set the list of inventory ids to exclude from select_all (max 5000).
    pub fn id_nin<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(ids.into_iter().map(Into::into).collect());
        self
    }
}


impl InventoryEndpointSurfaceService<'_> {
    /// Get inventory endpoint assets.
    ///
    /// Get inventory endpoint assets.
    ///
    /// `GET /web/api/v2.1/xdr/assets/surface/endpoint`
    pub async fn list(
        &self,
        query: &InventoryEndpointSurfaceListQuery,
    ) -> Result<Paginated<EndpointResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/surface/endpoint", q)
            .await?)
    }

    /// Assets using POST.
    ///
    /// POST API to get endpoint assets.
    ///
    /// `POST /web/api/v2.1/xdr/assets/surface/endpoint`
    pub async fn list_post(
        &self,
        query: &InventoryEndpointSurfaceListPostQuery,
        body: &InventoryEndpointSurfaceListPostBody,
    ) -> Result<Paginated<EndpointResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let path = if qs.is_empty() {
            "/web/api/v2.1/xdr/assets/surface/endpoint".to_string()
        } else {
            format!("/web/api/v2.1/xdr/assets/surface/endpoint?{qs}")
        };
        Ok(self.client.http().post(&path, body).await?)
    }

    /// Perform action.
    ///
    /// Perform action on selected assets.
    ///
    /// `POST /web/api/v2.1/xdr/assets/surface/endpoint/action`
    pub async fn action(
        &self,
        query: &InventoryEndpointSurfaceActionQuery,
        body: &InventoryEndpointSurfaceActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let path = if qs.is_empty() {
            "/web/api/v2.1/xdr/assets/surface/endpoint/action".to_string()
        } else {
            format!("/web/api/v2.1/xdr/assets/surface/endpoint/action?{qs}")
        };
        Ok(self.client.http().post(&path, body).await?)
    }

    /// Available actions.
    ///
    /// Get endpoint available actions.
    ///
    /// `POST /web/api/v2.1/xdr/assets/surface/endpoint/available-actions/with-status`
    pub async fn available_actions_with_status(
        &self,
        query: &InventoryEndpointSurfaceAvailableActionsQuery,
        body: &InventoryEndpointSurfaceAvailableActionsBody,
    ) -> Result<Response<AvailableActionWithStatusResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let base = "/web/api/v2.1/xdr/assets/surface/endpoint/available-actions/with-status";
        let path = if qs.is_empty() {
            base.to_string()
        } else {
            format!("{base}?{qs}")
        };
        Ok(self.client.http().post(&path, body).await?)
    }

    /// Export assets to CSV or JSON.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format.
    ///
    /// `GET /web/api/v2.1/xdr/assets/surface/endpoint/export`
    pub async fn export(
        &self,
        query: &InventoryEndpointSurfaceExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/surface/endpoint/export", q)
            .await?)
    }
}
