use sea_orm_migration::{prelude::*, sea_query::TableCreateStatement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(user_preferences_table()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserPreferences::Table).to_owned())
            .await
    }
}

fn user_preferences_table() -> TableCreateStatement {
    let mut table = Table::create();
    table.table(UserPreferences::Table).col(
        ColumnDef::new(UserPreferences::UserId)
            .big_integer()
            .not_null()
            .primary_key(),
    );

    for column in [
        UserPreferences::Navigation,
        UserPreferences::Sidebar,
        UserPreferences::MemberDisplay,
        UserPreferences::Calendar,
        UserPreferences::Theme,
        UserPreferences::Typography,
        UserPreferences::Accessibility,
        UserPreferences::MessageComposition,
        UserPreferences::CodeAppearance,
    ] {
        table.col(
            ColumnDef::new(column).json_binary().not_null().check(
                Func::cust(Alias::new("jsonb_typeof"))
                    .arg(Expr::col(column))
                    .eq("object"),
            ),
        );
    }

    table
        .foreign_key(
            ForeignKey::create()
                .name("user_preferences_user_id_fkey")
                .from(UserPreferences::Table, UserPreferences::UserId)
                .to(Users::Table, Users::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

#[derive(Clone, Copy, DeriveIden)]
enum UserPreferences {
    Table,
    UserId,
    Navigation,
    Sidebar,
    MemberDisplay,
    Calendar,
    Theme,
    Typography,
    Accessibility,
    MessageComposition,
    CodeAppearance,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
