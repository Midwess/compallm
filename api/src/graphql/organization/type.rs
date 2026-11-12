use compallm_core::organization::Organization as CoreOrganization;

pub struct Organization {
    pub id: u64,
    pub name: String,
}

impl From<CoreOrganization> for Organization {
    fn from(organization: CoreOrganization) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
        }
    }
}

#[juniper::graphql_object(context = crate::graphql::GraphqlContext)]
impl Organization {
    fn id(&self) -> juniper::ID {
        juniper::ID::new(self.id.to_string())
    }
    fn name(&self) -> &str {
        &self.name
    }
}
