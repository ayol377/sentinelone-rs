//! Models for the `Inventory` tag.
//!
//! Field nullability mirrors the SentinelOne `swagger_2_1.json` spec: a field is
//! a bare type only when it appears in the schema `required` array and is not
//! `x-nullable`; otherwise it is `Option<T>` (the spec's "default null"
//! behaviour). Enum-valued strings are kept as `String` for forward
//! compatibility, with the allowed values documented on each field.

use serde::Deserialize;

/// An inventory asset (`InventoryResponse` in the spec).
///
/// Returned by `GET`/`POST /web/api/v2.1/xdr/assets`. No field is marked
/// required in the spec, so every field is `Option`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryAsset {
    /// Id.
    pub id: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Category.
    pub category: Option<String>,
    /// Sub category.
    pub sub_category: Option<String>,
    /// Resource type.
    pub resource_type: Option<String>,
    /// S1 group name.
    pub s1_group_name: Option<String>,
    /// S1 group id.
    pub s1_group_id: Option<String>,
    /// S1 site name.
    pub s1_site_name: Option<String>,
    /// S1 site id.
    pub s1_site_id: Option<String>,
    /// S1 account name.
    pub s1_account_name: Option<String>,
    /// S1 account id.
    pub s1_account_id: Option<String>,
    /// S1 management id.
    pub s1_management_id: Option<i64>,
    /// S1 scope type.
    pub s1_scope_type: Option<i64>,
    /// S1 scope level.
    pub s1_scope_level: Option<String>,
    /// S1 scope id.
    pub s1_scope_id: Option<String>,
    /// S1 scope path.
    pub s1_scope_path: Option<String>,
    /// S1 updated at (date/time string).
    pub s1_updated_at: Option<String>,
    /// S1 onboarded account id.
    pub s1_onboarded_account_id: Option<i64>,
    /// S1 onboarded account name.
    pub s1_onboarded_account_name: Option<String>,
    /// S1 onboarded group id.
    pub s1_onboarded_group_id: Option<i64>,
    /// S1 onboarded group name.
    pub s1_onboarded_group_name: Option<String>,
    /// S1 onboarded site id.
    pub s1_onboarded_site_id: Option<i64>,
    /// S1 onboarded site name.
    pub s1_onboarded_site_name: Option<String>,
    /// S1 onboarded scope id.
    pub s1_onboarded_scope_id: Option<i64>,
    /// S1 onboarded scope level.
    pub s1_onboarded_scope_level: Option<String>,
    /// S1 onboarded scope path.
    pub s1_onboarded_scope_path: Option<String>,
    /// Cpu.
    pub cpu: Option<String>,
    /// Legacy identity policy name.
    pub legacy_identity_policy_name: Option<String>,
    /// Previous os type.
    pub previous_os_type: Option<String>,
    /// Previous os version.
    pub previous_os_version: Option<String>,
    /// Previous device function.
    pub previous_device_function: Option<String>,
    /// Asset criticality.
    pub asset_criticality: Option<String>,
    /// Asset status.
    pub asset_status: Option<String>,
    /// Asset environment.
    pub asset_environment: Option<String>,
    /// Asset contact email.
    pub asset_contact_email: Option<String>,
    /// Infection status.
    pub infection_status: Option<String>,
    /// Device review.
    pub device_review: Option<String>,
    /// Device review log.
    pub device_review_log: Option<Vec<InventoryDeviceReviewLog>>,
    /// Missing coverage.
    pub missing_coverage: Option<Vec<String>>,
    /// Active coverage.
    pub active_coverage: Option<Vec<String>>,
    /// Risk factors.
    pub risk_factors: Option<Vec<String>>,
    /// Surfaces.
    pub surfaces: Option<Vec<String>>,
    /// Tags (freeform objects).
    pub tags: Option<Vec<serde_json::Value>>,
    /// Cloud tags (freeform objects).
    pub cloud_tags: Option<Vec<serde_json::Value>>,
    /// Ranger tags.
    pub ranger_tags: Option<Vec<String>>,
    /// Source json (freeform object).
    pub source_json: Option<serde_json::Value>,
    /// Cloud provider resource group.
    pub cloud_provider_resource_group: Option<String>,
    /// Cloud provider organization.
    pub cloud_provider_organization: Option<String>,
    /// Cloud provider organization unit.
    pub cloud_provider_organization_unit: Option<String>,
    /// Cloud provider organization unit path.
    pub cloud_provider_organization_unit_path: Option<String>,
    /// Cloud provider account name.
    pub cloud_provider_account_name: Option<String>,
    /// Cloud provider account id.
    pub cloud_provider_account_id: Option<String>,
    /// Cloud provider subscription id.
    pub cloud_provider_subscription_id: Option<String>,
    /// Cloud provider project id.
    pub cloud_provider_project_id: Option<String>,
    /// Cloud provider url string.
    pub cloud_provider_url_string: Option<String>,
    /// Cloud resource id.
    pub cloud_resource_id: Option<String>,
    /// Cloud resource uid.
    pub cloud_resource_uid: Option<String>,
    /// Region.
    pub region: Option<String>,
    /// Is ad connector.
    pub is_ad_connector: Option<bool>,
    /// Is dc server.
    pub is_dc_server: Option<bool>,
    /// Ads enabled.
    pub ads_enabled: Option<bool>,
    /// Agent.
    pub agent: Option<InventoryAgentResponse>,
    /// Network name.
    pub network_name: Option<String>,
    /// Network names.
    pub network_names: Option<Vec<String>>,
    /// Serial number.
    pub serial_number: Option<String>,
    /// Detected from site.
    pub detected_from_site: Option<String>,
    /// Epp unsupported unknown.
    pub epp_unsupported_unknown: Option<String>,
    /// Alerts.
    pub alerts: Option<Vec<InventoryAlertResponse>>,
    /// Alerts count.
    pub alerts_count: Option<Vec<InventoryAlertResponse>>,
    /// Notes.
    pub notes: Option<Vec<InventoryNotesResponse>>,
    /// Identity.
    pub identity: Option<InventoryIdentityResponse>,
    /// Last active dt (date/time string).
    pub last_active_dt: Option<String>,
    /// Last reboot dt (date/time string).
    pub last_reboot_dt: Option<String>,
    /// First seen dt (date/time string).
    pub first_seen_dt: Option<String>,
    /// Last update dt (date/time string).
    pub last_update_dt: Option<String>,
    /// Created time (date/time string).
    pub created_time: Option<String>,
    /// Manufacturer.
    pub manufacturer: Option<String>,
    /// Tcp ports.
    pub tcp_ports: Option<Vec<String>>,
    /// Udp ports.
    pub udp_ports: Option<Vec<String>>,
    /// Discovery methods.
    pub discovery_methods: Option<Vec<String>>,
    /// Id secondary.
    pub id_secondary: Option<Vec<String>>,
}

/// A single device-review log entry (`DeviceReviewLog` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryDeviceReviewLog {
    /// Updated time (date/time string).
    pub updated_time_dt: Option<String>,
    /// Updated time (epoch).
    pub updated_time: Option<i64>,
    /// Username.
    pub username: Option<String>,
    /// Previous value.
    pub previous: Option<String>,
    /// Current value.
    pub current: Option<String>,
    /// Reason.
    pub reason: Option<String>,
}

/// An alert associated with an inventory asset (`AlertResponse` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryAlertResponse {
    /// Id.
    pub id: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Classification.
    pub classification: Option<String>,
    /// Severity.
    pub severity: Option<String>,
    /// Status.
    pub status: Option<String>,
    /// Activity.
    pub activity: Option<String>,
    /// Detected at (date/time string).
    pub detected_at: Option<String>,
    /// Time (epoch).
    pub time: Option<i64>,
    /// Count.
    pub count: Option<i64>,
    /// Attacks.
    pub attacks: Option<Vec<InventoryAttackResponse>>,
}

/// An attack referenced by an alert (`AttackResponse` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryAttackResponse {
    /// Uid.
    pub uid: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Version.
    pub version: Option<String>,
}

/// A note attached to an inventory asset (`NotesResponse` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryNotesResponse {
    /// Id.
    pub id: Option<String>,
    /// Resource id.
    pub resource_id: Option<String>,
    /// User id.
    pub user_id: Option<String>,
    /// User name.
    pub user_name: Option<String>,
    /// Note.
    pub note: Option<String>,
    /// Created at (date/time string).
    pub created_at: Option<String>,
    /// Updated at (date/time string).
    pub updated_at: Option<String>,
}

/// Identity information for an inventory asset (`IdentityResponse` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryIdentityResponse {
    /// Ad machine membership.
    pub ad_machine_membership: Option<Vec<String>>,
    /// Ad machine distinguished name.
    pub ad_machine_distinguished_name: Option<String>,
    /// Ad user distinguished name.
    pub ad_user_distinguished_name: Option<String>,
    /// Ad user membership.
    pub ad_user_membership: Option<Vec<String>>,
}

/// Agent information for an inventory asset (`AgentResponse` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryAgentResponse {
    /// Id.
    pub id: Option<String>,
    /// Uuid.
    pub uuid: Option<String>,
    /// Installer type.
    pub installer_type: Option<String>,
    /// K8s namespace.
    pub k8s_namespace: Option<String>,
    /// K8s pod.
    pub k8s_pod: Option<String>,
    /// Vss volumes.
    pub vss_volumes: Option<Vec<InventoryVssVolume>>,
    /// Has local config.
    pub has_local_config: Option<bool>,
    /// Disk encryption.
    pub disk_encryption: Option<bool>,
    /// Uninstalled.
    pub uninstalled: Option<bool>,
    /// Decommissioned.
    pub decommissioned: Option<bool>,
    /// Pending uninstall.
    pub pending_uninstall: Option<bool>,
    /// Ranger version.
    pub ranger_version: Option<String>,
    /// Customer identifier.
    pub customer_identifier: Option<String>,
    /// Missing permissions.
    pub missing_permissions: Option<Vec<String>>,
    /// Configurable network quarantine.
    pub configurable_network_quarantine: Option<bool>,
    /// Location.
    pub location: Option<Vec<String>>,
    /// Operational state.
    pub operational_state: Option<String>,
    /// Operational state expiration time dt (date/time string).
    pub operational_state_expiration_time_dt: Option<String>,
    /// Anti tampering status.
    pub anti_tampering_status: Option<String>,
    /// Disk metrics.
    pub disk_metrics: Option<Vec<InventoryDiskMetrics>>,
    /// Agent version.
    pub agent_version: Option<String>,
    /// Full disk scan dt (date/time string).
    pub full_disk_scan_dt: Option<String>,
    /// Pending actions.
    pub pending_actions: Option<Vec<String>>,
    /// Vss rollback status.
    pub vss_rollback_status: Option<String>,
    /// Vss protection status.
    pub vss_protection_status: Option<String>,
    /// Vss service status.
    pub vss_service_status: Option<String>,
    /// Vss last snapshot dt (date/time string).
    pub vss_last_snapshot_dt: Option<String>,
    /// Console migration status.
    pub console_migration_status: Option<String>,
    /// Pending upgrade.
    pub pending_upgrade: Option<bool>,
    /// Up to date.
    pub up_to_date: Option<bool>,
    /// Dv connectivity.
    pub dv_connectivity: Option<String>,
    /// Dv connectivity last updated dt (date/time string).
    pub dv_connectivity_last_updated_dt: Option<String>,
    /// Health status.
    pub health_status: Option<String>,
    /// Console connectivity.
    pub console_connectivity: Option<bool>,
    /// Last logged in user.
    pub last_logged_in_user: Option<String>,
    /// Firewall status.
    pub firewall_status: Option<bool>,
    /// Idr connectivity.
    pub idr_connectivity: Option<bool>,
    /// Location awareness.
    pub location_awareness: Option<bool>,
    /// Ranger status.
    pub ranger_status: Option<String>,
    /// Network status.
    pub network_status: Option<String>,
    /// Detection state.
    pub detection_state: Option<String>,
    /// Subscribe on dt (date/time string).
    pub subscribe_on_dt: Option<String>,
}

/// A VSS volume entry for an agent (`VssVolume` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryVssVolume {
    /// Diff area current used bytes.
    pub diff_area_current_used_bytes: Option<i64>,
    /// Diff area name.
    pub diff_area_name: Option<String>,
    /// Diff area free percentage.
    pub diff_area_free_percentage: Option<f64>,
    /// Diff area current allocated bytes.
    pub diff_area_current_allocated_bytes: Option<i64>,
    /// Diff area max limit bytes.
    pub diff_area_max_limit_bytes: Option<i64>,
}

/// Disk metrics for an agent volume (`DiskMetrics` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryDiskMetrics {
    /// Volume type.
    pub volume_type: Option<String>,
    /// Total number of free bytes.
    pub total_number_of_free_bytes: Option<i64>,
    /// Free bytes available to caller.
    pub free_bytes_available_to_caller: Option<i64>,
    /// Total number of bytes.
    pub total_number_of_bytes: Option<i64>,
    /// Free percentage.
    pub free_percentage: Option<f64>,
    /// Path.
    pub path: Option<String>,
}

/// Inventory category asset counts (`CategoriesResponse` in the spec).
///
/// Returned by `GET /web/api/v2.1/xdr/assets/categories`. Every field is the
/// count of assets in that category.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoriesResponse {
    /// Account.
    pub account: Option<i64>,
    /// Server.
    pub server: Option<i64>,
    /// Identity.
    pub identity: Option<i64>,
    /// Device.
    pub device: Option<i64>,
    /// Container.
    pub container: Option<i64>,
    /// Workstation.
    pub workstation: Option<i64>,
    /// Inventory.
    pub inventory: Option<i64>,
    /// Storage.
    pub storage: Option<i64>,
}

/// Asset counts broken down per subcategory for each category
/// (`SubCategoriesResponse` in the spec).
///
/// Returned by `GET /web/api/v2.1/xdr/assets/sub-categories` and embedded in
/// [`AssetCountsResponse`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubCategoriesResponse {
    /// Governance.
    pub governance: Option<CategoryDetails>,
    /// Inventory.
    pub inventory: Option<CategoryDetails>,
    /// Developer tool.
    pub developer_tool: Option<CategoryDetails>,
    /// Application integration.
    pub application_integration: Option<CategoryDetails>,
    /// Server.
    pub server: Option<CategoryDetails>,
    /// Identity.
    pub identity: Option<CategoryDetails>,
    /// Device.
    pub device: Option<CategoryDetails>,
    /// Ai ml.
    pub ai_ml: Option<CategoryDetails>,
    /// Code.
    pub code: Option<CategoryDetails>,
    /// Workstation.
    pub workstation: Option<CategoryDetails>,
    /// Secrets.
    pub secrets: Option<CategoryDetails>,
    /// Account.
    pub account: Option<CategoryDetails>,
    /// Cloud application.
    pub cloud_application: Option<CategoryDetails>,
    /// Storage.
    pub storage: Option<CategoryDetails>,
    /// Function.
    pub function: Option<CategoryDetails>,
    /// Data analysis.
    pub data_analysis: Option<CategoryDetails>,
    /// Data store.
    pub data_store: Option<CategoryDetails>,
    /// Network.
    pub network: Option<CategoryDetails>,
    /// Container.
    pub container: Option<CategoryDetails>,
}

/// Per-category count detail with its subcategory breakdown
/// (`CategoryDetails` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDetails {
    /// Subcategories.
    pub subcategories: Option<Vec<SubCategoryCount>>,
    /// Count.
    pub count: Option<i64>,
}

/// A single subcategory count (`SubCategoryCount` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubCategoryCount {
    /// Count.
    pub count: Option<i64>,
    /// Subcategory. Allowed values include: `All`, `Access Key and Secret`,
    /// `Access Management`, `Account`, `Account Group`, `AD Objects`,
    /// `Administrative Unit`, `Admission Controller`, `AI Service`,
    /// `AI Infrastructure`, `Analytics`, `API Gateway`, `Audio Visual`,
    /// `Audit Log`, `Backup`, `Block`, `Block Storage`, `Bucket`, `Cache`,
    /// `Certificate`, `CI CD`, `Cost Management and Optimization`, `Cluster`,
    /// `Code Repository`, `Configuration Policy`, `Container`, `Container Host`,
    /// `Container Management`, `Content Delivery Network`, `Database`,
    /// `Data Pipeline`, `Desktop`, `Developer Tool`, `Domain Name Service`,
    /// `ECS Workload`, `Embedded`, `Energy`, `Fargate`, `File`, `File Storage`,
    /// `Firewall`, `Function`, `Gaming`, `Gateway`, `Infrastructure as Code`,
    /// `IAM Policy`, `IP Phone`, `Image`, `Key-Value Store`,
    /// `Kubernetes Network`, `Kubernetes Secret`, `Kubernetes Storage`,
    /// `Kubernetes Workload`, `Laptop`, `Load Balancer`, `Machine Learning`,
    /// `Medical Device`, `Mobile`, `Monitoring and Logging`, `Namespace`,
    /// `Network Access Control`, `Network Device`, `Network Interface`,
    /// `Network Security Group`, `Network`, `Non-Relational Database - NoSQL`,
    /// `Notification Service`, `Object`, `Object Storage`, `Other Device`,
    /// `Other Server`, `Other Workstation`, `Payment System`, `Peering`,
    /// `Physical Server`, `Printer`, `Queuing Service`,
    /// `Relational Database - SQL`, `Repository`, `Resource Management`, `Role`,
    /// `Roles & Permissions`, `SaaS`, `Secret`, `Security`,
    /// `Security Management`, `Serverless Function`, `Server Infrastructure`,
    /// `Service Account`, `Smart Office`, `Smart Watch`, `Storage`, `UMPC`,
    /// `Users and Groups`, `Video`, `Virtual Disk`, `Virtual Machine`,
    /// `Virtual Network`.
    pub subcategory: Option<String>,
}

/// Inventory counts for categories, subcategories and surfaces
/// (`AssetCountsResponse` in the spec).
///
/// Returned by `GET /web/api/v2.1/xdr/assets/asset-counts`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetCountsResponse {
    /// Categories.
    pub categories: Option<SubCategoriesResponse>,
    /// Surfaces.
    pub surfaces: Option<AssetSurfaceResponse>,
}

/// Asset counts per surface (`AssetSurfaceResponse` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetSurfaceResponse {
    /// Cloud.
    pub cloud: Option<SurfaceDetails>,
    /// Endpoint.
    pub endpoint: Option<SurfaceDetails>,
    /// Identity.
    pub identity: Option<SurfaceDetails>,
    /// Network discovery.
    pub network_discovery: Option<SurfaceDetails>,
    /// Network.
    pub network: Option<SurfaceDetails>,
}

/// Count detail for a single surface (`SurfaceDetails` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceDetails {
    /// Count.
    pub count: Option<i64>,
}

/// Response wrapping the available actions and their status
/// (`AvailableActionWithStatusResponse` in the spec).
///
/// Returned by
/// `POST /web/api/v2.1/xdr/assets/available-actions/with-status`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatusResponse {
    /// Available actions.
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action with its enabled/disabled status
/// (`AvailableAction1` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// Action name. Allowed values: `Inventory`, `enable_protection`,
    /// `disable_protection`, `start_full_scan`, `stop_full_scan`,
    /// `enable_cws_monitoring`, `enable_cns`, `disable_cns`, `start_vm_scan`,
    /// `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`,
    /// `add_note`, `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`,
    /// `clear_tags`.
    pub name: Option<String>,
    /// Whether the action is disabled.
    pub is_disabled: Option<bool>,
    /// Reason the action is disabled, if applicable.
    pub disabled_reason: Option<String>,
}
