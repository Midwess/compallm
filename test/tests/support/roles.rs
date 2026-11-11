use compallm_core::organization::Role;

/// Rehydrate deterministic fixture records from the application-owned default policies.
/// Actual constructor generation is exercised separately; fixed test IDs keep assertions readable.
pub async fn default_roles(organization_id: u64, ids: [u64; 3]) -> Vec<Role> {
    compallm_postgres::init_id_generator("role-fixtures").unwrap();
    let mut roles = Role::defaults(organization_id).await.unwrap();
    let generated = roles.iter().map(|role| role.id).collect::<Vec<_>>();
    for (role, id) in roles.iter_mut().zip(ids) {
        for (old, new) in generated.iter().zip(ids) {
            role.cedar_source = role.cedar_source.replace(
                &format!("Compallm::Role::\"{organization_id}:{old}\""),
                &format!("Compallm::Role::\"{organization_id}:{new}\""),
            );
        }
        role.id = id;
    }
    roles
}
