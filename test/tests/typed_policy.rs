use cedar_policy::{EntityUid, Schema, SchemaFragment};
use compallm_core::{
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

#[test]
fn typed_vocabulary_matches_current_schema() -> anyhow::Result<()> {
    // Compare generated fragments with the checked-in schema contract,
    // including attributes, ancestry and each action’s target and context.
    let fragments = [
        User::policy(),
        Organization::policy(),
        OrganizationMember::policy(),
        Role::policy(),
        Group::policy(),
        Provider::policy(),
        ProviderModel::policy(),
        Model::policy(),
    ];
    assert_eq!(EntityType::ALL.len(), fragments.len());
    let baseline: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/cedar_schema_baseline.json"))?;
    assert_eq!(baseline.len(), fragments.len());
    for ((entity, fragment), expected) in EntityType::ALL.iter().zip(fragments).zip(baseline) {
        let typed: SchemaFragment = Policy::new(*entity).into();
        let actual: Value = fragment.to_json_value()?;
        let typed_json: Value = typed.to_json_value()?;
        assert_eq!(actual, expected);
        assert_eq!(typed_json, expected);
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
