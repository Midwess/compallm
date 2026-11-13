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
    if std::env::args().nth(1).as_deref() == Some("generate") {
        if std::env::args().len() != 2 {
            return Err("generate accepts no arguments".into());
        }
        return compallm_api::generation::generate();
    }
    if std::env::args().nth(1).as_deref() == Some("storage") {
        if std::env::args().nth(2).as_deref() != Some("setup") || std::env::args().len() != 3 {
            return Err("usage: storage setup".into());
        }
        return compallm_api::storage::setup().await;
    }
    cli::main::<App, Migrator>()
        .await
        .map_err(|error| Box::new(error) as Box<dyn Error>)
}
