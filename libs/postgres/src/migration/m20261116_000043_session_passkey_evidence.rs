use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager
            .has_column("session_authentication_methods", "passkey_id")
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(SessionAuthenticationMethods::Table)
                        .add_column(
                            ColumnDef::new(SessionAuthenticationMethods::PasskeyId)
                                .big_integer()
                                .null(),
                        )
                        .to_owned(),
                )
                .await?;
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("session_authentication_method_passkey_fk")
                        .from(
                            SessionAuthenticationMethods::Table,
                            SessionAuthenticationMethods::PasskeyId,
                        )
                        .to(WebauthnCredentials::Table, WebauthnCredentials::Id)
                        .on_delete(ForeignKeyAction::SetNull)
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(SessionAuthenticationMethods::Table)
                    .drop_column(SessionAuthenticationMethods::PasskeyId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum SessionAuthenticationMethods {
    Table,
    PasskeyId,
}
#[derive(DeriveIden)]
enum WebauthnCredentials {
    Table,
    Id,
}
