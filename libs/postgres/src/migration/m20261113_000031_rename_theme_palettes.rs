use sea_orm::{ConnectionTrait, sea_query::extension::postgres::PgExpr};
use sea_orm_migration::prelude::*;
use serde_json::json;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        rename(
            manager,
            &[
                ("light", "light", "basic_white"),
                ("light", "pure_light", "marshmallow_white"),
                ("dark", "dark", "basic_dark"),
                ("dark", "magic_blue", "blueberry_dark"),
                ("dark", "classic_dark", "marshmallow_dark"),
            ],
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        rename(
            manager,
            &[
                ("light", "basic_white", "light"),
                ("light", "blueberry_white", "light"),
                ("light", "marshmallow_white", "pure_light"),
                ("dark", "basic_dark", "dark"),
                ("dark", "blueberry_dark", "magic_blue"),
                ("dark", "marshmallow_dark", "classic_dark"),
            ],
        )
        .await
    }
}

async fn rename(manager: &SchemaManager<'_>, values: &[(&str, &str, &str)]) -> Result<(), DbErr> {
    for &(appearance, old, new) in values {
        let update = Query::update()
            .table(Alias::new("user_preferences"))
            .value(
                Alias::new("theme"),
                Func::cust(Alias::new("jsonb_set"))
                    .arg(Expr::col(Alias::new("theme")))
                    .arg(Expr::val(vec![
                        "palettes".to_owned(),
                        appearance.to_owned(),
                    ]))
                    .arg(Expr::val(json!(new))),
            )
            .and_where(
                Expr::col(Alias::new("theme"))
                    .get_json_field("palettes")
                    .cast_json_field(appearance)
                    .eq(old),
            )
            .to_owned();
        manager.get_connection().execute(&update).await?;
    }
    Ok(())
}
