//! Global provider catalog defaults. Kept in sync with the unapplied migration.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "provider_templates")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "String(StringLen::N(120))")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub icon_key: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub protocol: String,
    #[sea_orm(column_type = "String(StringLen::N(512))", nullable)]
    pub base_url: Option<String>,
    #[sea_orm(column_type = "JsonBinary")]
    pub headers: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::provider_model_templates::Entity")]
    ProviderModelTemplates,
}
impl Related<super::provider_model_templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProviderModelTemplates.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
