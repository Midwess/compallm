#![allow(elided_lifetimes_in_paths)]

use sea_orm_migration::prelude::*;

mod m20261104_000001_create_users;
mod m20261104_000002_create_organizations;
mod m20261104_000003_create_organization_members;
mod m20261104_000004_create_api_keys;
mod m20261104_000005_create_user_sessions;
mod m20261104_000006_create_providers;
mod m20261104_000007_create_models;
mod m20261104_000008_create_provider_models;
mod m20261104_000009_create_model_router;
mod m20261104_000010_create_model_usage;
mod m20261104_000011_create_model_usage_limit;
mod m20261104_000012_create_provider_token_usage;
mod m20261104_000013_create_provider_image_usage;
mod m20261104_000014_create_provider_audio_usage;
mod m20261104_000015_create_provider_video_usage;

mod m20261106_000016_create_provider_templates;
mod m20261106_000017_create_provider_model_templates;
mod m20261106_000018_seed_provider_templates;

mod m20261107_000019_create_organization_roles;

mod m20261107_000020_create_organization_groups;

mod m20261107_000021_create_organization_member_roles;

mod m20261107_000022_create_organization_group_members;

mod m20261107_000023_create_organization_group_roles;

mod m20261107_000024_add_session_authorization_version;

mod m20261107_000025_replace_membership_role_enum;

mod m20261108_000026_remove_reserved_role_ids;

mod m20261108_000022_create_lz4_dictionary;
mod m20261108_000023_seed_access_token_dictionary;
mod m20261108_000027_add_organization_owner_role;

mod m20261110_000028_restricted_reads;
mod m20261113_000029_create_user_preferences;

mod m20261113_000030_seed_user_preferences;
mod m20261113_000031_rename_theme_palettes;

mod m20261113_000032_add_user_avatar_path;

mod m20261114_000033_add_usernames;

mod m20261114_000034_add_organization_avatar_path;

mod m20261114_000035_member_directory;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261104_000001_create_users::Migration),
            Box::new(m20261104_000002_create_organizations::Migration),
            Box::new(m20261104_000003_create_organization_members::Migration),
            Box::new(m20261104_000004_create_api_keys::Migration),
            Box::new(m20261104_000005_create_user_sessions::Migration),
            Box::new(m20261104_000006_create_providers::Migration),
            Box::new(m20261104_000007_create_models::Migration),
            Box::new(m20261104_000008_create_provider_models::Migration),
            Box::new(m20261104_000009_create_model_router::Migration),
            Box::new(m20261104_000010_create_model_usage::Migration),
            Box::new(m20261104_000011_create_model_usage_limit::Migration),
            Box::new(m20261104_000012_create_provider_token_usage::Migration),
            Box::new(m20261104_000013_create_provider_image_usage::Migration),
            Box::new(m20261104_000014_create_provider_audio_usage::Migration),
            Box::new(m20261104_000015_create_provider_video_usage::Migration),
            Box::new(m20261106_000016_create_provider_templates::Migration),
            Box::new(m20261106_000017_create_provider_model_templates::Migration),
            Box::new(m20261106_000018_seed_provider_templates::Migration),
            Box::new(m20261107_000019_create_organization_roles::Migration),
            Box::new(m20261107_000020_create_organization_groups::Migration),
            Box::new(m20261107_000021_create_organization_member_roles::Migration),
            Box::new(m20261107_000022_create_organization_group_members::Migration),
            Box::new(m20261107_000023_create_organization_group_roles::Migration),
            Box::new(m20261107_000024_add_session_authorization_version::Migration),
            Box::new(m20261107_000025_replace_membership_role_enum::Migration),
            Box::new(m20261108_000026_remove_reserved_role_ids::Migration),
            Box::new(m20261108_000022_create_lz4_dictionary::Migration),
            Box::new(m20261108_000023_seed_access_token_dictionary::Migration),
            Box::new(m20261108_000027_add_organization_owner_role::Migration),
            Box::new(m20261110_000028_restricted_reads::Migration),
            Box::new(m20261113_000029_create_user_preferences::Migration),
            Box::new(m20261113_000030_seed_user_preferences::Migration),
            Box::new(m20261113_000031_rename_theme_palettes::Migration),
            Box::new(m20261113_000032_add_user_avatar_path::Migration),
            Box::new(m20261114_000033_add_usernames::Migration),
            Box::new(m20261114_000034_add_organization_avatar_path::Migration),
            Box::new(m20261114_000035_member_directory::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
