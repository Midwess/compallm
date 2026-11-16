use super::passkey_models::{
    session_authentication_methods as methods, user_identities as identities, users as handles,
    webauthn_challenges as challenges, webauthn_credentials as credentials,
};
use crate::postgres_connection::PostgresConnection;
use compallm_core::{
    authentication::{
        AuthError, Session,
        passkey::{Ceremony, PasskeyRecord},
    },
    persistent::{DbContext, passkey::PasskeyRepository as Repository},
};
use sea_orm::{
    QueryOrder, QuerySelect, QueryTrait, Set,
    entity::prelude::*,
    sea_query::{Expr, OnConflict},
};
use uuid::Uuid;

pub struct PasskeyRepository;
fn id(value: u64) -> Result<i64, AuthError> {
    crate::to_bigint(value).map_err(|e| AuthError::Unexpected(e.into()))
}
impl TryFrom<credentials::Model> for PasskeyRecord {
    type Error = AuthError;
    fn try_from(m: credentials::Model) -> Result<Self, AuthError> {
        Ok(Self {
            id: crate::from_bigint(m.id).map_err(|e| AuthError::Unexpected(e.into()))?,
            name: m.friendly_name,
            registered_at: u64::try_from(m.registered_at)
                .map_err(|e| AuthError::Unexpected(e.into()))?,
            last_used_at: m
                .last_used_at
                .map(u64::try_from)
                .transpose()
                .map_err(|e| AuthError::Unexpected(e.into()))?,
            credential: serde_json::from_value(m.credential)?,
        })
    }
}
impl TryFrom<challenges::Model> for Ceremony {
    type Error = AuthError;
    fn try_from(m: challenges::Model) -> Result<Self, AuthError> {
        Ok(Self {
            id: m.id,
            purpose: m.purpose,
            expires_at: u64::try_from(m.expires_at).map_err(|e| AuthError::Unexpected(e.into()))?,
            state: m.state,
        })
    }
}
#[async_trait::async_trait]
impl Repository for PasskeyRepository {
    async fn list(&self, user_id: u64) -> Result<Vec<PasskeyRecord>, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let owned = identities::Entity::find()
            .select_only()
            .column(identities::Column::PasskeyId)
            .filter(identities::Column::UserId.eq(id(user_id)?))
            .filter(identities::Column::Provider.eq("passkey"))
            .into_query();
        credentials::Entity::find()
            .filter(credentials::Column::Id.in_subquery(owned))
            .order_by_asc(credentials::Column::RegisteredAt)
            .order_by_asc(credentials::Column::Id)
            .all(db)
            .await
            .map_err(anyhow::Error::from)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
    async fn find_discoverable(
        &self,
        handle: Uuid,
        credential_id: &[u8],
    ) -> Result<Option<(u64, PasskeyRecord)>, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let user = handles::Entity::find()
            .filter(handles::Column::Handle.eq(handle))
            .one(db)
            .await
            .map_err(anyhow::Error::from)?;
        let Some(user) = user else {
            return Ok(None);
        };
        let owned = identities::Entity::find()
            .select_only()
            .column(identities::Column::PasskeyId)
            .filter(identities::Column::UserId.eq(user.user_id))
            .filter(identities::Column::Provider.eq("passkey"))
            .into_query();
        let record = credentials::Entity::find()
            .filter(credentials::Column::CredentialId.eq(credential_id.to_vec()))
            .filter(credentials::Column::Id.in_subquery(owned))
            .one(db)
            .await
            .map_err(anyhow::Error::from)?;
        record
            .map(|record| {
                Ok((
                    crate::from_bigint(user.user_id)
                        .map_err(|e| AuthError::Unexpected(e.into()))?,
                    record.try_into()?,
                ))
            })
            .transpose()
    }
    async fn rename(&self, user_id: u64, credential_id: u64, name: &str) -> Result<(), AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let owned = identities::Entity::find()
            .select_only()
            .column(identities::Column::PasskeyId)
            .filter(identities::Column::UserId.eq(id(user_id)?))
            .filter(identities::Column::Provider.eq("passkey"))
            .into_query();
        let result = credentials::Entity::update_many()
            .filter(credentials::Column::Id.eq(id(credential_id)?))
            .filter(credentials::Column::Id.in_subquery(owned))
            .col_expr(credentials::Column::FriendlyName, Expr::value(name))
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?;
        if result.rows_affected != 1 {
            return Err(AuthError::InvalidPasskey);
        }
        Ok(())
    }
    async fn session_passkey(&self, session: &Session) -> Result<Option<PasskeyRecord>, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let used = methods::Entity::find()
            .select_only()
            .column(methods::Column::PasskeyId)
            .filter(methods::Column::SessionId.eq(id(session.id)?))
            .filter(methods::Column::Method.eq("passkey"))
            .into_query();
        let owned = identities::Entity::find()
            .select_only()
            .column(identities::Column::PasskeyId)
            .filter(identities::Column::UserId.eq(id(session.user_id)?))
            .filter(identities::Column::Provider.eq("passkey"))
            .into_query();
        credentials::Entity::find()
            .filter(credentials::Column::Id.in_subquery(used))
            .filter(credentials::Column::Id.in_subquery(owned))
            .one(db)
            .await
            .map_err(anyhow::Error::from)?
            .map(TryInto::try_into)
            .transpose()
    }
    async fn lock_user(&self, user_id: u64) -> Result<Uuid, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let user = handles::Entity::find_by_id(id(user_id)?)
            .lock_exclusive()
            .one(db)
            .await
            .map_err(anyhow::Error::from)?
            .ok_or(AuthError::UserNotFound)?;
        if let Some(handle) = user.handle {
            return Ok(handle);
        }
        let handle = Uuid::new_v4();
        handles::Entity::update_many()
            .filter(handles::Column::UserId.eq(id(user_id)?))
            .col_expr(handles::Column::Handle, Expr::value(handle))
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?;
        Ok(handle)
    }
    async fn create(&self, user_id: u64, credential: PasskeyRecord) -> Result<(), AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        credentials::Entity::insert(credentials::ActiveModel {
            id: Set(id(credential.id)?),
            credential_id: Set(credential.credential.cred_id().as_ref().to_vec()),
            credential: Set(serde_json::to_value(credential.credential)?),
            friendly_name: Set(credential.name),
            registered_at: Set(id(credential.registered_at)?),
            last_used_at: Set(None),
        })
        .exec(db)
        .await
        .map_err(|error| {
            if matches!(
                error.sql_err(),
                Some(sea_orm::SqlErr::UniqueConstraintViolation(_))
            ) {
                AuthError::PasskeyConflict
            } else {
                AuthError::Unexpected(error.into())
            }
        })?;
        identities::Entity::insert(identities::ActiveModel {
            id: Set(id(credential.id)?),
            user_id: Set(id(user_id)?),
            provider: Set("passkey".into()),
            passkey_id: Set(Some(id(credential.id)?)),
            identity_data: Set(serde_json::json!({})),
        })
        .exec(db)
        .await
        .map_err(anyhow::Error::from)?;
        Ok(())
    }
    async fn update(&self, credential: &PasskeyRecord) -> Result<(), AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let result = credentials::Entity::update_many()
            .filter(credentials::Column::Id.eq(id(credential.id)?))
            .col_expr(
                credentials::Column::Credential,
                Expr::value(serde_json::to_value(&credential.credential)?),
            )
            .col_expr(
                credentials::Column::LastUsedAt,
                Expr::value(credential.last_used_at.map(id).transpose()?),
            )
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?;
        if result.rows_affected != 1 {
            return Err(AuthError::InvalidPasskey);
        }
        Ok(())
    }
    async fn remove(&self, user_id: u64, credential_id: u64) -> Result<bool, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let result = identities::Entity::delete_many()
            .filter(identities::Column::UserId.eq(id(user_id)?))
            .filter(identities::Column::Provider.eq("passkey"))
            .filter(identities::Column::PasskeyId.eq(id(credential_id)?))
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?;
        if result.rows_affected == 0 {
            return Ok(false);
        }
        credentials::Entity::delete_by_id(id(credential_id)?)
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?;
        Ok(true)
    }
    async fn insert_challenge(&self, c: Ceremony) -> Result<(), AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        challenges::Entity::insert(challenges::ActiveModel {
            id: Set(c.id),
            purpose: Set(c.purpose),
            state: Set(c.state),
            expires_at: Set(id(c.expires_at)?),
        })
        .exec(db)
        .await
        .map_err(anyhow::Error::from)?;
        Ok(())
    }
    async fn challenge(
        &self,
        challenge_id: Uuid,
        lock: bool,
    ) -> Result<Option<Ceremony>, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let query = challenges::Entity::find_by_id(challenge_id);
        let query = if lock { query.lock_exclusive() } else { query };
        query
            .one(db)
            .await
            .map_err(anyhow::Error::from)?
            .map(TryInto::try_into)
            .transpose()
    }
    async fn delete_challenge(&self, challenge_id: Uuid) -> Result<bool, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Ok(challenges::Entity::delete_by_id(challenge_id)
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?
            .rows_affected
            == 1)
    }
    async fn update_challenge(&self, c: Ceremony) -> Result<(), AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        challenges::Entity::update_many()
            .filter(challenges::Column::Id.eq(c.id))
            .col_expr(challenges::Column::State, Expr::value(c.state))
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?;
        Ok(())
    }
    async fn record_method(
        &self,
        session: &Session,
        method: &str,
        at: u64,
        credential_id: Option<u64>,
    ) -> Result<(), AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        methods::Entity::insert(methods::ActiveModel {
            session_id: Set(id(session.id)?),
            method: Set(method.into()),
            provider: Set(String::new()),
            authenticated_at: Set(id(at)?),
            passkey_id: Set(credential_id.map(id).transpose()?),
        })
        .on_conflict(
            OnConflict::columns([
                methods::Column::SessionId,
                methods::Column::Method,
                methods::Column::Provider,
            ])
            .update_columns([methods::Column::AuthenticatedAt, methods::Column::PasskeyId])
            .to_owned(),
        )
        .exec(db)
        .await
        .map_err(anyhow::Error::from)?;
        Ok(())
    }
    async fn recently_verified(&self, session: &Session, since: u64) -> Result<bool, AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Ok(methods::Entity::find()
            .filter(methods::Column::SessionId.eq(id(session.id)?))
            .filter(methods::Column::AuthenticatedAt.gt(id(since)?))
            .filter(methods::Column::Method.is_in(["email_code", "passkey"]))
            .one(db)
            .await
            .map_err(anyhow::Error::from)?
            .is_some())
    }
    async fn ensure_email_identity(&self, user_id: u64) -> Result<(), AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        identities::Entity::insert(identities::ActiveModel {
            id: Set(id(user_id)?),
            user_id: Set(id(user_id)?),
            provider: Set("email".into()),
            passkey_id: Set(None),
            identity_data: Set(serde_json::json!({})),
        })
        .on_conflict(
            OnConflict::column(identities::Column::Id)
                .do_nothing()
                .to_owned(),
        )
        .try_insert()
        .exec(db)
        .await
        .map_err(anyhow::Error::from)?;
        Ok(())
    }
    async fn cleanup(&self, now: u64) -> Result<(), AuthError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let expired = challenges::Entity::find()
            .select_only()
            .column(challenges::Column::Id)
            .filter(challenges::Column::ExpiresAt.lte(id(now)?))
            .limit(1000)
            .into_query();
        challenges::Entity::delete_many()
            .filter(challenges::Column::Id.in_subquery(expired))
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?;
        let referenced = identities::Entity::find()
            .select_only()
            .column(identities::Column::PasskeyId)
            .filter(identities::Column::PasskeyId.is_not_null())
            .into_query();
        let orphans = credentials::Entity::find()
            .select_only()
            .column(credentials::Column::Id)
            .filter(credentials::Column::Id.not_in_subquery(referenced))
            .limit(1000)
            .into_query();
        credentials::Entity::delete_many()
            .filter(credentials::Column::Id.in_subquery(orphans))
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?;
        Ok(())
    }
}
