#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]

use sea_orm_migration::prelude::*;

mod m20261024_000001_create_users;
mod m20261025_000001_create_organizations;
mod m20261025_000002_create_providers;
mod m20261026_000001_create_user_sessions;
mod m20261026_000002_create_models;
mod m20261026_000003_create_model_providers;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261024_000001_create_users::Migration),
            Box::new(m20261025_000001_create_organizations::Migration),
            Box::new(m20261025_000002_create_providers::Migration),
            Box::new(m20261026_000001_create_user_sessions::Migration),
            Box::new(m20261026_000002_create_models::Migration),
            Box::new(m20261026_000003_create_model_providers::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
