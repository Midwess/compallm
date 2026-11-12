use compallm_core::authorization::policy::Action;

pub struct PolicyAction {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub entity_type: String,
    pub resource_type: String,
}

impl From<Action> for PolicyAction {
    fn from(action: Action) -> Self {
        Self {
            name: action.name().into(),
            display_name: action.display_name().into(),
            description: action.description().into(),
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
    fn display_name(&self) -> &str {
        &self.display_name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn entity_type(&self) -> &str {
        &self.entity_type
    }
    fn resource_type(&self) -> &str {
        &self.resource_type
    }
}
