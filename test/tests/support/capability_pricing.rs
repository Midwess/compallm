#![allow(dead_code)]
#[path = "../../../libs/postgres/src/migration/m20261121_000052_capability_pricing/transform.rs"]
mod migration;
pub fn convert(value: serde_json::Value) -> serde_json::Value {
    migration::convert(&value)
}
