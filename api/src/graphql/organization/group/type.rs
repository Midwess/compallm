use juniper::ID;

pub struct CreatedGroup {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub role_ids: Vec<ID>,
}

pub struct Group {
    pub id: ID,
    pub name: String,
    pub description: String,
}

impl From<compallm_core::organization::group::GroupInfo> for Group {
    fn from(group: compallm_core::organization::group::GroupInfo) -> Self {
        Self { id: ID::new(group.id.to_string()), name: group.name, description: group.description }
    }
}


#[juniper::graphql_object]
impl CreatedGroup {
    fn id(&self) -> &juniper::ID { &self.id }
    fn name(&self) -> &str { &self.name }
    fn description(&self) -> &str { &self.description }
    fn role_ids(&self) -> &[juniper::ID] { &self.role_ids }
}

#[juniper::graphql_object]
impl Group {
    fn id(&self) -> &juniper::ID { &self.id }
    fn name(&self) -> &str { &self.name }
    fn description(&self) -> &str { &self.description }
}
