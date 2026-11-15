use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Events::Table)
                    .col(
                        ColumnDef::new(Events::OrganizationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Events::Id).big_integer().not_null())
                    .col(ColumnDef::new(Events::ActorId).big_integer().not_null())
                    .col(ColumnDef::new(Events::MemberId).big_integer().not_null())
                    .col(ColumnDef::new(Events::Owner).boolean().not_null())
                    .primary_key(Index::create().col(Events::OrganizationId).col(Events::Id))
                    .to_owned(),
            )
            .await?;
        manager.get_connection().execute_unprepared("ALTER TABLE public.organization_ownership_events ENABLE ROW LEVEL SECURITY; REVOKE ALL ON public.organization_ownership_events FROM compallm_restricted;").await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum Events {
    #[sea_orm(iden = "organization_ownership_events")]
    Table,
    OrganizationId,
    Id,
    ActorId,
    MemberId,
    Owner,
}
