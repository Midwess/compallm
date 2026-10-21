use std::num::TryFromIntError;

pub use hero::distributed_id::generator::DistributedIdError;

/// Initializes Hero's scoped Snowflake generator for one application instance.
pub fn init_id_generator(app_name: impl Into<String>) -> Result<(), DistributedIdError> {
    match hero::distributed_id::init_scoped_id_generator(app_name) {
        Ok(()) | Err(DistributedIdError::AlreadyInitialized) => Ok(()),
        Err(error) => Err(error),
    }
}

pub async fn next_id() -> Result<u64, DistributedIdError> {
    hero::distributed_id::try_gen_id().await
}

pub fn next_id_sync() -> Result<u64, DistributedIdError> {
    hero::distributed_id::try_gen_id_sync()
}

pub fn to_bigint(id: u64) -> Result<i64, TryFromIntError> {
    i64::try_from(id)
}

pub fn from_bigint(id: i64) -> Result<u64, TryFromIntError> {
    u64::try_from(id)
}
