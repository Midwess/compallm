use super::sea_orm_active_enums::{ProviderUsageStatus, UsageDirection};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "provider_audio_usage")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub organization_id: i64,
    pub model_usage_id: i64,
    pub attempt_id: i64,
    pub model_router_id: i64,
    pub provider_model_id: i64,
    pub provider_id: i64,
    #[sea_orm(column_type = "Text", nullable)]
    pub provider_request_id: Option<String>,
    pub status: ProviderUsageStatus,
    #[sea_orm(column_type = "Decimal(Some((20, 8)))", nullable)]
    pub usd_per_second: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((20, 8)))", nullable)]
    pub usd_per_million_characters: Option<Decimal>,
    pub direction: UsageDirection,
    #[sea_orm(column_type = "Decimal(Some((24, 9)))", nullable)]
    pub duration_seconds: Option<Decimal>,
    pub character_count: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::model_usage::Entity",
        from = "(Column::OrganizationId, Column::ModelUsageId)",
        to = "(super::model_usage::Column::OrganizationId, super::model_usage::Column::Id)",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ModelUsage,
    #[sea_orm(
        belongs_to = "super::model_router::Entity",
        from = "(Column::OrganizationId, Column::ProviderModelId, Column::ModelRouterId)",
        to = "(super::model_router::Column::OrganizationId, super::model_router::Column::ProviderModelId, super::model_router::Column::Id)",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    ModelRouter,
    #[sea_orm(
        belongs_to = "super::provider_models::Entity",
        from = "(Column::OrganizationId, Column::ProviderId, Column::ProviderModelId)",
        to = "(super::provider_models::Column::OrganizationId, super::provider_models::Column::ProviderId, super::provider_models::Column::Id)",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    ProviderModels,
    #[sea_orm(
        belongs_to = "super::providers::Entity",
        from = "(Column::OrganizationId, Column::ProviderId)",
        to = "(super::providers::Column::OrganizationId, super::providers::Column::Id)",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Providers,
}

impl Related<super::model_usage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelUsage.def()
    }
}
impl Related<super::model_router::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelRouter.def()
    }
}
impl Related<super::provider_models::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProviderModels.def()
    }
}
impl Related<super::providers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Providers.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
