//! Offline generation: never constructs DI, boots Loco, or contacts storage.
use std::{error::Error, fs, path::Path, process::Command};

pub fn generate() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root");
    let contract = root.join("libs/contract");
    let schema = crate::graphql::build_schema(16, 256, 15_000);
    fs::write(contract.join("schema.graphql"), schema.as_sdl())?;
    fs::write(
        contract.join("validation.schema.json"),
        serde_json::to_string_pretty(&crate::graphql::validation_schema())? + "\n",
    )?;
    let status = Command::new("node")
        .arg(contract.join("generate.mjs"))
        .current_dir(&contract)
        .status()?;
    if !status.success() {
        return Err("contract generation failed".into());
    }
    Ok(())
}
