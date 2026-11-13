//! Offline generation: never constructs DI, boots Loco, or contacts storage.
use std::{error::Error, fs, path::Path, process::Command};

pub fn generate() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root");
    let contract = root.join("libs/contract");
    let schema = crate::graphql::build_schema(24, 512, 15_000);
    fs::write(contract.join("schema.graphql"), schema.as_sdl())?;
    fs::write(
        contract.join("validation.schema.json"),
        serde_json::to_string_pretty(&crate::graphql::validation_schema())? + "\n",
    )?;
    generate_preference_defaults(&contract)?;
    let status = Command::new("node")
        .arg(contract.join("generate.mjs"))
        .current_dir(&contract)
        .status()?;
    if !status.success() {
        return Err("contract generation failed".into());
    }
    Ok(())
}

fn generate_preference_defaults(contract: &Path) -> Result<(), Box<dyn Error>> {
    use crate::graphql::preference::user::input::PreferenceDestination;
    use compallm_core::preference::{UserPreferences, navigation::NavigationPreferences};
    use validify::Validate;

    fn wire_enums(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::String(value) => *value = value.to_uppercase(),
            serde_json::Value::Array(values) => values.iter_mut().for_each(wire_enums),
            serde_json::Value::Object(values) => values.values_mut().for_each(wire_enums),
            _ => {}
        }
    }

    let mut defaults = serde_json::to_value(UserPreferences::default())?;
    wire_enums(&mut defaults);
    let destinations = serde_json::to_value(schemars::schema_for!(PreferenceDestination))?;
    let home: Vec<_> = destinations["enum"]
        .as_array()
        .ok_or("missing preference destinations")?
        .iter()
        .filter_map(|value| {
            let destination: PreferenceDestination = serde_json::from_value(value.clone()).ok()?;
            NavigationPreferences {
                home: destination.into(),
            }
            .validate()
            .is_ok()
            .then_some(value.clone())
        })
        .collect();
    let sidebar: Vec<_> = defaults["sidebar"]["entries"]
        .as_array()
        .ok_or("missing default sidebar entries")?
        .iter()
        .map(|entry| entry["item"].clone())
        .collect();
    fs::write(
        contract.join("preference.ts"),
        format!(
            "// Generated from Rust. Do not edit; run cargo run -p compallm-api -- generate.\n\
             import type {{ PreferenceDestination, UserPreferences }} from './graphql.ts'\n\
             type PreferenceValues<T> = T extends readonly (infer U)[] ? readonly PreferenceValues<U>[] : T extends object ? {{ [K in keyof T as K extends '__typename' ? never : K]: PreferenceValues<T[K]> }} : T\n\
             export const userPreferenceDefaults = {} as const satisfies PreferenceValues<UserPreferences>\n\
             export const userPreferenceHomeDestinations = {} as const satisfies readonly PreferenceDestination[]\n\
             export const userPreferenceSidebarDestinations = {} as const satisfies readonly PreferenceDestination[]\n",
            serde_json::to_string_pretty(&defaults)?,
            serde_json::to_string(&home)?,
            serde_json::to_string(&sidebar)?,
        ),
    )?;
    Ok(())
}
