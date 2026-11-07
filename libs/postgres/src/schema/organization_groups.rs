//! Organization-scoped authorization storage; queries and conversions stay in PostgreSQL.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "organization_groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub organization_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub description: String,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
