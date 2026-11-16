use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub display_name: String,
    pub email: String,
    pub status: crate::UserStatusSchema,
    pub sort_rank: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

pub struct MappedMember(pub compallm_core::organization::group::MemberInfo);
impl TryFrom<(Model, bool)> for MappedMember {
    type Error = compallm_core::persistent::organization::OrganizationRepositoryError;
    fn try_from((user, available): (Model, bool)) -> Result<Self, Self::Error> {
        Ok(Self(compallm_core::organization::group::MemberInfo {
            role: None,
            groups: Vec::new(),
            effective_role_ids: Vec::new(),
            group_ids: Vec::new(),
            id: crate::from_bigint(user.id).map_err(Self::Error::database)?,
            display_name: user.display_name,
            email: user.email,
            available,
        }))
    }
}
impl From<MappedMember> for compallm_core::organization::group::MemberInfo {
    fn from(value: MappedMember) -> Self {
        value.0
    }
}

pub struct MemberQuery {
    pub query: compallm_query::Query,
    pub role_id: Option<i64>,
}
impl TryFrom<compallm_query::Query> for MemberQuery {
    type Error = compallm_core::persistent::organization::OrganizationRepositoryError;
    fn try_from(query: compallm_query::Query) -> Result<Self, Self::Error> {
        use compallm_query::{Predicate, operation::Operation};
        let predicates = match query.predicate {
            Predicate::And(predicates) => predicates,
            predicate => vec![predicate],
        };
        let mut role_id = None;
        let mut remaining = Vec::new();
        for predicate in predicates {
            if let Predicate::Field { field, operation } = &predicate
                && field == "role_id"
            {
                let Operation::Id(value) = operation else {
                    return Err(Self::Error::InvalidQuery);
                };
                let value = value
                    .parse::<i64>()
                    .ok()
                    .filter(|value| *value > 0)
                    .ok_or(Self::Error::InvalidQuery)?;
                if role_id.replace(value).is_some() {
                    return Err(Self::Error::InvalidQuery);
                }
                continue;
            }
            remaining.push(predicate);
        }
        Ok(Self {
            query: compallm_query::Query {
                predicate: Predicate::And(remaining),
            },
            role_id,
        })
    }
}
