//! Service for the `Accounts` tag — listing and managing customer Accounts.
//!
//! Accounts are created by a Global User or by SentinelOne. Each Account
//! contains Sites, which can inherit assets and settings, and has one or more
//! SKUs that you assign to the Sites.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::Account;
use crate::pagination::{Paginated, Response};

/// `Accounts` tag — listing customer Accounts.
pub struct AccountsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Join an iterator of string-like items into a comma-separated string.
///
/// Array query params are serialized comma-joined, as the API expects.
fn join_csv<I, S>(items: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    items
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

/// Query params for `GET /web/api/v2.1/accounts` (Get Accounts).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `150`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `10`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: `"YWdlbnRfaWQ6NTgwMjkzODE="`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items will be returned, without any of
    /// the actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, the total number of items will not be calculated, which speeds
    /// up execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: `"id"`. Optional.
    ///
    /// Allowed values: `id`, `name`, `totalLicenses`, `expiration`,
    /// `accountType`, `state`, `createdAt`, `updatedAt`, `activeLicenses`,
    /// `activeAgents`, `numberOfSites`, `usageType`, `billingMode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: `"asc"`. Optional.
    ///
    /// Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// A list of Account IDs (comma-separated).
    /// Example: `"225494730938493804,225494730938493915"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of Account IDs to search for (comma-separated).
    /// Example: `"225494730938493804,225494730938493915"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Full text search for fields: name. (Note: on single-Account Consoles,
    /// the Account name will not be matched.) Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Name. Example: `"My Account"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Is default. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Account type. Example: `"Trial"`. Optional.
    ///
    /// Allowed values: `Trial`, `Paid`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Expiration. Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// Total licenses. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_licenses: Option<i64>,
    /// Active licenses. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_licenses: Option<i64>,
    /// Sku. Example: `"core"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Module. Example: `"star,rso"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// Timestamp of Account creation.
    /// Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Timestamp of last update.
    /// Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Account state. Example: `"active"`. Optional.
    ///
    /// Allowed values: `active`, `expired`, `deleted`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Filter by state, such as active or expired (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<String>,
    /// List of states to not filter (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_nin: Option<String>,
    /// Filter the list of Accounts for those that support this feature
    /// (comma-separated). Example: `"firewall-control"`. Optional.
    ///
    /// Allowed values: `firewall-control`, `device-control`, `ioc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    /// Usage type. Example: `"customer"`. Optional.
    ///
    /// Allowed values: `customer`, `mssp`, `ir`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
    /// Billing mode. Example: `"subscription"`. Optional.
    ///
    /// Allowed values: `subscription`, `consumption`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    /// Free-text filter by account name (supports multiple values,
    /// comma-separated). Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name__contains: Option<String>,
}

impl AccountsQuery {
    /// Skip first number of items (0-1000). Example: `150`.
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Limit number of returned items (1-1000). Example: `10`.
    pub fn limit(mut self, n: u32) -> Self {
        self.limit = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// If true, only the total number of items will be returned.
    pub fn count_only(mut self, b: bool) -> Self {
        self.count_only = Some(b);
        self
    }
    /// If true, the total number of items will not be calculated.
    pub fn skip_count(mut self, b: bool) -> Self {
        self.skip_count = Some(b);
        self
    }
    /// The column to sort the results by. Allowed: `id`, `name`,
    /// `totalLicenses`, `expiration`, `accountType`, `state`, `createdAt`,
    /// `updatedAt`, `activeLicenses`, `activeAgents`, `numberOfSites`,
    /// `usageType`, `billingMode`.
    pub fn sort_by(mut self, s: impl Into<String>) -> Self {
        self.sort_by = Some(s.into());
        self
    }
    /// Sort direction. Allowed: `asc`, `desc`.
    pub fn sort_order(mut self, s: impl Into<String>) -> Self {
        self.sort_order = Some(s.into());
        self
    }
    /// A list of Account IDs.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to search for.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// Full text search for fields: name.
    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.query = Some(q.into());
        self
    }
    /// Name. Example: `"My Account"`.
    pub fn name(mut self, n: impl Into<String>) -> Self {
        self.name = Some(n.into());
        self
    }
    /// Is default.
    pub fn is_default(mut self, b: bool) -> Self {
        self.is_default = Some(b);
        self
    }
    /// Account type. Allowed: `Trial`, `Paid`.
    pub fn account_type(mut self, s: impl Into<String>) -> Self {
        self.account_type = Some(s.into());
        self
    }
    /// Expiration. Example: `"2018-02-27T04:49:26.257525Z"`.
    pub fn expiration(mut self, s: impl Into<String>) -> Self {
        self.expiration = Some(s.into());
        self
    }
    /// Total licenses.
    pub fn total_licenses(mut self, n: i64) -> Self {
        self.total_licenses = Some(n);
        self
    }
    /// Active licenses.
    pub fn active_licenses(mut self, n: i64) -> Self {
        self.active_licenses = Some(n);
        self
    }
    /// Sku. Example: `"core"`.
    pub fn sku(mut self, s: impl Into<String>) -> Self {
        self.sku = Some(s.into());
        self
    }
    /// Module. Example: `"star,rso"`.
    pub fn module(mut self, s: impl Into<String>) -> Self {
        self.module = Some(s.into());
        self
    }
    /// Timestamp of Account creation.
    pub fn created_at(mut self, s: impl Into<String>) -> Self {
        self.created_at = Some(s.into());
        self
    }
    /// Timestamp of last update.
    pub fn updated_at(mut self, s: impl Into<String>) -> Self {
        self.updated_at = Some(s.into());
        self
    }
    /// Account state. Allowed: `active`, `expired`, `deleted`.
    pub fn state(mut self, s: impl Into<String>) -> Self {
        self.state = Some(s.into());
        self
    }
    /// Filter by state, such as active or expired.
    pub fn states<I, S>(mut self, states: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.states = Some(join_csv(states));
        self
    }
    /// List of states to not filter.
    pub fn states_nin<I, S>(mut self, states: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.states_nin = Some(join_csv(states));
        self
    }
    /// Filter for Accounts that support these features. Allowed:
    /// `firewall-control`, `device-control`, `ioc`.
    pub fn features<I, S>(mut self, features: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.features = Some(join_csv(features));
        self
    }
    /// Usage type. Allowed: `customer`, `mssp`, `ir`.
    pub fn usage_type(mut self, s: impl Into<String>) -> Self {
        self.usage_type = Some(s.into());
        self
    }
    /// Billing mode. Allowed: `subscription`, `consumption`.
    pub fn billing_mode(mut self, s: impl Into<String>) -> Self {
        self.billing_mode = Some(s.into());
        self
    }
    /// Free-text filter by account name (supports multiple values).
    pub fn name_contains<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name__contains = Some(join_csv(names));
        self
    }
}

/// Query params for `GET /web/api/v2.1/export/accounts` (Export Accounts).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsExportQuery {
    /// A list of Account IDs (comma-separated).
    /// Example: `"225494730938493804,225494730938493915"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of Account IDs to search for (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Full text search for fields: name. (Note: on single-Account Consoles,
    /// the Account name will not be matched.) Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Name. Example: `"My Account"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Is default. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Account type. Example: `"Trial"`. Optional.
    ///
    /// Allowed values: `Trial`, `Paid`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Expiration. Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// Total licenses. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_licenses: Option<i64>,
    /// Active licenses. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_licenses: Option<i64>,
    /// Sku. Example: `"core"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Module. Example: `"star,rso"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// Timestamp of Account creation. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Timestamp of last update. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Account state. Example: `"active"`. Optional.
    ///
    /// Allowed values: `active`, `expired`, `deleted`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Filter by state, such as active or expired (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<String>,
    /// List of states to not filter (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_nin: Option<String>,
    /// Filter for Accounts that support this feature (comma-separated).
    /// Example: `"firewall-control"`. Optional.
    ///
    /// Allowed values: `firewall-control`, `device-control`, `ioc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    /// Usage type. Example: `"customer"`. Optional.
    ///
    /// Allowed values: `customer`, `mssp`, `ir`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
    /// Billing mode. Example: `"subscription"`. Optional.
    ///
    /// Allowed values: `subscription`, `consumption`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    /// Free-text filter by account name (supports multiple values,
    /// comma-separated). Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name__contains: Option<String>,
}

impl AccountsExportQuery {
    /// A list of Account IDs.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to search for.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// Full text search for fields: name.
    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.query = Some(q.into());
        self
    }
    /// Name. Example: `"My Account"`.
    pub fn name(mut self, n: impl Into<String>) -> Self {
        self.name = Some(n.into());
        self
    }
    /// Is default.
    pub fn is_default(mut self, b: bool) -> Self {
        self.is_default = Some(b);
        self
    }
    /// Account type. Allowed: `Trial`, `Paid`.
    pub fn account_type(mut self, s: impl Into<String>) -> Self {
        self.account_type = Some(s.into());
        self
    }
    /// Expiration. Example: `"2018-02-27T04:49:26.257525Z"`.
    pub fn expiration(mut self, s: impl Into<String>) -> Self {
        self.expiration = Some(s.into());
        self
    }
    /// Total licenses.
    pub fn total_licenses(mut self, n: i64) -> Self {
        self.total_licenses = Some(n);
        self
    }
    /// Active licenses.
    pub fn active_licenses(mut self, n: i64) -> Self {
        self.active_licenses = Some(n);
        self
    }
    /// Sku. Example: `"core"`.
    pub fn sku(mut self, s: impl Into<String>) -> Self {
        self.sku = Some(s.into());
        self
    }
    /// Module. Example: `"star,rso"`.
    pub fn module(mut self, s: impl Into<String>) -> Self {
        self.module = Some(s.into());
        self
    }
    /// Timestamp of Account creation.
    pub fn created_at(mut self, s: impl Into<String>) -> Self {
        self.created_at = Some(s.into());
        self
    }
    /// Timestamp of last update.
    pub fn updated_at(mut self, s: impl Into<String>) -> Self {
        self.updated_at = Some(s.into());
        self
    }
    /// Account state. Allowed: `active`, `expired`, `deleted`.
    pub fn state(mut self, s: impl Into<String>) -> Self {
        self.state = Some(s.into());
        self
    }
    /// Filter by state, such as active or expired.
    pub fn states<I, S>(mut self, states: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.states = Some(join_csv(states));
        self
    }
    /// List of states to not filter.
    pub fn states_nin<I, S>(mut self, states: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.states_nin = Some(join_csv(states));
        self
    }
    /// Filter for Accounts that support these features. Allowed:
    /// `firewall-control`, `device-control`, `ioc`.
    pub fn features<I, S>(mut self, features: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.features = Some(join_csv(features));
        self
    }
    /// Usage type. Allowed: `customer`, `mssp`, `ir`.
    pub fn usage_type(mut self, s: impl Into<String>) -> Self {
        self.usage_type = Some(s.into());
        self
    }
    /// Billing mode. Allowed: `subscription`, `consumption`.
    pub fn billing_mode(mut self, s: impl Into<String>) -> Self {
        self.billing_mode = Some(s.into());
        self
    }
    /// Free-text filter by account name (supports multiple values).
    pub fn name_contains<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name__contains = Some(join_csv(names));
        self
    }
}

/// Request body for `POST /web/api/v2.1/accounts` (Create Account).
///
/// Wraps the `accounts.schemas_PostAccountSchema` definition, whose `data`
/// member is a free-form object describing the Account to create.
#[derive(Debug, Clone, Serialize)]
pub struct CreateAccountBody {
    /// Data. The Account fields to create (free-form object per the spec).
    pub data: serde_json::Value,
}

/// Request body for `PUT /web/api/v2.1/accounts/{account_id}` (Update Account).
///
/// Wraps the `accounts.schemas_AccountPutSchema` definition, whose `data`
/// member is a free-form object describing the Account changes.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateAccountBody {
    /// Data. The Account fields to change (free-form object per the spec).
    pub data: serde_json::Value,
}

/// Request body for `PUT /web/api/v2.1/accounts/{account_id}/reactivate`
/// (Reactivate Account).
///
/// Wraps the `accounts.schemas_ReactivateAccountSchema` definition, whose
/// `data` member is a free-form object.
#[derive(Debug, Clone, Serialize)]
pub struct ReactivateAccountBody {
    /// Data. Reactivation parameters (free-form object per the spec).
    pub data: serde_json::Value,
}

/// Request body for
/// `POST /web/api/v2.1/accounts/{account_id}/uninstall-password/generate`
/// (Generate/Regenerate Uninstall Password).
///
/// Wraps the `accounts.schemas_UninstallPasswordGenerateRequestSchema`
/// definition, whose `data` member is a free-form object.
#[derive(Debug, Clone, Serialize)]
pub struct UninstallPasswordGenerateBody {
    /// Data. Uninstall password generation parameters (free-form object).
    pub data: serde_json::Value,
}

/// Request body for `PUT /web/api/v2.1/accounts/{account_id}/revert-policy`
/// (Revert Policy).
///
/// Wraps the `policies_schemas_RevertPolicySchema` definition. The `data`
/// member is an optional free-form object.
#[derive(Debug, Clone, Default, Serialize)]
pub struct RevertPolicyBody {
    /// Data. Revert-policy parameters (free-form object per the spec). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl AccountsService<'_> {
    /// Get Accounts.
    ///
    /// Get the Accounts, and their data, that match the filter. This command
    /// gives the Account IDs, which other commands require.
    ///
    /// `GET /web/api/v2.1/accounts`
    pub async fn list(&self, query: &AccountsQuery) -> Result<Paginated<Account>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/accounts", q).await?)
    }

    /// Create Account.
    ///
    /// Create a new Account. This command requires Global permissions and an
    /// MSSP deployment. An Account is a logical segment with permissions to
    /// configure features for specific Sites.
    ///
    /// `POST /web/api/v2.1/accounts`
    pub async fn create(&self, body: &CreateAccountBody) -> Result<Response<Account>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/accounts", body).await?)
    }

    /// Get Account by ID.
    ///
    /// Get Account data from a given Account ID. To get an Account ID, run
    /// "accounts".
    ///
    /// `account_id`: Account ID. Example: `"225494730938493804"`.
    ///
    /// `GET /web/api/v2.1/accounts/{account_id}`
    pub async fn get(&self, account_id: &str) -> Result<Response<Account>, Error> {
        let path = format!("/web/api/v2.1/accounts/{account_id}");
        Ok(self.client.http().get(&path, None).await?)
    }

    /// Update Account.
    ///
    /// Change the data of an Account. This command requires a Global user or an
    /// Account user and Admin role. Use this command to change the name, ID,
    /// SKUs and how they are distributed among Sites and Agents, and more.
    ///
    /// `account_id`: Account ID. Example: `"225494730938493804"`.
    ///
    /// `PUT /web/api/v2.1/accounts/{account_id}`
    pub async fn update(
        &self,
        account_id: &str,
        body: &UpdateAccountBody,
    ) -> Result<Response<Account>, Error> {
        let path = format!("/web/api/v2.1/accounts/{account_id}");
        Ok(self
            .client
            .http()
            .request_json::<UpdateAccountBody, Response<Account>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// Expire an Account.
    ///
    /// Expire an Account immediately. The user must have Global access or
    /// Account access with permissions for the Account.
    ///
    /// `account_id`: Account ID. Example: `"225494730938493804"`.
    ///
    /// `POST /web/api/v2.1/accounts/{account_id}/expire-now`
    pub async fn expire_now(&self, account_id: &str) -> Result<Response<Account>, Error> {
        let path = format!("/web/api/v2.1/accounts/{account_id}/expire-now");
        Ok(self
            .client
            .http()
            .request_json::<(), Response<Account>>(Method::POST, &path, None, None)
            .await?)
    }

    /// Reactivate Account.
    ///
    /// Reactivate an expired Account. This command requires a Global user or
    /// Support.
    ///
    /// `account_id`: Account ID. Example: `"225494730938493804"`.
    ///
    /// `PUT /web/api/v2.1/accounts/{account_id}/reactivate`
    pub async fn reactivate(
        &self,
        account_id: &str,
        body: &ReactivateAccountBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/accounts/{account_id}/reactivate");
        Ok(self
            .client
            .http()
            .request_json::<ReactivateAccountBody, Response<serde_json::Value>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// Revert Policy.
    ///
    /// Revert the Account policy to the default Global policy. The policy of the
    /// Account is based on the default Global policy and is enforced by all
    /// endpoints in the Sites and Groups of the Account.
    ///
    /// `account_id`: Account ID. Example: `"225494730938493804"`.
    ///
    /// `PUT /web/api/v2.1/accounts/{account_id}/revert-policy`
    pub async fn revert_policy(
        &self,
        account_id: &str,
        body: &RevertPolicyBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/accounts/{account_id}/revert-policy");
        Ok(self
            .client
            .http()
            .request_json::<RevertPolicyBody, Response<serde_json::Value>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// Generate/Regenerate Uninstall Password.
    ///
    /// Set a new account-level uninstall password. You can uninstall all Agents
    /// of one Account with one command that requires a password. Applicable on
    /// Windows (versions 4.4+) and Linux (versions 21.7+) Agents.
    ///
    /// `account_id`: Account ID. Example: `"225494730938493804"`.
    ///
    /// `POST /web/api/v2.1/accounts/{account_id}/uninstall-password/generate`
    pub async fn generate_uninstall_password(
        &self,
        account_id: &str,
        body: &UninstallPasswordGenerateBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/accounts/{account_id}/uninstall-password/generate");
        Ok(self.client.http().post(&path, body).await?)
    }

    /// Get Uninstall Password Metadata.
    ///
    /// Get the uninstall password metadata, such as which user created and
    /// revoked it and when. The response data is a free-form object per the
    /// spec.
    ///
    /// `account_id`: Account ID. Example: `"225494730938493804"`.
    ///
    /// `GET /web/api/v2.1/accounts/{account_id}/uninstall-password/metadata`
    pub async fn get_uninstall_password_metadata(
        &self,
        account_id: &str,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/accounts/{account_id}/uninstall-password/metadata");
        Ok(self.client.http().get(&path, None).await?)
    }

    /// Revoke Uninstall Password.
    ///
    /// Delete the account-level uninstall password. If you do not delete it, you
    /// or another Console user can mistakenly use the Account passphrase (and
    /// uninstall all Agents) when you mean to uninstall one Agent.
    ///
    /// `account_id`: Account ID. Example: `"225494730938493804"`.
    ///
    /// `POST /web/api/v2.1/accounts/{account_id}/uninstall-password/revoke`
    pub async fn revoke_uninstall_password(
        &self,
        account_id: &str,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/accounts/{account_id}/uninstall-password/revoke");
        Ok(self
            .client
            .http()
            .request_json::<(), Response<serde_json::Value>>(Method::POST, &path, None, None)
            .await?)
    }

    /// Get Uninstall Password.
    ///
    /// Get the uninstall password to uninstall several Agents of one Account
    /// with one command. The response data is a free-form object per the spec.
    ///
    /// `account_id`: Account ID. Example: `"225494730938493804"`.
    ///
    /// `GET /web/api/v2.1/accounts/{account_id}/uninstall-password/view`
    pub async fn get_uninstall_password(
        &self,
        account_id: &str,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/accounts/{account_id}/uninstall-password/view");
        Ok(self.client.http().get(&path, None).await?)
    }

    /// Export Accounts.
    ///
    /// Export Accounts data to a CSV, for Accounts that match the filter. The
    /// response is an export payload rather than a typed envelope, so it is
    /// returned as a freeform JSON value.
    ///
    /// `GET /web/api/v2.1/export/accounts`
    pub async fn export(
        &self,
        query: &AccountsExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/export/accounts", q)
            .await?)
    }
}
