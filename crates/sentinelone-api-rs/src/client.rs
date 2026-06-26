use sentinelone_http::{Auth, HttpClient};

use crate::error::Error;
use crate::services::{accounts::AccountsService, agent_actions::AgentActionsService, agents::AgentsService};

/// Low-level Management API client. Cheap to clone.
#[derive(Clone)]
pub struct ManagementClient {
    http: HttpClient,
}

impl ManagementClient {
    /// `host` e.g. `https://apse1-2111-mssp.sentinelone.net`.
    pub fn new(host: &str, api_token: impl Into<String>) -> Result<Self, Error> {
        let http = HttpClient::new(host, Auth::ApiToken(api_token.into()))?;
        Ok(Self { http })
    }

    pub(crate) fn http(&self) -> &HttpClient {
        &self.http
    }

    // --- raw escape hatch (untyped JSON for any endpoint) ---

    /// Raw `GET` returning untyped JSON. `query` is a pre-serialized querystring
    /// (e.g. via `serde_urlencoded`). Lets callers reach any endpoint — including
    /// fields not yet modeled — without a typed wrapper. The full `{data, …}`
    /// envelope is returned as-is.
    pub async fn get_json(
        &self,
        path: &str,
        query: Option<&str>,
    ) -> Result<serde_json::Value, Error> {
        Ok(self.http.get(path, query).await?)
    }

    /// Raw `POST` of a JSON body returning untyped JSON.
    pub async fn post_json(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, Error> {
        Ok(self.http.post(path, body).await?)
    }

    // --- services (one accessor per Management API tag) ---

    pub fn accounts(&self) -> AccountsService<'_> {
        AccountsService { client: self }
    }

    pub fn agents(&self) -> AgentsService<'_> {
        AgentsService { client: self }
    }

    pub fn agent_actions(&self) -> AgentActionsService<'_> {
        AgentActionsService { client: self }
    }

    /// `activities` tag.
    pub fn activities(&self) -> crate::services::activities::ActivitiesService<'_> {
        crate::services::activities::ActivitiesService { client: self }
    }

    /// `agent_support_actions` tag.
    pub fn agent_support_actions(&self) -> crate::services::agent_support_actions::AgentSupportActionsService<'_> {
        crate::services::agent_support_actions::AgentSupportActionsService { client: self }
    }

    /// `agents_repository_beta` tag.
    pub fn agents_repository_beta(&self) -> crate::services::agents_repository_beta::AgentsRepositoryBetaService<'_> {
        crate::services::agents_repository_beta::AgentsRepositoryBetaService { client: self }
    }

    /// `alerts` tag.
    pub fn alerts(&self) -> crate::services::alerts::AlertsService<'_> {
        crate::services::alerts::AlertsService { client: self }
    }

    /// `application_control_rules` tag.
    pub fn application_control_rules(&self) -> crate::services::application_control_rules::ApplicationControlRulesService<'_> {
        crate::services::application_control_rules::ApplicationControlRulesService { client: self }
    }

    /// `application_control_settings_and_labels` tag.
    pub fn application_control_settings_and_labels(&self) -> crate::services::application_control_settings_and_labels::ApplicationControlSettingsAndLabelsService<'_> {
        crate::services::application_control_settings_and_labels::ApplicationControlSettingsAndLabelsService { client: self }
    }

    /// `application_management` tag.
    pub fn application_management(&self) -> crate::services::application_management::ApplicationManagementService<'_> {
        crate::services::application_management::ApplicationManagementService { client: self }
    }

    /// `application_management_settings` tag.
    pub fn application_management_settings(&self) -> crate::services::application_management_settings::ApplicationManagementSettingsService<'_> {
        crate::services::application_management_settings::ApplicationManagementSettingsService { client: self }
    }

    /// `application_risk` tag.
    pub fn application_risk(&self) -> crate::services::application_risk::ApplicationRiskService<'_> {
        crate::services::application_risk::ApplicationRiskService { client: self }
    }

    /// `application_risk_deprecated` tag.
    pub fn application_risk_deprecated(&self) -> crate::services::application_risk_deprecated::ApplicationRiskDeprecatedService<'_> {
        crate::services::application_risk_deprecated::ApplicationRiskDeprecatedService { client: self }
    }

    /// `auto_upgrade_policy` tag.
    pub fn auto_upgrade_policy(&self) -> crate::services::auto_upgrade_policy::AutoUpgradePolicyService<'_> {
        crate::services::auto_upgrade_policy::AutoUpgradePolicyService { client: self }
    }

    /// `cloud_funnel` tag.
    pub fn cloud_funnel(&self) -> crate::services::cloud_funnel::CloudFunnelService<'_> {
        crate::services::cloud_funnel::CloudFunnelService { client: self }
    }

    /// `cloud_resources` tag.
    pub fn cloud_resources(&self) -> crate::services::cloud_resources::CloudResourcesService<'_> {
        crate::services::cloud_resources::CloudResourcesService { client: self }
    }

    /// `config_overrides` tag.
    pub fn config_overrides(&self) -> crate::services::config_overrides::ConfigOverridesService<'_> {
        crate::services::config_overrides::ConfigOverridesService { client: self }
    }

    /// `custom_detection_rule` tag.
    pub fn custom_detection_rule(&self) -> crate::services::custom_detection_rule::CustomDetectionRuleService<'_> {
        crate::services::custom_detection_rule::CustomDetectionRuleService { client: self }
    }

    /// `datalake_unified_actions` tag.
    pub fn datalake_unified_actions(&self) -> crate::services::datalake_unified_actions::DatalakeUnifiedActionsService<'_> {
        crate::services::datalake_unified_actions::DatalakeUnifiedActionsService { client: self }
    }

    /// `deep_visibility` tag.
    pub fn deep_visibility(&self) -> crate::services::deep_visibility::DeepVisibilityService<'_> {
        crate::services::deep_visibility::DeepVisibilityService { client: self }
    }

    /// `default_reports` tag.
    pub fn default_reports(&self) -> crate::services::default_reports::DefaultReportsService<'_> {
        crate::services::default_reports::DefaultReportsService { client: self }
    }

    /// `device_control` tag.
    pub fn device_control(&self) -> crate::services::device_control::DeviceControlService<'_> {
        crate::services::device_control::DeviceControlService { client: self }
    }

    /// `dynamic_tag_rules` tag.
    pub fn dynamic_tag_rules(&self) -> crate::services::dynamic_tag_rules::DynamicTagRulesService<'_> {
        crate::services::dynamic_tag_rules::DynamicTagRulesService { client: self }
    }

    /// `exclusions_and_blocklist` tag.
    pub fn exclusions_and_blocklist(&self) -> crate::services::exclusions_and_blocklist::ExclusionsAndBlocklistService<'_> {
        crate::services::exclusions_and_blocklist::ExclusionsAndBlocklistService { client: self }
    }

    /// `exclusions_v2_1` tag.
    pub fn exclusions_v2_1(&self) -> crate::services::exclusions_v2_1::ExclusionsV21Service<'_> {
        crate::services::exclusions_v2_1::ExclusionsV21Service { client: self }
    }

    /// `filters` tag.
    pub fn filters(&self) -> crate::services::filters::FiltersService<'_> {
        crate::services::filters::FiltersService { client: self }
    }

    /// `firewall_control` tag.
    pub fn firewall_control(&self) -> crate::services::firewall_control::FirewallControlService<'_> {
        crate::services::firewall_control::FirewallControlService { client: self }
    }

    /// `forensics` tag.
    pub fn forensics(&self) -> crate::services::forensics::ForensicsService<'_> {
        crate::services::forensics::ForensicsService { client: self }
    }

    /// `gateways` tag.
    pub fn gateways(&self) -> crate::services::gateways::GatewaysService<'_> {
        crate::services::gateways::GatewaysService { client: self }
    }

    /// `graph` tag.
    pub fn graph(&self) -> crate::services::graph::GraphService<'_> {
        crate::services::graph::GraphService { client: self }
    }

    /// `graph_query_builder` tag.
    pub fn graph_query_builder(&self) -> crate::services::graph_query_builder::GraphQueryBuilderService<'_> {
        crate::services::graph_query_builder::GraphQueryBuilderService { client: self }
    }

    /// `graph_query_management` tag.
    pub fn graph_query_management(&self) -> crate::services::graph_query_management::GraphQueryManagementService<'_> {
        crate::services::graph_query_management::GraphQueryManagementService { client: self }
    }

    /// `groups` tag.
    pub fn groups(&self) -> crate::services::groups::GroupsService<'_> {
        crate::services::groups::GroupsService { client: self }
    }

    /// `hashes` tag.
    pub fn hashes(&self) -> crate::services::hashes::HashesService<'_> {
        crate::services::hashes::HashesService { client: self }
    }

    /// `hyperautomation` tag.
    pub fn hyperautomation(&self) -> crate::services::hyperautomation::HyperautomationService<'_> {
        crate::services::hyperautomation::HyperautomationService { client: self }
    }

    /// `identity_ad_service_actions` tag.
    pub fn identity_ad_service_actions(&self) -> crate::services::identity_ad_service_actions::IdentityAdServiceActionsService<'_> {
        crate::services::identity_ad_service_actions::IdentityAdServiceActionsService { client: self }
    }

    /// `identity_ad_service_configuration` tag.
    pub fn identity_ad_service_configuration(&self) -> crate::services::identity_ad_service_configuration::IdentityAdServiceConfigurationService<'_> {
        crate::services::identity_ad_service_configuration::IdentityAdServiceConfigurationService { client: self }
    }

    /// `identity_ad_service_connector` tag.
    pub fn identity_ad_service_connector(&self) -> crate::services::identity_ad_service_connector::IdentityAdServiceConnectorService<'_> {
        crate::services::identity_ad_service_connector::IdentityAdServiceConnectorService { client: self }
    }

    /// `identity_ad_service_onboarding` tag.
    pub fn identity_ad_service_onboarding(&self) -> crate::services::identity_ad_service_onboarding::IdentityAdServiceOnboardingService<'_> {
        crate::services::identity_ad_service_onboarding::IdentityAdServiceOnboardingService { client: self }
    }

    /// `inventory` tag.
    pub fn inventory(&self) -> crate::services::inventory::InventoryService<'_> {
        crate::services::inventory::InventoryService { client: self }
    }

    /// `inventory_account` tag.
    pub fn inventory_account(&self) -> crate::services::inventory_account::InventoryAccountService<'_> {
        crate::services::inventory_account::InventoryAccountService { client: self }
    }

    /// `inventory_account_filters` tag.
    pub fn inventory_account_filters(&self) -> crate::services::inventory_account_filters::InventoryAccountFiltersService<'_> {
        crate::services::inventory_account_filters::InventoryAccountFiltersService { client: self }
    }

    /// `inventory_ai_ml` tag.
    pub fn inventory_ai_ml(&self) -> crate::services::inventory_ai_ml::InventoryAiMlService<'_> {
        crate::services::inventory_ai_ml::InventoryAiMlService { client: self }
    }

    /// `inventory_ai_ml_filters` tag.
    pub fn inventory_ai_ml_filters(&self) -> crate::services::inventory_ai_ml_filters::InventoryAiMlFiltersService<'_> {
        crate::services::inventory_ai_ml_filters::InventoryAiMlFiltersService { client: self }
    }

    /// `inventory_application_integration` tag.
    pub fn inventory_application_integration(&self) -> crate::services::inventory_application_integration::InventoryApplicationIntegrationService<'_> {
        crate::services::inventory_application_integration::InventoryApplicationIntegrationService { client: self }
    }

    /// `inventory_application_integration_filters` tag.
    pub fn inventory_application_integration_filters(&self) -> crate::services::inventory_application_integration_filters::InventoryApplicationIntegrationFiltersService<'_> {
        crate::services::inventory_application_integration_filters::InventoryApplicationIntegrationFiltersService { client: self }
    }

    /// `inventory_cloud_application` tag.
    pub fn inventory_cloud_application(&self) -> crate::services::inventory_cloud_application::InventoryCloudApplicationService<'_> {
        crate::services::inventory_cloud_application::InventoryCloudApplicationService { client: self }
    }

    /// `inventory_cloud_application_filters` tag.
    pub fn inventory_cloud_application_filters(&self) -> crate::services::inventory_cloud_application_filters::InventoryCloudApplicationFiltersService<'_> {
        crate::services::inventory_cloud_application_filters::InventoryCloudApplicationFiltersService { client: self }
    }

    /// `inventory_cloud_surface` tag.
    pub fn inventory_cloud_surface(&self) -> crate::services::inventory_cloud_surface::InventoryCloudSurfaceService<'_> {
        crate::services::inventory_cloud_surface::InventoryCloudSurfaceService { client: self }
    }

    /// `inventory_cloud_surface_filters` tag.
    pub fn inventory_cloud_surface_filters(&self) -> crate::services::inventory_cloud_surface_filters::InventoryCloudSurfaceFiltersService<'_> {
        crate::services::inventory_cloud_surface_filters::InventoryCloudSurfaceFiltersService { client: self }
    }

    /// `inventory_container` tag.
    pub fn inventory_container(&self) -> crate::services::inventory_container::InventoryContainerService<'_> {
        crate::services::inventory_container::InventoryContainerService { client: self }
    }

    /// `inventory_container_filters` tag.
    pub fn inventory_container_filters(&self) -> crate::services::inventory_container_filters::InventoryContainerFiltersService<'_> {
        crate::services::inventory_container_filters::InventoryContainerFiltersService { client: self }
    }

    /// `inventory_data_analysis` tag.
    pub fn inventory_data_analysis(&self) -> crate::services::inventory_data_analysis::InventoryDataAnalysisService<'_> {
        crate::services::inventory_data_analysis::InventoryDataAnalysisService { client: self }
    }

    /// `inventory_data_analysis_filters` tag.
    pub fn inventory_data_analysis_filters(&self) -> crate::services::inventory_data_analysis_filters::InventoryDataAnalysisFiltersService<'_> {
        crate::services::inventory_data_analysis_filters::InventoryDataAnalysisFiltersService { client: self }
    }

    /// `inventory_data_store` tag.
    pub fn inventory_data_store(&self) -> crate::services::inventory_data_store::InventoryDataStoreService<'_> {
        crate::services::inventory_data_store::InventoryDataStoreService { client: self }
    }

    /// `inventory_data_store_filters` tag.
    pub fn inventory_data_store_filters(&self) -> crate::services::inventory_data_store_filters::InventoryDataStoreFiltersService<'_> {
        crate::services::inventory_data_store_filters::InventoryDataStoreFiltersService { client: self }
    }

    /// `inventory_developer_tool` tag.
    pub fn inventory_developer_tool(&self) -> crate::services::inventory_developer_tool::InventoryDeveloperToolService<'_> {
        crate::services::inventory_developer_tool::InventoryDeveloperToolService { client: self }
    }

    /// `inventory_developer_tool_filters` tag.
    pub fn inventory_developer_tool_filters(&self) -> crate::services::inventory_developer_tool_filters::InventoryDeveloperToolFiltersService<'_> {
        crate::services::inventory_developer_tool_filters::InventoryDeveloperToolFiltersService { client: self }
    }

    /// `inventory_device` tag.
    pub fn inventory_device(&self) -> crate::services::inventory_device::InventoryDeviceService<'_> {
        crate::services::inventory_device::InventoryDeviceService { client: self }
    }

    /// `inventory_device_filters` tag.
    pub fn inventory_device_filters(&self) -> crate::services::inventory_device_filters::InventoryDeviceFiltersService<'_> {
        crate::services::inventory_device_filters::InventoryDeviceFiltersService { client: self }
    }

    /// `inventory_endpoint_surface` tag.
    pub fn inventory_endpoint_surface(&self) -> crate::services::inventory_endpoint_surface::InventoryEndpointSurfaceService<'_> {
        crate::services::inventory_endpoint_surface::InventoryEndpointSurfaceService { client: self }
    }

    /// `inventory_endpoint_surface_filters` tag.
    pub fn inventory_endpoint_surface_filters(&self) -> crate::services::inventory_endpoint_surface_filters::InventoryEndpointSurfaceFiltersService<'_> {
        crate::services::inventory_endpoint_surface_filters::InventoryEndpointSurfaceFiltersService { client: self }
    }

    /// `inventory_filters` tag.
    pub fn inventory_filters(&self) -> crate::services::inventory_filters::InventoryFiltersService<'_> {
        crate::services::inventory_filters::InventoryFiltersService { client: self }
    }

    /// `inventory_function` tag.
    pub fn inventory_function(&self) -> crate::services::inventory_function::InventoryFunctionService<'_> {
        crate::services::inventory_function::InventoryFunctionService { client: self }
    }

    /// `inventory_function_filters` tag.
    pub fn inventory_function_filters(&self) -> crate::services::inventory_function_filters::InventoryFunctionFiltersService<'_> {
        crate::services::inventory_function_filters::InventoryFunctionFiltersService { client: self }
    }

    /// `inventory_governance` tag.
    pub fn inventory_governance(&self) -> crate::services::inventory_governance::InventoryGovernanceService<'_> {
        crate::services::inventory_governance::InventoryGovernanceService { client: self }
    }

    /// `inventory_governance_filters` tag.
    pub fn inventory_governance_filters(&self) -> crate::services::inventory_governance_filters::InventoryGovernanceFiltersService<'_> {
        crate::services::inventory_governance_filters::InventoryGovernanceFiltersService { client: self }
    }

    /// `inventory_identity` tag.
    pub fn inventory_identity(&self) -> crate::services::inventory_identity::InventoryIdentityService<'_> {
        crate::services::inventory_identity::InventoryIdentityService { client: self }
    }

    /// `inventory_identity_filters` tag.
    pub fn inventory_identity_filters(&self) -> crate::services::inventory_identity_filters::InventoryIdentityFiltersService<'_> {
        crate::services::inventory_identity_filters::InventoryIdentityFiltersService { client: self }
    }

    /// `inventory_identity_surface` tag.
    pub fn inventory_identity_surface(&self) -> crate::services::inventory_identity_surface::InventoryIdentitySurfaceService<'_> {
        crate::services::inventory_identity_surface::InventoryIdentitySurfaceService { client: self }
    }

    /// `inventory_identity_surface_filters` tag.
    pub fn inventory_identity_surface_filters(&self) -> crate::services::inventory_identity_surface_filters::InventoryIdentitySurfaceFiltersService<'_> {
        crate::services::inventory_identity_surface_filters::InventoryIdentitySurfaceFiltersService { client: self }
    }

    /// `inventory_network` tag.
    pub fn inventory_network(&self) -> crate::services::inventory_network::InventoryNetworkService<'_> {
        crate::services::inventory_network::InventoryNetworkService { client: self }
    }

    /// `inventory_network_discovery_surface` tag.
    pub fn inventory_network_discovery_surface(&self) -> crate::services::inventory_network_discovery_surface::InventoryNetworkDiscoverySurfaceService<'_> {
        crate::services::inventory_network_discovery_surface::InventoryNetworkDiscoverySurfaceService { client: self }
    }

    /// `inventory_network_discovery_surface_filters` tag.
    pub fn inventory_network_discovery_surface_filters(&self) -> crate::services::inventory_network_discovery_surface_filters::InventoryNetworkDiscoverySurfaceFiltersService<'_> {
        crate::services::inventory_network_discovery_surface_filters::InventoryNetworkDiscoverySurfaceFiltersService { client: self }
    }

    /// `inventory_network_filters` tag.
    pub fn inventory_network_filters(&self) -> crate::services::inventory_network_filters::InventoryNetworkFiltersService<'_> {
        crate::services::inventory_network_filters::InventoryNetworkFiltersService { client: self }
    }

    /// `inventory_notes` tag.
    pub fn inventory_notes(&self) -> crate::services::inventory_notes::InventoryNotesService<'_> {
        crate::services::inventory_notes::InventoryNotesService { client: self }
    }

    /// `inventory_server` tag.
    pub fn inventory_server(&self) -> crate::services::inventory_server::InventoryServerService<'_> {
        crate::services::inventory_server::InventoryServerService { client: self }
    }

    /// `inventory_server_filters` tag.
    pub fn inventory_server_filters(&self) -> crate::services::inventory_server_filters::InventoryServerFiltersService<'_> {
        crate::services::inventory_server_filters::InventoryServerFiltersService { client: self }
    }

    /// `inventory_storage` tag.
    pub fn inventory_storage(&self) -> crate::services::inventory_storage::InventoryStorageService<'_> {
        crate::services::inventory_storage::InventoryStorageService { client: self }
    }

    /// `inventory_storage_filters` tag.
    pub fn inventory_storage_filters(&self) -> crate::services::inventory_storage_filters::InventoryStorageFiltersService<'_> {
        crate::services::inventory_storage_filters::InventoryStorageFiltersService { client: self }
    }

    /// `inventory_tags` tag.
    pub fn inventory_tags(&self) -> crate::services::inventory_tags::InventoryTagsService<'_> {
        crate::services::inventory_tags::InventoryTagsService { client: self }
    }

    /// `inventory_unified_actions` tag.
    pub fn inventory_unified_actions(&self) -> crate::services::inventory_unified_actions::InventoryUnifiedActionsService<'_> {
        crate::services::inventory_unified_actions::InventoryUnifiedActionsService { client: self }
    }

    /// `inventory_workstation` tag.
    pub fn inventory_workstation(&self) -> crate::services::inventory_workstation::InventoryWorkstationService<'_> {
        crate::services::inventory_workstation::InventoryWorkstationService { client: self }
    }

    /// `ispm` tag.
    pub fn ispm(&self) -> crate::services::ispm::IspmService<'_> {
        crate::services::ispm::IspmService { client: self }
    }

    /// `licenses` tag.
    pub fn licenses(&self) -> crate::services::licenses::LicensesService<'_> {
        crate::services::licenses::LicensesService { client: self }
    }

    /// `live_updates` tag.
    pub fn live_updates(&self) -> crate::services::live_updates::LiveUpdatesService<'_> {
        crate::services::live_updates::LiveUpdatesService { client: self }
    }

    /// `locations` tag.
    pub fn locations(&self) -> crate::services::locations::LocationsService<'_> {
        crate::services::locations::LocationsService { client: self }
    }

    /// `log_collection` tag.
    pub fn log_collection(&self) -> crate::services::log_collection::LogCollectionService<'_> {
        crate::services::log_collection::LogCollectionService { client: self }
    }

    /// `long_running_query` tag.
    pub fn long_running_query(&self) -> crate::services::long_running_query::LongRunningQueryService<'_> {
        crate::services::long_running_query::LongRunningQueryService { client: self }
    }

    /// `marketplace` tag.
    pub fn marketplace(&self) -> crate::services::marketplace::MarketplaceService<'_> {
        crate::services::marketplace::MarketplaceService { client: self }
    }

    /// `mobile_integration` tag.
    pub fn mobile_integration(&self) -> crate::services::mobile_integration::MobileIntegrationService<'_> {
        crate::services::mobile_integration::MobileIntegrationService { client: self }
    }

    /// `network_discovery` tag.
    pub fn network_discovery(&self) -> crate::services::network_discovery::NetworkDiscoveryService<'_> {
        crate::services::network_discovery::NetworkDiscoveryService { client: self }
    }

    /// `network_discovery_self_enablement` tag.
    pub fn network_discovery_self_enablement(&self) -> crate::services::network_discovery_self_enablement::NetworkDiscoverySelfEnablementService<'_> {
        crate::services::network_discovery_self_enablement::NetworkDiscoverySelfEnablementService { client: self }
    }

    /// `network_quarantine_control` tag.
    pub fn network_quarantine_control(&self) -> crate::services::network_quarantine_control::NetworkQuarantineControlService<'_> {
        crate::services::network_quarantine_control::NetworkQuarantineControlService { client: self }
    }

    /// `overview` tag.
    pub fn overview(&self) -> crate::services::overview::OverviewService<'_> {
        crate::services::overview::OverviewService { client: self }
    }

    /// `platform_detection_rules` tag.
    pub fn platform_detection_rules(&self) -> crate::services::platform_detection_rules::PlatformDetectionRulesService<'_> {
        crate::services::platform_detection_rules::PlatformDetectionRulesService { client: self }
    }

    /// `policies` tag.
    pub fn policies(&self) -> crate::services::policies::PoliciesService<'_> {
        crate::services::policies::PoliciesService { client: self }
    }

    /// `rbac` tag.
    pub fn rbac(&self) -> crate::services::rbac::RbacService<'_> {
        crate::services::rbac::RbacService { client: self }
    }

    /// `remote_ops_mms` tag.
    pub fn remote_ops_mms(&self) -> crate::services::remote_ops_mms::RemoteOpsMmsService<'_> {
        crate::services::remote_ops_mms::RemoteOpsMmsService { client: self }
    }

    /// `remoteops_forensics` tag.
    pub fn remoteops_forensics(&self) -> crate::services::remoteops_forensics::RemoteopsForensicsService<'_> {
        crate::services::remoteops_forensics::RemoteopsForensicsService { client: self }
    }

    /// `remoteops_scripts` tag.
    pub fn remoteops_scripts(&self) -> crate::services::remoteops_scripts::RemoteopsScriptsService<'_> {
        crate::services::remoteops_scripts::RemoteopsScriptsService { client: self }
    }

    /// `saved_searches` tag.
    pub fn saved_searches(&self) -> crate::services::saved_searches::SavedSearchesService<'_> {
        crate::services::saved_searches::SavedSearchesService { client: self }
    }

    /// `sentinel_deploy` tag.
    pub fn sentinel_deploy(&self) -> crate::services::sentinel_deploy::SentinelDeployService<'_> {
        crate::services::sentinel_deploy::SentinelDeployService { client: self }
    }

    /// `service_users` tag.
    pub fn service_users(&self) -> crate::services::service_users::ServiceUsersService<'_> {
        crate::services::service_users::ServiceUsersService { client: self }
    }

    /// `settings` tag.
    pub fn settings(&self) -> crate::services::settings::SettingsService<'_> {
        crate::services::settings::SettingsService { client: self }
    }

    /// `sites` tag.
    pub fn sites(&self) -> crate::services::sites::SitesService<'_> {
        crate::services::sites::SitesService { client: self }
    }

    /// `system` tag.
    pub fn system(&self) -> crate::services::system::SystemService<'_> {
        crate::services::system::SystemService { client: self }
    }

    /// `tag_manager` tag.
    pub fn tag_manager(&self) -> crate::services::tag_manager::TagManagerService<'_> {
        crate::services::tag_manager::TagManagerService { client: self }
    }

    /// `tags` tag.
    pub fn tags(&self) -> crate::services::tags::TagsService<'_> {
        crate::services::tags::TagsService { client: self }
    }

    /// `tasks` tag.
    pub fn tasks(&self) -> crate::services::tasks::TasksService<'_> {
        crate::services::tasks::TasksService { client: self }
    }

    /// `threat_intelligence` tag.
    pub fn threat_intelligence(&self) -> crate::services::threat_intelligence::ThreatIntelligenceService<'_> {
        crate::services::threat_intelligence::ThreatIntelligenceService { client: self }
    }

    /// `threat_notes` tag.
    pub fn threat_notes(&self) -> crate::services::threat_notes::ThreatNotesService<'_> {
        crate::services::threat_notes::ThreatNotesService { client: self }
    }

    /// `threats` tag.
    pub fn threats(&self) -> crate::services::threats::ThreatsService<'_> {
        crate::services::threats::ThreatsService { client: self }
    }

    /// `unprotected_endpoints_discovery` tag.
    pub fn unprotected_endpoints_discovery(&self) -> crate::services::unprotected_endpoints_discovery::UnprotectedEndpointsDiscoveryService<'_> {
        crate::services::unprotected_endpoints_discovery::UnprotectedEndpointsDiscoveryService { client: self }
    }

    /// `updates` tag.
    pub fn updates(&self) -> crate::services::updates::UpdatesService<'_> {
        crate::services::updates::UpdatesService { client: self }
    }

    /// `users` tag.
    pub fn users(&self) -> crate::services::users::UsersService<'_> {
        crate::services::users::UsersService { client: self }
    }

    /// `vcs_integration` tag.
    pub fn vcs_integration(&self) -> crate::services::vcs_integration::VcsIntegrationService<'_> {
        crate::services::vcs_integration::VcsIntegrationService { client: self }
    }
}
