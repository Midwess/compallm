#[derive(Default, schemars::JsonSchema, juniper::GraphQLInputObject, compallm_query::IntoQuery)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderTemplateFilterInput {
    pub name: Option<compallm_query::filter::StringFilter>,
}
#[derive(schemars::JsonSchema, juniper::GraphQLInputObject)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderTemplateOrderInput {
    pub name: Option<compallm_query::OrderByDirection>,
    pub id: Option<compallm_query::OrderByDirection>,
}
impl TryFrom<ProviderTemplateOrderInput> for compallm_query::OrderBy {
    type Error = compallm_query::order::OrderByError;
    fn try_from(value: ProviderTemplateOrderInput) -> Result<Self, Self::Error> {
        let mut fields = [("name", value.name), ("id", value.id)]
            .into_iter()
            .filter_map(|(field, direction)| {
                direction.map(|direction| Self {
                    field: field.into(),
                    direction,
                })
            });
        let result = fields.next().ok_or(Self::Error::ExactlyOneField)?;
        if fields.next().is_some() {
            return Err(Self::Error::ExactlyOneField);
        }
        Ok(result)
    }
}
