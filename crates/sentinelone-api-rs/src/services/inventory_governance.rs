//! Service for the `Inventory Governance` tag — Inventory Governance Resources.
//!
//! Hand-written for strict 1:1 parity with the SentinelOne Management API
//! (`docs/inventory-governance.md` + `swagger_2_1.json`). Five endpoints are
//! implemented: list assets (`GET`), list assets via `POST`, perform an action
//! on selected assets, list available actions with status, and export assets to
//! CSV or JSON.
//!
//! Every query parameter from the spec is exposed on a per-method `*Query`
//! struct. Array params are serialized comma-joined (the format the API
//! expects). Required params are passed as method arguments rather than struct
//! fields.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_governance::{AvailableActionWithStatusResponse, Governance};
use crate::pagination::{Paginated, Response};

/// `Inventory Governance` tag — Inventory Governance Resources.
///
/// Query, export, and govern XDR assets: list the asset inventory, perform
/// governance actions (criticality, contact, review, tags, notes, export) on a
/// selection, and discover which actions are currently available for a
/// selection.
pub struct InventoryGovernanceService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Body structs
// ---------------------------------------------------------------------------

/// Request body for `POST /web/api/v2.1/xdr/assets/governance` (Assets using
/// POST).
///
/// Spec definition: `v2_1.inventory.governance.schemas_GovernanceViewInputSchema`.
/// `filter` is the only top-level `required` field; the freeform paginated
/// governance filter is large/dynamic, so it is exposed as a JSON value. `data`
/// is nullable and optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceViewBody {
    /// Filter. Required top-level field (the paginated governance filter object).
    ///
    /// Freeform per spec (`PaginatedGovernanceFilter`) -> `serde_json::Value`.
    pub filter: serde_json::Value,
    /// Data. Optional/nullable per spec.
    ///
    /// Freeform (`EmptyStrict`) -> `serde_json::Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl GovernanceViewBody {
    /// New body with the required `filter` object.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { filter, data: None }
    }
    /// Optional `data` payload.
    pub fn data(mut self, v: serde_json::Value) -> Self {
        self.data = Some(v);
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/governance/action` (Perform
/// action).
///
/// Spec definition:
/// `v2_1.inventory.governance.schemas_GovernanceActionPayloadSchema`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceActionBody {
    /// Action name. Required top-level field.
    ///
    /// Enum (kept as `String` for forward-compat). Allowed values:
    /// `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`,
    /// `add_note`, `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`,
    /// `clear_tags`.
    pub action_name: String,
    /// List of selected inventory ids (max 5000).
    ///
    /// Optional per spec -> `Option`.
    #[serde(rename = "id__in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000).
    ///
    /// Optional per spec -> `Option`.
    #[serde(rename = "id__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl GovernanceActionBody {
    /// New body with the required `actionName`.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self {
            action_name: action_name.into(),
            id_in: None,
            id_nin: None,
        }
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
/// `POST /web/api/v2.1/xdr/assets/governance/available-actions/with-status`
/// (Available actions).
///
/// Spec definition: `v2_1.inventory.schemas_AffectedResourcesSchema`. Both
/// fields are optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResourcesBody {
    /// List of selected inventory ids (max 5000).
    ///
    /// Optional per spec -> `Option`.
    #[serde(rename = "id__in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000).
    ///
    /// Optional per spec -> `Option`.
    #[serde(rename = "id__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl AffectedResourcesBody {
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

// ---------------------------------------------------------------------------
// Query param structs (one per endpoint; every spec query param represented)
// ---------------------------------------------------------------------------

/// Query params for `POST /web/api/v2.1/xdr/assets/governance` (Assets using
/// POST).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceViewQuery {
    /// List of Account IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl GovernanceViewQuery {
    /// List of Account IDs to filter by
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.account_ids = Some(joined);
        self
    }
    /// List of Site IDs to filter by
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.site_ids = Some(joined);
        self
    }
    /// List of Group IDs to filter by
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.group_ids = Some(joined);
        self
    }
}

/// Query params for `GET /web/api/v2.1/xdr/assets/governance` (Assets).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGovernanceQuery {
    /// Free-text filter by tag key (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetCriticality__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountName__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// Tag Keys (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudResourceId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderSubscriptionId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The region
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "riskFactors__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor".
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The cloud provider account ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "imageName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "activeCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// User and cloud tags (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "region__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The cloud tags key value (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "missingCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The column to sort the results by.
    ///
    /// Allowed values: `s1GroupName`, `s1OnboardedAccountName`, `cloudProviderProjectId`, `category`, `s1UpdatedAt`, `s1GroupId`, `cloudProviderAccountId`, `s1OnboardedScopeLevel`, `createdTime`, `subCategory`, `cloudProviderAccountName`, `s1OnboardedScopeId`, `cloudProviderResourceGroup`, `deviceReview`, `region`, `cloudProviderOrganization`, `s1ManagementId`, `name`, `assetCriticality`, `s1OnboardedAccountId`, `s1OnboardedScopePath`, `s1ScopePath`, `cloudResourceUid`, `s1OnboardedGroupName`, `cloudProviderSubscriptionId`, `cloudResourceId`, `cloudProviderOrganizationUnit`, `cloudProviderOrganizationUnitPath`, `s1OnboardedSiteName`, `s1ScopeId`, `assetStatus`, `assetContactEmail`, `s1ScopeType`, `assetEnvironment`, `resourceType`, `s1OnboardedGroupId`, `s1SiteId`, `s1AccountId`, `id`, `s1SiteName`, `infectionStatus`, `s1AccountName`, `s1ScopeLevel`, `s1OnboardedSiteId`, `cloudProviderUrlString`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The geographical area where cloud resources are hosted
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "region__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The asset review (not in)
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "deviceReview__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The cloud provider account name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetContactEmail__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "resourceType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// Asset Contact Email
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "surfaces__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "id__in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "resourceType__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// Tags
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction
    ///
    /// Allowed values: `asc`, `desc`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The cloud provider organization unit
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderOrganizationUnit__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderOrganization__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountId__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up execution time.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// User and cloud tag keys not exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "s1UpdatedAt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetEnvironment__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "names__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The cloud tags key
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "subCategory__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The cloud provider account id
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Limit number of returned items (1-1000)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "id__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "infectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The cloud provider project ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderProjectId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl GetGovernanceQuery {
    /// Free-text filter by tag key (supports multiple values)
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_contains = Some(joined);
        self
    }
    /// The criticality that each asset belongs to (not in)
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_criticality_nin = Some(joined);
        self
    }
    /// The missing coverage for the asset
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.missing_coverage = Some(joined);
        self
    }
    /// User and cloud tags
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_value = Some(joined);
        self
    }
    /// The cloud provider account name (not in)
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name_nin = Some(joined);
        self
    }
    /// Tag Keys (not in)
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_nin = Some(joined);
        self
    }
    /// The cloud resource ID
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_resource_id_contains = Some(joined);
        self
    }
    /// The cloud provider subscription ID
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_subscription_id_contains = Some(joined);
        self
    }
    /// Tag Keys exists
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_exists = Some(joined);
        self
    }
    /// The region
    pub fn region<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region = Some(joined);
        self
    }
    /// Free-text filter by tag key value (supports multiple values)
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value_contains = Some(joined);
        self
    }
    /// The cloud tags key (not in)
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_nin = Some(joined);
        self
    }
    /// Tag Keys
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key = Some(joined);
        self
    }
    /// The risk factors associated with the asset (not in)
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.risk_factors_nin = Some(joined);
        self
    }
    /// Tag Keys not exists
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_nexists = Some(joined);
        self
    }
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor".
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// The cloud provider account ID
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id_contains = Some(joined);
        self
    }
    /// Free-text filter by the image name
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.image_name_contains = Some(joined);
        self
    }
    /// List of Group IDs to filter by
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.group_ids = Some(joined);
        self
    }
    /// The active coverage for the asset (not in)
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.active_coverage_nin = Some(joined);
        self
    }
    /// User and cloud tags (not in)
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_value_nin = Some(joined);
        self
    }
    /// The region (not in)
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region_nin = Some(joined);
        self
    }
    /// User and cloud tag keys (not in)
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_nin = Some(joined);
        self
    }
    /// The cloud tags key value (not in)
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value_nin = Some(joined);
        self
    }
    /// The Surface that each asset belongs to
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.surfaces = Some(joined);
        self
    }
    /// The missing coverage for the asset (not in)
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.missing_coverage_nin = Some(joined);
        self
    }
    /// The status of the asset (not in)
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_status_nin = Some(joined);
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// The geographical area where cloud resources are hosted
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region_contains = Some(joined);
        self
    }
    /// The asset review (not in)
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.device_review_nin = Some(joined);
        self
    }
    /// The cloud provider account name
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name = Some(joined);
        self
    }
    /// Asset Contact Email (not in)
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_contact_email_nin = Some(joined);
        self
    }
    /// The severity of the alert
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.alert_severity = Some(joined);
        self
    }
    /// List of Account IDs to filter by
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.account_ids = Some(joined);
        self
    }
    /// The cloud tags key value
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value = Some(joined);
        self
    }
    /// The canonical name for the resource type (not in)
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type_nin = Some(joined);
        self
    }
    /// Asset Contact Email
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_contact_email = Some(joined);
        self
    }
    /// The risk factors associated with the asset
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.risk_factors = Some(joined);
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_environment = Some(joined);
        self
    }
    /// The Surface that each asset belongs to (not in)
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.surfaces_nin = Some(joined);
        self
    }
    /// The ID
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.id_in = Some(joined);
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values)
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value_contains = Some(joined);
        self
    }
    /// The Asset Type
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type_contains = Some(joined);
        self
    }
    /// Tags
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value = Some(joined);
        self
    }
    /// Sort direction
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// The cloud provider organization unit
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_organization_unit_contains = Some(joined);
        self
    }
    /// The cloud provider organization
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_organization_contains = Some(joined);
        self
    }
    /// If true, only total number of items will be returned, without any of the actual objects.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Name
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.names = Some(joined);
        self
    }
    /// The name
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.name_contains = Some(joined);
        self
    }
    /// The criticality that each asset belongs to
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_criticality = Some(joined);
        self
    }
    /// The cloud provider account id (not in)
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id_nin = Some(joined);
        self
    }
    /// If true, total number of items will not be calculated, which speeds up execution time.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// User and cloud tag keys not exists
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_nexists = Some(joined);
        self
    }
    /// List of Site IDs to filter by
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.site_ids = Some(joined);
        self
    }
    /// The Last Seen date and time for the asset
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    /// The canonical name for the resource type
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type = Some(joined);
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_environment_nin = Some(joined);
        self
    }
    /// Name (not in)
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.names_nin = Some(joined);
        self
    }
    /// The cloud tags key
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key = Some(joined);
        self
    }
    /// The sub-category that each resource belongs to (not in)
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.sub_category_nin = Some(joined);
        self
    }
    /// The status alerts of the asset
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.infection_status = Some(joined);
        self
    }
    /// The active coverage for the asset
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.active_coverage = Some(joined);
        self
    }
    /// The columns for which filter count would be returned for
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.counts_for = Some(joined);
        self
    }
    /// The ID of the CSV file to filter by
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// The cloud provider account id
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id = Some(joined);
        self
    }
    /// The sub-category that each resource belongs to
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.sub_category = Some(joined);
        self
    }
    /// The asset review
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.device_review = Some(joined);
        self
    }
    /// User and cloud tag keys
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key = Some(joined);
        self
    }
    /// Limit number of returned items (1-1000)
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// The ID
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.id_contains = Some(joined);
        self
    }
    /// The cloud provider account name
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name_contains = Some(joined);
        self
    }
    /// The status of the asset
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_status = Some(joined);
        self
    }
    /// Free-text filter by cloud tag key (supports multiple values)
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_contains = Some(joined);
        self
    }
    /// Tags (not in)
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value_nin = Some(joined);
        self
    }
    /// User and cloud tag keys exists
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_exists = Some(joined);
        self
    }
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// The status alerts of the asset (not in)
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.infection_status_nin = Some(joined);
        self
    }
    /// The cloud provider project ID
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_project_id_contains = Some(joined);
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/governance/action` (Perform action).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformActionQuery {
    /// Free-text filter by tag key (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The Asset Type
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "resourceType__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetCriticality__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "infectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// Tags
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider organization unit
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderOrganizationUnit__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider account name (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountName__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The geographical area where cloud resources are hosted
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "region__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// Tag Keys (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudResourceId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderSubscriptionId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// The cloud provider account id
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The asset review (not in)
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "deviceReview__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The cloud provider organization
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderOrganization__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// Tag Keys exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The cloud provider account name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The asset review
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// Asset Contact Email (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetContactEmail__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// User and cloud tag keys
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The region
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The severity of the alert
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// Name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "riskFactors__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// The cloud provider account id (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountId__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "resourceType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The ID of the CSV file to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "id__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "imageName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The cloud provider account name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// User and cloud tag keys not exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// Asset Contact Email
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "activeCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Last Seen date and time for the asset
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "s1UpdatedAt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetEnvironment__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "missingCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// User and cloud tags (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "region__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// Name (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "names__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "surfaces__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// User and cloud tag keys exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The cloud tags key value (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The cloud tags key
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "subCategory__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "id__in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// The status alerts of the asset
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The cloud provider project ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderProjectId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl PerformActionQuery {
    /// Free-text filter by tag key (supports multiple values)
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_contains = Some(joined);
        self
    }
    /// The Asset Type
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type_contains = Some(joined);
        self
    }
    /// The criticality that each asset belongs to (not in)
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_criticality_nin = Some(joined);
        self
    }
    /// The status alerts of the asset (not in)
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.infection_status_nin = Some(joined);
        self
    }
    /// Tags
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value = Some(joined);
        self
    }
    /// The missing coverage for the asset
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.missing_coverage = Some(joined);
        self
    }
    /// User and cloud tags
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_value = Some(joined);
        self
    }
    /// The cloud provider organization unit
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_organization_unit_contains = Some(joined);
        self
    }
    /// The cloud provider account name (not in)
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name_nin = Some(joined);
        self
    }
    /// The geographical area where cloud resources are hosted
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region_contains = Some(joined);
        self
    }
    /// Tag Keys (not in)
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_nin = Some(joined);
        self
    }
    /// The cloud resource ID
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_resource_id_contains = Some(joined);
        self
    }
    /// The cloud provider subscription ID
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_subscription_id_contains = Some(joined);
        self
    }
    /// The cloud provider account id
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id = Some(joined);
        self
    }
    /// The asset review (not in)
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.device_review_nin = Some(joined);
        self
    }
    /// The sub-category that each resource belongs to
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.sub_category = Some(joined);
        self
    }
    /// The cloud provider organization
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_organization_contains = Some(joined);
        self
    }
    /// Tag Keys exists
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_exists = Some(joined);
        self
    }
    /// The cloud provider account name
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name = Some(joined);
        self
    }
    /// The status of the asset (not in)
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_status_nin = Some(joined);
        self
    }
    /// The asset review
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.device_review = Some(joined);
        self
    }
    /// Asset Contact Email (not in)
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_contact_email_nin = Some(joined);
        self
    }
    /// User and cloud tag keys
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key = Some(joined);
        self
    }
    /// The region
    pub fn region<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region = Some(joined);
        self
    }
    /// The severity of the alert
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.alert_severity = Some(joined);
        self
    }
    /// List of Account IDs to filter by
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.account_ids = Some(joined);
        self
    }
    /// The cloud tags key value
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value = Some(joined);
        self
    }
    /// Name
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.names = Some(joined);
        self
    }
    /// Free-text filter by tag key value (supports multiple values)
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value_contains = Some(joined);
        self
    }
    /// The cloud tags key (not in)
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_nin = Some(joined);
        self
    }
    /// Tag Keys
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key = Some(joined);
        self
    }
    /// The name
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.name_contains = Some(joined);
        self
    }
    /// The criticality that each asset belongs to
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_criticality = Some(joined);
        self
    }
    /// The risk factors associated with the asset (not in)
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.risk_factors_nin = Some(joined);
        self
    }
    /// Tag Keys not exists
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_nexists = Some(joined);
        self
    }
    /// The cloud provider account id (not in)
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id_nin = Some(joined);
        self
    }
    /// The canonical name for the resource type (not in)
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type_nin = Some(joined);
        self
    }
    /// The ID of the CSV file to filter by
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// The ID
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.id_contains = Some(joined);
        self
    }
    /// The cloud provider account ID
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id_contains = Some(joined);
        self
    }
    /// Free-text filter by the image name
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.image_name_contains = Some(joined);
        self
    }
    /// List of Group IDs to filter by
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.group_ids = Some(joined);
        self
    }
    /// The cloud provider account name
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name_contains = Some(joined);
        self
    }
    /// The status of the asset
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_status = Some(joined);
        self
    }
    /// User and cloud tag keys not exists
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_nexists = Some(joined);
        self
    }
    /// Asset Contact Email
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_contact_email = Some(joined);
        self
    }
    /// The risk factors associated with the asset
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.risk_factors = Some(joined);
        self
    }
    /// The active coverage for the asset (not in)
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.active_coverage_nin = Some(joined);
        self
    }
    /// List of Site IDs to filter by
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.site_ids = Some(joined);
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_environment = Some(joined);
        self
    }
    /// The Last Seen date and time for the asset
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    /// The canonical name for the resource type
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type = Some(joined);
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_environment_nin = Some(joined);
        self
    }
    /// The missing coverage for the asset (not in)
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.missing_coverage_nin = Some(joined);
        self
    }
    /// User and cloud tags (not in)
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_value_nin = Some(joined);
        self
    }
    /// The region (not in)
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region_nin = Some(joined);
        self
    }
    /// User and cloud tag keys (not in)
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_nin = Some(joined);
        self
    }
    /// Name (not in)
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.names_nin = Some(joined);
        self
    }
    /// Free-text filter by cloud tag key (supports multiple values)
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_contains = Some(joined);
        self
    }
    /// Tags (not in)
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value_nin = Some(joined);
        self
    }
    /// The Surface that each asset belongs to (not in)
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.surfaces_nin = Some(joined);
        self
    }
    /// User and cloud tag keys exists
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_exists = Some(joined);
        self
    }
    /// The cloud tags key value (not in)
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value_nin = Some(joined);
        self
    }
    /// The cloud tags key
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key = Some(joined);
        self
    }
    /// The Surface that each asset belongs to
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.surfaces = Some(joined);
        self
    }
    /// The sub-category that each resource belongs to (not in)
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.sub_category_nin = Some(joined);
        self
    }
    /// The ID
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.id_in = Some(joined);
        self
    }
    /// The status alerts of the asset
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.infection_status = Some(joined);
        self
    }
    /// The active coverage for the asset
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.active_coverage = Some(joined);
        self
    }
    /// The columns for which filter count would be returned for
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.counts_for = Some(joined);
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values)
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value_contains = Some(joined);
        self
    }
    /// The cloud provider project ID
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_project_id_contains = Some(joined);
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/governance/available-actions/with-status` (Available actions).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionsQuery {
    /// Free-text filter by tag key (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The Asset Type
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "resourceType__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetCriticality__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "infectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// Tags
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider organization unit
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderOrganizationUnit__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider account name (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountName__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The geographical area where cloud resources are hosted
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "region__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// Tag Keys (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudResourceId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderSubscriptionId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// The cloud provider account id
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The asset review (not in)
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "deviceReview__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The cloud provider organization
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderOrganization__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// Tag Keys exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The cloud provider account name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The asset review
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// Asset Contact Email (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetContactEmail__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// User and cloud tag keys
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The region
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The severity of the alert
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// Name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "riskFactors__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// The cloud provider account id (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountId__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "resourceType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The ID of the CSV file to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "id__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "imageName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The cloud provider account name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// User and cloud tag keys not exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// Asset Contact Email
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "activeCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Last Seen date and time for the asset
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "s1UpdatedAt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetEnvironment__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "missingCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// User and cloud tags (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "region__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// Name (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "names__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "surfaces__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// User and cloud tag keys exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The cloud tags key value (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The cloud tags key
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "subCategory__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "id__in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// The status alerts of the asset
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The cloud provider project ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderProjectId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl AvailableActionsQuery {
    /// Free-text filter by tag key (supports multiple values)
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_contains = Some(joined);
        self
    }
    /// The Asset Type
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type_contains = Some(joined);
        self
    }
    /// The criticality that each asset belongs to (not in)
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_criticality_nin = Some(joined);
        self
    }
    /// The status alerts of the asset (not in)
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.infection_status_nin = Some(joined);
        self
    }
    /// Tags
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value = Some(joined);
        self
    }
    /// The missing coverage for the asset
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.missing_coverage = Some(joined);
        self
    }
    /// User and cloud tags
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_value = Some(joined);
        self
    }
    /// The cloud provider organization unit
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_organization_unit_contains = Some(joined);
        self
    }
    /// The cloud provider account name (not in)
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name_nin = Some(joined);
        self
    }
    /// The geographical area where cloud resources are hosted
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region_contains = Some(joined);
        self
    }
    /// Tag Keys (not in)
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_nin = Some(joined);
        self
    }
    /// The cloud resource ID
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_resource_id_contains = Some(joined);
        self
    }
    /// The cloud provider subscription ID
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_subscription_id_contains = Some(joined);
        self
    }
    /// The cloud provider account id
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id = Some(joined);
        self
    }
    /// The asset review (not in)
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.device_review_nin = Some(joined);
        self
    }
    /// The sub-category that each resource belongs to
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.sub_category = Some(joined);
        self
    }
    /// The cloud provider organization
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_organization_contains = Some(joined);
        self
    }
    /// Tag Keys exists
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_exists = Some(joined);
        self
    }
    /// The cloud provider account name
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name = Some(joined);
        self
    }
    /// The status of the asset (not in)
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_status_nin = Some(joined);
        self
    }
    /// The asset review
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.device_review = Some(joined);
        self
    }
    /// Asset Contact Email (not in)
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_contact_email_nin = Some(joined);
        self
    }
    /// User and cloud tag keys
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key = Some(joined);
        self
    }
    /// The region
    pub fn region<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region = Some(joined);
        self
    }
    /// The severity of the alert
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.alert_severity = Some(joined);
        self
    }
    /// List of Account IDs to filter by
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.account_ids = Some(joined);
        self
    }
    /// The cloud tags key value
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value = Some(joined);
        self
    }
    /// Name
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.names = Some(joined);
        self
    }
    /// Free-text filter by tag key value (supports multiple values)
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value_contains = Some(joined);
        self
    }
    /// The cloud tags key (not in)
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_nin = Some(joined);
        self
    }
    /// Tag Keys
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key = Some(joined);
        self
    }
    /// The name
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.name_contains = Some(joined);
        self
    }
    /// The criticality that each asset belongs to
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_criticality = Some(joined);
        self
    }
    /// The risk factors associated with the asset (not in)
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.risk_factors_nin = Some(joined);
        self
    }
    /// Tag Keys not exists
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_nexists = Some(joined);
        self
    }
    /// The cloud provider account id (not in)
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id_nin = Some(joined);
        self
    }
    /// The canonical name for the resource type (not in)
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type_nin = Some(joined);
        self
    }
    /// The ID of the CSV file to filter by
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// The ID
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.id_contains = Some(joined);
        self
    }
    /// The cloud provider account ID
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id_contains = Some(joined);
        self
    }
    /// Free-text filter by the image name
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.image_name_contains = Some(joined);
        self
    }
    /// List of Group IDs to filter by
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.group_ids = Some(joined);
        self
    }
    /// The cloud provider account name
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name_contains = Some(joined);
        self
    }
    /// The status of the asset
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_status = Some(joined);
        self
    }
    /// User and cloud tag keys not exists
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_nexists = Some(joined);
        self
    }
    /// Asset Contact Email
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_contact_email = Some(joined);
        self
    }
    /// The risk factors associated with the asset
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.risk_factors = Some(joined);
        self
    }
    /// The active coverage for the asset (not in)
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.active_coverage_nin = Some(joined);
        self
    }
    /// List of Site IDs to filter by
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.site_ids = Some(joined);
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_environment = Some(joined);
        self
    }
    /// The Last Seen date and time for the asset
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    /// The canonical name for the resource type
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type = Some(joined);
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_environment_nin = Some(joined);
        self
    }
    /// The missing coverage for the asset (not in)
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.missing_coverage_nin = Some(joined);
        self
    }
    /// User and cloud tags (not in)
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_value_nin = Some(joined);
        self
    }
    /// The region (not in)
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region_nin = Some(joined);
        self
    }
    /// User and cloud tag keys (not in)
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_nin = Some(joined);
        self
    }
    /// Name (not in)
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.names_nin = Some(joined);
        self
    }
    /// Free-text filter by cloud tag key (supports multiple values)
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_contains = Some(joined);
        self
    }
    /// Tags (not in)
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value_nin = Some(joined);
        self
    }
    /// The Surface that each asset belongs to (not in)
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.surfaces_nin = Some(joined);
        self
    }
    /// User and cloud tag keys exists
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_exists = Some(joined);
        self
    }
    /// The cloud tags key value (not in)
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value_nin = Some(joined);
        self
    }
    /// The cloud tags key
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key = Some(joined);
        self
    }
    /// The Surface that each asset belongs to
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.surfaces = Some(joined);
        self
    }
    /// The sub-category that each resource belongs to (not in)
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.sub_category_nin = Some(joined);
        self
    }
    /// The ID
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.id_in = Some(joined);
        self
    }
    /// The status alerts of the asset
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.infection_status = Some(joined);
        self
    }
    /// The active coverage for the asset
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.active_coverage = Some(joined);
        self
    }
    /// The columns for which filter count would be returned for
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.counts_for = Some(joined);
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values)
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value_contains = Some(joined);
        self
    }
    /// The cloud provider project ID
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_project_id_contains = Some(joined);
        self
    }
}

/// Query params for `GET /web/api/v2.1/xdr/assets/governance/export` (Export assets to CSV or JSON).
///
/// The required `exportFormat` param is passed as a method argument, not part
/// of this struct. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportQuery {
    /// Free-text filter by tag key (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetCriticality__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountName__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// Tag Keys (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudResourceId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderSubscriptionId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The region
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "riskFactors__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor".
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The cloud provider account ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "imageName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "activeCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// User and cloud tags (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "region__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The cloud tags key value (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "missingCoverage__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The column to sort the results by.
    ///
    /// Allowed values: `s1GroupName`, `s1OnboardedAccountName`, `cloudProviderProjectId`, `category`, `s1UpdatedAt`, `s1GroupId`, `cloudProviderAccountId`, `s1OnboardedScopeLevel`, `createdTime`, `subCategory`, `cloudProviderAccountName`, `s1OnboardedScopeId`, `cloudProviderResourceGroup`, `deviceReview`, `region`, `cloudProviderOrganization`, `s1ManagementId`, `name`, `assetCriticality`, `s1OnboardedAccountId`, `s1OnboardedScopePath`, `s1ScopePath`, `cloudResourceUid`, `s1OnboardedGroupName`, `cloudProviderSubscriptionId`, `cloudResourceId`, `cloudProviderOrganizationUnit`, `cloudProviderOrganizationUnitPath`, `s1OnboardedSiteName`, `s1ScopeId`, `assetStatus`, `assetContactEmail`, `s1ScopeType`, `assetEnvironment`, `resourceType`, `s1OnboardedGroupId`, `s1SiteId`, `s1AccountId`, `id`, `s1SiteName`, `infectionStatus`, `s1AccountName`, `s1ScopeLevel`, `s1OnboardedSiteId`, `cloudProviderUrlString`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The geographical area where cloud resources are hosted
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "region__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The asset review (not in)
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "deviceReview__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The cloud provider account name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetContactEmail__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "resourceType__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// Asset Contact Email
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "surfaces__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "id__in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKeyValue__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "resourceType__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// Tags
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction
    ///
    /// Allowed values: `asc`, `desc`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The cloud provider organization unit
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderOrganizationUnit__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderOrganization__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "name__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountId__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up execution time.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// User and cloud tag keys not exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__nexists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "s1UpdatedAt__between")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`, `AD GPO`, `AD Group`, `AD OU`, `AD Security Principals`, `AD Service Account`, `AD User`, `Alarm`, `Alibaba Account`, `Alibaba Action Trail`, `Alibaba Anti DDOS Domain Log Status`, `Alibaba Application Load Balancer`, `Alibaba Application Load Balancer Listener`, `Alibaba Auto Scaling Configuration`, `Alibaba Auto Scaling Group`, `Alibaba Auto Scaling Image`, `Alibaba Bucket`, `Alibaba Bucket Policy`, `Alibaba Container Registry`, `Alibaba Container Repository`, `Alibaba ECS Disk`, `Alibaba ECS Instance`, `Alibaba ECS Network Interface`, `Alibaba Folder`, `Alibaba Kubernetes Cluster`, `Alibaba Management`, `Alibaba Network Load Balancer`, `Alibaba Network Load Balancer Listener`, `Alibaba RAM Access Key`, `Alibaba RAM Group`, `Alibaba RAM Password Policy`, `Alibaba RAM Policy`, `Alibaba RAM Role`, `Alibaba RAM User`, `Alibaba RDS Instance`, `Alibaba Security Center Agent Status`, `Alibaba Security Center Antivirus Config`, `Alibaba Security Center Vulnerability Config`, `Alibaba Security Center WebShell Configuration`, `Alibaba Security Group`, `Alibaba Server Load Balancer`, `Alibaba Server Load Balancer Listener`, `Alibaba VPC`, `Alibaba VPC Flow Log`, `Alibaba Web Application Firewall Domain Log Status`, `Amplifier`, `AV Solution`, `AWS Access Analyzer`, `AWS Account`, `AWS ACM Certificate`, `AWS API Gateway API`, `AWS API Gateway API Stage`, `AWS API Gateway Client Certificate`, `AWS API Gateway Domain`, `AWS API Gateway Rest API`, `AWS API Gateway Rest API Resource`, `AWS API Gateway Rest API Stage`, `AWS API Gateway Rest Domain`, `AWS Athena WorkGroup`, `AWS AutoScaling Launch Configuration`, `AWS Auto Scaling Group`, `AWS Backup Vault`, `AWS Bedrock Agent`, `AWS Bedrock Agent Version`, `AWS Bedrock Batch Inference Job`, `AWS Bedrock Custom Model`, `AWS Bedrock Guardrail`, `AWS Bedrock Knowledge Base`, `AWS Bedrock Knowledge Base Data Source`, `AWS Bedrock Model Customization Job`, `AWS Bedrock Model Invocation Logging Config`, `AWS Bedrock Prompt`, `AWS Bedrock Prompt Flows`, `AWS Budgets`, `AWS Classic Load Balancer`, `AWS CloudFormation Stack`, `AWS CloudFront Distribution`, `AWS CloudFront Distribution Origin`, `AWS CloudTrail Event Selectors`, `AWS CloudTrail Trail`, `AWS CloudWatch Alarm`, `AWS CloudWatch Log Group`, `AWS CloudWatch Metric Filter`, `AWS Config Recorder`, `AWS Config Recorder Status`, `AWS Container Registry ECR`, `AWS Container Repository`, `AWS Cost Explorer`, `AWS DAX Cluster`, `AWS DMS Certificate`, `AWS DMS Replication Instance`, `AWS Document DB Cluster`, `AWS Document DB SnapShot Cluster`, `AWS DynamoDB Backup`, `AWS DynamoDB Table`, `AWS EBS Snapshot`, `AWS EBS Volume`, `AWS EBS Volume Encryption Account Setting`, `AWS ECS Cluster`, `AWS ECS Container`, `AWS ECS Service`, `AWS ECS Task`, `AWS ECS Task Definition`, `AWS ECS Node`, `AWS EC2 Elastic IP`, `AWS EC2 Instance`, `AWS EC2 Key Pair`, `AWS EC2 Network Interface`, `AWS EC2 Security Group`, `AWS Egress Only Internet Gateways`, `AWS Elasticsearch Domain`, `AWS Elastic BeanStalk Configuration Setting`, `AWS Elastic BeanStalk Environment`, `AWS Elastic Cache Cluster`, `AWS Elastic File System`, `AWS Elastic Load Balancer`, `AWS Elastic Load Balancer Listener`, `AWS Elastic Loadbalancer Listener Rule`, `AWS ELBv2 Target Group`, `AWS ELBv2 Target Group Health`, `AWS EMR Cluster`, `AWS EMR Cluster Security Configuration`, `AWS FSx File System`, `AWS Fargate Profile`, `AWS Glue Database`, `AWS Glue Security Configuration`, `AWS IAM Account Summary`, `AWS IAM Group`, `AWS IAM Inline Policy`, `AWS IAM Password Policy`, `AWS IAM Permission Boundary`, `AWS IAM Policy`, `AWS IAM Role`, `AWS IAM Server Certificate`, `AWS IAM User`, `AWS IAM User Access Key`, `AWS IAM User SSH Key`, `AWS IAM Virtual MFA Device`, `AWS Identity Center Group`, `AWS Identity Center Instance`, `AWS Identity Center User`, `AWS Kafka Cluster`, `AWS Kinesis Firehose Stream`, `AWS Kinesis Stream`, `AWS Kubernetes Cluster EKS`, `AWS KMS Key`, `AWS KMS Key Policy`, `AWS Lambda Function`, `AWS Lambda Layer`, `AWS Lightsail Bucket`, `AWS Lightsail Database`, `AWS Lightsail Instance`, `AWS Lightsail Instance Alarm`, `AWS Lightsail Load Balancers`, `AWS Machine Image`, `AWS MemoryDB Cluster`, `AWS MQ Broker`, `AWS MWAA Environment`, `AWS Neptune DB Cluster`, `AWS Neptune DB Cluster Parameter Group`, `AWS OpenSearch Domain`, `AWS Organization`, `AWS Organizational Unit`, `AWS Permission Set`, `AWS RDS Cluster`, `AWS RDS Cluster Parameter`, `AWS RDS Cluster Snapshot`, `AWS RDS Instance`, `AWS RDS Parameter`, `AWS RDS Snapshot`, `AWS Redshift Cluster`, `AWS Redshift Cluster Parameter`, `AWS Redshift Reserved Node`, `AWS Root Organizational Unit`, `AWS Route53 Domain`, `AWS Route53 Record Value`, `AWS S3 Bucket`, `AWS SageMaker Compilation Jobs`, `AWS SageMaker Endpoint`, `AWS SageMaker Endpoint Config`, `AWS SageMaker Hyper Parameter Tuning Job`, `AWS SageMaker Inference Recommender`, `AWS SageMaker Instance`, `AWS SageMaker Labeling Job`, `AWS SageMaker Model`, `AWS SageMaker Model Package`, `AWS SageMaker Processing Jobs`, `AWS SageMaker Shadow Test`, `AWS SageMaker Training Job`, `AWS SageMaker Transformation Jobs`, `AWS Secrets Manager`, `AWS SNS Topic`, `AWS SQS Queue`, `AWS Shield Emergency Contact`, `AWS Shield Protection`, `AWS Shield Subscription`, `AWS SSM Association`, `AWS SSM Instance Information`, `AWS SSM Parameter`, `AWS Transfer Server`, `AWS Trusted Advisor`, `AWS Virtual Private Cloud`, `AWS VPC Accepter Peering Connection`, `AWS VPC Endpoint`, `AWS VPC Flow Log`, `AWS VPC Internet Gateway`, `AWS VPC NAT Gateway`, `AWS VPC Network ACL`, `AWS VPC Peering Connection`, `AWS VPC Requester Peering Connection`, `AWS VPC Route Table`, `AWS VPC Subnet`, `AWS VPC Transit Gateway`, `AWS VPN Connection`, `AWS VPN Gateway`, `AWS WAF ACL`, `AWS WAF Regional`, `AWS WorkSpaces Directory`, `AWS WorkSpaces Group`, `AWS WorkSpaces Space`, `AWS XRay Encryption Config`, `Azure Activity Log Alert`, `Azure Advisor`, `Azure AI Content Filter Policy`, `Azure AI Custom Vision Service`, `Azure AI Deployment`, `Azure AI Face API Service`, `Azure AI Service`, `Azure AI Service Multi Account`, `Azure AI Speech Service`, `Azure AI Translator Service`, `Azure All Activity Log Alert`, `Azure All Defender For Cloud Pricing Configurations`, `Azure All Defender For Cloud Settings`, `Azure Api Management Named Value`, `Azure Api Management Service`, `Azure Api Management Service Backend`, `Azure Api Management Service Portal Setting`, `Azure App Configuration Store`, `Azure Application Gateway`, `Azure Application Gateway Web Application Firewall Policy`, `Azure App Registration`, `Azure App Service Certificate`, `Azure App Service Plan`, `Azure App Service Web App`, `Azure App Service Web App Auth Settings`, `Azure App Service Web App Configuration`, `Azure App Service Web App Slot`, `Azure Authorization Policy`, `Azure Automation Account`, `Azure Automation Account Variable`, `Azure Backend Address Pool`, `Azure Blob Container`, `Azure Blob Service`, `Azure Bot Service`, `Azure CDN Endpoint`, `Azure CDN Profile`, `Azure Classic Front Door`, `Azure Computer Vision Service`, `Azure Container Instance`, `Azure Container App`, `Azure Container Registry`, `Azure Container Repository`, `Azure Content Safety Service`, `Azure Cosmos DB Account`, `Azure Cosmos DB Account Advanced Threat Protection`, `Azure Cost Management and Billing`, `Azure Data Factory`, `Azure Data Factory Integration Runtime`, `Azure Data Factory Linked Service`, `Azure Defender For Cloud Auto Provisioning Setting`, `Azure Defender For Cloud Pricing Configurations`, `Azure Defender For Cloud Settings`, `Azure Diagnostic Setting`, `Azure Directory Role Definition`, `Azure Directory Role Assignment`, `Azure Disk`, `Azure DNS Record Set`, `Azure DNS Record Value`, `Azure DNS Zone`, `Azure Document Intelligence`, `Azure Frontend IP Configuration`, `Azure Front Door Web Application Firewall Policy`, `Azure Health Insights`, `Azure Health Probe`, `Azure IAM Custom Role`, `Azure IAM Role`, `Azure Identity`, `Azure Immersive Reader Service`, `Azure Inbound NAT Rule`, `Azure Key Vault`, `Azure Key Vault Certificate`, `Azure Key Vault Key`, `Azure Key Vault Secret`, `Azure Kubernetes Cluster AKS`, `Azure Kubernetes Cluster Upgrade Profile`, `Azure Language Service`, `Azure Load Balancer`, `Azure Load Balancing Rule`, `Azure Log Profile`, `Azure Machine Learning Workspace`, `Azure Management Group`, `Azure MariaDB Server Security Policy`, `Azure Maria DB Server`, `Azure MySQL Database`, `Azure MySQL Flexible Server`, `Azure MySQL Flexible Server Firewall Rule`, `Azure MySQL Server`, `Azure MySQL Server Firewall Rule`, `Azure MySQL Server Security Alert Policy`, `Azure NAT Gateway`, `Azure Network Interface`, `Azure Network Security Group`, `Azure Network Watcher`, `Azure Network Watcher Flow Log`, `Azure OpenAI Account`, `Azure Outbound Rule`, `Azure Postgres Database`, `Azure Postgres Flexible Server`, `Azure Postgres Flexible Server All Parameters`, `Azure Postgres Flexible Server Firewall Rule`, `Azure Postgres Flexible Server Parameter`, `Azure Postgres Server`, `Azure Postgres Server All Parameters`, `Azure Postgres Server Firewall Rule`, `Azure Postgres Server Parameter`, `Azure Postgres Server Security Alert Policy`, `Azure Private Endpoint`, `Azure Private Endpoint Connection`, `Azure Private Link`, `Azure Public IP Address`, `Azure Queue`, `Azure Redis Cache`, `Azure Resource Group`, `Azure Role Assignment`, `Azure Route Table`, `Azure Scale Set Virtual Machine`, `Azure Scale Set Virtual Machine Extension`, `Azure Security Contact`, `Azure Security Rule`, `Azure Service Principal`, `Azure SQL Advanced Threat Protection Setting`, `Azure SQL Blob Auditing Policy`, `Azure SQL Database`, `Azure SQL Database Blob Auditing Policy`, `Azure SQL Database Security Alert Policy`, `Azure SQL Managed Instance`, `Azure SQL Managed Instance Security Alert`, `Azure SQL Managed Instance vulnerability assessment`, `Azure SQL Server`, `Azure SQL Server Blob Auditing Policy`, `Azure SQL Server ENCRYPTION Protector`, `Azure SQL Server Firewall Rule`, `Azure SQL Server Vulnerability assessment`, `Azure SQL Vulnerability assessment setting`, `Azure Static Web App`, `Azure Storage Account`, `Azure Storage Account Blob All Diagnostic Setting`, `Azure Storage Account Queue All Diagnostic Setting`, `Azure Storage Account Table`, `Azure Storage Account Table All Diagnostic Setting`, `Azure Subnet`, `Azure Subscription`, `Azure Subscription Geolocations`, `Azure Synapse Sql Pool`, `Azure Synapse Sql Pool Vulnerability Assessment`, `Azure Synapse Workspace`, `Azure Synapse Workspace Sql Server Tls Setting`, `Azure Tenant`, `Azure Traffic Manager`, `Azure User`, `Azure User Group`, `Azure User Registration Details`, `Azure Virtual Machine`, `Azure Virtual Machine Extension`, `Azure Virtual Machine Scale Set`, `Azure Virtual Network`, `Azure Virtual Network Gateway`, `Camera`, `Cameras and Vision Platforms`, `Car Multimedia`, `Clocks and NTP Servers`, `Cloud Endpoint`, `Cloud NAT`, `Cloud Router`, `Conferencing Solution`, `Container Image`, `Container Image Tag`, `Container Registry`, `Container Repository`, `Copier`, `Developer Repository`, `DigitalOcean App`, `DigitalOcean CDN Endpoint`, `DigitalOcean Container Registry`, `DigitalOcean Container Repository`, `DigitalOcean Database`, `DigitalOcean Database Cluster`, `DigitalOcean Database User`, `DigitalOcean Domain`, `DigitalOcean Domain Record`, `DigitalOcean Domain Record Value`, `DigitalOcean Droplet`, `DigitalOcean Droplet Backup`, `DigitalOcean Droplet Snapshot`, `DigitalOcean Firewall`, `DigitalOcean Kubernetes Cluster`, `DigitalOcean Kubernetes Node`, `DigitalOcean Kubernetes Node Pool`, `DigitalOcean Load Balancer`, `DigitalOcean Reserved IP`, `DigitalOcean SSH Key`, `DigitalOcean Volume`, `DigitalOcean Volume Snapshot`, `DigitalOcean VPC`, `Dynamic Admission Controller`, `Doorbell`, `DVR`, `eBook`, `Embedded`, `Enterprise IoT`, `Entra ID Group`, `Entra ID User`, `Extender`, `External DNS Hosted Zone`, `External DNS Name`, `External DNS Value`, `Fire Detection and Access Control`, `Gaming Console`, `Gateway`, `GCP API Key`, `GCP Artifact Registry`, `GCP Artifact Repository`, `GCP BigQuery Dataset`, `GCP Bigtable Instance`, `GCP Bigtable Instance Cluster`, `GCP Cloud Armor Security Policy`, `GCP Cloud Deploy Delivery Pipeline`, `GCP Cloud Deploy Target`, `GCP Cloud Function`, `GCP Cloud Run Job`, `GCP Cloud Run Revision`, `GCP Cloud Run Service`, `GCP Cloud Storage`, `GCP Composer Environment`, `GCP Compute Auto Scaler`, `GCP Compute Disk`, `GCP Compute Image`, `GCP Compute Instance`, `GCP Compute Instance Group`, `GCP Compute Instance Group Manager`, `GCP Compute Instance Template`, `GCP Compute Snapshot`, `GCP Container Registry`, `GCP Container Repository`, `GCP Data Fusion Instance`, `GCP Dataflow Job`, `GCP Dataplex Lake`, `GCP Dataplex Lake Zone`, `GCP Dataproc Cluster`, `GCP Deployment Manager Deployment`, `GCP Deployment Manifest`, `GCP DNS Policy`, `GCP DNS Record Set`, `GCP DNS Record Value`, `GCP DNS Zone`, `GCP Filestore Backup`, `GCP Filestore Instance`, `GCP Filestore Instance Snapshot`, `GCP Firestore Database`, `GCP Folder`, `GCP IAM Group`, `GCP IAM Member`, `GCP IAM Role`, `GCP IAM Role Assignment`, `GCP IAM Service Account`, `GCP IAM Service Account Key`, `GCP KMS Crypto Key`, `GCP KMS Key Ring`, `GCP Kubernetes Cluster GKE`, `GCP Kubernetes Engine Node Pool`, `GCP Load Balancer Backend Bucket`, `GCP Load Balancer Backend Service`, `GCP Load Balancer Forwarding Rule`, `GCP Load Balancer SSL Policy`, `GCP Load Balancer Target HTTPS Proxy`, `GCP Load Balancer Target HTTP Proxy`, `GCP Load Balancer URL Map`, `GCP Logging Sink`, `GCP MemoryStore Memcached Instance`, `GCP MemoryStore Redis Instance`, `GCP Organization`, `GCP Project`, `GCP Pub Sub Subscription`, `GCP Pub Sub Topic`, `GCP Secret Manager Secret`, `GCP Spanner Database`, `GCP Spanner Instance`, `GCP Spanner Instance Backup`, `GCP Spanner Instance Config`, `GCP SQL Instance`, `GCP SQL User`, `GCP Vertex AI Batch Prediction`, `GCP Vertex AI Custom Jobs`, `GCP Vertex AI Dataset`, `GCP Vertex AI Deployment Resource Pool`, `GCP Vertex AI Endpoint`, `GCP Vertex AI Hyper Parameter Tuning Jobs`, `GCP Vertex AI Metadata`, `GCP Vertex AI Models Registry`, `GCP Vertex AI Notebook Instance`, `GCP Vertex AI Persistent Resources`, `GCP Vertex AI Tensorboard Instance`, `GCP Vertex AI Training Pipeline`, `GCP Vertex AI Vector Search Indexes`, `GCP Vertex AI Vector Search Index Endpoint`, `GCP VPC Firewall`, `GCP VPC Firewall Network Tag`, `GCP VPC Network`, `GCP VPC Sub Network`, `Google Cloud Billing`, `Google Cloud Cost Management`, `Google Cloud Recommender`, `Google My Drive`, `Google Shared Drive`, `Health Monitor`, `Home Assistant`, `Home Hub`, `Hub`, `ID Card Printer`, `IP Phone`, `Kubernetes Cluster Role`, `Kubernetes Cluster Role Binding`, `Kubernetes Config Map`, `Kubernetes CronJob`, `Kubernetes Daemon Set`, `Kubernetes Deployment`, `Kubernetes Group`, `Kubernetes Job`, `Kubernetes Namespace`, `Kubernetes Network Policy`, `Kubernetes Persistent Volume`, `Kubernetes Replica Set`, `Kubernetes Role`, `Kubernetes Role Binding`, `Kubernetes Secret`, `Kubernetes Service`, `Kubernetes Service Account`, `Kubernetes Stateful Set`, `Kubernetes User`, `Kubernetes Volume`, `Kubernetes Node`, `K8s Cluster Node`, `K8s Pod`, `Lighting Solution`, `Light Bulb`, `Light Controller`, `Linux desktop`, `Linux laptop`, `Linux Server`, `Linux workstation`, `macOS desktop`, `macOS laptop`, `macOS Server`, `macOS workstation`, `Mesh`, `Microsoft 365 OneDrive`, `Mobile`, `NAS`, `Network Device`, `Network Storage`, `Okta User`, `Okta Group`, `Oracle Compartment`, `Oracle Tenant`, `Oracle Artifact`, `Oracle Artifact Repository`, `Oracle Authentication Policy`, `Oracle Block Volume`, `Oracle Block Volume Backup`, `Oracle Boot Volume`, `Oracle Boot Volume Backup`, `Oracle Bucket`, `Oracle Cloud Guard Config`, `Oracle Compute Instance`, `Oracle Container Registry`, `Oracle Container Repository`, `Oracle Customer Secret Key`, `Oracle Database`, `Oracle Database Home`, `Oracle DNS Zone`, `Oracle Event Rule`, `Oracle File System`, `Oracle File System Export`, `Oracle File System Export Options`, `Oracle Group`, `Oracle IAM Role`, `Oracle Instance Pool`, `Oracle Kubernetes Cluster`, `Oracle Kubernetes Node Pool`, `Oracle Load Balancer`, `Oracle Log`, `Oracle Log Group`, `Oracle Network Load Balancer`, `Oracle Network Security Group`, `Oracle Policy`, `Oracle Reserved IP`, `Oracle Security List`, `Oracle Subnet`, `Oracle User`, `Oracle User API Key`, `Oracle User Auth Token`, `Oracle VCN`, `Oracle VNIC`, `Oracle VNIC Attachment`, `Oracle Volume Backup Policy`, `Oracle Zone Record`, `Oracle Zone Record Value`, `Organization Repository`, `Ping ID User`, `Ping ID Group`, `Phone Adapter`, `Physical Server`, `POS solutions`, `Printer`, `Radio`, `Receiver`, `Repository`, `Router`, `Safety, Security and Communication System`, `Security`, `Self Managed Kubernetes Cluster`, `Server Infrastructure`, `Set Top Boxes`, `Smart Home`, `Smart Office`, `Smart Plug`, `Smart TV`, `Smart Watch`, `Snowflake Database`, `Solar Energy Solution`, `Speaker`, `Static Admission Controller`, `Storage`, `Streamer`, `Subnet`, `Surveillance System`, `Switch`, `Tablet`, `Touchscreens and control System`, `TV Tuner`, `Unknown Device`, `Unknown Server`, `Unknown workstation`, `UPS`, `User`, `Vacuum`, `Video`, `Virtual Desktop Interface`, `Virtual Network Peering`, `Virtual Server`, `Water Control`, `Windows desktop`, `Windows laptop`, `Windows Server`, `Windows workstation`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "assetEnvironment__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "names__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The cloud tags key
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "subCategory__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`, `CNS VM Scan`, `CNS Secret Scan`, `CNS IaC Scan`, `CNS Image Scan`, `CNS Detect`, `CNS Remediate`, `IDR`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The cloud provider account id
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`, `AI Service`, `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`, `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`, `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`, `Code Repository`, `Configuration Policy`, `Container`, `Container Host`, `Container Management`, `Content Delivery Network`, `Database`, `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`, `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`, `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`, `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`, `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`, `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`, `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`, `Network Access Control`, `Network Device`, `Network Interface`, `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`, `Notification Service`, `Object`, `Object Storage`, `Other Device`, `Other Server`, `Other Workstation`, `Payment System`, `Peering`, `Physical Server`, `Printer`, `Queuing Service`, `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`, `Roles & Permissions`, `SaaS`, `Secret`, `Security`, `Security Management`, `Serverless Function`, `Server Infrastructure`, `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`, `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`, `Virtual Network`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Limit number of returned items (1-1000)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "id__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account name
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderAccountName__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudTagsKey__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in)
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "tagsKeyValue__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "allTagsKey__exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "infectionStatus__nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The cloud provider project ID
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "cloudProviderProjectId__contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl ExportQuery {
    /// Free-text filter by tag key (supports multiple values)
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_contains = Some(joined);
        self
    }
    /// The criticality that each asset belongs to (not in)
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_criticality_nin = Some(joined);
        self
    }
    /// The missing coverage for the asset
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.missing_coverage = Some(joined);
        self
    }
    /// User and cloud tags
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_value = Some(joined);
        self
    }
    /// The cloud provider account name (not in)
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name_nin = Some(joined);
        self
    }
    /// Tag Keys (not in)
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_nin = Some(joined);
        self
    }
    /// The cloud resource ID
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_resource_id_contains = Some(joined);
        self
    }
    /// The cloud provider subscription ID
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_subscription_id_contains = Some(joined);
        self
    }
    /// Tag Keys exists
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_exists = Some(joined);
        self
    }
    /// The region
    pub fn region<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region = Some(joined);
        self
    }
    /// Free-text filter by tag key value (supports multiple values)
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value_contains = Some(joined);
        self
    }
    /// The cloud tags key (not in)
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_nin = Some(joined);
        self
    }
    /// Tag Keys
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key = Some(joined);
        self
    }
    /// The risk factors associated with the asset (not in)
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.risk_factors_nin = Some(joined);
        self
    }
    /// Tag Keys not exists
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_nexists = Some(joined);
        self
    }
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor".
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// The cloud provider account ID
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id_contains = Some(joined);
        self
    }
    /// Free-text filter by the image name
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.image_name_contains = Some(joined);
        self
    }
    /// List of Group IDs to filter by
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.group_ids = Some(joined);
        self
    }
    /// The active coverage for the asset (not in)
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.active_coverage_nin = Some(joined);
        self
    }
    /// User and cloud tags (not in)
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_value_nin = Some(joined);
        self
    }
    /// The region (not in)
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region_nin = Some(joined);
        self
    }
    /// User and cloud tag keys (not in)
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_nin = Some(joined);
        self
    }
    /// The cloud tags key value (not in)
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value_nin = Some(joined);
        self
    }
    /// The Surface that each asset belongs to
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.surfaces = Some(joined);
        self
    }
    /// The missing coverage for the asset (not in)
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.missing_coverage_nin = Some(joined);
        self
    }
    /// The status of the asset (not in)
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_status_nin = Some(joined);
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// The geographical area where cloud resources are hosted
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.region_contains = Some(joined);
        self
    }
    /// The asset review (not in)
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.device_review_nin = Some(joined);
        self
    }
    /// The cloud provider account name
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name = Some(joined);
        self
    }
    /// Asset Contact Email (not in)
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_contact_email_nin = Some(joined);
        self
    }
    /// The severity of the alert
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.alert_severity = Some(joined);
        self
    }
    /// List of Account IDs to filter by
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.account_ids = Some(joined);
        self
    }
    /// The cloud tags key value
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value = Some(joined);
        self
    }
    /// The canonical name for the resource type (not in)
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type_nin = Some(joined);
        self
    }
    /// Asset Contact Email
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_contact_email = Some(joined);
        self
    }
    /// The risk factors associated with the asset
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.risk_factors = Some(joined);
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_environment = Some(joined);
        self
    }
    /// The Surface that each asset belongs to (not in)
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.surfaces_nin = Some(joined);
        self
    }
    /// The ID
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.id_in = Some(joined);
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values)
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_value_contains = Some(joined);
        self
    }
    /// The Asset Type
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type_contains = Some(joined);
        self
    }
    /// Tags
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value = Some(joined);
        self
    }
    /// Sort direction
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// The cloud provider organization unit
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_organization_unit_contains = Some(joined);
        self
    }
    /// The cloud provider organization
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_organization_contains = Some(joined);
        self
    }
    /// If true, only total number of items will be returned, without any of the actual objects.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Name
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.names = Some(joined);
        self
    }
    /// The name
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.name_contains = Some(joined);
        self
    }
    /// The criticality that each asset belongs to
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_criticality = Some(joined);
        self
    }
    /// The cloud provider account id (not in)
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id_nin = Some(joined);
        self
    }
    /// If true, total number of items will not be calculated, which speeds up execution time.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// User and cloud tag keys not exists
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_nexists = Some(joined);
        self
    }
    /// List of Site IDs to filter by
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.site_ids = Some(joined);
        self
    }
    /// The Last Seen date and time for the asset
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    /// The canonical name for the resource type
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.resource_type = Some(joined);
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_environment_nin = Some(joined);
        self
    }
    /// Name (not in)
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.names_nin = Some(joined);
        self
    }
    /// The cloud tags key
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key = Some(joined);
        self
    }
    /// The sub-category that each resource belongs to (not in)
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.sub_category_nin = Some(joined);
        self
    }
    /// The status alerts of the asset
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.infection_status = Some(joined);
        self
    }
    /// The active coverage for the asset
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.active_coverage = Some(joined);
        self
    }
    /// The columns for which filter count would be returned for
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.counts_for = Some(joined);
        self
    }
    /// The ID of the CSV file to filter by
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// The cloud provider account id
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_id = Some(joined);
        self
    }
    /// The sub-category that each resource belongs to
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.sub_category = Some(joined);
        self
    }
    /// The asset review
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.device_review = Some(joined);
        self
    }
    /// User and cloud tag keys
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key = Some(joined);
        self
    }
    /// Limit number of returned items (1-1000)
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// The ID
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.id_contains = Some(joined);
        self
    }
    /// The cloud provider account name
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_account_name_contains = Some(joined);
        self
    }
    /// The status of the asset
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.asset_status = Some(joined);
        self
    }
    /// Free-text filter by cloud tag key (supports multiple values)
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_tags_key_contains = Some(joined);
        self
    }
    /// Tags (not in)
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.tags_key_value_nin = Some(joined);
        self
    }
    /// User and cloud tag keys exists
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.all_tags_key_exists = Some(joined);
        self
    }
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// The status alerts of the asset (not in)
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.infection_status_nin = Some(joined);
        self
    }
    /// The cloud provider project ID
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = v
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.cloud_provider_project_id_contains = Some(joined);
        self
    }
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl InventoryGovernanceService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/governance` — Assets.
    ///
    /// Get assets.
    pub async fn list(
        &self,
        query: &GetGovernanceQuery,
    ) -> Result<Paginated<Governance>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/governance", q)
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/governance` — Assets using POST.
    ///
    /// POST API to get Assets.
    ///
    /// The `accountIds`, `siteIds`, and `groupIds` scope filters are accepted as
    /// query params; pass an already-serialized querystring or `None`.
    pub async fn list_post(
        &self,
        query: &GovernanceViewQuery,
        body: &GovernanceViewBody,
    ) -> Result<Paginated<Governance>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<GovernanceViewBody, Paginated<Governance>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/governance",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/governance/action` — Perform action.
    ///
    /// Perform action on selected assets.
    pub async fn perform_action(
        &self,
        query: &PerformActionQuery,
        body: &GovernanceActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<GovernanceActionBody, Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/governance/action",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/governance/available-actions/with-status`
    /// — Available actions.
    ///
    /// Get cloud inventory governance available-actions.
    pub async fn available_actions(
        &self,
        query: &AvailableActionsQuery,
        body: &AffectedResourcesBody,
    ) -> Result<Response<AvailableActionWithStatusResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<AffectedResourcesBody, Response<AvailableActionWithStatusResponse>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/governance/available-actions/with-status",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/governance/export` — Export assets to CSV
    /// or JSON.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format.
    ///
    /// `export_format` is the required `exportFormat` query param. Allowed
    /// values: `csv`, `json`.
    pub async fn export(
        &self,
        export_format: impl Into<String>,
        query: &ExportQuery,
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
            .get("/web/api/v2.1/xdr/assets/governance/export", q)
            .await?)
    }
}
