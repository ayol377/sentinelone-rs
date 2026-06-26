use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::default_reports::{InsightData, Report, ReportTask, SuccessResponse};
use crate::pagination::{Paginated, Response};

/// `Default Reports` tag.
///
/// Default Reports related endpoints. Requires Reports permission and an
/// appropriate license.
pub struct DefaultReportsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/report-tasks`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReportTasksQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `150`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `10`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
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
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `sites`, `frequency`, `day`, `scope`, `createdAt`, `scheduleType`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Id. Example: `225494730938493804`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Created at lte. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Created at gte. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Scope. Allowed values: `group`, `site`, `account`, `tenant`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Frequency. Allowed values: `manually`, `weekly`, `monthly`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// Day. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,
    /// Creator id. Example: `225494730938493804`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// Creator name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,
    /// Query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Report type. Allowed values: `manually`, `scheduled`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<String>,
    /// Id in. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
}

impl ListReportTasksQuery {
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
    /// If true, only the total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, the total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `sites`, `frequency`, `day`, `scope`, `createdAt`, `scheduleType`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// Id.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    /// Created at lte (date-time string).
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Created at gte (date-time string).
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// Scope. Allowed values: `group`, `site`, `account`, `tenant`.
    pub fn scope(mut self, v: impl Into<String>) -> Self {
        self.scope = Some(v.into());
        self
    }
    /// Frequency. Allowed values: `manually`, `weekly`, `monthly`.
    pub fn frequency(mut self, v: impl Into<String>) -> Self {
        self.frequency = Some(v.into());
        self
    }
    /// Day.
    pub fn day(mut self, v: impl Into<String>) -> Self {
        self.day = Some(v.into());
        self
    }
    /// Creator id.
    pub fn creator_id(mut self, v: impl Into<String>) -> Self {
        self.creator_id = Some(v.into());
        self
    }
    /// Creator name.
    pub fn creator_name(mut self, v: impl Into<String>) -> Self {
        self.creator_name = Some(v.into());
        self
    }
    /// Query.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Report type. Allowed values: `manually`, `scheduled`.
    pub fn schedule_type(mut self, v: impl Into<String>) -> Self {
        self.schedule_type = Some(v.into());
        self
    }
    /// Id in.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `GET /web/api/v2.1/reports`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReportsQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `150`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `10`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
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
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `sites`, `frequency`, `interval`, `scope`, `createdAt`, `scheduleType`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Id. Example: `225494730938493804`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Created at lte. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Created at gte. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Scope. Allowed values: `group`, `site`, `account`, `tenant`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Frequency. Allowed values: `manually`, `weekly`, `monthly`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// Interval. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// From date. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    /// To date. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    /// Query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Report type. Allowed values: `manually`, `scheduled`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<String>,
    /// Task id. Example: `225494730938493804`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// Id in. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
}

impl ListReportsQuery {
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
    /// If true, only the total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, the total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `sites`, `frequency`, `interval`, `scope`, `createdAt`, `scheduleType`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// Id.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    /// Created at lte (date-time string).
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Created at gte (date-time string).
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// Scope. Allowed values: `group`, `site`, `account`, `tenant`.
    pub fn scope(mut self, v: impl Into<String>) -> Self {
        self.scope = Some(v.into());
        self
    }
    /// Frequency. Allowed values: `manually`, `weekly`, `monthly`.
    pub fn frequency(mut self, v: impl Into<String>) -> Self {
        self.frequency = Some(v.into());
        self
    }
    /// Interval.
    pub fn interval(mut self, v: impl Into<String>) -> Self {
        self.interval = Some(v.into());
        self
    }
    /// From date (date-time string).
    pub fn from_date(mut self, v: impl Into<String>) -> Self {
        self.from_date = Some(v.into());
        self
    }
    /// To date (date-time string).
    pub fn to_date(mut self, v: impl Into<String>) -> Self {
        self.to_date = Some(v.into());
        self
    }
    /// Query.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Report type. Allowed values: `manually`, `scheduled`.
    pub fn schedule_type(mut self, v: impl Into<String>) -> Self {
        self.schedule_type = Some(v.into());
        self
    }
    /// Task id.
    pub fn task_id(mut self, v: impl Into<String>) -> Self {
        self.task_id = Some(v.into());
        self
    }
    /// Id in.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `GET /web/api/v2.1/reports/insights/types`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInsightTypesQuery {
    /// List of Site IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Force update. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_update: Option<bool>,
}

impl ListInsightTypesQuery {
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// Force update.
    pub fn force_update(mut self, v: bool) -> Self {
        self.force_update = Some(v);
        self
    }
}

/// `data` object for `POST /web/api/v2.1/report-tasks`.
///
/// `insightTypes`, `name`, and `scheduleType` are required by the spec; the
/// rest are optional.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReportTaskData {
    /// Name of the report. Required.
    pub name: String,
    /// Schedule type. Allowed values: `manually`, `scheduled`. Required.
    pub schedule_type: String,
    /// List of reports (free-form Insight type objects). Required.
    pub insight_types: Vec<serde_json::Value>,
    /// From date (date-time string). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    /// To date (date-time string). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    /// Report Period. Allowed values: `manually`, `weekly`, `monthly`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// Day of the week for a weekly report. Allowed values: `sunday`, `monday`,
    /// `tuesday`, `wednesday`, `thursday`, `friday`, `saturday`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,
    /// Type of attachments for the report. Must be supplied if recipients are
    /// added. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_types: Option<Vec<String>>,
    /// List of recipients for the report. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    /// If is trend is true then the period will be last month. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_trend: Option<bool>,
}

/// `filter` object for `POST /web/api/v2.1/report-tasks`.
///
/// The whole `filter` object is required by the spec, but each member is
/// optional.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReportTaskFilter {
    /// List of Site IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Scope. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// Request body for `POST /web/api/v2.1/report-tasks`
/// (`reports_ReportTasksPostSchema`).
///
/// Both `data` and `filter` are required by the spec.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReportTaskBody {
    /// Data. Required.
    pub data: CreateReportTaskData,
    /// Filter. Required.
    pub filter: CreateReportTaskFilter,
}

/// `data` object for `PUT /web/api/v2.1/report-tasks/{task_id}`.
///
/// `name` is required by the spec; the rest are optional.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReportTaskData {
    /// Name of the report. Required.
    pub name: String,
    /// Database id of the task. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Type of documents for the report. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_types: Option<Vec<String>>,
    /// List of recipients for the report. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
}

/// Request body for `PUT /web/api/v2.1/report-tasks/{task_id}`
/// (`reports_ReportTasksPutSchema`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReportTaskBody {
    /// Data. Optional in the spec (no top-level `required`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UpdateReportTaskData>,
}

/// `filter` object for `POST /web/api/v2.1/reports/delete-reports`
/// (`reports_ReportDeleteSchema`).
///
/// Every member is optional. Array members are sent as JSON arrays.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteReportsFilter {
    /// List of Site IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Created at lte (date-time string). Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Created at gte (date-time string). Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Scope. Allowed values: `group`, `site`, `account`, `tenant`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Frequency. Allowed values: `manually`, `weekly`, `monthly`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// Interval. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// From date (date-time string). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    /// To date (date-time string). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    /// Query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Report type. Allowed values: `manually`, `scheduled`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<String>,
    /// Id in (up to 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

/// Request body for `POST /web/api/v2.1/reports/delete-reports`
/// (`reports_ReportDeleteSchema`).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteReportsBody {
    /// Filter. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DeleteReportsFilter>,
    /// Data. Free-form / nullable object. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// `filter` object for `POST /web/api/v2.1/reports/delete-tasks`
/// (`reports_ReportTaskDeleteSchema`).
///
/// Every member is optional. Array members are sent as JSON arrays.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteReportTasksFilter {
    /// List of Site IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Created at lte (date-time string). Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Created at gte (date-time string). Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Scope. Allowed values: `group`, `site`, `account`, `tenant`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Frequency. Allowed values: `manually`, `weekly`, `monthly`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// Interval. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Report type. Allowed values: `manually`, `scheduled`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<String>,
    /// Id in (up to 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// Sites. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<String>,
    /// Is trend. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_trend: Option<bool>,
}

/// Request body for `POST /web/api/v2.1/reports/delete-tasks`
/// (`reports_ReportTaskDeleteSchema`).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteReportTasksBody {
    /// Filter. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DeleteReportTasksFilter>,
    /// Data. Free-form / nullable object. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Join an iterator of string-like items into a comma-separated string.
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

impl DefaultReportsService<'_> {
    /// `GET /web/api/v2.1/report-tasks` — Get Default Report Tasks.
    ///
    /// Get the tasks that were done to generate default reports and to schedule
    /// future default reports. Default Reports require Reports permission and
    /// appropriate license. Best Practice: Use a filter. Each task includes
    /// many lines of data and can quickly fill the page limit. Use this command
    /// to get the ID of a report task to use in other commands.
    pub async fn list_report_tasks(
        &self,
        query: &ListReportTasksQuery,
    ) -> Result<Paginated<ReportTask>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/report-tasks", q)
            .await?)
    }

    /// `POST /web/api/v2.1/report-tasks` — Create Default Report Task.
    ///
    /// Create a task to generate a default report immediately, one time in the
    /// future, or on a schedule. Default Reports require Reports permission and
    /// appropriate license. Best Practice: Get Default Report Tasks first, to
    /// have a basis for a new task.
    pub async fn create_report_task(
        &self,
        body: &CreateReportTaskBody,
    ) -> Result<Response<SuccessResponse>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/report-tasks", body)
            .await?)
    }

    /// `PUT /web/api/v2.1/report-tasks/{task_id}` — Update Default Report Task.
    ///
    /// Update the default report task of the given ID. Default Reports require
    /// Reports permission and appropriate license. To get the task ID, and the
    /// data to change, run Get Default Report Tasks.
    pub async fn update_report_task(
        &self,
        task_id: impl Into<String>,
        body: &UpdateReportTaskBody,
    ) -> Result<Response<ReportTask>, Error> {
        let path = format!("/web/api/v2.1/report-tasks/{}", task_id.into());
        Ok(self
            .client
            .http()
            .request_json(sentinelone_http::Method::PUT, &path, None, Some(body))
            .await?)
    }

    /// `GET /web/api/v2.1/reports` — Get Default Reports.
    ///
    /// Get the default reports that match the filter and the data of the
    /// reports. Default Reports require Reports permission and appropriate
    /// license. Use this command to get the ID of reports to use in other
    /// commands. Other data in the response: schedule, Insight Type, name and
    /// ID of the user who created the report, the date range, and more.
    pub async fn list_reports(
        &self,
        query: &ListReportsQuery,
    ) -> Result<Paginated<Report>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/reports", q).await?)
    }

    /// `POST /web/api/v2.1/reports/delete-reports` — Delete Default Reports.
    ///
    /// Delete the default reports that match the filter. Default Reports require
    /// Reports permission and appropriate license. To delete a specific report,
    /// use its ID (see Get Default Reports).
    pub async fn delete_reports(
        &self,
        body: &DeleteReportsBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/reports/delete-reports", body)
            .await?)
    }

    /// `POST /web/api/v2.1/reports/delete-tasks` — Delete Default Report Tasks.
    ///
    /// You can schedule a default report to be generated on a routine. Default
    /// Reports require Reports permission and appropriate license. Use this
    /// command to remove a task to generate a report in the future. To get an ID
    /// to delete a specific task, see Get Default Report Tasks.
    pub async fn delete_report_tasks(
        &self,
        body: &DeleteReportTasksBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/reports/delete-tasks", body)
            .await?)
    }

    /// `GET /web/api/v2.1/reports/insights/types` — Get Default Insight Reports.
    ///
    /// Get the Insight Report types for Default Reports. Default Reports require
    /// Reports permission and appropriate license. These reports show high-level
    /// and detailed information on the state of your endpoint security. Reports
    /// include statistics, trends, and summaries with easy to read and
    /// actionable information about your network. Use this command to see the
    /// predefined reports. This command does not give data for specific reports.
    pub async fn list_insight_types(
        &self,
        query: &ListInsightTypesQuery,
    ) -> Result<Response<InsightData>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/reports/insights/types", q)
            .await?)
    }

    /// `GET /web/api/v2.1/reports/{report_id}/{report_format}` — Download
    /// Default Report.
    ///
    /// When the Management generates a default report, it is uploaded to the
    /// Management Console. Default Reports require Reports permission and
    /// appropriate license. Use this command to get the report as a PDF or HTML
    /// file. To get the ID of the report, see Get Default Reports.
    ///
    /// `report_format` allowed values: `pdf`, `html`.
    ///
    /// Note: the API returns the raw report file. The shared HTTP layer only
    /// exposes JSON helpers, so the response is decoded as a generic
    /// `serde_json::Value`; binary/HTML payloads may not deserialize cleanly.
    pub async fn download_report(
        &self,
        report_id: impl Into<String>,
        report_format: impl Into<String>,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/reports/{}/{}",
            report_id.into(),
            report_format.into()
        );
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `GET /web/api/v2.1/sentinelonerss` — S1 RSS Feed.
    ///
    /// Get the SentinelOne RSS feed. In the SentinelOne Management Console, we
    /// show the feed contents in the Dashboard.
    pub async fn sentinelone_rss(&self) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/sentinelonerss", None)
            .await?)
    }
}
