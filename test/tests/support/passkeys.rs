use compallm_core::{
    authentication::{
        AuthError, Session,
        passkey::{Ceremony, PasskeyRecord},
    },
    persistent::passkey::PasskeyRepository,
};
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;
#[derive(Default)]
pub struct MemoryPasskeys {
    pub state: Mutex<State>,
}
#[derive(Default)]
pub struct State {
    pub records: HashMap<u64, (u64, PasskeyRecord)>,
    pub challenges: HashMap<Uuid, Ceremony>,
    pub evidence: HashMap<(u64, String), u64>,
    pub handles: HashMap<u64, Uuid>,
}
#[async_trait::async_trait]
impl PasskeyRepository for MemoryPasskeys {
    async fn list(&self, user: u64) -> Result<Vec<PasskeyRecord>, AuthError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .records
            .values()
            .filter(|(u, _)| *u == user)
            .map(|(_, r)| r.clone())
            .collect())
    }
    async fn lock_user(&self, user: u64) -> Result<Uuid, AuthError> {
        Ok(*self
            .state
            .lock()
            .unwrap()
            .handles
            .entry(user)
            .or_insert_with(Uuid::new_v4))
    }
    async fn create(&self, user: u64, record: PasskeyRecord) -> Result<(), AuthError> {
        let mut state = self.state.lock().unwrap();
        if state
            .records
            .values()
            .any(|(_, r)| r.credential.cred_id() == record.credential.cred_id())
        {
            return Err(AuthError::InvalidPasskey);
        }
        state.records.insert(record.id, (user, record));
        Ok(())
    }
    async fn update(&self, record: &PasskeyRecord) -> Result<(), AuthError> {
        let mut state = self.state.lock().unwrap();
        let entry = state
            .records
            .get_mut(&record.id)
            .ok_or(AuthError::InvalidPasskey)?;
        entry.1 = record.clone();
        Ok(())
    }
    async fn remove(&self, user: u64, id: u64) -> Result<bool, AuthError> {
        let mut state = self.state.lock().unwrap();
        if !state.records.get(&id).is_some_and(|(u, _)| *u == user) {
            return Ok(false);
        }
        Ok(state.records.remove(&id).is_some())
    }
    async fn insert_challenge(&self, c: Ceremony) -> Result<(), AuthError> {
        self.state.lock().unwrap().challenges.insert(c.id, c);
        Ok(())
    }
    async fn challenge(&self, id: Uuid, _: bool) -> Result<Option<Ceremony>, AuthError> {
        Ok(self.state.lock().unwrap().challenges.get(&id).cloned())
    }
    async fn delete_challenge(&self, id: Uuid) -> Result<bool, AuthError> {
        Ok(self.state.lock().unwrap().challenges.remove(&id).is_some())
    }
    async fn update_challenge(&self, c: Ceremony) -> Result<(), AuthError> {
        self.state.lock().unwrap().challenges.insert(c.id, c);
        Ok(())
    }
    async fn record_method(
        &self,
        session: &Session,
        method: &str,
        at: u64,
    ) -> Result<(), AuthError> {
        self.state
            .lock()
            .unwrap()
            .evidence
            .insert((session.id, method.into()), at);
        Ok(())
    }
    async fn recently_verified(&self, session: &Session, since: u64) -> Result<bool, AuthError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .evidence
            .iter()
            .any(|((s, _), at)| *s == session.id && *at > since))
    }
    async fn ensure_email_identity(&self, _: u64) -> Result<(), AuthError> {
        Ok(())
    }
    async fn cleanup(&self, now: u64) -> Result<(), AuthError> {
        self.state
            .lock()
            .unwrap()
            .challenges
            .retain(|_, c| c.expires_at > now);
        Ok(())
    }
}
