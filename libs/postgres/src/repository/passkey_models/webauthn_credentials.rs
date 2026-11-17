use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "webauthn_credentials", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub credential_id: Vec<u8>,
    #[sea_orm(column_type = "JsonBinary")]
    pub credential: Json,
    pub friendly_name: String,
    pub registered_at: i64,
    pub last_used_at: Option<i64>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
