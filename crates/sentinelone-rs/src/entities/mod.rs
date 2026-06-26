//! High-level active-record entities. Each wraps a raw model + a `SentinelOne`
//! handle so it can act on itself. `Agent` is the reference pattern; the rest
//! (Site, Threat, Policy, …) follow it.

pub mod agent;
