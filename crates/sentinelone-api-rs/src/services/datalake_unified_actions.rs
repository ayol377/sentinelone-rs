//! `Datalake Unified Actions` tag.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::datalake_unified_actions::AvailableActionsResponse;
use crate::pagination::Response;

/// `Datalake Unified Actions` tag.
///
/// Endpoints for discovering and performing "unified actions" on assets/entities
/// surfaced by the XDR Datalake.
pub struct DatalakeUnifiedActionsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// =====================================================================
// Query params
// =====================================================================

/// Query params for `POST /web/api/v2.1/xdr/action-controller/fetch-surface-ids`.
///
/// All scoping params are optional comma-separated id lists.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchSurfaceIdsQuery {
    /// List of Account IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl FetchSurfaceIdsQuery {
    /// List of Account IDs to filter by. Joined by comma.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","),
        );
        self
    }
    /// List of Site IDs to filter by. Joined by comma.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
    /// List of Group IDs to filter by. Joined by comma.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/action-controller/fetch-unified-actions`.
///
/// All scoping params are optional comma-separated id lists.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchUnifiedActionsQuery {
    /// List of Account IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl FetchUnifiedActionsQuery {
    /// List of Account IDs to filter by. Joined by comma.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","),
        );
        self
    }
    /// List of Site IDs to filter by. Joined by comma.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
    /// List of Group IDs to filter by. Joined by comma.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/action-controller/perform-unified-action`.
///
/// All scoping params are optional comma-separated id lists.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformUnifiedActionQuery {
    /// List of Account IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl PerformUnifiedActionQuery {
    /// List of Account IDs to filter by. Joined by comma.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","),
        );
        self
    }
    /// List of Site IDs to filter by. Joined by comma.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
    /// List of Group IDs to filter by. Joined by comma.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
}

/// Query params for
/// `POST /web/api/v2.1/xdr/action-controller/perform-unified-action/notify`.
///
/// All scoping params are optional comma-separated id lists.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformUnifiedActionNotifyQuery {
    /// List of Account IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl PerformUnifiedActionNotifyQuery {
    /// List of Account IDs to filter by. Joined by comma.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","),
        );
        self
    }
    /// List of Site IDs to filter by. Joined by comma.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
    /// List of Group IDs to filter by. Joined by comma.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(ids.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
}

// =====================================================================
// Body params
// =====================================================================

/// Request body for
/// `POST /web/api/v2.1/xdr/action-controller/fetch-surface-ids`
/// (`v2_1.action_controller.schemas_FetchSurfaceIdsRequestSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchSurfaceIdsBody {
    /// List of selected entity ids, keyed by entity type
    /// (`{ "<entityType>": ["id", ...] }`). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<serde_json::Value>,
    /// List of entity ids to exclude from select_all, keyed by entity type.
    /// Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<serde_json::Value>,
    /// Freeform action payload. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    /// Action path. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_path: Option<String>,
}

/// Request body for
/// `POST /web/api/v2.1/xdr/action-controller/fetch-unified-actions`
/// (`v2_1.action_controller.schemas_AffectedEntitiesSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchUnifiedActionsBody {
    /// List of selected entity ids, keyed by entity type
    /// (`{ "<entityType>": ["id", ...] }`). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<serde_json::Value>,
    /// List of entity ids to exclude from select_all, keyed by entity type.
    /// Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<serde_json::Value>,
}

/// Request body for
/// `POST /web/api/v2.1/xdr/action-controller/perform-unified-action`
/// (`v2_1.action_controller.schemas_PerformActionRequestSchema`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformUnifiedActionBody {
    /// Action path. Required.
    pub action_path: String,
    /// List of selected entity ids, keyed by entity type
    /// (`{ "<entityType>": ["id", ...] }`). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<serde_json::Value>,
    /// List of entity ids to exclude from select_all, keyed by entity type.
    /// Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<serde_json::Value>,
    /// Freeform action payload. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

impl PerformUnifiedActionBody {
    /// Construct with the required `actionPath`.
    pub fn new(action_path: impl Into<String>) -> Self {
        Self {
            action_path: action_path.into(),
            id_in: None,
            id_nin: None,
            payload: None,
        }
    }
}

/// Request body for
/// `POST /web/api/v2.1/xdr/action-controller/perform-unified-action/notify`
/// (`v2_1.action_controller.schemas_PerformActionNotificationSchema`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformUnifiedActionNotifyBody {
    /// Action path. Required.
    pub action_path: String,
    /// Notify only. Required. Allowed value (spec `enum`): `true`.
    pub notify_only: bool,
    /// List of selected entity ids, keyed by entity type
    /// (`{ "<entityType>": ["id", ...] }`). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<serde_json::Value>,
    /// List of entity ids to exclude from select_all, keyed by entity type.
    /// Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<serde_json::Value>,
    /// Freeform action payload. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

impl PerformUnifiedActionNotifyBody {
    /// Construct with the required `actionPath`; `notifyOnly` is fixed to `true`
    /// per the spec.
    pub fn new(action_path: impl Into<String>) -> Self {
        Self {
            action_path: action_path.into(),
            notify_only: true,
            id_in: None,
            id_nin: None,
            payload: None,
        }
    }
}

// =====================================================================
// Service methods
// =====================================================================

impl DatalakeUnifiedActionsService<'_> {
    /// `POST /web/api/v2.1/xdr/action-controller/fetch-surface-ids` — Fetch
    /// surface ids in case of select all with filters.
    ///
    /// Fetch surface ids in case of select all with filters.
    pub async fn fetch_surface_ids(
        &self,
        query: &FetchSurfaceIdsQuery,
        body: &FetchSurfaceIdsBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<FetchSurfaceIdsBody, Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/xdr/action-controller/fetch-surface-ids",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/action-controller/fetch-unified-actions` — Get
    /// Available Actions by Asset/Entity Type.
    ///
    /// Get Available Actions by Asset/Entity Type.
    pub async fn fetch_unified_actions(
        &self,
        query: &FetchUnifiedActionsQuery,
        body: &FetchUnifiedActionsBody,
    ) -> Result<Response<AvailableActionsResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<FetchUnifiedActionsBody, Response<AvailableActionsResponse>>(
                Method::POST,
                "/web/api/v2.1/xdr/action-controller/fetch-unified-actions",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/action-controller/perform-unified-action` —
    /// Perform an Action on selected assets/entities.
    ///
    /// Perform an Action on selected assets/entities.
    pub async fn perform_unified_action(
        &self,
        query: &PerformUnifiedActionQuery,
        body: &PerformUnifiedActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<PerformUnifiedActionBody, Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/xdr/action-controller/perform-unified-action",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/action-controller/perform-unified-action/notify`
    /// — Internal api only to notify action was triggered without actually
    /// performing it.
    ///
    /// Internal api only to notify action was triggered without actually
    /// performing it.
    pub async fn perform_unified_action_notify(
        &self,
        query: &PerformUnifiedActionNotifyQuery,
        body: &PerformUnifiedActionNotifyBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<PerformUnifiedActionNotifyBody, Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/xdr/action-controller/perform-unified-action/notify",
                q,
                Some(body),
            )
            .await?)
    }
}
