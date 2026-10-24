mod app;
mod controllers;
mod dtos;
mod tasks;
mod workers;

#[cfg(test)]
mod tests;

use app::App;
use clap::{Parser, Subcommand};
use compallm_postgres::Migrator;
use loco_rs::cli;
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Parser)]
#[command(name = "compallm-api")]
struct CompallmCli {
    #[command(subcommand)]
    command: CompallmCommand,
}

#[derive(Subcommand)]
enum CompallmCommand {
    /// Generate OpenAPI and the TypeScript API client.
    Generate,
}

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("api crate should live below the project root")
        .to_path_buf()
}

pub(crate) fn portal_root() -> PathBuf {
    project_root().join("portal")
}

pub(crate) fn node_binary(root: &Path, name: &str) -> PathBuf {
    let executable = if cfg!(windows) {
        format!("{name}.cmd")
    } else {
        name.to_owned()
    };
    root.join("node_modules").join(".bin").join(executable)
}

fn export_openapi(output: &Path) -> Result<(), Box<dyn Error>> {
    let document = serde_json::to_string_pretty(&controllers::openapi())?;
    fs::write(output, document)?;
    println!("wrote {}", output.display());
    Ok(())
}

pub(crate) fn generate_contract() -> Result<(), Box<dyn Error>> {
    let portal = portal_root();
    export_openapi(&portal.join("openapi.json"))?;

    let status = Command::new(node_binary(&portal, "orval"))
        .args(["--config", "orval.config.ts"])
        .current_dir(&portal)
        .status()?;

    if !status.success() {
        return Err(io::Error::other(format!("Orval exited with status {status}")).into());
    }

    println!("generated {}", portal.join("src/contract/api.ts").display());
    Ok(())
}

fn compallm_command() -> Option<CompallmCommand> {
    CompallmCli::try_parse().ok().map(|cli| cli.command)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Some(CompallmCommand::Generate) = compallm_command() {
        return generate_contract();
    }

    cli::main::<App, Migrator>()
        .await
        .map_err(|error| Box::new(error) as Box<dyn Error>)
}
