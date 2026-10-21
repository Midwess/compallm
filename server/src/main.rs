mod app;
mod controllers;
mod dtos;
mod initializers;
mod tasks;
mod workers;

#[cfg(test)]
mod tests;

use app::App;
use clap::{Parser, Subcommand};
use cosin_postgres::Migrator;
use loco_rs::cli;
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Parser)]
#[command(name = "cosin-server")]
struct CosinCli {
    #[command(subcommand)]
    command: CosinCommand,
}

#[derive(Subcommand)]
enum CosinCommand {
    /// Generate OpenAPI and the TypeScript API client.
    Generate,
}

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("server crate should live below the project root")
        .to_path_buf()
}

fn export_openapi(output: &Path) -> Result<(), Box<dyn Error>> {
    let document = serde_json::to_string_pretty(&controllers::openapi())?;
    fs::write(output, document)?;
    println!("wrote {}", output.display());
    Ok(())
}

fn generate_contract() -> Result<(), Box<dyn Error>> {
    let root = project_root();
    export_openapi(&root.join("openapi.json"))?;

    let status = Command::new("pnpm")
        .args(["exec", "orval", "--config", "orval.config.ts"])
        .current_dir(&root)
        .status()?;

    if !status.success() {
        return Err(io::Error::other(format!("Orval exited with status {status}")).into());
    }

    println!("generated {}", root.join("src/contract/api.ts").display());
    Ok(())
}

fn cosin_command() -> Option<CosinCommand> {
    CosinCli::try_parse().ok().map(|cli| cli.command)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Some(CosinCommand::Generate) = cosin_command() {
        return generate_contract();
    }

    cli::main::<App, Migrator>()
        .await
        .map_err(|error| Box::new(error) as Box<dyn Error>)
}
