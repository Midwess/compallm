#[derive(juniper::GraphQLObject)]
pub struct PermissionDecision {
    pub index: i32,
    pub allowed: bool,
}
