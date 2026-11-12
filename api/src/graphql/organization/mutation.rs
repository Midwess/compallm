pub struct Mutation;

#[juniper::graphql_object(name = "OrganizationMutation", context = crate::graphql::GraphqlContext, scalar = juniper::DefaultScalarValue)]
impl Mutation {
    fn roles() -> super::role::mutation::Mutation {
        super::role::mutation::Mutation
    }
    fn groups() -> super::group::mutation::Mutation {
        super::group::mutation::Mutation
    }
}
