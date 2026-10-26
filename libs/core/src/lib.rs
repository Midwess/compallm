//! Shared business capabilities, organized by domain.
//!
//! Authentication coordinates user, organization, and persisted session workflows.
//! HTTP cookie transport and concrete email delivery remain separate work.

pub mod auth;
pub mod mail;
pub mod organization;
pub mod persistent;
pub mod user;
