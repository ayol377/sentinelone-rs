//! `Device Control` tag — service, query and body types.
//!
//! 1:1 with `docs/device-control.md` / `swagger_2_1.json`. Every endpoint is an
//! async method. Query params live in per-method `*Query` structs (all fields
//! optional, camelCase, comma-joined arrays). Request bodies live in per-method
//! `*Body` structs. Enum-valued fields are `String` for forward-compatibility;
//! allowed values are documented inline.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::device_control::{
    AffectedResult, DeviceControlConfiguration, DeviceControlEvent, DeviceControlRule,
    SuccessResult,
};
use crate::pagination::{Paginated, Response};

/// `Device Control` tag. Device control related endpoints.
pub struct DeviceControlService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Shared body filter
// ---------------------------------------------------------------------------

/// Scope/attribute filter shared by the rule mutation bodies (delete, create,
/// copy, move, enable, reorder, configuration). Every field is optional; set
/// only the ones relevant to the call. Array fields hold the raw values; enum
/// values are documented per builder.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceControlFilter {
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// A free-text search term, will match applicable attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return only device rules in this scope.
    ///
    /// Allowed values: `global`, `group`, `account`, `site`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// Return device rules with the filtered interface.
    ///
    /// Allowed values: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<String>>,
    /// Return device rules with the filtered device class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_classes: Option<Vec<String>>,
    /// Return device rules with the filtered service class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_classes: Option<Vec<String>>,
    /// Return device rules with the filtered rule name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// Return device rules with the filtered vendor id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_ids: Option<Vec<String>>,
    /// Return device rules with the filtered product id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<Vec<String>>,
    /// Return device rules with the filtered uId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uids: Option<Vec<String>>,
    /// Return device rules with the filtered action.
    ///
    /// Allowed values: `Allow`, `Block`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// Return device rules with the filtered status.
    ///
    /// Allowed values: `Enabled`, `Disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    /// Return device rules created before this timestamp.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Return device rules created after this timestamp.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Return device rules created before or at this timestamp.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Return device rules created after or at this timestamp.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Return device rules created within this range (inclusive).
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// List of ids to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// Return device rules with the filtered versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
    /// Return device rules with the filtered minor classes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_classes: Option<Vec<String>>,
    /// Access permission in.
    ///
    /// Allowed values: `Read-Only`, `Read-Write`, `Not-Applicable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_permissions: Option<Vec<String>>,
    /// Return device rules with the filtered bluetooth addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_addresses: Option<Vec<String>>,
    /// Return device rules with the filtered GATT services.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatt_services: Option<Vec<String>>,
    /// Return device rules with the filtered manufacturer names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_names: Option<Vec<String>>,
    /// Return device rules with the filtered device names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_names: Option<Vec<String>>,
    /// Return device rules with the filtered device information service info keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_information_service_info_keys: Option<Vec<String>>,
    /// Return device rules with the filtered device id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<String>>,
    /// The physical bus type of the device. Used by the reorder filter.
    ///
    /// Allowed values: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
}

// ---------------------------------------------------------------------------
// GET /web/api/v2.1/device-control  — list rules
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/device-control`.
///
/// All fields optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeviceRulesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items is returned, without the objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items is not calculated, speeding up execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by.
    ///
    /// Allowed values: `id`, `interface`, `deviceClass`, `ruleName`, `action`,
    /// `status`, `order`, `version`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// A free-text search term, will match applicable attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return only device rules in this scope (comma-joined).
    ///
    /// Allowed values: `global`, `group`, `account`, `site`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    /// Return device rules with the filtered interface (comma-joined).
    ///
    /// Allowed values: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<String>,
    /// Return device rules with the filtered device class (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_classes: Option<String>,
    /// Return device rules with the filtered service class (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_classes: Option<String>,
    /// Return device rules with the filtered rule name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// Return device rules with the filtered vendor id (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_ids: Option<String>,
    /// Return device rules with the filtered product id (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<String>,
    /// Return device rules with the filtered uId (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uids: Option<String>,
    /// Return device rules with the filtered action (comma-joined).
    ///
    /// Allowed values: `Allow`, `Block`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
    /// Return device rules with the filtered status (comma-joined).
    ///
    /// Allowed values: `Enabled`, `Disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    /// Return device rules created before this timestamp.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Return device rules created after this timestamp.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Return device rules created before or at this timestamp.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Return device rules created after or at this timestamp.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Return device rules created within this range (inclusive).
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// List of ids to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Return device rules with the filtered versions (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<String>,
    /// Return device rules with the filtered minor classes (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_classes: Option<String>,
    /// Access permission in (comma-joined).
    ///
    /// Allowed values: `Read-Only`, `Read-Write`, `Not-Applicable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_permissions: Option<String>,
    /// Return device rules with the filtered bluetooth addresses (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_addresses: Option<String>,
    /// Return device rules with the filtered GATT services (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatt_services: Option<String>,
    /// Return device rules with the filtered manufacturer names (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_names: Option<String>,
    /// Return device rules with the filtered device names (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_names: Option<String>,
    /// Return device rules with the filtered device information service info keys (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_information_service_info_keys: Option<String>,
    /// Return device rules with the filtered device id (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<String>,
    /// If true, all rules for the requested scope will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_pagination: Option<bool>,
}

impl GetDeviceRulesQuery {
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
    /// If true, only the total number of items is returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, total number of items is not calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Sort column: `id`, `interface`, `deviceClass`, `ruleName`, `action`,
    /// `status`, `order`, `version`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction: `asc` or `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// A free-text search term.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Scopes: `global`, `group`, `account`, `site`.
    pub fn scopes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scopes = Some(join(v));
        self
    }
    /// Interfaces: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    pub fn interfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.interfaces = Some(join(v));
        self
    }
    /// Device classes filter.
    pub fn device_classes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_classes = Some(join(v));
        self
    }
    /// Service classes filter.
    pub fn service_classes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.service_classes = Some(join(v));
        self
    }
    /// Rule name filter.
    pub fn rule_name(mut self, v: impl Into<String>) -> Self {
        self.rule_name = Some(v.into());
        self
    }
    /// Vendor ids filter.
    pub fn vendor_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.vendor_ids = Some(join(v));
        self
    }
    /// Product ids filter.
    pub fn product_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.product_ids = Some(join(v));
        self
    }
    /// uIds filter.
    pub fn uids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uids = Some(join(v));
        self
    }
    /// Actions: `Allow`, `Block`.
    pub fn actions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.actions = Some(join(v));
        self
    }
    /// Statuses: `Enabled`, `Disabled`.
    pub fn statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.statuses = Some(join(v));
        self
    }
    /// Return device rules created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Return device rules created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Return device rules created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Return device rules created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Return device rules created within this range (inclusive).
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// List of ids to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join(v));
        self
    }
    /// Versions filter.
    pub fn versions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.versions = Some(join(v));
        self
    }
    /// Minor classes filter.
    pub fn minor_classes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.minor_classes = Some(join(v));
        self
    }
    /// Access permissions: `Read-Only`, `Read-Write`, `Not-Applicable`.
    pub fn access_permissions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.access_permissions = Some(join(v));
        self
    }
    /// Bluetooth addresses filter.
    pub fn bluetooth_addresses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.bluetooth_addresses = Some(join(v));
        self
    }
    /// GATT services filter.
    pub fn gatt_services<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gatt_services = Some(join(v));
        self
    }
    /// Manufacturer names filter.
    pub fn manufacturer_names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_names = Some(join(v));
        self
    }
    /// Device names filter.
    pub fn device_names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_names = Some(join(v));
        self
    }
    /// Device information service info keys filter.
    pub fn device_information_service_info_keys<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_information_service_info_keys = Some(join(v));
        self
    }
    /// Device ids filter.
    pub fn device_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_ids = Some(join(v));
        self
    }
    /// If true, all rules for the requested scope will be returned.
    pub fn disable_pagination(mut self, v: bool) -> Self {
        self.disable_pagination = Some(v);
        self
    }
}

// ---------------------------------------------------------------------------
// GET /web/api/v2.1/device-control/export  — export rules query
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/device-control/export`.
///
/// Same scope/attribute filters as the list endpoint, minus pagination. All
/// fields optional; array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportRulesQuery {
    /// List of Account IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// A free-text search term, will match applicable attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return only device rules in this scope (comma-joined).
    ///
    /// Allowed values: `global`, `group`, `account`, `site`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    /// Return device rules with the filtered interface (comma-joined).
    ///
    /// Allowed values: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<String>,
    /// Return device rules with the filtered device class (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_classes: Option<String>,
    /// Return device rules with the filtered service class (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_classes: Option<String>,
    /// Return device rules with the filtered rule name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// Return device rules with the filtered vendor id (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_ids: Option<String>,
    /// Return device rules with the filtered product id (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<String>,
    /// Return device rules with the filtered uId (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uids: Option<String>,
    /// Return device rules with the filtered action (comma-joined).
    ///
    /// Allowed values: `Allow`, `Block`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
    /// Return device rules with the filtered status (comma-joined).
    ///
    /// Allowed values: `Enabled`, `Disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    /// Return device rules created before this timestamp.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Return device rules created after this timestamp.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Return device rules created before or at this timestamp.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Return device rules created after or at this timestamp.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Return device rules created within this range (inclusive).
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// List of ids to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Return device rules with the filtered versions (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<String>,
    /// Return device rules with the filtered minor classes (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_classes: Option<String>,
    /// Access permission in (comma-joined).
    ///
    /// Allowed values: `Read-Only`, `Read-Write`, `Not-Applicable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_permissions: Option<String>,
    /// Return device rules with the filtered bluetooth addresses (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_addresses: Option<String>,
    /// Return device rules with the filtered GATT services (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatt_services: Option<String>,
    /// Return device rules with the filtered manufacturer names (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_names: Option<String>,
    /// Return device rules with the filtered device names (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_names: Option<String>,
    /// Return device rules with the filtered device information service info keys (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_information_service_info_keys: Option<String>,
    /// Return device rules with the filtered device id (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<String>,
}

impl ExportRulesQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// A free-text search term.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Scopes: `global`, `group`, `account`, `site`.
    pub fn scopes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scopes = Some(join(v));
        self
    }
    /// Interfaces: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    pub fn interfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.interfaces = Some(join(v));
        self
    }
    /// Device classes filter.
    pub fn device_classes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_classes = Some(join(v));
        self
    }
    /// Service classes filter.
    pub fn service_classes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.service_classes = Some(join(v));
        self
    }
    /// Rule name filter.
    pub fn rule_name(mut self, v: impl Into<String>) -> Self {
        self.rule_name = Some(v.into());
        self
    }
    /// Vendor ids filter.
    pub fn vendor_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.vendor_ids = Some(join(v));
        self
    }
    /// Product ids filter.
    pub fn product_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.product_ids = Some(join(v));
        self
    }
    /// uIds filter.
    pub fn uids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uids = Some(join(v));
        self
    }
    /// Actions: `Allow`, `Block`.
    pub fn actions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.actions = Some(join(v));
        self
    }
    /// Statuses: `Enabled`, `Disabled`.
    pub fn statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.statuses = Some(join(v));
        self
    }
    /// Return device rules created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Return device rules created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Return device rules created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Return device rules created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Return device rules created within this range (inclusive).
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// List of ids to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join(v));
        self
    }
    /// Versions filter.
    pub fn versions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.versions = Some(join(v));
        self
    }
    /// Minor classes filter.
    pub fn minor_classes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.minor_classes = Some(join(v));
        self
    }
    /// Access permissions: `Read-Only`, `Read-Write`, `Not-Applicable`.
    pub fn access_permissions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.access_permissions = Some(join(v));
        self
    }
    /// Bluetooth addresses filter.
    pub fn bluetooth_addresses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.bluetooth_addresses = Some(join(v));
        self
    }
    /// GATT services filter.
    pub fn gatt_services<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gatt_services = Some(join(v));
        self
    }
    /// Manufacturer names filter.
    pub fn manufacturer_names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_names = Some(join(v));
        self
    }
    /// Device names filter.
    pub fn device_names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_names = Some(join(v));
        self
    }
    /// Device information service info keys filter.
    pub fn device_information_service_info_keys<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_information_service_info_keys = Some(join(v));
        self
    }
    /// Device ids filter.
    pub fn device_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_ids = Some(join(v));
        self
    }
}

// ---------------------------------------------------------------------------
// GET /web/api/v2.1/device-control/events  — events query
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/device-control/events`.
///
/// All fields optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventsQuery {
    /// Skip first number of items (0-1000). Use `cursor` beyond 1000 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items is not calculated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by.
    ///
    /// Allowed values: `id`, `eventTime`, `eventType`, `agentId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// A free-text search term, will match applicable attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return events generated after this time.
    #[serde(rename = "eventTime__gt", skip_serializing_if = "Option::is_none")]
    pub event_time_gt: Option<String>,
    /// Return events generated before this time.
    #[serde(rename = "eventTime__lt", skip_serializing_if = "Option::is_none")]
    pub event_time_lt: Option<String>,
    /// Return events generated after or at this time.
    #[serde(rename = "eventTime__gte", skip_serializing_if = "Option::is_none")]
    pub event_time_gte: Option<String>,
    /// Return events generated before or at this time.
    #[serde(rename = "eventTime__lte", skip_serializing_if = "Option::is_none")]
    pub event_time_lte: Option<String>,
    /// Return events created within this range (inclusive).
    #[serde(rename = "eventTime__between", skip_serializing_if = "Option::is_none")]
    pub event_time_between: Option<String>,
    /// List of IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of event IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ids: Option<String>,
    /// List of interfaces to filter by (comma-joined).
    ///
    /// Allowed values: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<String>,
    /// List of device classes to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_classes: Option<String>,
    /// List of service classes to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_classes: Option<String>,
    /// List of vendor IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_ids: Option<String>,
    /// List of product IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<String>,
    /// List of uIds to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uids: Option<String>,
    /// List of event types to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<String>,
    /// Access permission in (comma-joined).
    ///
    /// Allowed values: `Read-Only`, `Read-Write`, `Not-Applicable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_permissions: Option<String>,
    /// List of agent IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<String>,
    /// List of device IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<String>,
}

impl GetEventsQuery {
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
    /// If true, only the total number of items is returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, total number of items is not calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Sort column: `id`, `eventTime`, `eventType`, `agentId`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction: `asc` or `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// A free-text search term.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Return events generated after this time.
    pub fn event_time_gt(mut self, v: impl Into<String>) -> Self {
        self.event_time_gt = Some(v.into());
        self
    }
    /// Return events generated before this time.
    pub fn event_time_lt(mut self, v: impl Into<String>) -> Self {
        self.event_time_lt = Some(v.into());
        self
    }
    /// Return events generated after or at this time.
    pub fn event_time_gte(mut self, v: impl Into<String>) -> Self {
        self.event_time_gte = Some(v.into());
        self
    }
    /// Return events generated before or at this time.
    pub fn event_time_lte(mut self, v: impl Into<String>) -> Self {
        self.event_time_lte = Some(v.into());
        self
    }
    /// Return events created within this range (inclusive).
    pub fn event_time_between(mut self, v: impl Into<String>) -> Self {
        self.event_time_between = Some(v.into());
        self
    }
    /// List of IDs to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join(v));
        self
    }
    /// List of event IDs to filter by.
    pub fn event_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.event_ids = Some(join(v));
        self
    }
    /// Interfaces: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    pub fn interfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.interfaces = Some(join(v));
        self
    }
    /// List of device classes to filter by.
    pub fn device_classes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_classes = Some(join(v));
        self
    }
    /// List of service classes to filter by.
    pub fn service_classes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.service_classes = Some(join(v));
        self
    }
    /// List of vendor IDs to filter by.
    pub fn vendor_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.vendor_ids = Some(join(v));
        self
    }
    /// List of product IDs to filter by.
    pub fn product_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.product_ids = Some(join(v));
        self
    }
    /// List of uIds to filter by.
    pub fn uids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uids = Some(join(v));
        self
    }
    /// List of event types to filter by.
    pub fn event_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.event_types = Some(join(v));
        self
    }
    /// Access permissions: `Read-Only`, `Read-Write`, `Not-Applicable`.
    pub fn access_permissions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.access_permissions = Some(join(v));
        self
    }
    /// List of agent IDs to filter by.
    pub fn agent_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ids = Some(join(v));
        self
    }
    /// List of device IDs to filter by.
    pub fn device_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_ids = Some(join(v));
        self
    }
}

// ---------------------------------------------------------------------------
// GET /web/api/v2.1/device-control/configuration  — config query
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/device-control/configuration`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationQuery {
    /// List of Account IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl GetConfigurationQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
}

// ---------------------------------------------------------------------------
// Request bodies
// ---------------------------------------------------------------------------

/// Body for `DELETE /web/api/v2.1/device-control` (Delete Rules).
///
/// `filter` (required) selects the rules to delete.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRulesBody {
    /// Filter selecting the rules to delete. Required.
    pub filter: DeviceControlFilter,
}

/// The rule definition portion of a create/update request.
///
/// Mirrors `device_control.schemas_PostDeviceSchema.data`. On create, `action`,
/// `interface`, `ruleName`, `ruleType` and `status` are required by the API;
/// the remaining fields depend on `ruleType`. For `PUT` (update) every field is
/// optional. Enum-valued fields are documented inline.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRuleData {
    /// The physical bus type of the device.
    ///
    /// Allowed values: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    /// Physical device identifier. Mandatory when rule type is device id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// The Device Class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,
    /// Relevant for Bluetooth devices only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_class: Option<String>,
    /// The name of the device rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// Vendor identifier. Mandatory when rule type is vendor id or product id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
    /// Product identifier. Unique for a specific product module, per vendor ID, Interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Relevant USB Mass storage devices only (Interface=USB, Class=mass storage).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// Defines if the agent shall Block or Allow use of matching devices.
    ///
    /// Allowed values: `Allow`, `Block`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Defines if the rule is Enabled or Disabled.
    ///
    /// Allowed values: `Enabled`, `Disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Rule type. Depending on the type, each rule requires different parameters.
    ///
    /// Allowed values: `class`, `productId`, `vendorId`, `deviceId`, `uid`,
    /// `hwIdentifiers`, `bluetoothVersion`, `sdCard`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// The version of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Minor classes (Bluetooth).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_classes: Option<Vec<String>>,
    /// Access permission.
    ///
    /// Allowed values: `Read-Only`, `Read-Write`, `Not-Applicable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_permission: Option<String>,
    /// Bluetooth Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_address: Option<String>,
    /// GATT service IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatt_service: Option<Vec<String>>,
    /// Manufacturer Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_name: Option<String>,
    /// Device Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// Device Information Service Info Key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_information_service_info_key: Option<String>,
    /// Device Information Service Info Value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_information_service_info_value: Option<String>,
}

/// Scope selector used by the create-rule body (subset of the full filter).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRuleScopeFilter {
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

/// Body for `POST /web/api/v2.1/device-control` (Create Device Control Rule).
///
/// Both `data` (the rule definition) and `filter` (the target scope) are required.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRuleBody {
    /// The rule definition. Required.
    pub data: DeviceRuleData,
    /// Target scope for the new rule. Required.
    pub filter: DeviceRuleScopeFilter,
}

/// Body for `PUT /web/api/v2.1/device-control/{rule_id}` (Update Device Rule).
///
/// `data` (required) holds the fields to change. All fields within are optional.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRuleBody {
    /// The rule fields to change. Required.
    pub data: DeviceRuleData,
}

/// Configuration values for `PUT /web/api/v2.1/device-control/configuration`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSettingsData {
    /// Device control enabled for the scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Agent should report blocked events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_blocked: Option<bool>,
    /// Agent should report connected/disconnected events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_approved: Option<bool>,
    /// Agent should report 'connected as read-only' events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_read_only: Option<bool>,
    /// True if rules are decoupled from parent rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherits: Option<bool>,
    /// If null it means it is own policy else site or global to state which
    /// policy is being inherited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<String>,
    /// Disable RFCOMM for Bluetooth devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rfcomm: Option<bool>,
    /// Disallow access permission control (treat Read-Only rules as Read-Write).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallow_access_permission_control: Option<bool>,
    /// Disable Bluetooth LE Communication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_ble_communication: Option<bool>,
}

/// Body for `PUT /web/api/v2.1/device-control/configuration` (Update Configuration).
///
/// Both `data` (the new settings) and `filter` (the target scope) are required.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConfigurationBody {
    /// Target scope for the configuration change. Required.
    pub filter: DeviceRuleScopeFilter,
    /// The new configuration values. Required.
    pub data: DeviceSettingsData,
}

/// A single copy/move target (used in the `data` array of copy/move bodies).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyRuleTarget {
    /// Target account (or `null` for global scope). Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Target site (or `null` for global scope). Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// Target group(s). Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

/// Body for `POST /web/api/v2.1/device-control/copy-rules` (Copy Rules) and
/// `POST /web/api/v2.1/device-control/move-rules` (Move rules).
///
/// `filter` (required) selects the source rules; `data` (required) lists the
/// targets to copy/move them to.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyRuleBody {
    /// Filter selecting the source rules to copy/move. Required.
    pub filter: DeviceControlFilter,
    /// Targets (Accounts, Sites or Groups) to copy/move the rules to. Required.
    pub data: Vec<CopyRuleTarget>,
}

/// The status to apply in an enable/disable request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableRuleData {
    /// Whether the rules should be enabled or disabled. Required.
    ///
    /// Allowed values: `Enabled`, `Disabled`.
    pub status: String,
}

/// Body for `PUT /web/api/v2.1/device-control/enable` (Enable/Disable Rules).
///
/// `filter` (required) selects the rules, `data` (required) carries the new status.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableRuleBody {
    /// Filter selecting the rules to enable/disable. Required.
    pub filter: DeviceControlFilter,
    /// The new status to apply. Required.
    pub data: EnableRuleData,
}

/// A single rule's desired position in a reorder request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderEntry {
    /// Rule ID. Required.
    pub id: String,
    /// Desired position in the list of rules (minimum 1). Required.
    pub order: i64,
}

/// Scope/interface selector for a reorder request.
///
/// `interface` is required; the scope ids are optional.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderFilter {
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// The physical bus type of the device. Required.
    ///
    /// Allowed values: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    pub interface: String,
}

/// Body for `PUT /web/api/v2.1/device-control/reorder` (Reorder Rules).
///
/// `data` (required) lists the new positions; `filter` (required) scopes the change.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderBody {
    /// The rules and their desired positions. Required.
    pub data: Vec<ReorderEntry>,
    /// Scope and interface selector. Required (`interface` mandatory).
    pub filter: ReorderFilter,
}

// ---------------------------------------------------------------------------
// Service implementation
// ---------------------------------------------------------------------------

impl DeviceControlService<'_> {
    /// **Delete Rules** — Delete Device Control rules that match the filter.
    ///
    /// `DELETE /web/api/v2.1/device-control`
    pub async fn delete_rules(
        &self,
        body: &DeleteRulesBody,
    ) -> Result<Response<AffectedResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json(Method::DELETE, "/web/api/v2.1/device-control", None, Some(body))
            .await?)
    }

    /// **Get Device Rules** — Get the Device Control rules of a specified
    /// Account, Site, Group or Global (tenant) that match the filter.
    ///
    /// `GET /web/api/v2.1/device-control`
    pub async fn list(
        &self,
        query: &GetDeviceRulesQuery,
    ) -> Result<Paginated<DeviceControlRule>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/device-control", q).await?)
    }

    /// **Create Device Control Rule** — Create a new Device Control rule. These
    /// rules allow or block devices, based on device identifiers. Rules apply to
    /// a scope: Global (tenant), Account, Site, or Group. See
    /// <https://support.sentinelone.com/hc/en-us/articles/360023338494>.
    /// Recommended: review Device Control Known Limitations
    /// (<https://support.sentinelone.com/hc/en-us/articles/360021104114>) first.
    /// Device Control requires the Control SKU. Linux Agents do not support it.
    ///
    /// `POST /web/api/v2.1/device-control`
    pub async fn create_rule(
        &self,
        body: &CreateRuleBody,
    ) -> Result<Response<DeviceControlRule>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/device-control", body).await?)
    }

    /// **Get Configuration** — Get Device Control configuration for a given scope.
    /// To filter the results for a scope: Global — `tenant=true` and no other
    /// scope ID; Account — `tenant=false` and at least one Account ID; Site —
    /// `tenant=false` and at least one Site ID. Device Control requires the
    /// Control SKU and is not supported on Linux.
    ///
    /// `GET /web/api/v2.1/device-control/configuration`
    pub async fn get_configuration(
        &self,
        query: &GetConfigurationQuery,
    ) -> Result<Response<DeviceControlConfiguration>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/device-control/configuration", q)
            .await?)
    }

    /// **Update Configuration** — Change the Device Control configuration. Enter
    /// a Group ID, Site ID, Account ID, or `tenant=true`. If only `tenant` is set
    /// and the other scopes are empty, the change applies to the Global policy.
    /// Device Control requires the Control SKU and is not supported on Linux.
    ///
    /// `PUT /web/api/v2.1/device-control/configuration`
    pub async fn update_configuration(
        &self,
        body: &UpdateConfigurationBody,
    ) -> Result<Response<DeviceControlConfiguration>, Error> {
        Ok(self
            .client
            .http()
            .request_json(
                Method::PUT,
                "/web/api/v2.1/device-control/configuration",
                None,
                Some(body),
            )
            .await?)
    }

    /// **Copy Rules** — Copy a set of Device Control rules to use in other
    /// Accounts, Sites, or Groups. Copy from a source Group, Site, or Account to
    /// target Groups, Sites, or Accounts. Define the rules with the filters.
    /// Device Control requires the Control SKU. Linux Agents do not support it.
    ///
    /// `POST /web/api/v2.1/device-control/copy-rules`
    pub async fn copy_rules(
        &self,
        body: &CopyRuleBody,
    ) -> Result<Response<AffectedResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/device-control/copy-rules", body)
            .await?)
    }

    /// **Enable/Disable Rules** — Change the status of rules between Enabled and
    /// Disabled. It is best practice to disable a rule rather than delete it.
    /// Note (Windows): an already-connected USB device is unaffected until it
    /// reconnects; Bluetooth rules require pairing after the Agent supports
    /// Bluetooth (reboot or re-pair if previously paired). On macOS, changes
    /// apply to already-connected devices.
    ///
    /// `PUT /web/api/v2.1/device-control/enable`
    pub async fn enable_rules(
        &self,
        body: &EnableRuleBody,
    ) -> Result<Response<AffectedResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json(
                Method::PUT,
                "/web/api/v2.1/device-control/enable",
                None,
                Some(body),
            )
            .await?)
    }

    /// **Get Device Control Events** — Get the data of Device Control events on
    /// Windows and macOS endpoints with Device Control-enabled Agents that match
    /// the filter. Device Control requires the Control SKU. Linux Agents do not
    /// support it.
    ///
    /// `GET /web/api/v2.1/device-control/events`
    pub async fn events(
        &self,
        query: &GetEventsQuery,
    ) -> Result<Paginated<DeviceControlEvent>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/device-control/events", q)
            .await?)
    }

    /// **Export Rules** — Export Device Control rules to a CSV file.
    ///
    /// The endpoint returns a CSV payload; this method deserializes the response
    /// as a generic JSON value (the spec does not define a typed schema).
    ///
    /// `GET /web/api/v2.1/device-control/export`
    pub async fn export(
        &self,
        query: &ExportRulesQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/device-control/export", q)
            .await?)
    }

    /// **Move rules** — Move a set of Device Control rules to other Accounts,
    /// Sites, or Groups. This removes the rule from the source and copies it to
    /// the targets. Define the rules with the filters. Device Control requires
    /// the Control SKU. Linux Agents do not support it.
    ///
    /// `POST /web/api/v2.1/device-control/move-rules`
    pub async fn move_rules(
        &self,
        body: &CopyRuleBody,
    ) -> Result<Response<AffectedResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/device-control/move-rules", body)
            .await?)
    }

    /// **Reorder Rules** — Change the order of rules for a specific scope. When a
    /// device connects, the Agent evaluates rules top-to-bottom and applies the
    /// first match. Device Control requires the Control SKU. Linux Agents do not
    /// support it.
    ///
    /// `PUT /web/api/v2.1/device-control/reorder`
    pub async fn reorder(
        &self,
        body: &ReorderBody,
    ) -> Result<Response<SuccessResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json(
                Method::PUT,
                "/web/api/v2.1/device-control/reorder",
                None,
                Some(body),
            )
            .await?)
    }

    /// **Update Device Rule** — Change the Device Control rule that matches the
    /// filter. See
    /// <https://support.sentinelone.com/hc/en-us/articles/360023338494>.
    ///
    /// `PUT /web/api/v2.1/device-control/{rule_id}`
    pub async fn update_rule(
        &self,
        rule_id: impl Into<String>,
        body: &UpdateRuleBody,
    ) -> Result<Response<DeviceControlRule>, Error> {
        let path = format!("/web/api/v2.1/device-control/{}", rule_id.into());
        Ok(self
            .client
            .http()
            .request_json(Method::PUT, &path, None, Some(body))
            .await?)
    }
}

/// Join an iterator of string-likes into a single comma-separated value, as the
/// SentinelOne API expects for array query params.
fn join<I, S>(values: I) -> String
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
