use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ZstdDictionaryType::Enum)
                    .values([ZstdDictionaryType::AccessToken])
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(ZstdDictionary::Table)
                    .col(
                        ColumnDef::new(ZstdDictionary::Type)
                            .custom(ZstdDictionaryType::Enum)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ZstdDictionary::Version)
                            .integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(ZstdDictionary::Content).text().not_null())
                    .primary_key(
                        Index::create()
                            .col(ZstdDictionary::Type)
                            .col(ZstdDictionary::Version),
                    )
                    .check(Expr::col(ZstdDictionary::Version).gt(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ZstdDictionary::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(ZstdDictionaryType::Enum).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ZstdDictionary {
    Table,
    Type,
    Version,
    Content,
}

#[derive(DeriveIden)]
enum ZstdDictionaryType {
    #[sea_orm(iden = "zstd_dictionary_type")]
    Enum,
    /// Shared Zstd dictionary for the serialized user access token and its Cedar policy snapshot.
    AccessToken,
}
