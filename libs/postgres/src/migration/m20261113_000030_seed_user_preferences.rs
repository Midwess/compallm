use sea_orm::{
    ConnectionTrait,
    sea_query::{OnConflict, Query},
};
use sea_orm_migration::prelude::*;
use serde_json::json;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut select = Query::select();
        select
            .column((Alias::new("users"), Alias::new("id")))
            .from(Alias::new("users"));
        select.expr(Expr::val(json!({"home": "chat"})));
        select.expr(Expr::val(json!({"badgeStyle": "dot", "entries": [{"item": "providers", "visibility": "always"}, {"item": "models", "visibility": "always"}, {"item": "notebooks", "visibility": "always"}, {"item": "plugins", "visibility": "always"}]})));
        select.expr(Expr::val(json!({"nameFormat": "full_name"})));
        select.expr(Expr::val(json!({"firstDay": "monday"})));
        select.expr(Expr::val(
            json!({"mode": "system", "palettes": {"light": "light", "dark": "dark"}}),
        ));
        select.expr(Expr::val(json!({"family": "default", "size": "default"})));
        select.expr(Expr::val(
            json!({"underlineLinks": false, "reduceMotion": true, "pointerCursors": false}),
        ));
        select.expr(Expr::val(
            json!({"convertEmoticons": true, "sendShortcut": "modifier_enter"}),
        ));
        select.expr(Expr::val(
            json!({"theme": "follow_interface", "font": {"family": "default", "size": "px12"}}),
        ));

        let mut insert = Query::insert();
        insert
            .into_table(Alias::new("user_preferences"))
            .columns([
                Alias::new("user_id"),
                Alias::new("navigation"),
                Alias::new("sidebar"),
                Alias::new("member_display"),
                Alias::new("calendar"),
                Alias::new("theme"),
                Alias::new("typography"),
                Alias::new("accessibility"),
                Alias::new("message_composition"),
                Alias::new("code_appearance"),
            ])
            .select_from(select.to_owned())
            .map_err(|error| DbErr::Custom(error.to_string()))?
            .on_conflict(
                OnConflict::column(Alias::new("user_id"))
                    .do_nothing()
                    .to_owned(),
            );
        manager.get_connection().execute(&insert).await?;
        Ok(())
    }

    // Backfilled rows may contain later user edits and must survive downgrade.
    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
