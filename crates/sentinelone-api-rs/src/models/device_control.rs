//! `Device Control` entity models.
//!
//! Field nullability mirrors `swagger_2_1.json`: a field is a bare type only
//! when it is in the schema `required` array and not `x-nullable`; everything
//! else is `Option<T>` (the SentinelOne "default null" behaviour). Enum-valued
//! fields are kept as `String` for forward-compatibility; the allowed values
//! are documented in each field's doc comment.

use serde::Deserialize;

/// A Device Control rule.
///
/// Returned by `GET /web/api/v2.1/device-control` (list),
/// `POST /web/api/v2.1/device-control` (create) and
/// `PUT /web/api/v2.1/device-control/{rule_id}` (update).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceControlRule {
    /// Rule ID. Nullable/optional in the spec.
    pub id: Option<String>,
    /// Position in the list of rules.
    pub order: Option<i64>,
    /// The physical bus type of the device.
    ///
    /// Allowed values: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    pub interface: Option<String>,
    /// The id of the physical device connected to the interface.
    pub device_id: Option<String>,
    /// The Device Class key. Valid for all rule types.
    pub device_class: Option<String>,
    /// The Device Class name. Valid for all rule types. (read-only, free-form)
    pub device_class_name: Option<serde_json::Value>,
    /// The name of the device rule.
    pub rule_name: Option<String>,
    /// Vendor identifier. Mandatory when rule type is vendor id or product id.
    pub vendor_id: Option<String>,
    /// Product identifier. Unique for a specific product module, per vendor ID, Interface.
    pub product_id: Option<String>,
    /// Relevant USB Mass storage devices only (Interface=USB, Class=mass storage).
    pub uid: Option<String>,
    /// Defines if the agent shall Block or Allow use of matching devices.
    ///
    /// Allowed values: `Allow`, `Block`.
    pub action: Option<String>,
    /// Defines if the rule is Enabled or Disabled.
    ///
    /// Allowed values: `Enabled`, `Disabled`.
    pub status: Option<String>,
    /// Scope of the rule.
    ///
    /// Allowed values: `global`, `group`, `account`, `site`.
    pub scope: Option<String>,
    /// Extended name of the scope.
    pub scope_name: Option<String>,
    /// The id representing a group or a site dependent on the scope.
    pub scope_id: Option<String>,
    /// True if the rule can be modified at this scope level.
    pub editable: Option<bool>,
    /// Defines a set of fields that are mandatory.
    ///
    /// Allowed values: `class`, `productId`, `vendorId`, `deviceId`, `uid`,
    /// `hwIdentifiers`, `bluetoothVersion`, `sdCard`.
    pub rule_type: Option<String>,
    /// Date of rule creation (ISO-8601 timestamp).
    pub created_at: Option<String>,
    /// Date of last update (ISO-8601 timestamp).
    pub updated_at: Option<String>,
    /// Full name of the creating user.
    pub creator: Option<String>,
    /// Id of the creating user.
    pub creator_id: Option<String>,
    /// The version of the device.
    pub version: Option<String>,
    /// List of Bluetooth minor classes.
    pub minor_classes: Option<Vec<String>>,
    /// Access permission.
    ///
    /// Allowed values: `Read-Only`, `Read-Write`, `Not-Applicable`.
    pub access_permission: Option<String>,
    /// Bluetooth Address.
    pub bluetooth_address: Option<String>,
    /// GATT Service IDs.
    pub gatt_service: Option<Vec<String>>,
    /// Manufacturer Name.
    pub manufacturer_name: Option<String>,
    /// Device Name.
    pub device_name: Option<String>,
    /// Device Information Service Info Key.
    pub device_information_service_info_key: Option<String>,
    /// Device Information Service Info Value.
    pub device_information_service_info_value: Option<String>,
}

/// A Device Control event.
///
/// Returned by `GET /web/api/v2.1/device-control/events`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceControlEvent {
    /// Id.
    pub id: Option<String>,
    /// Created at (ISO-8601 timestamp).
    pub created_at: Option<String>,
    /// Updated at (ISO-8601 timestamp).
    pub updated_at: Option<String>,
    /// Event id.
    pub event_id: Option<String>,
    /// Interface.
    ///
    /// Allowed values: `USB`, `Bluetooth`, `Thunderbolt`, `SDCard`.
    pub interface: Option<String>,
    /// Device class.
    pub device_class: Option<String>,
    /// Service class.
    pub service_class: Option<String>,
    /// Rule id.
    pub rule_id: Option<String>,
    /// Vendor id.
    pub vendor_id: Option<String>,
    /// Product id.
    pub product_id: Option<String>,
    /// Event time (ISO-8601 timestamp).
    pub event_time: Option<String>,
    /// Event type.
    pub event_type: Option<String>,
    /// Device name.
    pub device_name: Option<String>,
    /// U id.
    pub u_id: Option<String>,
    /// Agent id.
    pub agent_id: Option<String>,
    /// Minor class.
    pub minor_class: Option<String>,
    /// Profile uuids.
    pub profile_uuids: Option<String>,
    /// Lmp version.
    pub lmp_version: Option<String>,
    /// Access permission.
    ///
    /// Allowed values: `Read-Only`, `Read-Write`, `Not-Applicable`.
    pub access_permission: Option<String>,
    /// Computer name.
    pub computer_name: Option<String>,
    /// Last logged in user name.
    pub last_logged_in_user_name: Option<String>,
    /// Device id.
    pub device_id: Option<String>,
}

/// Device Control configuration for a scope.
///
/// Returned by `GET /web/api/v2.1/device-control/configuration` and
/// `PUT /web/api/v2.1/device-control/configuration`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceControlConfiguration {
    /// Device control enabled for the scope.
    pub enabled: Option<bool>,
    /// Agent should report blocked events.
    pub report_blocked: Option<bool>,
    /// Agent should report connected/disconnected events.
    pub report_approved: Option<bool>,
    /// Agent should report 'connected as read-only' events.
    pub report_read_only: Option<bool>,
    /// True if rules are decoupled from parent rules.
    pub inherits: Option<bool>,
    /// If null it means it is own policy else it will be site or global to
    /// state which policy is being inherited. Nullable.
    pub inherited_from: Option<String>,
    /// Disable RFCOMM for Bluetooth devices.
    pub disable_rfcomm: Option<bool>,
    /// Disallow access permission control (i.e. treat Read-Only rules as Read-Write).
    pub disallow_access_permission_control: Option<bool>,
    /// Disable Bluetooth LE Communication.
    pub disable_ble_communication: Option<bool>,
}

/// Number of entities affected by a requested operation.
///
/// Returned by the delete / copy / move / enable endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResult {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
}

/// Indicates a successful operation.
///
/// Returned by `PUT /web/api/v2.1/device-control/reorder`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResult {
    /// Indicates a successful operation.
    pub success: Option<bool>,
}
