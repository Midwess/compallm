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
    pub session_credentials: HashMap<u64, u64>,
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
    async fn find_discoverable(
        &self,
        handle: Uuid,
        credential_id: &[u8],
    ) -> Result<Option<(u64, PasskeyRecord)>, AuthError> {
        let state = self.state.lock().unwrap();
        Ok(state
            .records
            .values()
            .find(|(user, record)| {
                state.handles.get(user) == Some(&handle)
                    && record.credential.cred_id().as_ref() == credential_id
            })
            .cloned())
    }
    async fn rename(&self, user: u64, id: u64, name: &str) -> Result<(), AuthError> {
        let mut state = self.state.lock().unwrap();
        let (owner, record) = state
            .records
            .get_mut(&id)
            .ok_or(AuthError::InvalidPasskey)?;
        if *owner != user {
            return Err(AuthError::InvalidPasskey);
        }
        record.name = name.into();
        Ok(())
    }
    async fn session_passkey(&self, session: &Session) -> Result<Option<PasskeyRecord>, AuthError> {
        let state = self.state.lock().unwrap();
        Ok(state
            .session_credentials
            .get(&session.id)
            .and_then(|id| state.records.get(id))
            .filter(|(owner, _)| *owner == session.user_id)
            .map(|(_, r)| r.clone()))
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
        credential_id: Option<u64>,
    ) -> Result<(), AuthError> {
        if let Some(id) = credential_id {
            self.state
                .lock()
                .unwrap()
                .session_credentials
                .insert(session.id, id);
        }
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

pub struct ResidentAuthenticator {
    inner: webauthn_authenticator_rs::softpasskey::SoftPasskey,
    credential: Option<webauthn_rs_core::proto::AllowCredentials>,
    handle: Option<webauthn_rs_core::proto::Base64UrlSafeData>,
}
impl ResidentAuthenticator {
    pub fn new() -> Self {
        Self {
            inner: webauthn_authenticator_rs::softpasskey::SoftPasskey::new(true),
            credential: None,
            handle: None,
        }
    }
}
impl webauthn_authenticator_rs::AuthenticatorBackend for ResidentAuthenticator {
    fn perform_register(
        &mut self,
        origin: url::Url,
        mut options: webauthn_rs_core::proto::PublicKeyCredentialCreationOptions,
        timeout: u32,
    ) -> Result<
        webauthn_rs_core::proto::RegisterPublicKeyCredential,
        webauthn_authenticator_rs::error::WebauthnCError,
    > {
        self.handle = Some(options.user.id.clone());
        if let Some(selection) = &mut options.authenticator_selection {
            selection.require_resident_key = false;
            selection.resident_key = None;
        }
        let response = self.inner.perform_register(origin, options, timeout)?;
        self.credential = Some(webauthn_rs_core::proto::AllowCredentials {
            type_: "public-key".into(),
            id: response.raw_id.clone(),
            transports: None,
        });
        Ok(response)
    }
    fn perform_auth(
        &mut self,
        origin: url::Url,
        mut options: webauthn_rs_core::proto::PublicKeyCredentialRequestOptions,
        timeout: u32,
    ) -> Result<
        webauthn_rs_core::proto::PublicKeyCredential,
        webauthn_authenticator_rs::error::WebauthnCError,
    > {
        if options.allow_credentials.is_empty() {
            options
                .allow_credentials
                .push(self.credential.clone().unwrap());
        }
        let mut response = self.inner.perform_auth(origin, options, timeout)?;
        response.response.user_handle = self.handle.clone();
        Ok(response)
    }
}
