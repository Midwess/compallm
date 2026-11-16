use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::Username).text().null().check(
                        Expr::col(Users::Username).eq(Func::lower(Expr::col(Users::Username))),
                    ))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("users_username_unique")
                    .table(Users::Table)
                    .col(Users::Username)
                    .unique()
                    .to_owned(),
            )
            .await?;
        let db = manager.get_connection();
        let mut after = 0_i64;
        loop {
            let rows: Vec<(i64, String)> = backfill::Entity::find()
                .select_only()
                .column(backfill::Column::Id)
                .column(backfill::Column::Email)
                .filter(sea_orm::sea_query::Expr::col(backfill::Column::Id).gt(after))
                .order_by_asc(backfill::Column::Id)
                .limit(500)
                .into_tuple()
                .all(db)
                .await?;
            if rows.is_empty() {
                break;
            }
            for (id, email) in rows {
                let mut attempt = 0;
                let username = loop {
                    let candidate = compallm_core::user::User::username_candidate(&email, attempt);
                    let existing = backfill::Entity::find()
                        .filter(
                            sea_orm::sea_query::Expr::col(backfill::Column::Username)
                                .eq(candidate.clone()),
                        )
                        .one(db)
                        .await?;
                    if existing.is_none() {
                        break candidate;
                    }
                    attempt += 1;
                };
                backfill::ActiveModel {
                    id: Set(id),
                    email: sea_orm::ActiveValue::NotSet,
                    username: Set(Some(username)),
                }
                .update(db)
                .await?;
                after = id;
            }
        }
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .modify_column(ColumnDef::new(Users::Username).text().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("users_username_unique")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::Username)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Username,
}

mod backfill {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub email: String,
        pub username: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
