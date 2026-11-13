use compallm_storage::{S3Settings, S3Storage};
use loco_rs::environment::{Environment, resolve_from_env};
use serde::Deserialize;

#[derive(Deserialize)]
struct Settings {
    s3: S3Settings,
}

/// Load configured storage without initializing application services.
pub fn settings_from_environment() -> Result<S3Settings, Box<dyn std::error::Error>> {
    let config = Environment::from(resolve_from_env()).load()?;
    let settings: Settings = serde_json::from_value(config.settings.ok_or("missing S3 settings")?)?;
    if !settings.s3.enabled {
        return Err("S3 is disabled".into());
    }
    Ok(settings.s3)
}

/// Provision storage without booting the application or opening its database.
pub async fn setup() -> Result<(), Box<dyn std::error::Error>> {
    S3Storage::new(settings_from_environment()?)?
        .setup()
        .await?;
    println!("S3 bucket, CORS, and avatar delivery policy configured");
    Ok(())
}
