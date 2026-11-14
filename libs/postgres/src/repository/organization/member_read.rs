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

pub struct MappedMemberOption(pub compallm_core::organization::group::MemberOption);
impl TryFrom<(Model, bool, bool)> for MappedMemberOption {
    type Error = compallm_core::persistent::organization::OrganizationRepositoryError;
    fn try_from((user, available, included): (Model, bool, bool)) -> Result<Self, Self::Error> {
        Ok(Self(compallm_core::organization::group::MemberOption {
            included,
            member: compallm_core::organization::group::MemberInfo {
                id: crate::from_bigint(user.id).map_err(Self::Error::database)?,
                display_name: user.display_name,
                email: user.email,
                available,
            },
        }))
    }
}
impl From<MappedMemberOption> for compallm_core::organization::group::MemberOption {
    fn from(value: MappedMemberOption) -> Self {
        value.0
    }
}
