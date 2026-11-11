use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Lz4DictionaryType::Enum)
                    .values([Lz4DictionaryType::AccessToken])
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Lz4Dictionary::Table)
                    .col(
                        ColumnDef::new(Lz4Dictionary::Type)
                            .custom(Lz4DictionaryType::Enum)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Lz4Dictionary::Version)
                            .integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Lz4Dictionary::Content).text().not_null())
                    .primary_key(
                        Index::create()
                            .col(Lz4Dictionary::Type)
                            .col(Lz4Dictionary::Version),
                    )
                    .check(Expr::col(Lz4Dictionary::Version).gt(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Lz4Dictionary::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Lz4DictionaryType::Enum).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Lz4Dictionary {
    Table,
    Type,
    Version,
    Content,
}

#[derive(DeriveIden)]
enum Lz4DictionaryType {
    #[sea_orm(iden = "lz4_dictionary_type")]
    Enum,
    /// Shared LZ4 dictionary for the serialized user access token and its Cedar policy snapshot.
    AccessToken,
}
