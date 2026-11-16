use crate::UserSessionSchema;
use anyhow::Context;
use compallm_core::{authentication::Session, persistent::session::SessionRepositoryError};

pub struct MappedSession(Session);

impl From<MappedSession> for Session {
    fn from(mapped: MappedSession) -> Self {
        mapped.0
    }
}

impl TryFrom<UserSessionSchema> for MappedSession {
    type Error = SessionRepositoryError;

    fn try_from(model: UserSessionSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id).context("invalid session ID")?;
        let user_id = crate::from_bigint(model.user_id).context("invalid session user ID")?;
        let organization_id =
            crate::from_bigint(model.organization_id).context("invalid session organization ID")?;
        let authorization_version = u32::try_from(model.authorization_version)
            .context("invalid session authorization version")?;
        if authorization_version == 0 {
            return Err(anyhow::anyhow!("invalid session authorization version").into());
        }
        let revoked_at = model
            .revoked_at
            .map(|at| u64::try_from(at.timestamp()))
            .transpose()
            .context("invalid session revocation timestamp")?;
        Ok(Self(Session {
            id,
            user_id,
            organization_id,
            authorization_version,
            revoked_at,
        }))
    }
}

/// Adapter-local conversion of one statement's complete authorization rows.
pub struct MappedPolicyMembership((Session, compallm_core::organization::OrganizationMember));

impl From<MappedPolicyMembership> for (Session, compallm_core::organization::OrganizationMember) {
    fn from(value: MappedPolicyMembership) -> Self {
        value.0
    }
}

impl TryFrom<Vec<sea_orm::QueryResult>> for MappedPolicyMembership {
    type Error = SessionRepositoryError;

    fn try_from(rows: Vec<sea_orm::QueryResult>) -> Result<Self, Self::Error> {
        use compallm_core::organization::{Group, OrganizationMember, Role};
        use sea_orm::FromQueryResult;
        use std::collections::BTreeMap;
        let first = rows.first().context("missing authorization row")?;
        let session = Session::from(MappedSession::try_from(
            UserSessionSchema::from_query_result(first, "").map_err(anyhow::Error::from)?,
        )?);
        let mut direct_role = None;
        let mut groups = BTreeMap::<u64, Group>::new();
        for row in rows {
            let group_id: Option<i64> = row.try_get("", "group_id").map_err(anyhow::Error::from)?;
            let role_id: Option<i64> = row.try_get("", "role_id").map_err(anyhow::Error::from)?;
            let role = role_id
                .map(|id| -> anyhow::Result<Role> {
                    Role::try_from(crate::OrganizationRoleSchema {
                        id,
                        organization_id: i64::try_from(session.organization_id)?,
                        name: row.try_get("", "role_name")?,
                        description: row.try_get("", "role_description")?,
                        cedar_source: row.try_get("", "cedar_source")?,
                        default_member: false,
                        org_owner: row.try_get("", "org_owner")?,
                    })
                })
                .transpose()?;
            if let Some(group_id) = group_id {
                let id = crate::from_bigint(group_id).context("invalid group ID")?;
                if let std::collections::btree_map::Entry::Vacant(entry) = groups.entry(id) {
                    entry.insert(
                        Group::new(
                            id,
                            session.organization_id,
                            row.try_get("", "group_name").map_err(anyhow::Error::from)?,
                            row.try_get("", "group_description")
                                .map_err(anyhow::Error::from)?,
                            Vec::new(),
                        )
                        .map_err(anyhow::Error::from)?,
                    );
                }
                if let Some(role) = role {
                    let group = groups.get_mut(&id).context("group is missing")?;
                    let mut assigned = group.roles().to_vec();
                    assigned.push(role);
                    group.replace_roles(assigned).map_err(anyhow::Error::from)?;
                }
            } else if let Some(role) = role
                && direct_role.replace(role).is_some()
            {
                return Err(anyhow::anyhow!("membership has multiple direct roles").into());
            }
        }
        let member = OrganizationMember {
            organization_id: session.organization_id,
            user_id: session.user_id,
            role: direct_role.context("membership requires a direct role")?,
            groups: groups.into_values().collect(),
        };
        Ok(Self((session, member)))
    }
}
