use cedar_policy::{Authorizer, Context, Decision, Entities, PolicySet, Request};
use compallm_core::{
    authorization::PolicySnapshot,
    organization::{Group, OrganizationMember, Role},
};

fn member(roles: Vec<Role>) -> OrganizationMember {
    OrganizationMember {
        organization_id: 7,
        user_id: 11,
        roles,
        groups: vec![],
    }
}

#[test]
fn default_sources_are_compact_and_preserve_authorization() -> anyhow::Result<()> {
    for role in Role::defaults(7) {
        let original = role.cedar_source.clone();
        let member = member(vec![role.clone()]);
        let snapshot = PolicySnapshot::try_from(&member)?;
        let compact = &snapshot.cedar_source;
        println!(
            "{}: {} -> {} source bytes; {} snapshot JSON bytes",
            role.name,
            original.len(),
            compact.len(),
            serde_json::to_vec(&snapshot)?.len()
        );
        assert!(compact.len() < original.len());
        assert_eq!(member.roles[0].cedar_source, original);
        let before = PolicySet::try_from(&role)?;
        let after = PolicySet::try_from(&snapshot)?;
        let structures = |set: &PolicySet| -> anyhow::Result<Vec<String>> {
            let mut values = set
                .policies()
                .map(|p| Ok(serde_json::to_string(&p.to_json()?)?))
                .collect::<anyhow::Result<Vec<_>>>()?;
            values.sort();
            Ok(values)
        };
        assert_eq!(structures(&before)?, structures(&after)?);
        for org in [7, 8] {
            for action in ["model.list", "organization.delete"] {
                let request = Request::new(
                    "Compallm::User::\"11\"".parse()?,
                    format!("Compallm::Action::\"{action}\"").parse()?,
                    format!("Compallm::Organization::\"{org}\"").parse()?,
                    Context::empty(),
                    None,
                )?;
                let entities = Entities::try_from(&member)?;
                let a = Authorizer::new();
                let expected = a.is_authorized(&request, &before, &entities).decision();
                assert_eq!(
                    a.is_authorized(&request, &after, &entities).decision(),
                    expected
                );
                if org == 8 {
                    assert_eq!(expected, Decision::Deny);
                }
            }
        }
    }
    Ok(())
}

#[test]
fn assignments_are_deterministic_and_sources_are_not_duplicated() -> anyhow::Result<()> {
    let roles = Role::defaults(7);
    let mut member = member(vec![roles[2].clone(), roles[0].clone(), roles[2].clone()]);
    member.groups = vec![Group {
        id: 8,
        organization_id: 7,
        name: "Team".into(),
        description: String::new(),
        roles: vec![roles[2].clone()],
    }];
    let snapshot = PolicySnapshot::try_from(&member)?;
    assert_eq!(snapshot.role_ids, vec![1, 3]);
    assert_eq!(snapshot.group_roles[&8], vec![3]);
    assert_eq!(PolicySet::try_from(&snapshot)?.policies().count(), 8);
    member.roles.reverse();
    assert_eq!(snapshot, PolicySnapshot::try_from(&member)?);
    let encoded = serde_json::to_string(&snapshot)?;
    assert_eq!(snapshot, serde_json::from_str(&encoded)?);
    assert!(!encoded.contains("authorization_version"));
    assert!(
        serde_json::from_str::<PolicySnapshot>(&encoded.replacen("{", "{\"unknown\":0,", 1))
            .is_err()
    );
    Ok(())
}

#[test]
fn invalid_or_foreign_authority_is_rejected() -> anyhow::Result<()> {
    let mut role = Role::defaults(8).remove(0);
    assert!(PolicySnapshot::try_from(&member(vec![role.clone()])).is_err());
    role.organization_id = 7;
    for source in [
        "invalid",
        "permit(principal, action == Compallm::Action::\"unknown\", resource);",
        "permit(principal == ?principal, action, resource);",
    ] {
        role.cedar_source = source.into();
        assert!(PolicySnapshot::try_from(&member(vec![role.clone()])).is_err());
    }
    let snapshot = PolicySnapshot::try_from(&member(vec![]))?;
    assert_eq!(PolicySet::try_from(&snapshot)?.policies().count(), 0);
    Ok(())
}

#[test]
fn quoted_content_annotations_and_forbids_survive() -> anyhow::Result<()> {
    let mut role = Role::defaults(7).remove(0);
    role.cedar_source = r#"// remove this comment
@note("tiếng Việt // keep  two spaces")
permit(principal in Compallm::Role::"7:1", action == Compallm::Action::"model.list", resource == Compallm::Organization::"7");
forbid(principal in Compallm::Role::"7:1", action == Compallm::Action::"model.list", resource == Compallm::Organization::"7") when { "a // b" == "a // b" };
"#.into();
    let snapshot = PolicySnapshot::try_from(&member(vec![role.clone()]))?;
    assert!(
        snapshot
            .cedar_source
            .contains("tiếng Việt // keep  two spaces")
    );
    let original = PolicySet::try_from(&role)?;
    let restored = PolicySet::try_from(&snapshot)?;
    assert_eq!(original.policies().count(), restored.policies().count());
    let request = Request::new(
        "Compallm::User::\"11\"".parse()?,
        "Compallm::Action::\"model.list\"".parse()?,
        "Compallm::Organization::\"7\"".parse()?,
        Context::empty(),
        None,
    )?;
    let entities = Entities::try_from(&member(vec![role]))?;
    assert_eq!(
        Authorizer::new()
            .is_authorized(&request, &restored, &entities)
            .decision(),
        Decision::Deny
    );
    Ok(())
}

#[test]
fn parallel_constructor_preserves_deterministic_authority() -> anyhow::Result<()> {
    let member = member(Role::defaults(7));
    let expected = PolicySnapshot::try_from(&member)?;
    for workers in [1, 2, 4] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()?;
        let actual = pool.install(|| PolicySnapshot::new(&member))?;
        assert_eq!(serde_json::to_vec(&actual)?, serde_json::to_vec(&expected)?);
        assert_eq!(
            PolicySet::try_from(&actual)?.policies().count(),
            PolicySet::try_from(&expected)?.policies().count()
        );
    }
    Ok(())
}

#[test]
fn merged_roles_deduplicate_and_inherited_forbid_overrides_direct_permit() -> anyhow::Result<()> {
    let permit = r#"permit(principal, action == Compallm::Action::"model.list", resource == Compallm::Organization::"7");"#;
    let forbid = r#"forbid(principal, action == Compallm::Action::"model.list", resource == Compallm::Organization::"7");"#;
    let role = |id, source: String| Role {
        id,
        organization_id: 7,
        name: format!("Role {id}"),
        description: String::new(),
        cedar_source: source,
    };
    let direct = role(10, permit.into());
    let inherited = role(
        20,
        format!("// same permit with different formatting\n{permit}\n{forbid}"),
    );
    let mut member = member(vec![direct]);
    member.groups = vec![Group {
        id: 8,
        organization_id: 7,
        name: "Team".into(),
        description: String::new(),
        roles: vec![inherited],
    }];
    let snapshot = PolicySnapshot::new(&member)?;
    let policies: PolicySet = snapshot.cedar_source.parse()?;
    assert_eq!(policies.policies().count(), 2);
    assert_eq!(snapshot.role_ids, vec![10]);
    assert_eq!(snapshot.group_roles[&8], vec![20]);
    let entities = Entities::try_from(&member)?;
    let request = Request::new(
        "Compallm::User::\"11\"".parse()?,
        "Compallm::Action::\"model.list\"".parse()?,
        "Compallm::Organization::\"7\"".parse()?,
        Context::empty(),
        None,
    )?;
    assert_eq!(
        Authorizer::new()
            .is_authorized(&request, &policies, &entities)
            .decision(),
        Decision::Deny
    );
    member.groups.clear();
    let without_forbid = PolicySet::try_from(&PolicySnapshot::new(&member)?)?;
    assert_eq!(
        Authorizer::new()
            .is_authorized(&request, &without_forbid, &Entities::try_from(&member)?)
            .decision(),
        Decision::Allow
    );
    Ok(())
}

#[test]
fn snapshot_work_and_output_limits_reject_without_truncation() {
    let mut role = Role::defaults(7).remove(2);
    role.cedar_source = "invalid".repeat(PolicySnapshot::MAX_SOURCE_BYTES / 7 + 1);
    assert!(
        PolicySnapshot::new(&member(vec![role]))
            .unwrap_err()
            .to_string()
            .contains("source limit")
    );
    let role = Role::defaults(7).remove(2);
    assert!(
        PolicySnapshot::new(&member(vec![
            role.clone();
            PolicySnapshot::MAX_ASSIGNMENTS + 1
        ]))
        .unwrap_err()
        .to_string()
        .contains("assignment limit")
    );
    let roles = (1..=PolicySnapshot::MAX_ROLES + 1)
        .map(|id| Role {
            id: id as u64,
            ..role.clone()
        })
        .collect();
    assert!(
        PolicySnapshot::new(&member(roles))
            .unwrap_err()
            .to_string()
            .contains("role limit")
    );
    let mut large = role;
    large.cedar_source = format!(
        "@note(\"{}\")\n{}",
        "x".repeat(PolicySnapshot::MAX_JSON_BYTES),
        large.cedar_source
    );
    assert!(
        PolicySnapshot::new(&member(vec![large]))
            .unwrap_err()
            .to_string()
            .contains("JSON limit")
    );
}
