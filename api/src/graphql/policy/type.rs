use compallm_core::authorization::policy::Action;

pub struct PolicyAction {
    pub name: String,
    pub entity_type: String,
    pub resource_type: String,
}

impl From<Action> for PolicyAction {
    fn from(action: Action) -> Self {
        Self {
            name: action.name().into(),
            entity_type: action.owner().name().into(),
            resource_type: action.resource().name().into(),
        }
    }
}

#[juniper::graphql_object]
impl PolicyAction {
    fn name(&self) -> &str {
        &self.name
    }
    fn entity_type(&self) -> &str {
        &self.entity_type
    }
    fn resource_type(&self) -> &str {
        &self.resource_type
    }
}
