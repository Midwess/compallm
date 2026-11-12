pub struct Role {
    pub id: juniper::ID,
    pub name: String,
    pub description: String,
    pub color: String,
}

impl From<compallm_core::organization::Role> for Role {
    fn from(role: compallm_core::organization::Role) -> Self {
        let color = role.color();
        Self {
            id: juniper::ID::new(role.id.to_string()),
            name: role.name,
            description: role.description,
            color,
        }
    }
}

#[juniper::graphql_object]
impl Role {
    fn id(&self) -> &juniper::ID {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn color(&self) -> &str {
        &self.color
    }
}
