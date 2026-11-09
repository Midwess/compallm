//! Rust-owned GraphQL contract definitions.
//!
//! Input objects carry transport normalization and validation. Call Validify
//! explicitly before invoking Core services. Output objects project existing Core
//! entities; constructing a projection does not establish caller authority.
//! Axum delivery constructs a typed Juniper context with verified access and the existing DI.

pub mod auth;
pub mod organization;
pub mod provider;
pub mod user;

mod context;
mod schema;

pub use context::GraphqlContext;
pub use schema::{ApplicationSchema, Query, build_schema};
