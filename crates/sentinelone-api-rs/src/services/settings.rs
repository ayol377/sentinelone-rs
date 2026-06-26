use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::settings::{
    AdFqdns, AdSettings, CancelPendingEmailNotifications, MicrosoftSettings,
    NotificationRecipient, NotificationRecipientsList, NotificationSettings, SmsSettings,
    SmtpSettings, SsoSettings, SsoSpCertificate, SsoTest, SuccessResponse, SyslogSettings,
    TestAdSettings, TestMicrosoftSettings, TestSmtpSettings, TestSyslogSettings,
};
use crate::pagination::Response;

/// `Settings` tag — System settings.
///
/// Endpoints to read, update, and test the various global/scope settings of the
/// Management console: Active Directory, Microsoft (deprecated), notifications,
/// notification recipients, SMS (deprecated), SMTP, SSO, and syslog.
pub struct SettingsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Scope filter query params shared by every `GET` settings endpoint.
///
/// Both filters are optional arrays serialized comma-joined, as the API expects
/// (e.g. `"225494730938493804,225494730938493915"`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeQuery {
    /// List of Site IDs to filter by. Example:
    /// `"225494730938493804,225494730938493915"`. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example:
    /// `"225494730938493804,225494730938493915"`. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl ScopeQuery {
    /// List of Site IDs to filter by. (optional)
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            ids.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Account IDs to filter by. (optional)
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            ids.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query params for `GET /web/api/v2.1/settings/recipients`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipientsQuery {
    /// List of Site IDs to filter by. Example:
    /// `"225494730938493804,225494730938493915"`. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example:
    /// `"225494730938493804,225494730938493915"`. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Name. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Email. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Sms. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms: Option<String>,
    /// Full text search for fields: name, email, sms. (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl RecipientsQuery {
    /// List of Site IDs to filter by. (optional)
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            ids.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Account IDs to filter by. (optional)
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            ids.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Name. (optional)
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    /// Email. (optional)
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
    /// Sms. (optional)
    pub fn sms(mut self, sms: impl Into<String>) -> Self {
        self.sms = Some(sms.into());
        self
    }
    /// Full text search for fields: name, email, sms. (optional)
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }
}

impl SettingsService<'_> {
    // -- Active Directory ----------------------------------------------------

    /// `GET /web/api/v2.1/settings/active-directory` — Get AD Settings.
    ///
    /// Get the Global Active Directory settings.
    pub async fn get_active_directory(
        &self,
        query: &ScopeQuery,
    ) -> Result<Response<AdSettings>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/settings/active-directory", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/settings/active-directory` — Set AD Settings.
    ///
    /// Update the Global Active Directory settings.
    ///
    /// `body`: `settings_AdSettingsPutSchema` (freeform JSON object).
    pub async fn set_active_directory(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<AdSettings>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<AdSettings>>(
                Method::PUT,
                "/web/api/v2.1/settings/active-directory",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/settings/active-directory/scope-mapping` — Get AD
    /// FQDNs.
    ///
    /// Get the map of Active Directory FQDNs to user roles of the given Sites
    /// (use "sites" to get IDs) or Accounts ("accounts").
    pub async fn get_active_directory_scope_mapping(
        &self,
        query: &ScopeQuery,
    ) -> Result<Response<AdFqdns>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/settings/active-directory/scope-mapping", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/settings/active-directory/scope-mapping` — Set AD
    /// FQDNs.
    ///
    /// Update the Active Directory FQDNs of a Site or Account.
    ///
    /// `body`: `settings_AdFqdnsPutSchema` (freeform JSON object).
    pub async fn set_active_directory_scope_mapping(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<AdFqdns>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<AdFqdns>>(
                Method::PUT,
                "/web/api/v2.1/settings/active-directory/scope-mapping",
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/settings/active-directory/test` — Test AD Settings.
    ///
    /// Test Active Directory settings.
    ///
    /// `body`: `settings_AdSettingsPutSchema` (freeform JSON object).
    pub async fn test_active_directory(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<TestAdSettings>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/settings/active-directory/test", body)
            .await?)
    }

    // -- Microsoft (DEPRECATED) ---------------------------------------------

    /// `GET /web/api/v2.1/settings/microsoft` — Get Microsoft Settings.
    ///
    /// [DEPRECATED] Gets the Microsoft settings of the Sites or Accounts.
    pub async fn get_microsoft(
        &self,
        query: &ScopeQuery,
    ) -> Result<Response<MicrosoftSettings>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/settings/microsoft", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/settings/microsoft` — Set Microsoft Settings.
    ///
    /// [DEPRECATED] Update Microsoft settings for the given Sites or Accounts.
    ///
    /// `body`: `settings_MicrosoftSettingsPutSchema` (freeform JSON object).
    pub async fn set_microsoft(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<MicrosoftSettings>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<MicrosoftSettings>>(
                Method::PUT,
                "/web/api/v2.1/settings/microsoft",
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/settings/microsoft/test` — Test Microsoft Settings.
    ///
    /// [DEPRECATED] Test Microsoft settings.
    ///
    /// `body`: `settings_MicrosoftSettingsPutSchema` (freeform JSON object).
    pub async fn test_microsoft(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<TestMicrosoftSettings>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/settings/microsoft/test", body)
            .await?)
    }

    // -- Notifications -------------------------------------------------------

    /// `GET /web/api/v2.1/settings/notifications` — Get Notification Settings.
    ///
    /// Get the notification settings for the given Sites (to get the IDs, run
    /// "settings") or Accounts ("accounts"). The response shows every possible
    /// notification and whether it is active and if so, for email or syslog or
    /// both. It also shows the ID string for each notification, which can be
    /// used in other commands. Note: Each notification also shows "sms" which is
    /// deprecated.
    pub async fn get_notifications(
        &self,
        query: &ScopeQuery,
    ) -> Result<Response<NotificationSettings>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/settings/notifications", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/settings/notifications` — Set Notification Settings.
    ///
    /// Change the notifications for the given Sites (to get the IDs, run
    /// "settings") or Accounts ("accounts"). Best practice: Get the current
    /// settings (see Get Notification Settings) before you run this command.
    ///
    /// `body`: `notifications_schemas_NotificationSettingsPutSchema` (freeform
    /// JSON object).
    pub async fn set_notifications(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<NotificationSettings>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<NotificationSettings>>(
                Method::PUT,
                "/web/api/v2.1/settings/notifications",
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/settings/notifications/cancel-pending-emails` — Clear
    /// Pending Emails.
    ///
    /// Clear (discard without sending) pending email notifications for the given
    /// Sites (to get the IDs, run "sites") or Accounts ("accounts"). When you
    /// set email recipients to get notifications for activities in the system,
    /// you can set too many, or in other ways cause issues that demand that the
    /// queue be cleared.
    ///
    /// `body`:
    /// `notifications_schemas_CancelPendingEmailNotificationsPostSchema`
    /// (freeform JSON object).
    pub async fn cancel_pending_emails(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<CancelPendingEmailNotifications>, Error> {
        Ok(self
            .client
            .http()
            .post(
                "/web/api/v2.1/settings/notifications/cancel-pending-emails",
                body,
            )
            .await?)
    }

    // -- Notification recipients --------------------------------------------

    /// `GET /web/api/v2.1/settings/recipients` — Get Notification Recipients.
    ///
    /// Get the emails that are configured to receive notifications.
    pub async fn get_recipients(
        &self,
        query: &RecipientsQuery,
    ) -> Result<Response<NotificationRecipientsList>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/settings/recipients", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/settings/recipients` — Set Notification Recipients.
    ///
    /// Set the emails of recipients to get notifications.
    ///
    /// `body`: `settings_NotificationRecipientSettingsPutSchema` (freeform JSON
    /// object).
    pub async fn set_recipients(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<NotificationRecipient>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<NotificationRecipient>>(
                Method::PUT,
                "/web/api/v2.1/settings/recipients",
                None,
                Some(body),
            )
            .await?)
    }

    /// `DELETE /web/api/v2.1/settings/recipients/{recipient_id}` — Delete
    /// Notification Recipient.
    ///
    /// Delete a notification recipient by ID. To get the IDs of recipients, run
    /// "recipients" (see Get Notification Recipients).
    ///
    /// `recipient_id`: Recipient ID. Example: `"225494730938493804"`.
    /// (required)
    pub async fn delete_recipient(
        &self,
        recipient_id: impl Into<String>,
    ) -> Result<Response<SuccessResponse>, Error> {
        let path = format!(
            "/web/api/v2.1/settings/recipients/{}",
            recipient_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<(), Response<SuccessResponse>>(Method::DELETE, &path, None, None)
            .await?)
    }

    // -- SMS (DEPRECATED) ----------------------------------------------------

    /// `GET /web/api/v2.1/settings/sms` — Get SMS Settings.
    ///
    /// [DEPRECATED] Gets the site's SMS settings.
    pub async fn get_sms(&self, query: &ScopeQuery) -> Result<Response<SmsSettings>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/settings/sms", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/settings/sms` — Set SMS Settings.
    ///
    /// [DEPRECATED] Set SMS settings.
    ///
    /// `body`: `settings_SmsSettingsPutSchema` (freeform JSON object).
    pub async fn set_sms(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<SmsSettings>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<SmsSettings>>(
                Method::PUT,
                "/web/api/v2.1/settings/sms",
                None,
                Some(body),
            )
            .await?)
    }

    // -- SMTP ----------------------------------------------------------------

    /// `GET /web/api/v2.1/settings/smtp` — Get SMTP Settings.
    ///
    /// Get the SMTP server configuration of the given Sites (to get the IDs, run
    /// "sites") or Accounts ("accounts"). The SMTP integration is required to
    /// send notifications by email.
    pub async fn get_smtp(&self, query: &ScopeQuery) -> Result<Response<SmtpSettings>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/settings/smtp", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/settings/smtp` — Set SMTP Settings.
    ///
    /// Change the SMTP server configuration for the given Sites or Accounts. Use
    /// this command to integrate a different SMTP server, which is required to
    /// send notifications by email.
    ///
    /// `body`: `settings_SmtpSettingsPutSchema` (freeform JSON object).
    pub async fn set_smtp(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<SmtpSettings>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<SmtpSettings>>(
                Method::PUT,
                "/web/api/v2.1/settings/smtp",
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/settings/smtp/test` — Test SMTP Settings.
    ///
    /// Test SMTP settings between the Management and the SMTP server. This
    /// integration is required if you use email notifications.
    ///
    /// `body`: `settings_SmtpSettingsTestSchema` (freeform JSON object).
    pub async fn test_smtp(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<TestSmtpSettings>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/settings/smtp/test", body)
            .await?)
    }

    // -- SSO -----------------------------------------------------------------

    /// `GET /web/api/v2.1/settings/sso` — Get SSO Settings.
    ///
    /// Get the Single Sign-On configuration for the given Sites (to get the IDs,
    /// run "sites") or Accounts ("accounts").
    pub async fn get_sso(&self, query: &ScopeQuery) -> Result<Response<SsoSettings>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/settings/sso", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/settings/sso` — Set SSO Settings.
    ///
    /// Change the Single Sign-On configuration for the given Sites (to get the
    /// IDs, run "sites") or Accounts ("accounts"). The Management supports SAML
    /// 2.0 and will integrate with SAML 2.0 compliant SSO providers.
    /// SentinelOne Technical Support can help you with issues related to the
    /// provider we tested: Okta. To use a different ID provider, see the
    /// provider documentation and support. For requirements and best practices
    /// of Okta integration, see
    /// https://support.sentinelone.com/hc/en-us/articles/360004195714.
    ///
    /// `body`: `settings_SsoSettingsPutSchema` (freeform JSON object).
    pub async fn set_sso(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<SsoSettings>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<SsoSettings>>(
                Method::PUT,
                "/web/api/v2.1/settings/sso",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/settings/sso/sp-cert` — Get SSO Service Provider
    /// Certificate.
    ///
    /// Get the Service Provider Certificate for the Single Sign-On
    /// configuration for the given scope.
    pub async fn get_sso_sp_cert(
        &self,
        query: &ScopeQuery,
    ) -> Result<Response<SsoSpCertificate>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/settings/sso/sp-cert", q)
            .await?)
    }

    /// `GET /web/api/v2.1/settings/sso/sp-cert/download` — Download SSO Service
    /// Provider Certificate.
    ///
    /// Download the Service Provider Certificate for the Single Sign-On
    /// configuration for the given scope. The response is the raw certificate
    /// payload (no documented JSON envelope), returned as a `serde_json::Value`.
    pub async fn download_sso_sp_cert(
        &self,
        query: &ScopeQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/settings/sso/sp-cert/download", q)
            .await?)
    }

    /// `POST /web/api/v2.1/settings/sso/test` — Test SSO Settings.
    ///
    /// Test Single Sign-On settings.
    ///
    /// `body`: `settings_SsoSettingsPutSchema` (freeform JSON object).
    pub async fn test_sso(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<SsoTest>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/settings/sso/test", body)
            .await?)
    }

    // -- Syslog --------------------------------------------------------------

    /// `GET /web/api/v2.1/settings/syslog` — Get Syslog Settings.
    ///
    /// Get the configuration of the syslog server integrated with the given
    /// Sites (to get the IDs, run "sites") or Accounts ("accounts").
    pub async fn get_syslog(
        &self,
        query: &ScopeQuery,
    ) -> Result<Response<SyslogSettings>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/settings/syslog", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/settings/syslog` — Set Syslog Settings.
    ///
    /// Change the configuration of the syslog server of the given Sites (to get
    /// the IDs, run "sites") or Accounts ("accounts"). Use this command to send
    /// notifications to a different syslog server. Best Practice: Get Syslog
    /// Settings before you run this command.
    ///
    /// `body`: `settings_SyslogSettingsPutSchema` (freeform JSON object).
    pub async fn set_syslog(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<SyslogSettings>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<SyslogSettings>>(
                Method::PUT,
                "/web/api/v2.1/settings/syslog",
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/settings/syslog/test` — Test Syslog Settings.
    ///
    /// Test Syslog settings. The Management tests the connection to the Syslog
    /// server.
    ///
    /// `body`: `settings_SyslogSettingsPutSchema` (freeform JSON object).
    pub async fn test_syslog(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<TestSyslogSettings>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/settings/syslog/test", body)
            .await?)
    }
}
