//! User domain identity and state used by services and repository contracts.
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
