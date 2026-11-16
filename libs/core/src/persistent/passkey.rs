use crate::authentication::{
    AuthError, Session,
    passkey::{Ceremony, PasskeyRecord},
};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait PasskeyRepository: Send + Sync {
    async fn list(&self, user_id: u64) -> Result<Vec<PasskeyRecord>, AuthError>;
    async fn lock_user(&self, user_id: u64) -> Result<Uuid, AuthError>;
    async fn create(&self, user_id: u64, credential: PasskeyRecord) -> Result<(), AuthError>;
    async fn update(&self, credential: &PasskeyRecord) -> Result<(), AuthError>;
    async fn remove(&self, user_id: u64, id: u64) -> Result<bool, AuthError>;
    async fn insert_challenge(&self, challenge: Ceremony) -> Result<(), AuthError>;
    async fn challenge(&self, id: Uuid, lock: bool) -> Result<Option<Ceremony>, AuthError>;
    async fn delete_challenge(&self, id: Uuid) -> Result<bool, AuthError>;
    async fn update_challenge(&self, challenge: Ceremony) -> Result<(), AuthError>;
    async fn record_method(
        &self,
        session: &Session,
        method: &str,
        at: u64,
    ) -> Result<(), AuthError>;
    async fn recently_verified(&self, session: &Session, since: u64) -> Result<bool, AuthError>;
    async fn ensure_email_identity(&self, user_id: u64) -> Result<(), AuthError>;
    async fn cleanup(&self, now: u64) -> Result<(), AuthError>;
}
