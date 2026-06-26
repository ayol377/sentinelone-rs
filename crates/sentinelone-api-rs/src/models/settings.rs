//! Models for the `Settings` tag (System settings).
//!
//! Hand-written for strict 1:1 parity with `swagger_2_1.json`. Each response
//! `data` object is modeled as a dedicated struct. Per the parity rules, a field
//! is a bare `T` only when it is in the schema `required` array and is not
//! `x-nullable`; otherwise it is `Option<T>` (the "default null" behaviour).

use serde::Deserialize;

/// Global Active Directory settings.
///
/// `GET`/`PUT /web/api/v2.1/settings/active-directory` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdSettings {
    /// Is AD service enabled? (nullable)
    pub enabled: Option<bool>,
    /// Active Directory server address. (nullable)
    pub host: Option<String>,
    /// Active Directory server port. (nullable)
    pub port: Option<i64>,
    /// Username used to log in to active directory. (nullable)
    pub username: Option<String>,
    /// Root Domain Name of Active Directory. (nullable)
    pub root_dn: Option<String>,
    /// Should we speak to the Active Directory server using SSL? (nullable)
    pub ssl: Option<bool>,
}

/// Result of testing Active Directory settings.
///
/// `POST /web/api/v2.1/settings/active-directory/test` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestAdSettings {
    /// Status. (required, not nullable)
    pub status: bool,
}

/// Active Directory FQDN to user-role mapping for a scope.
///
/// `GET`/`PUT /web/api/v2.1/settings/active-directory/scope-mapping` response
/// `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdFqdns {
    /// FQDNs mapped to the Admin role. (nullable)
    pub admin: Option<Vec<String>>,
    /// FQDNs mapped to the Viewer role. (nullable)
    pub viewer: Option<Vec<String>>,
}

/// Microsoft settings (DEPRECATED).
///
/// `GET`/`PUT /web/api/v2.1/settings/microsoft` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicrosoftSettings {
    /// Microsoft service is enabled. (nullable)
    pub enabled: Option<bool>,
    /// URL used to authenticate with microsoft. (nullable)
    pub url: Option<String>,
    /// True if site inherits settings from the global scope, False if using
    /// custom settings. (nullable)
    pub inherits: Option<bool>,
    /// The expiry time of the given url (date-time string). (nullable)
    pub expiry_date: Option<String>,
}

/// Result of testing Microsoft settings (DEPRECATED).
///
/// `POST /web/api/v2.1/settings/microsoft/test` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestMicrosoftSettings {
    /// True if succeeded. (nullable)
    pub success: Option<bool>,
    /// Reason for unsuccessful call. (nullable)
    pub reason: Option<String>,
}

/// Indicates whether each notification channel is properly configured.
///
/// Nested under [`NotificationSettings::configurations`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationConfigurations {
    /// If not empty, email configuration is missing. (nullable)
    pub email: Option<String>,
    /// If not empty, sms configuration is missing. (nullable)
    pub sms: Option<String>,
    /// If not empty, syslog configuration is missing. (nullable)
    pub syslog: Option<String>,
}

/// The available notification categories. Each category is a freeform map of
/// notification id -> settings, so each is modeled as `serde_json::Value`.
///
/// Nested under [`NotificationSettings::notifications`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationCategories {
    /// Administrative category items. (nullable)
    pub administrative: Option<serde_json::Value>,
    /// Operations category items. (nullable)
    pub operations: Option<serde_json::Value>,
    /// Malware category items. (nullable)
    pub malware: Option<serde_json::Value>,
    /// Mitigation category items. (nullable)
    pub mitigation: Option<serde_json::Value>,
    /// Whitelist/blacklist category items. (nullable)
    pub whitelistblacklist: Option<serde_json::Value>,
    /// Device control category items. (nullable)
    pub devicecontrol: Option<serde_json::Value>,
    /// Firewall control category items. (nullable)
    pub firewallcontrol: Option<serde_json::Value>,
    /// Remote shell category items. (nullable)
    pub remoteshell: Option<serde_json::Value>,
    /// Locations category items. (nullable)
    pub locations: Option<serde_json::Value>,
    /// Ranger category items. (nullable)
    pub ranger: Option<serde_json::Value>,
    /// Threat management category items. (nullable)
    pub threatmanagement: Option<serde_json::Value>,
    /// Custom rules category items. (nullable)
    pub customrules: Option<serde_json::Value>,
    /// Endpoint tagging category items. (nullable)
    pub endpointtagging: Option<serde_json::Value>,
    /// Active Directory category items. (nullable)
    pub activedirectory: Option<serde_json::Value>,
    /// Compromised credentials protection category items. (nullable)
    pub compromisedcredentialsprotection: Option<serde_json::Value>,
    /// Identity policy configuration category items. (nullable)
    pub identitypolicyconfiguration: Option<serde_json::Value>,
    /// Hyperautomation category items. (nullable)
    pub hyperautomation: Option<serde_json::Value>,
    /// Identity fault audit log category items. (nullable)
    pub identityfaultauditlog: Option<serde_json::Value>,
}

/// Last-modified metadata for notification settings.
///
/// Nested under [`NotificationSettings::last_modified`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationLastModified {
    /// Last modified timestamp (date-time string). (nullable)
    pub updated_at: Option<String>,
    /// User that last modified the settings. (nullable)
    pub updated_by: Option<String>,
}

/// Notification settings for a scope.
///
/// `GET`/`PUT /web/api/v2.1/settings/notifications` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    /// Channel configuration status. (nullable)
    pub configurations: Option<NotificationConfigurations>,
    /// Notification categories. (nullable)
    pub notifications: Option<NotificationCategories>,
    /// Last modified metadata. (nullable)
    pub last_modified: Option<NotificationLastModified>,
}

/// Result of clearing pending email notifications.
///
/// `POST /web/api/v2.1/settings/notifications/cancel-pending-emails` response
/// `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelPendingEmailNotifications {
    /// Number of canceled emails. (required, not nullable)
    pub canceled: i64,
}

/// A single notification recipient.
///
/// Element of [`NotificationRecipientsList::recipients`] and the body of
/// `PUT /web/api/v2.1/settings/recipients` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRecipient {
    /// Notification recipient ID. (nullable)
    pub id: Option<String>,
    /// Notification recipient name. (nullable)
    pub name: Option<String>,
    /// Notification recipient email. (nullable)
    pub email: Option<String>,
    /// Notification recipient SMS. (nullable)
    pub sms: Option<String>,
    /// Creation timestamp (date-time string). (nullable)
    pub created_at: Option<String>,
    /// Update timestamp (date-time string). (nullable)
    pub updated_at: Option<String>,
}

/// List of notification recipients.
///
/// `GET /web/api/v2.1/settings/recipients` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRecipientsList {
    /// Notification recipients. (nullable)
    pub recipients: Option<Vec<NotificationRecipient>>,
}

/// SMS settings (DEPRECATED).
///
/// `GET`/`PUT /web/api/v2.1/settings/sms` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsSettings {
    /// SMS service is enabled. (nullable)
    pub enabled: Option<bool>,
}

/// SMTP server configuration.
///
/// `GET`/`PUT /web/api/v2.1/settings/smtp` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmtpSettings {
    /// True if site inherits SMTP settings from global scope, False if using
    /// custom settings. (nullable)
    pub inherits: Option<bool>,
    /// SMTP service is enabled. (nullable)
    pub enabled: Option<bool>,
    /// SMTP service host. (nullable)
    pub host: Option<String>,
    /// SMTP service port. (nullable)
    pub port: Option<i64>,
    /// SMTP service encryption type. Allowed values: `ssl`, `tls`. (nullable)
    pub encryption: Option<String>,
    /// SMTP service username. (nullable)
    pub username: Option<String>,
    /// SMTP service password. Required when creating new SMTP settings or
    /// updating host and/or port of the existing one. (nullable)
    pub password: Option<String>,
    /// SMTP service no-reply email. (nullable)
    pub no_reply_email: Option<String>,
}

/// Result of testing SMTP settings.
///
/// `POST /web/api/v2.1/settings/smtp/test` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestSmtpSettings {
    /// Status. (required, not nullable)
    pub status: bool,
}

/// Single Sign-On (SAML 2.0) configuration.
///
/// `GET`/`PUT /web/api/v2.1/settings/sso` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SsoSettings {
    /// Indicates if SSO is enabled. (nullable)
    pub enabled: Option<bool>,
    /// The SSO URL of the Identity Provider (Login URL). (nullable)
    pub idp_sso_url: Option<String>,
    /// Identity provider's Entity ID (a.k.a. Issuer). (nullable)
    pub idp_entity_id: Option<String>,
    /// Identity provider's certificate file name. (nullable)
    pub idp_cert_name: Option<String>,
    /// Management console Assertion Consumer Service (ACS) URL. (nullable)
    pub sp_acs_url: Option<String>,
    /// Identifier the Management console creates to dialogue with the SSO
    /// provider. (nullable)
    pub sp_entity_id: Option<String>,
    /// The role name of the default role for a new user logging in via SSO for
    /// the first time. (nullable)
    pub default_user_role: Option<String>,
    /// The role ID of the default role for a new SSO user. (nullable)
    pub default_user_role_id: Option<String>,
    /// True if the user should be auto provisioned. (nullable)
    pub auto_provisioning: Option<bool>,
    /// A list of domain names associated with the scope. (nullable)
    pub domains: Option<Vec<String>>,
    /// True if the domains should be propagated to children scopes. (nullable)
    pub sso_propagate_domains_to_children: Option<bool>,
    /// Scope(s) to inherit domains from. Allowed item values: `group`, `site`,
    /// `account`, `tenant`. (nullable)
    pub sso_inherit_domains_from: Option<Vec<String>>,
    /// A dictionary of inheritable domains. (nullable)
    pub sso_inheritable_domains: Option<serde_json::Value>,
    /// Type of re-authentication used for session elevation. Allowed values:
    /// `totp`, `idp`. (nullable)
    pub sso_elevated_session_reauth_type: Option<String>,
    /// Marks whether re-auth type choice should be available in SSO settings.
    /// (nullable)
    pub sso_elevated_session_reauth_type_enabled: Option<bool>,
    /// Indicates if SAML Request Signing is enabled. (nullable)
    pub sign_request: Option<bool>,
}

/// Service Provider certificate for the SSO configuration.
///
/// `GET /web/api/v2.1/settings/sso/sp-cert` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SsoSpCertificate {
    /// File name of the signing certificate used by the service provider to
    /// sign SAML requests. (nullable)
    pub file_name: Option<String>,
    /// Certificate in PEM format. (nullable)
    pub pem: Option<String>,
    /// Certificate issued at (date-time string). (nullable)
    pub issued_at: Option<String>,
    /// Certificate expires at (date-time string). (nullable)
    pub expires_at: Option<String>,
}

/// Result of testing SSO settings.
///
/// `POST /web/api/v2.1/settings/sso/test` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SsoTest {
    /// The url to redirect to for the test. (nullable)
    pub redirect_url: Option<String>,
}

/// Syslog server configuration.
///
/// `GET`/`PUT /web/api/v2.1/settings/syslog` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyslogSettings {
    /// SysLog service is enabled. (nullable)
    pub enabled: Option<bool>,
    /// SysLog service host. (nullable)
    pub host: Option<String>,
    /// SysLog service port. (nullable)
    pub port: Option<i64>,
    /// SysLog service uses ssl. (nullable)
    pub ssl: Option<bool>,
    /// SysLog service format. Allowed values: `cef`, `stix`, `ioc`, `cef2`,
    /// `rfc-5424`. (nullable)
    pub format: Option<String>,
    /// SysLog service server certificate name. (nullable)
    pub server_cert_name: Option<String>,
    /// SysLog service server certificate content in Base64. (nullable)
    pub server_cert_content: Option<String>,
    /// SysLog service client certificate name. (nullable)
    pub client_cert_name: Option<String>,
    /// SysLog service client certificate content in Base64. (nullable)
    pub client_cert_content: Option<String>,
    /// SysLog service client key name. (nullable)
    pub client_key_name: Option<String>,
    /// SysLog service client key content in Base64. (nullable)
    pub client_key_content: Option<String>,
    /// SysLog server token. (nullable)
    pub token: Option<String>,
}

/// Result of testing syslog settings.
///
/// `POST /web/api/v2.1/settings/syslog/test` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestSyslogSettings {
    /// Status. (required, not nullable)
    pub status: bool,
}

/// Generic success response.
///
/// `DELETE /web/api/v2.1/settings/recipients/{recipient_id}` response `data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    /// Indicates a successful operation. (nullable)
    pub success: Option<bool>,
}
