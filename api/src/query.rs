use compallm_postgres::sea_orm::{EntityTrait, Select};

pub trait IntoQuery {
    type Table: EntityTrait;

    fn into_query(self) -> Select<Self::Table>;
}
