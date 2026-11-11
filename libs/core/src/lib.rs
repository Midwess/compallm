//! Shared business capabilities, organized by domain.
//!
//! Authentication coordinates user, organization, and persisted session workflows.
//! HTTP cookie transport and concrete email delivery remain separate work.

pub mod authentication;
pub mod authorization;
pub mod compression;
pub mod mail;
pub mod model;
pub mod organization;
pub mod persistent;
pub mod provider;
pub mod repository;
pub mod user;
