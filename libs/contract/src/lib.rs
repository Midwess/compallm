pub mod provider;

pub use compallm_contract_macros::ContractSchema;

extern crate self as compallm_contract;

pub fn merge_constraints(target: &mut serde_json::Value, constraints: serde_json::Value) {
    target.as_object_mut().expect("schema object").extend(
        constraints
            .as_object()
            .expect("custom rule schema object")
            .clone(),
    );
}
