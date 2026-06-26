//! Low-level async client for the SentinelOne **Management** API.
//!
//! Host: e.g. `https://apse1-2111-mssp.sentinelone.net`. Auth: `ApiToken`.
//! Hand-written service-per-tag layer at 1:1 parity with the Swagger 2.0 spec:
//! 117 tags, 826 endpoints.
//!
//! Query-filter fields mirror the API's wire names exactly, including its
//! double-underscore operator suffixes (e.g. `created_at__gt`,
//! `computer_name__contains`); `non_snake_case` is therefore allowed crate-wide.

#![allow(non_snake_case)]

pub mod client;
pub mod error;
pub mod models;
pub mod pagination;
pub mod services;

pub use client::ManagementClient;
pub use error::Error;
pub use pagination::{Paginated, Pagination, Response};
