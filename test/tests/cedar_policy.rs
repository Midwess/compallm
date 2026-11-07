use std::str::FromStr;

use cedar_policy::PolicySet;

#[test]
fn cedar() {
    let policies_src = r#"
        permit(
            principal in Role::"viewer",
            action == Action::"view",
            resource is Document
        );

        forbid(
            principal,
            action == Action::"edit",
            resource is Document
        );
    "#;

    let policy_set = PolicySet::from_str(policies_src).unwrap();
    for template in policy_set.policies() {
        println!("{:?}", template.action_constraint());
    }
}
