#[derive(Default, schemars::JsonSchema, juniper::GraphQLInputObject, compallm_query::IntoQuery)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberFilterInput {
    #[query(field = "display_name")]
    pub display_name: Option<compallm_query::filter::StringFilter>,
    pub email: Option<compallm_query::filter::StringFilter>,
    pub or: Option<Vec<MemberMatchInput>>,
}

#[derive(Default, schemars::JsonSchema, juniper::GraphQLInputObject, compallm_query::IntoQuery)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberMatchInput {
    #[query(field = "display_name")]
    pub display_name: Option<compallm_query::filter::StringFilter>,
    pub email: Option<compallm_query::filter::StringFilter>,
}

#[derive(schemars::JsonSchema, juniper::GraphQLInputObject)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberOrderInput {
    pub id: Option<compallm_query::OrderByDirection>,
    pub display_name: Option<compallm_query::OrderByDirection>,
    pub email: Option<compallm_query::OrderByDirection>,
}
impl TryFrom<MemberOrderInput> for compallm_query::OrderBy {
    type Error = compallm_query::order::OrderByError;
    fn try_from(value: MemberOrderInput) -> Result<Self, Self::Error> {
        let fields = [
            ("id", value.id),
            ("display_name", value.display_name),
            ("email", value.email),
        ];
        let mut specified = fields.into_iter().filter_map(|(field, direction)| {
            direction.map(|direction| Self {
                field: field.into(),
                direction,
            })
        });
        let result = specified.next().ok_or(Self::Error::ExactlyOneField)?;
        if specified.next().is_some() {
            return Err(Self::Error::ExactlyOneField);
        }
        Ok(result)
    }
}
