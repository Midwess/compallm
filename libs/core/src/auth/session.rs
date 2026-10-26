use super::RefreshToken;

/// One successful login. Refresh updates activity without creating another login.
#[derive(Clone, Debug)]
pub struct Session {
    pub id: u64,
    pub user_id: u64,
    pub refresh_token_id: String,
    pub created_at: u64,
    pub last_used_at: u64,
    pub expires_at: u64,
    pub revoked_at: Option<u64>,
}

impl Session {
    pub fn new(refresh: &RefreshToken) -> Self {
        Self {
            id: refresh.session_id,
            user_id: refresh.user_id,
            refresh_token_id: refresh.token_id.clone(),
            created_at: refresh.validity.issued_at,
            last_used_at: refresh.validity.issued_at,
            expires_at: refresh.validity.expires_at,
            revoked_at: None,
        }
    }

    pub fn is_active(&self, user_id: u64, now: u64) -> bool {
        self.user_id == user_id && self.revoked_at.is_none() && now < self.expires_at
    }

    pub fn renew(&mut self, refresh: &RefreshToken) {
        self.refresh_token_id = refresh.token_id.clone();
        self.last_used_at = refresh.validity.issued_at;
        self.expires_at = refresh.validity.expires_at;
    }
}
