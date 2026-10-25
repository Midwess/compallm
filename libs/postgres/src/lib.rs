#[cfg(feature = "cli")]
pub mod codegen;
pub mod connection;
pub mod id;
pub mod mappers;
pub mod migration;
pub mod postgres_connection;
pub mod repository;
/// Generated models are exposed through custom schema names.
///
/// ```
/// use compallm_postgres::schema::UserSchema;
/// let user: Option<UserSchema> = None;
/// assert!(user.is_none());
/// ```
///
/// ```compile_fail
/// use compallm_postgres::schema::users::Model;
/// ```
///
/// ```compile_fail
/// use compallm_postgres::schema::UserStatus;
/// ```
pub mod schema;

pub use connection::Connection;
pub use id::{
    DistributedIdError, from_bigint, init_id_generator, next_id, next_id_sync, to_bigint,
};
pub use migration::Migrator;
pub use schema::*;
pub use sea_orm;
