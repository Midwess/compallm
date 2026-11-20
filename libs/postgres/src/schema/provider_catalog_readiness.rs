use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "provider_catalog_readiness")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub provider_template_id: String,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    pub readiness: super::sea_orm_active_enums::CatalogReadiness,
    pub provider_readiness: super::sea_orm_active_enums::CatalogReadiness,
    pub model_readiness: super::sea_orm_active_enums::CatalogReadiness,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
