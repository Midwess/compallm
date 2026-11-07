//! Global provider-scoped model defaults. Contains no organization or user identity.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "provider_model_templates")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub provider_template_id: String,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "JsonBinary")]
    pub protocols: Json,
    #[sea_orm(column_type = "JsonBinary")]
    pub paths: Json,
    #[sea_orm(column_type = "JsonBinary")]
    pub capabilities: Json,
    pub max_context_tokens: Option<i64>,
    #[sea_orm(column_type = "JsonBinary")]
    pub effort_mapping: Json,
    #[sea_orm(column_type = "Decimal(Some((20, 8)))", nullable)]
    pub input_price: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((20, 8)))", nullable)]
    pub output_price: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((20, 8)))", nullable)]
    pub cached_price: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::provider_templates::Entity",
        from = "Column::ProviderTemplateId",
        to = "super::provider_templates::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ProviderTemplates,
}
impl Related<super::provider_templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProviderTemplates.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
