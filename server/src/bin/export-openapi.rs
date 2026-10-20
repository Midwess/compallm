use std::{env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("openapi.json"));
    let document = serde_json::to_string_pretty(&cosin_server::controllers::openapi())?;

    fs::write(&output, document)?;
    println!("wrote {}", output.display());

    Ok(())
}
