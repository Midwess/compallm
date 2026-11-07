use compallm_contract::provider::api::{
    ApiContracts, CreateProvidersRequest, PatchProviderRequest, ReplaceProviderRequest,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use validify::Validify;

fn accepts<T: DeserializeOwned + Validify>(value: Value) -> bool {
    serde_json::from_value::<T>(value).is_ok_and(|mut request| request.validify().is_ok())
}

#[test]
fn provider_management_acceptance_fixtures() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("provider-api-fixtures.json")).unwrap();
    for case in cases {
        let input = case["value"].clone();
        let valid = match case["kind"].as_str().unwrap() {
            "create" => accepts::<CreateProvidersRequest>(input),
            "replace" => accepts::<ReplaceProviderRequest>(input),
            "patch" => accepts::<PatchProviderRequest>(input),
            _ => panic!("unknown fixture"),
        };
        assert_eq!(valid, case["valid"].as_bool().unwrap(), "{}", case["name"]);
    }
}

#[test]
fn safe_reads_exclude_secrets_and_patch_fields_are_optional_but_not_nullable() {
    use utoipa::OpenApi;
    let api = serde_json::to_value(ApiContracts::openapi()).unwrap();
    let schemas = &api["components"]["schemas"];
    assert!(
        schemas["ProviderDetailResponse"]["properties"]
            .get("apiKey")
            .is_none()
    );
    assert!(
        schemas["ProviderHeaderMetadata"]["properties"]
            .get("value")
            .is_none()
    );
    assert_eq!(
        schemas["PatchProviderRequest"]["properties"]["apiKey"]["writeOnly"],
        true
    );
    assert!(schemas["PatchProviderRequest"]["required"].is_null());
    assert_eq!(
        schemas["PatchProviderRequest"]["properties"]["apiKey"]["type"],
        "string"
    );
    assert_eq!(
        schemas["ProviderHeaderUpdate"]["properties"]["key"]["maxLength"],
        128
    );
}
