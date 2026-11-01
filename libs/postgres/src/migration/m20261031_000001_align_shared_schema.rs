use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(OrganizationMembers::Table)
                    .add_column(
                        ColumnDef::new(OrganizationMembers::Status)
                            .custom(UserStatus::Enum)
                            .not_null()
                            .default("active"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Models::Table)
                    .drop_column(Models::CreatedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Models::Table)
                    .add_column(Self::created_at_column())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(OrganizationMembers::Table)
                    .drop_column(OrganizationMembers::Status)
                    .to_owned(),
            )
            .await
    }
}

impl Migration {
    fn created_at_column() -> ColumnDef {
        ColumnDef::new(Models::CreatedAt)
            .timestamp_with_time_zone()
            .not_null()
            .default(Expr::current_timestamp())
            .to_owned()
    }
}

#[derive(DeriveIden)]
enum UserStatus {
    #[sea_orm(iden = "user_status")]
    Enum,
}

#[derive(DeriveIden)]
enum OrganizationMembers {
    Table,
    Status,
}

#[derive(DeriveIden)]
enum Models {
    Table,
    CreatedAt,
}
