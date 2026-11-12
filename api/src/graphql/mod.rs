//! Rust-owned GraphQL contract definitions.
//!
//! Input objects carry transport normalization and validation. Call Validify
//! explicitly before invoking Core services. Output objects project existing Core
//! entities; constructing a projection does not establish caller authority.
//! Axum delivery constructs a typed Juniper context with verified access and the existing DI.

pub mod authentication;
pub mod base_type;
pub mod endpoint;
pub mod organization;
pub mod policy;
pub mod provider;
pub mod user;

mod context;
mod error;
#[path = "graphql.rs"]
mod runtime;

pub use context::GraphqlContext;
pub use runtime::{ApplicationSchema, Query, build_schema};

pub(crate) use runtime::validation_schema;
