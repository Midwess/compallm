mod app;
mod tasks;

use app::App;
use compallm_postgres::Migrator;
use loco_rs::cli;
use std::error::Error;
#[cfg(debug_assertions)]
use std::path::{Path, PathBuf};

#[cfg(debug_assertions)]
pub(crate) fn node_binary(root: &Path, name: &str) -> PathBuf {
    let executable = if cfg!(windows) {
        format!("{name}.cmd")
    } else {
        name.to_owned()
    };
    root.join("node_modules").join(".bin").join(executable)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    cli::main::<App, Migrator>()
        .await
        .map_err(|error| Box::new(error) as Box<dyn Error>)
}
