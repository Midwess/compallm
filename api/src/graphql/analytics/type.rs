#[derive(juniper::GraphQLObject)]
pub struct AnalyticUserProfile {
    pub key: String,
    pub value: String,
}
impl From<compallm_core::analytics::UserProfileValue> for AnalyticUserProfile {
    fn from(v: compallm_core::analytics::UserProfileValue) -> Self {
        Self {
            key: v.key,
            value: v.value,
        }
    }
}
