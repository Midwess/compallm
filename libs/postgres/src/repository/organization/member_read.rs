use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub display_name: String,
    pub email: String,
    pub status: crate::UserStatusSchema,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

pub struct MappedMember(pub compallm_core::organization::group::MemberInfo);
impl TryFrom<(Model, bool)> for MappedMember {
    type Error = compallm_core::persistent::organization::OrganizationRepositoryError;
    fn try_from((user, available): (Model, bool)) -> Result<Self, Self::Error> {
        Ok(Self(compallm_core::organization::group::MemberInfo {
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
