//! Shared business capabilities, organized by domain.
//!
//! Authentication coordinates a user repository and token operations. HTTP,
//! email delivery, and persisted refresh-session handling remain separate work.

pub mod auth;
pub mod mail;
pub mod organization;
pub mod persistent;
pub mod user;
