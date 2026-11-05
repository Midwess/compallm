use compallm_contract::provider::{
    ProviderFields, ProviderHeader, ProviderHeaders, ProviderInputRules, ProviderModelFields,
    ProviderModels,
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use validify::Validate;

fn valid<T: DeserializeOwned + Validate>(value: Value) -> bool {
    serde_json::from_value::<T>(value).is_ok_and(|v| v.validate().is_ok())
}

#[test]
fn shared_input_acceptance_fixtures() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("input-contract-fixtures.json")).unwrap();
    for (index, case) in cases.into_iter().enumerate() {
        let value = case["value"].clone();
        let kind = case["kind"].as_str().unwrap();
        let accepted = match kind {
            "header" => valid::<ProviderHeader>(value),
            "headerKey" | "headerValue" => {
                let mut input = json!({"key": "X-Test", "value": ""});
                input[if kind == "headerKey" { "key" } else { "value" }] = value;
                valid::<ProviderHeader>(input)
            }
            "name" | "baseUrl" | "apiKey" => {
                let mut input = json!({"name": "Provider", "baseUrl": "https://example.com", "apiKey": "secret"});
                input[kind] = value;
                valid::<ProviderFields>(input)
            }
            "modelName" | "path" | "price" | "context" => {
                let mut input = json!({"name": "model", "path": "/responses", "maxContextTokens": 1, "inputPrice": "0", "outputPrice": "0", "cachedPrice": "0"});
                input[match kind {
                    "modelName" => "name",
                    "price" => "inputPrice",
                    "context" => "maxContextTokens",
                    _ => "path",
                }] = value;
                valid::<ProviderModelFields>(input)
            }
            "effort" | "authorization" => {
                let mut input = json!({"effortValue": "low", "authorization": "Bearer abc", "positiveInteger": "1"});
                input[if kind == "effort" {
                    "effortValue"
                } else {
                    "authorization"
                }] = value;
                valid::<ProviderInputRules>(input)
            }
            _ => panic!("unknown fixture kind"),
        };
        assert_eq!(
            accepted,
            case["valid"].as_bool().unwrap(),
            "fixture {index}: {kind}"
        );
    }
}

#[test]
fn contract_exports_validation_and_rejects_unknown_fields() {
    use utoipa::PartialSchema;
    let schema = serde_json::to_value(ProviderFields::schema()).unwrap();
    assert_eq!(schema["properties"]["name"]["maxLength"], 120);
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(schema["x-compallm-input"], true);
    assert!(!valid::<ProviderFields>(
        json!({"name":"Provider", "baseUrl":"https://example.com", "apiKey":"secret", "unknown":true})
    ));
}

#[test]
fn collections_validate_entries_and_uniqueness() {
    assert!(valid::<ProviderHeaders>(
        json!({"entries": [{"key": "X-Key", "value": ""}]})
    ));
    assert!(!valid::<ProviderHeaders>(
        json!({"entries": [{"key": "X-Key", "value": ""}, {"key": "x-key", "value": ""}]})
    ));
    assert!(!valid::<ProviderHeaders>(
        json!({"entries": [{"key": "Authorization", "value": "Bearer "}]})
    ));
    assert!(!valid::<ProviderModels>(json!({"entries": []})));
    let model = json!({"name":"model", "path":"/responses", "maxContextTokens":1, "inputPrice":"0", "outputPrice":"0", "cachedPrice":"0"});
    assert!(valid::<ProviderModels>(json!({"entries": [model.clone()]})));
    assert!(!valid::<ProviderModels>(
        json!({"entries": [model.clone(), model]})
    ));
}
