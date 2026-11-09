//! Rust-owned GraphQL contract definitions.
//!
//! Input objects carry transport normalization and validation. Call Validify
//! explicitly before invoking Core services. Output objects project existing Core
//! entities; constructing a projection does not establish caller authority.
//! HTTP delivery constructs verified request context before executing the schema.

pub mod auth;
pub mod organization;
pub mod provider;
pub mod user;

mod schema;
mod services;

pub use schema::{ApplicationSchema, Query, build_schema};

pub use services::Services;
