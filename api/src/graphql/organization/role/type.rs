pub struct Role {
    pub id: juniper::ID,
    pub name: String,
    pub description: String,
}

impl From<compallm_core::organization::Role> for Role {
    fn from(role: compallm_core::organization::Role) -> Self {
        Self { id: juniper::ID::new(role.id.to_string()), name: role.name, description: role.description }
    }
}


#[juniper::graphql_object]
impl Role {
    fn id(&self) -> &juniper::ID { &self.id }
    fn name(&self) -> &str { &self.name }
    fn description(&self) -> &str { &self.description }
}
