//! User identity and state used by business services and repository contracts.
//! Persistence adapters map their storage models into these types.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Disabled,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub display_name: String,
    pub status: UserStatus,
}

/// Verified user data required for account creation, not a database model.
pub struct NewUser {
    pub id: u64,
    pub email: String,
    pub display_name: String,
    /// Unix seconds from successful email verification.
    pub verified_at: u64,
}
