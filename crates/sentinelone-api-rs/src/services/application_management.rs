//! `Application Management` tag — application inventory and risk/CVE views.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::application_management::{
    AgentApplication, AggregatedApplicationRisk, ApplicationInventory, ApplicationInventoryEndpoint,
    ApplicationRisk, BaseRisksCve, RiskyCve, RiskyEndpoint, ScanResult,
};
use crate::pagination::{Paginated, Response};

/// Join an iterator of string-like values into a comma-separated string, as the
/// SentinelOne API expects for array query parameters.
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

/// `Application Management` tag — application inventory views and vulnerability
/// (CVE) risk operations.
pub struct ApplicationManagementService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ===========================================================================
// GET /web/api/v2.1/application-management/inventory
// ===========================================================================

/// Query params for `GET /web/api/v2.1/application-management/inventory`.
///
/// All fields are optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryQuery {
    /// Single Site ID to filter by. Example: `"225494730938493804"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Single Account ID to filter by. Example: `"225494730938493804"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The column to sort the results by. Allowed values: `applicationName`,
    /// `applicationVendor`, `applicationVersionsCount`, `endpointsCount`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Free-text filter by endpoint uuid (supports multiple values). Optional.
    #[serde(rename = "endpointUuid__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_uuid_contains: Option<String>,
    /// Included OS versions. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_versions: Option<String>,
    /// Included vendors. Example: `"Microsoft,Apple"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendors: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Free-text filter by os version (supports multiple values). Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// Free-text filter by endpoint name (supports multiple values). Optional.
    #[serde(rename = "endpointName__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_name_contains: Option<String>,
    /// Free-text filter by application name (supports multiple values). Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by vendor (supports multiple values). Optional.
    #[serde(rename = "vendor__contains", skip_serializing_if = "Option::is_none")]
    pub vendor_contains: Option<String>,
    /// Included OS architectures. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_architectures: Option<String>,
    /// Single Group ID to filter by. Example: `"225494730938493804"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Included OS types. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
}

impl InventoryQuery {
    /// Single Site ID to filter by (array, comma-joined).
    pub fn site_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Single Account ID to filter by (array, comma-joined).
    pub fn account_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Free-text filter by endpoint uuid (array, comma-joined).
    pub fn endpoint_uuid_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_uuid_contains = Some(join_csv(v));
        self
    }
    /// Included OS versions (array, comma-joined).
    pub fn os_versions<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_versions = Some(join_csv(v));
        self
    }
    /// Included vendors (array, comma-joined).
    pub fn vendors<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.vendors = Some(join_csv(v));
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Free-text filter by os version (array, comma-joined).
    pub fn os_version_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_version_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by endpoint name (array, comma-joined).
    pub fn endpoint_name_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by application name (array, comma-joined).
    pub fn name_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by vendor (array, comma-joined).
    pub fn vendor_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.vendor_contains = Some(join_csv(v));
        self
    }
    /// Included OS architectures (array, comma-joined).
    pub fn os_architectures<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_architectures = Some(join_csv(v));
        self
    }
    /// Single Group ID to filter by (array, comma-joined).
    pub fn group_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Included OS types (array, comma-joined).
    pub fn os_types<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_types = Some(join_csv(v));
        self
    }
}

// ===========================================================================
// GET /web/api/v2.1/application-management/inventory/applications
// ===========================================================================

/// Query params for
/// `GET /web/api/v2.1/application-management/inventory/applications`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointAppsQuery {
    /// Agent ID list. Example: `"225494730938493804,225494730938493915"`.
    /// Required (array, comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
}

impl EndpointAppsQuery {
    /// Agent ID list (array, comma-joined). Required.
    pub fn ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.ids = Some(join_csv(v));
        self
    }
}

// ===========================================================================
// GET /web/api/v2.1/application-management/inventory/endpoints
// ===========================================================================

/// Query params for
/// `GET /web/api/v2.1/application-management/inventory/endpoints`.
///
/// `applicationName` and `applicationVendor` are required by the API.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryEndpointsQuery {
    /// Single Site ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Application detection date after or at this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__gte", skip_serializing_if = "Option::is_none")]
    pub detection_date_gte: Option<String>,
    /// Single Account ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Skip first number of items (0-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Application name. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// The column to sort the results by. Allowed values: `endpointName`,
    /// `endpointType`, `osType`, `osVersion`, `accountName`, `siteName`,
    /// `groupName`, `version`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Free-text filter by endpoint uuid (array, comma-joined). Optional.
    #[serde(rename = "endpointUuid__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_uuid_contains: Option<String>,
    /// Included OS versions (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_versions: Option<String>,
    /// Application detection date before this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__lt", skip_serializing_if = "Option::is_none")]
    pub detection_date_lt: Option<String>,
    /// Application vendor. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_vendor: Option<String>,
    /// Application detection date after this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__gt", skip_serializing_if = "Option::is_none")]
    pub detection_date_gt: Option<String>,
    /// If true, total number of items will not be calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Date range for application detection date (format:
    /// `<from_timestamp>-<to_timestamp>`, inclusive). Optional.
    #[serde(rename = "detectionDate__between", skip_serializing_if = "Option::is_none")]
    pub detection_date_between: Option<String>,
    /// Application detection date before or at this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__lte", skip_serializing_if = "Option::is_none")]
    pub detection_date_lte: Option<String>,
    /// If true, only total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Included application versions (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<String>,
    /// Free-text filter by endpoint name (array, comma-joined). Optional.
    #[serde(rename = "endpointName__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_name_contains: Option<String>,
    /// Included OS architectures (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_architectures: Option<String>,
    /// Single Group ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Included OS types (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
}

impl InventoryEndpointsQuery {
    /// Create with the required `applicationName` and `applicationVendor`.
    pub fn new(application_name: impl Into<String>, application_vendor: impl Into<String>) -> Self {
        Self {
            application_name: Some(application_name.into()),
            application_vendor: Some(application_vendor.into()),
            ..Default::default()
        }
    }
    /// Single Site ID to filter by (array, comma-joined).
    pub fn site_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Application detection date after or at this timestamp (date-time).
    pub fn detection_date_gte(mut self, v: impl Into<String>) -> Self {
        self.detection_date_gte = Some(v.into());
        self
    }
    /// Single Account ID to filter by (array, comma-joined).
    pub fn account_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Free-text filter by endpoint uuid (array, comma-joined).
    pub fn endpoint_uuid_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_uuid_contains = Some(join_csv(v));
        self
    }
    /// Included OS versions (array, comma-joined).
    pub fn os_versions<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_versions = Some(join_csv(v));
        self
    }
    /// Application detection date before this timestamp (date-time).
    pub fn detection_date_lt(mut self, v: impl Into<String>) -> Self {
        self.detection_date_lt = Some(v.into());
        self
    }
    /// Application detection date after this timestamp (date-time).
    pub fn detection_date_gt(mut self, v: impl Into<String>) -> Self {
        self.detection_date_gt = Some(v.into());
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Date range for application detection date.
    pub fn detection_date_between(mut self, v: impl Into<String>) -> Self {
        self.detection_date_between = Some(v.into());
        self
    }
    /// Application detection date before or at this timestamp (date-time).
    pub fn detection_date_lte(mut self, v: impl Into<String>) -> Self {
        self.detection_date_lte = Some(v.into());
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Included application versions (array, comma-joined).
    pub fn versions<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.versions = Some(join_csv(v));
        self
    }
    /// Free-text filter by endpoint name (array, comma-joined).
    pub fn endpoint_name_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_name_contains = Some(join_csv(v));
        self
    }
    /// Included OS architectures (array, comma-joined).
    pub fn os_architectures<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_architectures = Some(join_csv(v));
        self
    }
    /// Single Group ID to filter by (array, comma-joined).
    pub fn group_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Included OS types (array, comma-joined).
    pub fn os_types<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_types = Some(join_csv(v));
        self
    }
}

// ===========================================================================
// GET /web/api/v2.1/application-management/inventory/endpoints/export/csv
// ===========================================================================

/// Query params for
/// `GET /web/api/v2.1/application-management/inventory/endpoints/export/csv`.
///
/// `applicationName` and `applicationVendor` are required by the API.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryEndpointsExportQuery {
    /// Single Site ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Application detection date after or at this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__gte", skip_serializing_if = "Option::is_none")]
    pub detection_date_gte: Option<String>,
    /// Single Account ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// CSV delimiter. Allowed values: `,`, `;`. Defaults to `,`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_delimiter: Option<String>,
    /// Application name. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// Free-text filter by endpoint uuid (array, comma-joined). Optional.
    #[serde(rename = "endpointUuid__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_uuid_contains: Option<String>,
    /// Included OS versions (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_versions: Option<String>,
    /// Application detection date before this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__lt", skip_serializing_if = "Option::is_none")]
    pub detection_date_lt: Option<String>,
    /// Application vendor. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_vendor: Option<String>,
    /// Application detection date after this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__gt", skip_serializing_if = "Option::is_none")]
    pub detection_date_gt: Option<String>,
    /// Date range for application detection date. Optional.
    #[serde(rename = "detectionDate__between", skip_serializing_if = "Option::is_none")]
    pub detection_date_between: Option<String>,
    /// Application detection date before or at this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__lte", skip_serializing_if = "Option::is_none")]
    pub detection_date_lte: Option<String>,
    /// Included application versions (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<String>,
    /// Free-text filter by endpoint name (array, comma-joined). Optional.
    #[serde(rename = "endpointName__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_name_contains: Option<String>,
    /// Included OS architectures (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_architectures: Option<String>,
    /// Single Group ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Included OS types (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
}

impl InventoryEndpointsExportQuery {
    /// Create with the required `applicationName` and `applicationVendor`.
    pub fn new(application_name: impl Into<String>, application_vendor: impl Into<String>) -> Self {
        Self {
            application_name: Some(application_name.into()),
            application_vendor: Some(application_vendor.into()),
            ..Default::default()
        }
    }
    /// Single Site ID to filter by (array, comma-joined).
    pub fn site_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// Application detection date after or at this timestamp (date-time).
    pub fn detection_date_gte(mut self, v: impl Into<String>) -> Self {
        self.detection_date_gte = Some(v.into());
        self
    }
    /// Single Account ID to filter by (array, comma-joined).
    pub fn account_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// CSV delimiter. Allowed values: `,`, `;`.
    pub fn csv_delimiter(mut self, v: impl Into<String>) -> Self {
        self.csv_delimiter = Some(v.into());
        self
    }
    /// Free-text filter by endpoint uuid (array, comma-joined).
    pub fn endpoint_uuid_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_uuid_contains = Some(join_csv(v));
        self
    }
    /// Included OS versions (array, comma-joined).
    pub fn os_versions<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_versions = Some(join_csv(v));
        self
    }
    /// Application detection date before this timestamp (date-time).
    pub fn detection_date_lt(mut self, v: impl Into<String>) -> Self {
        self.detection_date_lt = Some(v.into());
        self
    }
    /// Application detection date after this timestamp (date-time).
    pub fn detection_date_gt(mut self, v: impl Into<String>) -> Self {
        self.detection_date_gt = Some(v.into());
        self
    }
    /// Date range for application detection date.
    pub fn detection_date_between(mut self, v: impl Into<String>) -> Self {
        self.detection_date_between = Some(v.into());
        self
    }
    /// Application detection date before or at this timestamp (date-time).
    pub fn detection_date_lte(mut self, v: impl Into<String>) -> Self {
        self.detection_date_lte = Some(v.into());
        self
    }
    /// Included application versions (array, comma-joined).
    pub fn versions<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.versions = Some(join_csv(v));
        self
    }
    /// Free-text filter by endpoint name (array, comma-joined).
    pub fn endpoint_name_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_name_contains = Some(join_csv(v));
        self
    }
    /// Included OS architectures (array, comma-joined).
    pub fn os_architectures<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_architectures = Some(join_csv(v));
        self
    }
    /// Single Group ID to filter by (array, comma-joined).
    pub fn group_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Included OS types (array, comma-joined).
    pub fn os_types<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_types = Some(join_csv(v));
        self
    }
}

// ===========================================================================
// GET /web/api/v2.1/application-management/inventory/export/csv
// ===========================================================================

/// Query params for
/// `GET /web/api/v2.1/application-management/inventory/export/csv`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryExportQuery {
    /// Single Site ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Single Account ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// CSV delimiter. Allowed values: `,`, `;`. Defaults to `,`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_delimiter: Option<String>,
    /// Free-text filter by endpoint uuid (array, comma-joined). Optional.
    #[serde(rename = "endpointUuid__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_uuid_contains: Option<String>,
    /// Included OS versions (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_versions: Option<String>,
    /// Included vendors (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendors: Option<String>,
    /// Free-text filter by os version (array, comma-joined). Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// Free-text filter by endpoint name (array, comma-joined). Optional.
    #[serde(rename = "endpointName__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_name_contains: Option<String>,
    /// Free-text filter by application name (array, comma-joined). Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by vendor (array, comma-joined). Optional.
    #[serde(rename = "vendor__contains", skip_serializing_if = "Option::is_none")]
    pub vendor_contains: Option<String>,
    /// Included OS architectures (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_architectures: Option<String>,
    /// Single Group ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Included OS types (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
}

impl InventoryExportQuery {
    /// Single Site ID to filter by (array, comma-joined).
    pub fn site_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// Single Account ID to filter by (array, comma-joined).
    pub fn account_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// CSV delimiter. Allowed values: `,`, `;`.
    pub fn csv_delimiter(mut self, v: impl Into<String>) -> Self {
        self.csv_delimiter = Some(v.into());
        self
    }
    /// Free-text filter by endpoint uuid (array, comma-joined).
    pub fn endpoint_uuid_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_uuid_contains = Some(join_csv(v));
        self
    }
    /// Included OS versions (array, comma-joined).
    pub fn os_versions<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_versions = Some(join_csv(v));
        self
    }
    /// Included vendors (array, comma-joined).
    pub fn vendors<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.vendors = Some(join_csv(v));
        self
    }
    /// Free-text filter by os version (array, comma-joined).
    pub fn os_version_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_version_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by endpoint name (array, comma-joined).
    pub fn endpoint_name_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by application name (array, comma-joined).
    pub fn name_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by vendor (array, comma-joined).
    pub fn vendor_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.vendor_contains = Some(join_csv(v));
        self
    }
    /// Included OS architectures (array, comma-joined).
    pub fn os_architectures<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_architectures = Some(join_csv(v));
        self
    }
    /// Single Group ID to filter by (array, comma-joined).
    pub fn group_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Included OS types (array, comma-joined).
    pub fn os_types<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_types = Some(join_csv(v));
        self
    }
}

// ===========================================================================
// GET /web/api/v2.1/application-management/risks
//  &  GET .../risks/export/csv  (export omits sortOrder/sortBy/limit/skip/cursor
//                                and adds csvDelimiter)
// ===========================================================================

/// Query params for `GET /web/api/v2.1/application-management/risks` and
/// `GET /web/api/v2.1/application-management/risks/export/csv`.
///
/// The export variant additionally honours `csv_delimiter` and ignores
/// pagination/sort fields, which the API simply omits from the CSV.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RisksQuery {
    /// Single Site ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Significant CVE updates after this timestamp (date-time). Optional.
    #[serde(rename = "riskUpdatedDate__gt", skip_serializing_if = "Option::is_none")]
    pub risk_updated_date_gt: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// CVE detection date after or at this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__gte", skip_serializing_if = "Option::is_none")]
    pub detection_date_gte: Option<String>,
    /// Single Account ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Included application names. Example: `"Office 1.1,Test"` (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_names: Option<String>,
    /// Free-text filter by domain (array, comma-joined). Optional.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// Included exploit code maturity values. Available for Singularity
    /// Vulnerability Management SKU (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_code_maturity: Option<String>,
    /// Mitigation status values (array, comma-joined). Allowed values:
    /// `NOT_MITIGATED`, `TO_BE_PATCHED`, `ON_HOLD`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_status: Option<String>,
    /// Date range for CVE publish date. Optional.
    #[serde(rename = "publishedDate__between", skip_serializing_if = "Option::is_none")]
    pub published_date_between: Option<String>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Skip first number of items (0-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Risk score (inclusive). Available for Singularity Vulnerability Management
    /// SKU. Example: `"5-8.9"`. Optional.
    #[serde(rename = "riskScore__between", skip_serializing_if = "Option::is_none")]
    pub risk_score_between: Option<String>,
    /// Included remediation level values (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_levels: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Free-text filter by vendor (array, comma-joined). Optional.
    #[serde(rename = "applicationVendor__contains", skip_serializing_if = "Option::is_none")]
    pub application_vendor_contains: Option<String>,
    /// CSV delimiter (export endpoint only). Allowed values: `,`, `;`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_delimiter: Option<String>,
    /// CVE published date after or at this timestamp (date-time). Optional.
    #[serde(rename = "publishedDate__gte", skip_serializing_if = "Option::is_none")]
    pub published_date_gte: Option<String>,
    /// The column to sort the results by. Allowed values: `id`, `accountId`,
    /// `siteId`, `cveId`, `endpointName`, `application`, `applicationVendor`,
    /// `baseScore`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Included exploited in the wild values (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploited_in_the_wild: Option<String>,
    /// Significant CVE updates before this timestamp (date-time). Optional.
    #[serde(rename = "riskUpdatedDate__lt", skip_serializing_if = "Option::is_none")]
    pub risk_updated_date_lt: Option<String>,
    /// Included severities. Example: `"CRITICAL,HIGH"` (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<String>,
    /// Include also removed CVEs in the results. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_removals: Option<bool>,
    /// Significant CVE updates after or at this timestamp (date-time).
    /// Recommended for fetching delta-changes. Optional.
    #[serde(rename = "riskUpdatedDate__gte", skip_serializing_if = "Option::is_none")]
    pub risk_updated_date_gte: Option<String>,
    /// Included OS versions (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_versions: Option<String>,
    /// Analyst verdict (array, comma-joined). Allowed values: `Default`,
    /// `False Positive`, `Added CVE`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyst_verdict: Option<String>,
    /// Included network domains (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// CVE published date before this timestamp (date-time). Optional.
    #[serde(rename = "publishedDate__lt", skip_serializing_if = "Option::is_none")]
    pub published_date_lt: Option<String>,
    /// CVE published date after this timestamp (date-time). Optional.
    #[serde(rename = "publishedDate__gt", skip_serializing_if = "Option::is_none")]
    pub published_date_gt: Option<String>,
    /// CVE detection date before this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__lt", skip_serializing_if = "Option::is_none")]
    pub detection_date_lt: Option<String>,
    /// CVE detection date after this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__gt", skip_serializing_if = "Option::is_none")]
    pub detection_date_gt: Option<String>,
    /// If true, total number of items will not be calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// CVE detection date before or at this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__lte", skip_serializing_if = "Option::is_none")]
    pub detection_date_lte: Option<String>,
    /// Date range for CVE detection date. Optional.
    #[serde(rename = "detectionDate__between", skip_serializing_if = "Option::is_none")]
    pub detection_date_between: Option<String>,
    /// CVE published date before or at this timestamp (date-time). Optional.
    #[serde(rename = "publishedDate__lte", skip_serializing_if = "Option::is_none")]
    pub published_date_lte: Option<String>,
    /// Included report confidence values (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_confidence: Option<String>,
    /// Significant CVE updates within this date range. Optional.
    #[serde(rename = "riskUpdatedDate__between", skip_serializing_if = "Option::is_none")]
    pub risk_updated_date_between: Option<String>,
    /// If true, only total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Free-text filter by endpoint name (array, comma-joined). Optional.
    #[serde(rename = "endpointName__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_name_contains: Option<String>,
    /// Significant CVE updates before or at this timestamp (date-time). Optional.
    #[serde(rename = "riskUpdatedDate__lte", skip_serializing_if = "Option::is_none")]
    pub risk_updated_date_lte: Option<String>,
    /// Included vendors (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendors: Option<String>,
    /// Free-text filter by application name and version (array, comma-joined). Optional.
    #[serde(rename = "application__contains", skip_serializing_if = "Option::is_none")]
    pub application_contains: Option<String>,
    /// Free-text filter by CVE id (array, comma-joined). Optional.
    #[serde(rename = "cveId__contains", skip_serializing_if = "Option::is_none")]
    pub cve_id_contains: Option<String>,
    /// Included endpoint types (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_types: Option<String>,
    /// Single Group ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Included OS types (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Included last scan results (array, comma-joined). Allowed values:
    /// `Succeeded`, `Failed`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scan_results: Option<String>,
    /// Days from CVE detection, e.g. 12 days or more. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_from_cve_detection: Option<i64>,
}

impl RisksQuery {
    /// Single Site ID to filter by (array, comma-joined).
    pub fn site_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// Significant CVE updates after this timestamp (date-time).
    pub fn risk_updated_date_gt(mut self, v: impl Into<String>) -> Self {
        self.risk_updated_date_gt = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// CVE detection date after or at this timestamp (date-time).
    pub fn detection_date_gte(mut self, v: impl Into<String>) -> Self {
        self.detection_date_gte = Some(v.into());
        self
    }
    /// Single Account ID to filter by (array, comma-joined).
    pub fn account_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// Included application names (array, comma-joined).
    pub fn application_names<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.application_names = Some(join_csv(v));
        self
    }
    /// Free-text filter by domain (array, comma-joined).
    pub fn domain_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.domain_contains = Some(join_csv(v));
        self
    }
    /// Included exploit code maturity values (array, comma-joined).
    pub fn exploit_code_maturity<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.exploit_code_maturity = Some(join_csv(v));
        self
    }
    /// Mitigation status values (array, comma-joined).
    pub fn mitigation_status<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.mitigation_status = Some(join_csv(v));
        self
    }
    /// Date range for CVE publish date.
    pub fn published_date_between(mut self, v: impl Into<String>) -> Self {
        self.published_date_between = Some(v.into());
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Risk score (inclusive). Example: `"5-8.9"`.
    pub fn risk_score_between(mut self, v: impl Into<String>) -> Self {
        self.risk_score_between = Some(v.into());
        self
    }
    /// Included remediation level values (array, comma-joined).
    pub fn remediation_levels<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.remediation_levels = Some(join_csv(v));
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// Free-text filter by vendor (array, comma-joined).
    pub fn application_vendor_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.application_vendor_contains = Some(join_csv(v));
        self
    }
    /// CSV delimiter (export endpoint only). Allowed values: `,`, `;`.
    pub fn csv_delimiter(mut self, v: impl Into<String>) -> Self {
        self.csv_delimiter = Some(v.into());
        self
    }
    /// CVE published date after or at this timestamp (date-time).
    pub fn published_date_gte(mut self, v: impl Into<String>) -> Self {
        self.published_date_gte = Some(v.into());
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Included exploited in the wild values (array, comma-joined).
    pub fn exploited_in_the_wild<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.exploited_in_the_wild = Some(join_csv(v));
        self
    }
    /// Significant CVE updates before this timestamp (date-time).
    pub fn risk_updated_date_lt(mut self, v: impl Into<String>) -> Self {
        self.risk_updated_date_lt = Some(v.into());
        self
    }
    /// Included severities (array, comma-joined).
    pub fn severities<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.severities = Some(join_csv(v));
        self
    }
    /// Include also removed CVEs in the results.
    pub fn include_removals(mut self, v: bool) -> Self {
        self.include_removals = Some(v);
        self
    }
    /// Significant CVE updates after or at this timestamp (date-time).
    pub fn risk_updated_date_gte(mut self, v: impl Into<String>) -> Self {
        self.risk_updated_date_gte = Some(v.into());
        self
    }
    /// Included OS versions (array, comma-joined).
    pub fn os_versions<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_versions = Some(join_csv(v));
        self
    }
    /// Analyst verdict (array, comma-joined).
    pub fn analyst_verdict<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.analyst_verdict = Some(join_csv(v));
        self
    }
    /// Included network domains (array, comma-joined).
    pub fn domains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.domains = Some(join_csv(v));
        self
    }
    /// CVE published date before this timestamp (date-time).
    pub fn published_date_lt(mut self, v: impl Into<String>) -> Self {
        self.published_date_lt = Some(v.into());
        self
    }
    /// CVE published date after this timestamp (date-time).
    pub fn published_date_gt(mut self, v: impl Into<String>) -> Self {
        self.published_date_gt = Some(v.into());
        self
    }
    /// CVE detection date before this timestamp (date-time).
    pub fn detection_date_lt(mut self, v: impl Into<String>) -> Self {
        self.detection_date_lt = Some(v.into());
        self
    }
    /// CVE detection date after this timestamp (date-time).
    pub fn detection_date_gt(mut self, v: impl Into<String>) -> Self {
        self.detection_date_gt = Some(v.into());
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// CVE detection date before or at this timestamp (date-time).
    pub fn detection_date_lte(mut self, v: impl Into<String>) -> Self {
        self.detection_date_lte = Some(v.into());
        self
    }
    /// Date range for CVE detection date.
    pub fn detection_date_between(mut self, v: impl Into<String>) -> Self {
        self.detection_date_between = Some(v.into());
        self
    }
    /// CVE published date before or at this timestamp (date-time).
    pub fn published_date_lte(mut self, v: impl Into<String>) -> Self {
        self.published_date_lte = Some(v.into());
        self
    }
    /// Included report confidence values (array, comma-joined).
    pub fn report_confidence<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.report_confidence = Some(join_csv(v));
        self
    }
    /// Significant CVE updates within this date range.
    pub fn risk_updated_date_between(mut self, v: impl Into<String>) -> Self {
        self.risk_updated_date_between = Some(v.into());
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Free-text filter by endpoint name (array, comma-joined).
    pub fn endpoint_name_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_name_contains = Some(join_csv(v));
        self
    }
    /// Significant CVE updates before or at this timestamp (date-time).
    pub fn risk_updated_date_lte(mut self, v: impl Into<String>) -> Self {
        self.risk_updated_date_lte = Some(v.into());
        self
    }
    /// Included vendors (array, comma-joined).
    pub fn vendors<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.vendors = Some(join_csv(v));
        self
    }
    /// Free-text filter by application name and version (array, comma-joined).
    pub fn application_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.application_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by CVE id (array, comma-joined).
    pub fn cve_id_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.cve_id_contains = Some(join_csv(v));
        self
    }
    /// Included endpoint types (array, comma-joined).
    pub fn endpoint_types<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_types = Some(join_csv(v));
        self
    }
    /// Single Group ID to filter by (array, comma-joined).
    pub fn group_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Included OS types (array, comma-joined).
    pub fn os_types<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_types = Some(join_csv(v));
        self
    }
    /// Included last scan results (array, comma-joined). Allowed values:
    /// `Succeeded`, `Failed`.
    pub fn last_scan_results<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.last_scan_results = Some(join_csv(v));
        self
    }
    /// Days from CVE detection, e.g. 12 days or more.
    pub fn days_from_cve_detection(mut self, v: i64) -> Self {
        self.days_from_cve_detection = Some(v);
        self
    }
}

// ===========================================================================
// GET /web/api/v2.1/application-management/risks/aggregated-applications
//  &  .../risks/applications  (same field set; applications adds
//     application__contains and omits domains-as-array semantics — both share
//     this struct, with sortBy values documented per endpoint method)
//  &  their /export/csv variants (add csvDelimiter, drop sort/pagination).
// ===========================================================================

/// Query params for the application-risk listing/export endpoints:
/// `risks/aggregated-applications`, `risks/applications`, and their
/// `/export/csv` variants.
///
/// `csv_delimiter` applies only to the export variants; sort/pagination fields
/// are ignored by the export endpoints.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationsRiskQuery {
    /// Single Site ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Included highest severities. Example: `"CRITICAL,HIGH"` (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_severities: Option<String>,
    /// Application detection date after or at this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__gte", skip_serializing_if = "Option::is_none")]
    pub detection_date_gte: Option<String>,
    /// Application type. Available with Ranger Insights (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_types: Option<String>,
    /// Single Account ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Included exploit code maturity values. Available with Ranger Insights
    /// (array, comma-joined). Allowed values: `Unproven`, `High`, `Not Defined`,
    /// `Proof of Concept`, `Functional`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_code_maturity: Option<String>,
    /// Included most common status values. Available with Ranger Insights
    /// (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_common_statuses: Option<String>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Skip first number of items (0-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Included remediation level values. Available with Ranger Insights
    /// (array, comma-joined). Allowed values: `Temporary Fix`, `Not Defined`,
    /// `Official Fix`, `Workaround`, `Unavailable`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_levels: Option<String>,
    /// CSV delimiter (export endpoints only). Allowed values: `,`, `;`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_delimiter: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The column to sort the results by. For `aggregated-applications`: `name`,
    /// `vendor`, `highestRiskScore`, `highestNvdBaseScore`, `highestSeverity`,
    /// `exploitedInTheWild`, `exploitCodeMaturity`, `remediationLevel`. For
    /// `applications`: `name`, `vendor`, `highestNvdBaseScore`, `highestSeverity`,
    /// `cveCount`, `endpointCount`, `detectionDate`, `daysDetected`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Included exploited in the wild values. Available with Ranger Insights
    /// (array, comma-joined). Allowed values: `Not Defined`, `Unknown`, `Yes`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploited_in_the_wild: Option<String>,
    /// Free-text filter by endpoint uuid (array, comma-joined). Optional.
    #[serde(rename = "endpointUuid__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_uuid_contains: Option<String>,
    /// Included domains (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// Application detection date before this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__lt", skip_serializing_if = "Option::is_none")]
    pub detection_date_lt: Option<String>,
    /// Included vendors (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendors: Option<String>,
    /// Application detection date after this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__gt", skip_serializing_if = "Option::is_none")]
    pub detection_date_gt: Option<String>,
    /// If true, total number of items will not be calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Date range for application detection date. Optional.
    #[serde(rename = "detectionDate__between", skip_serializing_if = "Option::is_none")]
    pub detection_date_between: Option<String>,
    /// Application detection date before or at this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__lte", skip_serializing_if = "Option::is_none")]
    pub detection_date_lte: Option<String>,
    /// If true, only total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Free-text filter by endpoint name (array, comma-joined). Optional.
    #[serde(rename = "endpointName__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_name_contains: Option<String>,
    /// Free-text filter by application name and version (array, comma-joined).
    /// (`risks/applications` and its export only.) Optional.
    #[serde(rename = "application__contains", skip_serializing_if = "Option::is_none")]
    pub application_contains: Option<String>,
    /// Free-text filter by CVE id (array, comma-joined). Optional.
    #[serde(rename = "cveId__contains", skip_serializing_if = "Option::is_none")]
    pub cve_id_contains: Option<String>,
    /// Included endpoint types (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_types: Option<String>,
    /// Free-text filter by vendor (array, comma-joined). Optional.
    #[serde(rename = "vendor__contains", skip_serializing_if = "Option::is_none")]
    pub vendor_contains: Option<String>,
    /// Free-text filter by application name (array, comma-joined).
    /// (`aggregated-applications` and its export only.) Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Single Group ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Included OS types. Example: `"windows,linux"` (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Days from application detection, e.g. 12 days or more. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_from_detection: Option<i64>,
}

impl ApplicationsRiskQuery {
    /// Single Site ID to filter by (array, comma-joined).
    pub fn site_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Included highest severities (array, comma-joined).
    pub fn highest_severities<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.highest_severities = Some(join_csv(v));
        self
    }
    /// Application detection date after or at this timestamp (date-time).
    pub fn detection_date_gte(mut self, v: impl Into<String>) -> Self {
        self.detection_date_gte = Some(v.into());
        self
    }
    /// Application type (array, comma-joined).
    pub fn application_types<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.application_types = Some(join_csv(v));
        self
    }
    /// Single Account ID to filter by (array, comma-joined).
    pub fn account_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// Included exploit code maturity values (array, comma-joined).
    pub fn exploit_code_maturity<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.exploit_code_maturity = Some(join_csv(v));
        self
    }
    /// Included most common status values (array, comma-joined).
    pub fn most_common_statuses<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.most_common_statuses = Some(join_csv(v));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Included remediation level values (array, comma-joined).
    pub fn remediation_levels<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.remediation_levels = Some(join_csv(v));
        self
    }
    /// CSV delimiter (export endpoints only). Allowed values: `,`, `;`.
    pub fn csv_delimiter(mut self, v: impl Into<String>) -> Self {
        self.csv_delimiter = Some(v.into());
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Included exploited in the wild values (array, comma-joined).
    pub fn exploited_in_the_wild<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.exploited_in_the_wild = Some(join_csv(v));
        self
    }
    /// Free-text filter by endpoint uuid (array, comma-joined).
    pub fn endpoint_uuid_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_uuid_contains = Some(join_csv(v));
        self
    }
    /// Included domains (array, comma-joined).
    pub fn domains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.domains = Some(join_csv(v));
        self
    }
    /// Application detection date before this timestamp (date-time).
    pub fn detection_date_lt(mut self, v: impl Into<String>) -> Self {
        self.detection_date_lt = Some(v.into());
        self
    }
    /// Included vendors (array, comma-joined).
    pub fn vendors<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.vendors = Some(join_csv(v));
        self
    }
    /// Application detection date after this timestamp (date-time).
    pub fn detection_date_gt(mut self, v: impl Into<String>) -> Self {
        self.detection_date_gt = Some(v.into());
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Date range for application detection date.
    pub fn detection_date_between(mut self, v: impl Into<String>) -> Self {
        self.detection_date_between = Some(v.into());
        self
    }
    /// Application detection date before or at this timestamp (date-time).
    pub fn detection_date_lte(mut self, v: impl Into<String>) -> Self {
        self.detection_date_lte = Some(v.into());
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Free-text filter by endpoint name (array, comma-joined).
    pub fn endpoint_name_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by application name and version (array, comma-joined).
    pub fn application_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.application_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by CVE id (array, comma-joined).
    pub fn cve_id_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.cve_id_contains = Some(join_csv(v));
        self
    }
    /// Included endpoint types (array, comma-joined).
    pub fn endpoint_types<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_types = Some(join_csv(v));
        self
    }
    /// Free-text filter by vendor (array, comma-joined).
    pub fn vendor_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.vendor_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by application name (array, comma-joined).
    pub fn name_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// Single Group ID to filter by (array, comma-joined).
    pub fn group_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Included OS types (array, comma-joined).
    pub fn os_types<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_types = Some(join_csv(v));
        self
    }
    /// Days from application detection, e.g. 12 days or more.
    pub fn days_from_detection(mut self, v: i64) -> Self {
        self.days_from_detection = Some(v);
        self
    }
}

// ===========================================================================
// GET /web/api/v2.1/application-management/risks/cves
//  &  .../risks/cves/export/csv (adds csvDelimiter, drops sort/pagination)
// ===========================================================================

/// Query params for `GET /web/api/v2.1/application-management/risks/cves` and
/// its `/export/csv` variant.
///
/// Use `application_ids` to include a single application id (required without
/// Ranger Insights); Ranger Insights users may pass multiple application IDs or
/// use `application_name` + `application_vendor`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationCvesQuery {
    /// Single Site ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Included application versions by id (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_ids: Option<String>,
    /// Single Account ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Application vendor. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_vendor: Option<String>,
    /// Date range for CVE publish date. Optional.
    #[serde(rename = "publishedDate__between", skip_serializing_if = "Option::is_none")]
    pub published_date_between: Option<String>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Skip first number of items (0-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Application name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// Included remediation level values (array, comma-joined). Available with
    /// Ranger Insights. Allowed values: `Temporary Fix`, `Not Defined`,
    /// `Official Fix`, `Workaround`, `Unavailable`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_levels: Option<String>,
    /// CSV delimiter (export endpoint only). Allowed values: `,`, `;`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_delimiter: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// CVE published date after or at this timestamp (date-time). Optional.
    #[serde(rename = "publishedDate__gte", skip_serializing_if = "Option::is_none")]
    pub published_date_gte: Option<String>,
    /// The column to sort the results by. Allowed values: `cveId`, `severity`,
    /// `nvdBaseScore`, `publishedDate`, `riskScore`, `exploitedInTheWild`,
    /// `exploitCodeMaturity`, `remediationLevel`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Included exploited in the wild values (array, comma-joined). Available with
    /// Ranger Insights. Allowed values: `Not Defined`, `Unknown`, `Yes`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploited_in_the_wild: Option<String>,
    /// Included severities (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<String>,
    /// CVE published date before this timestamp (date-time). Optional.
    #[serde(rename = "publishedDate__lt", skip_serializing_if = "Option::is_none")]
    pub published_date_lt: Option<String>,
    /// CVE published date after this timestamp (date-time). Optional.
    #[serde(rename = "publishedDate__gt", skip_serializing_if = "Option::is_none")]
    pub published_date_gt: Option<String>,
    /// If true, total number of items will not be calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// CVE published date before or at this timestamp (date-time). Optional.
    #[serde(rename = "publishedDate__lte", skip_serializing_if = "Option::is_none")]
    pub published_date_lte: Option<String>,
    /// Included report confidence values (array, comma-joined). Available with
    /// Ranger Insights. Allowed values: `Not Defined`, `Reasonable`, `Unknown`,
    /// `Confirmed`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_confidence: Option<String>,
    /// If true, only total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Included application versions (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_versions: Option<String>,
    /// Free-text filter by CVE id (array, comma-joined). Optional.
    #[serde(rename = "cveId__contains", skip_serializing_if = "Option::is_none")]
    pub cve_id_contains: Option<String>,
    /// Single Group ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Analyst verdict (array, comma-joined). Allowed values: `Added CVE`,
    /// `False Positive`, `Default`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyst_verdict: Option<String>,
    /// Included exploit code maturity values (array, comma-joined). Available with
    /// Ranger Insights. Allowed values: `Unproven`, `High`, `Not Defined`,
    /// `Proof of Concept`, `Functional`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_code_maturity: Option<String>,
}

impl ApplicationCvesQuery {
    /// Single Site ID to filter by (array, comma-joined).
    pub fn site_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Included application versions by id (array, comma-joined).
    pub fn application_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.application_ids = Some(join_csv(v));
        self
    }
    /// Single Account ID to filter by (array, comma-joined).
    pub fn account_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// Application vendor.
    pub fn application_vendor(mut self, v: impl Into<String>) -> Self {
        self.application_vendor = Some(v.into());
        self
    }
    /// Date range for CVE publish date.
    pub fn published_date_between(mut self, v: impl Into<String>) -> Self {
        self.published_date_between = Some(v.into());
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Application name.
    pub fn application_name(mut self, v: impl Into<String>) -> Self {
        self.application_name = Some(v.into());
        self
    }
    /// Included remediation level values (array, comma-joined).
    pub fn remediation_levels<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.remediation_levels = Some(join_csv(v));
        self
    }
    /// CSV delimiter (export endpoint only). Allowed values: `,`, `;`.
    pub fn csv_delimiter(mut self, v: impl Into<String>) -> Self {
        self.csv_delimiter = Some(v.into());
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// CVE published date after or at this timestamp (date-time).
    pub fn published_date_gte(mut self, v: impl Into<String>) -> Self {
        self.published_date_gte = Some(v.into());
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Included exploited in the wild values (array, comma-joined).
    pub fn exploited_in_the_wild<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.exploited_in_the_wild = Some(join_csv(v));
        self
    }
    /// Included severities (array, comma-joined).
    pub fn severities<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.severities = Some(join_csv(v));
        self
    }
    /// CVE published date before this timestamp (date-time).
    pub fn published_date_lt(mut self, v: impl Into<String>) -> Self {
        self.published_date_lt = Some(v.into());
        self
    }
    /// CVE published date after this timestamp (date-time).
    pub fn published_date_gt(mut self, v: impl Into<String>) -> Self {
        self.published_date_gt = Some(v.into());
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// CVE published date before or at this timestamp (date-time).
    pub fn published_date_lte(mut self, v: impl Into<String>) -> Self {
        self.published_date_lte = Some(v.into());
        self
    }
    /// Included report confidence values (array, comma-joined).
    pub fn report_confidence<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.report_confidence = Some(join_csv(v));
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Included application versions (array, comma-joined).
    pub fn application_versions<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.application_versions = Some(join_csv(v));
        self
    }
    /// Free-text filter by CVE id (array, comma-joined).
    pub fn cve_id_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.cve_id_contains = Some(join_csv(v));
        self
    }
    /// Single Group ID to filter by (array, comma-joined).
    pub fn group_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Analyst verdict (array, comma-joined).
    pub fn analyst_verdict<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.analyst_verdict = Some(join_csv(v));
        self
    }
    /// Included exploit code maturity values (array, comma-joined).
    pub fn exploit_code_maturity<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.exploit_code_maturity = Some(join_csv(v));
        self
    }
}

// ===========================================================================
// GET /web/api/v2.1/application-management/risks/endpoints
//  &  .../risks/endpoints/export/csv (adds csvDelimiter, drops sort/pagination)
// ===========================================================================

/// Query params for `GET /web/api/v2.1/application-management/risks/endpoints`
/// and its `/export/csv` variant.
///
/// Use `application_ids` to include a single application id (required without
/// Ranger Insights); Ranger Insights users may pass multiple application IDs or
/// use `application_name` + `application_vendor`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskyEndpointsQuery {
    /// Single Site ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Free-text filter by status message (array, comma-joined). Available with
    /// Ranger Insights. Optional.
    #[serde(rename = "statusMessage__contains", skip_serializing_if = "Option::is_none")]
    pub status_message_contains: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Included application versions by id (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_ids: Option<String>,
    /// Application detection date after or at this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__gte", skip_serializing_if = "Option::is_none")]
    pub detection_date_gte: Option<String>,
    /// Single Account ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Application vendor. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_vendor: Option<String>,
    /// Free-text filter by domain (array, comma-joined). Optional.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// Date range for days left to mitigation. Available with Ranger Insights
    /// when using ticket integration. Example: `"1-30"`. Optional.
    #[serde(rename = "daysToMitigation__between", skip_serializing_if = "Option::is_none")]
    pub days_to_mitigation_between: Option<String>,
    /// Included statuses (array, comma-joined). Available with Ranger Insights. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Skip first number of items (0-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Application name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// CSV delimiter (export endpoint only). Allowed values: `,`, `;`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_delimiter: Option<String>,
    /// The column to sort the results by. Allowed values: `name`, `osType`,
    /// `osVersion`, `endpointType`, `account`, `site`, `group`, `domain`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Free-text filter by endpoint uuid (array, comma-joined). Optional.
    #[serde(rename = "endpointUuid__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_uuid_contains: Option<String>,
    /// Last scan date before this timestamp (date-time). Optional.
    #[serde(rename = "lastScanDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_scan_date_lt: Option<String>,
    /// Included OS versions (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_versions: Option<String>,
    /// Included endpoint domains (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// Last scan date after this timestamp (date-time). Optional.
    #[serde(rename = "lastScanDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_scan_date_gt: Option<String>,
    /// Application detection date before this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__lt", skip_serializing_if = "Option::is_none")]
    pub detection_date_lt: Option<String>,
    /// Free-text filter by endpoint id (array, comma-joined). Optional.
    #[serde(rename = "endpointId__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_id_contains: Option<String>,
    /// Application detection date after this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__gt", skip_serializing_if = "Option::is_none")]
    pub detection_date_gt: Option<String>,
    /// If true, total number of items will not be calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Application detection date before or at this timestamp (date-time). Optional.
    #[serde(rename = "detectionDate__lte", skip_serializing_if = "Option::is_none")]
    pub detection_date_lte: Option<String>,
    /// Date range for application detection date. Optional.
    #[serde(rename = "detectionDate__between", skip_serializing_if = "Option::is_none")]
    pub detection_date_between: Option<String>,
    /// If true, only total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Included application versions (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_versions: Option<String>,
    /// Free-text filter by ticket id (array, comma-joined). Available with Ranger
    /// Insights when using ticket integration. Optional.
    #[serde(rename = "ticketId__contains", skip_serializing_if = "Option::is_none")]
    pub ticket_id_contains: Option<String>,
    /// Free-text filter by endpoint name (array, comma-joined). Optional.
    #[serde(rename = "endpointName__contains", skip_serializing_if = "Option::is_none")]
    pub endpoint_name_contains: Option<String>,
    /// Last scan date before or at this timestamp (date-time). Optional.
    #[serde(rename = "lastScanDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_scan_date_lte: Option<String>,
    /// Date range for last scan date. Optional.
    #[serde(rename = "lastScanDate__between", skip_serializing_if = "Option::is_none")]
    pub last_scan_date_between: Option<String>,
    /// Included endpoint types (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_types: Option<String>,
    /// Single Group ID to filter by (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Included OS types (array, comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Included last scan results (array, comma-joined). Allowed values:
    /// `Succeeded`, `Failed`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scan_results: Option<String>,
    /// Days from application detection, e.g. 12 days or more. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_from_detection: Option<i64>,
    /// Last scan date after or at this timestamp (date-time). Optional.
    #[serde(rename = "lastScanDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_scan_date_gte: Option<String>,
}

impl RiskyEndpointsQuery {
    /// Single Site ID to filter by (array, comma-joined).
    pub fn site_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// Free-text filter by status message (array, comma-joined).
    pub fn status_message_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.status_message_contains = Some(join_csv(v));
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Included application versions by id (array, comma-joined).
    pub fn application_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.application_ids = Some(join_csv(v));
        self
    }
    /// Application detection date after or at this timestamp (date-time).
    pub fn detection_date_gte(mut self, v: impl Into<String>) -> Self {
        self.detection_date_gte = Some(v.into());
        self
    }
    /// Single Account ID to filter by (array, comma-joined).
    pub fn account_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// Application vendor.
    pub fn application_vendor(mut self, v: impl Into<String>) -> Self {
        self.application_vendor = Some(v.into());
        self
    }
    /// Free-text filter by domain (array, comma-joined).
    pub fn domain_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.domain_contains = Some(join_csv(v));
        self
    }
    /// Date range for days left to mitigation. Example: `"1-30"`.
    pub fn days_to_mitigation_between(mut self, v: impl Into<String>) -> Self {
        self.days_to_mitigation_between = Some(v.into());
        self
    }
    /// Included statuses (array, comma-joined).
    pub fn statuses<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.statuses = Some(join_csv(v));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Application name.
    pub fn application_name(mut self, v: impl Into<String>) -> Self {
        self.application_name = Some(v.into());
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// CSV delimiter (export endpoint only). Allowed values: `,`, `;`.
    pub fn csv_delimiter(mut self, v: impl Into<String>) -> Self {
        self.csv_delimiter = Some(v.into());
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Free-text filter by endpoint uuid (array, comma-joined).
    pub fn endpoint_uuid_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_uuid_contains = Some(join_csv(v));
        self
    }
    /// Last scan date before this timestamp (date-time).
    pub fn last_scan_date_lt(mut self, v: impl Into<String>) -> Self {
        self.last_scan_date_lt = Some(v.into());
        self
    }
    /// Included OS versions (array, comma-joined).
    pub fn os_versions<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_versions = Some(join_csv(v));
        self
    }
    /// Included endpoint domains (array, comma-joined).
    pub fn domains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.domains = Some(join_csv(v));
        self
    }
    /// Last scan date after this timestamp (date-time).
    pub fn last_scan_date_gt(mut self, v: impl Into<String>) -> Self {
        self.last_scan_date_gt = Some(v.into());
        self
    }
    /// Application detection date before this timestamp (date-time).
    pub fn detection_date_lt(mut self, v: impl Into<String>) -> Self {
        self.detection_date_lt = Some(v.into());
        self
    }
    /// Free-text filter by endpoint id (array, comma-joined).
    pub fn endpoint_id_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_id_contains = Some(join_csv(v));
        self
    }
    /// Application detection date after this timestamp (date-time).
    pub fn detection_date_gt(mut self, v: impl Into<String>) -> Self {
        self.detection_date_gt = Some(v.into());
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Application detection date before or at this timestamp (date-time).
    pub fn detection_date_lte(mut self, v: impl Into<String>) -> Self {
        self.detection_date_lte = Some(v.into());
        self
    }
    /// Date range for application detection date.
    pub fn detection_date_between(mut self, v: impl Into<String>) -> Self {
        self.detection_date_between = Some(v.into());
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Included application versions (array, comma-joined).
    pub fn application_versions<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.application_versions = Some(join_csv(v));
        self
    }
    /// Free-text filter by ticket id (array, comma-joined).
    pub fn ticket_id_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.ticket_id_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by endpoint name (array, comma-joined).
    pub fn endpoint_name_contains<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_name_contains = Some(join_csv(v));
        self
    }
    /// Last scan date before or at this timestamp (date-time).
    pub fn last_scan_date_lte(mut self, v: impl Into<String>) -> Self {
        self.last_scan_date_lte = Some(v.into());
        self
    }
    /// Date range for last scan date.
    pub fn last_scan_date_between(mut self, v: impl Into<String>) -> Self {
        self.last_scan_date_between = Some(v.into());
        self
    }
    /// Included endpoint types (array, comma-joined).
    pub fn endpoint_types<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.endpoint_types = Some(join_csv(v));
        self
    }
    /// Single Group ID to filter by (array, comma-joined).
    pub fn group_ids<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Included OS types (array, comma-joined).
    pub fn os_types<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.os_types = Some(join_csv(v));
        self
    }
    /// Included last scan results (array, comma-joined). Allowed values:
    /// `Succeeded`, `Failed`.
    pub fn last_scan_results<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, v: I) -> Self {
        self.last_scan_results = Some(join_csv(v));
        self
    }
    /// Days from application detection, e.g. 12 days or more.
    pub fn days_from_detection(mut self, v: i64) -> Self {
        self.days_from_detection = Some(v);
        self
    }
    /// Last scan date after or at this timestamp (date-time).
    pub fn last_scan_date_gte(mut self, v: impl Into<String>) -> Self {
        self.last_scan_date_gte = Some(v.into());
        self
    }
}

// ===========================================================================
// POST /web/api/v2.1/application-management/scan
// ===========================================================================

/// Filter for the application vulnerability scan body.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanFilter {
    /// If the entire tenant scope should be filtered. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Single Site ID to filter by (max 1 item). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// Single Account ID to filter by (max 1 item). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

impl ScanFilter {
    /// Filter the entire tenant scope.
    pub fn tenant(v: bool) -> Self {
        Self {
            tenant: Some(v),
            ..Default::default()
        }
    }
    /// Filter by a single Site ID.
    pub fn site_id(mut self, id: impl Into<String>) -> Self {
        self.site_ids = Some(vec![id.into()]);
        self
    }
    /// Filter by a single Account ID.
    pub fn account_id(mut self, id: impl Into<String>) -> Self {
        self.account_ids = Some(vec![id.into()]);
        self
    }
}

/// Request body for `POST /web/api/v2.1/application-management/scan`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanBody {
    /// Filter. Required.
    pub filter: ScanFilter,
}

impl ApplicationManagementService<'_> {
    /// `GET /web/api/v2.1/application-management/inventory` — Get Application
    /// Inventory.
    ///
    /// Get application inventory data grouped by application name and vendor.
    pub async fn get_inventory(
        &self,
        query: &InventoryQuery,
    ) -> Result<Paginated<ApplicationInventory>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/application-management/inventory", q)
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/inventory/applications` — Get
    /// Endpoint Apps.
    ///
    /// Get the installed applications for a specific endpoint. To get the Agent
    /// ID, run "agents".
    pub async fn get_endpoint_apps(
        &self,
        query: &EndpointAppsQuery,
    ) -> Result<Paginated<AgentApplication>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/application-management/inventory/applications", q)
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/inventory/endpoints` — Get App
    /// Inventory Endpoints.
    ///
    /// Get endpoint data for a specific application.
    pub async fn get_inventory_endpoints(
        &self,
        query: &InventoryEndpointsQuery,
    ) -> Result<Paginated<ApplicationInventoryEndpoint>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/application-management/inventory/endpoints", q)
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/inventory/endpoints/export/csv`
    /// — Inventory Endpoints Data Export.
    ///
    /// Export application inventory endpoints data to CSV. The raw response is
    /// returned as a JSON value (the spec declares no schema for this export).
    pub async fn export_inventory_endpoints_csv(
        &self,
        query: &InventoryEndpointsExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/application-management/inventory/endpoints/export/csv",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/inventory/export/csv` —
    /// Inventory Data Export.
    ///
    /// Export application inventory data to CSV. The raw response is returned as
    /// a JSON value (the spec declares no schema for this export).
    pub async fn export_inventory_csv(
        &self,
        query: &InventoryExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/application-management/inventory/export/csv", q)
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/risks` — Get CVE data.
    ///
    /// Get the CVE vulnerability data for each CVE.
    pub async fn get_risks(&self, query: &RisksQuery) -> Result<Paginated<BaseRisksCve>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/application-management/risks", q)
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/risks/aggregated-applications`
    /// — Get Aggregated Applications With Risk.
    ///
    /// Get data for all applications. Available with Ranger Insights license.
    pub async fn get_aggregated_applications(
        &self,
        query: &ApplicationsRiskQuery,
    ) -> Result<Paginated<AggregatedApplicationRisk>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/application-management/risks/aggregated-applications",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/risks/aggregated-applications/export/csv`
    /// — Aggregated Application Risk Data Export.
    ///
    /// Export aggregated application data to CSV. Available with Ranger Insights
    /// license. The raw response is returned as a JSON value (the spec declares
    /// no schema for this export).
    pub async fn export_aggregated_applications_csv(
        &self,
        query: &ApplicationsRiskQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/application-management/risks/aggregated-applications/export/csv",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/risks/applications` — Get
    /// Applications With Risk.
    ///
    /// Get data for each version of all applications.
    pub async fn get_applications(
        &self,
        query: &ApplicationsRiskQuery,
    ) -> Result<Paginated<ApplicationRisk>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/application-management/risks/applications", q)
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/risks/applications/export/csv`
    /// — Application Risk Data Export.
    ///
    /// Export application data to CSV. The raw response is returned as a JSON
    /// value (the spec declares no schema for this export).
    pub async fn export_applications_csv(
        &self,
        query: &ApplicationsRiskQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/application-management/risks/applications/export/csv",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/risks/cves` — Get Application
    /// CVEs.
    ///
    /// Get CVE data for a specific application. Use `applicationIds` to include a
    /// single application id (required). For Ranger Insights license users,
    /// either use `applicationIds` with multiple application IDs, or the
    /// `applicationName` and `applicationVendor` query parameters.
    pub async fn get_application_cves(
        &self,
        query: &ApplicationCvesQuery,
    ) -> Result<Paginated<RiskyCve>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/application-management/risks/cves", q)
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/risks/cves/export/csv` —
    /// Application CVE Data Export.
    ///
    /// Export CVE data to CSV. Use `applicationIds` to include a single
    /// application id (required). For Ranger Insights license users, either use
    /// `applicationIds` with multiple application IDs, or the `applicationName`
    /// and `applicationVendor` query parameters. The raw response is returned as
    /// a JSON value (the spec declares no schema for this export).
    pub async fn export_application_cves_csv(
        &self,
        query: &ApplicationCvesQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/application-management/risks/cves/export/csv", q)
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/risks/endpoints` — Get Endpoints
    /// For Vulnerable App.
    ///
    /// Get a list of all endpoints installed with a specific application that
    /// contains vulnerabilities. Use `applicationIds` to include a single
    /// application id (required). For Ranger Insights license users, either use
    /// `applicationIds` with multiple application IDs, or the `applicationName`
    /// and `applicationVendor` query parameters.
    pub async fn get_risk_endpoints(
        &self,
        query: &RiskyEndpointsQuery,
    ) -> Result<Paginated<RiskyEndpoint>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/application-management/risks/endpoints", q)
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/risks/endpoints/export/csv` —
    /// Risk Endpoint Data Export.
    ///
    /// Export endpoint data to CSV. Use `applicationIds` to include a single
    /// application id (required). For Ranger Insights license users, either use
    /// `applicationIds` with multiple application IDs, or the `applicationName`
    /// and `applicationVendor` query parameters. The raw response is returned as
    /// a JSON value (the spec declares no schema for this export).
    pub async fn export_risk_endpoints_csv(
        &self,
        query: &RiskyEndpointsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/application-management/risks/endpoints/export/csv",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/application-management/risks/export/csv` — Risks Data
    /// Export.
    ///
    /// Export risks data to CSV. The raw response is returned as a JSON value
    /// (the spec declares no schema for this export).
    pub async fn export_risks_csv(
        &self,
        query: &RisksQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/application-management/risks/export/csv", q)
            .await?)
    }

    /// `POST /web/api/v2.1/application-management/scan` — Initiate scan.
    ///
    /// Initiate an application vulnerability scan.
    pub async fn scan(&self, body: &ScanBody) -> Result<Response<ScanResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/application-management/scan", body)
            .await?)
    }
}
