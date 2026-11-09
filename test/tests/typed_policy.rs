use cedar_policy::{EntityUid, Schema, SchemaFragment};
use compallm_core::{
    auth::{Session, UserAuth},
    authorization::{
        SystemPolicy,
        policy::{Action, EntityType, Policy},
    },
    model::Model,
    organization::{Group, Organization, OrganizationMember, Role},
    provider::{Provider, ProviderModel},
    user::User,
};
use serde_json::Value;

fn normalize_references(value: &mut Value) {
    match value {
        Value::Object(object) => {
            if object.get("type").and_then(Value::as_str) == Some("EntityOrCommon") {
                if object.get("name").and_then(Value::as_str) == Some("Bool") {
                    object.insert("type".into(), Value::String("Boolean".into()));
                    object.remove("name");
                } else {
                    object.insert("type".into(), Value::String("Entity".into()));
                }
            }
            object.values_mut().for_each(normalize_references);
        }
        Value::Array(values) => values.iter_mut().for_each(normalize_references),
        _ => {}
    }
}

#[test]
fn typed_vocabulary_matches_current_schema() -> anyhow::Result<()> {
    // Compare Cedar's normalized fragment output, including attributes,
    // ancestry and every action's principal, resource and context.
    let fragments = [
        User::policy(),
        Organization::policy(),
        OrganizationMember::policy(),
        Role::policy(),
        Group::policy(),
        UserAuth::policy(),
        Provider::policy(),
        ProviderModel::policy(),
        Model::policy(),
        Session::policy(),
    ];
    assert_eq!(EntityType::ALL.len(), fragments.len());
    for (entity, legacy) in EntityType::ALL.iter().zip(fragments) {
        let typed: SchemaFragment = Policy::new(*entity).into();
        let mut legacy_json: Value = legacy.to_json_value()?;
        let typed_json: Value = typed.to_json_value()?;
        // Cedar text syntax retains an unresolved EntityOrCommon reference;
        // these definitions contain no common types, so it resolves to Entity.
        normalize_references(&mut legacy_json);
        assert_eq!(typed_json, legacy_json);
    }
    let typed = EntityType::ALL
        .iter()
        .map(|entity| SchemaFragment::from(Policy::new(*entity)));
    let schema = Schema::from_schema_fragments(typed)?;
    assert_eq!(schema.actions().count(), Action::ALL.len());
    assert_eq!(schema.entity_types().count(), EntityType::ALL.len());
    for action in Action::ALL {
        let uid: EntityUid = (*action).into();
        assert!(schema.actions().any(|registered| registered == &uid));
    }
    Ok(())
}
