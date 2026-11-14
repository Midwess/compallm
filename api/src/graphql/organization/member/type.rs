pub struct Member(pub compallm_core::organization::group::MemberInfo);
impl From<compallm_core::organization::group::MemberInfo> for Member {
    fn from(value: compallm_core::organization::group::MemberInfo) -> Self {
        Self(value)
    }
}
#[juniper::graphql_object]
impl Member {
    fn id(&self) -> juniper::ID {
        juniper::ID::new(self.0.id.to_string())
    }
    fn display_name(&self) -> &str {
        &self.0.display_name
    }
    fn email(&self) -> &str {
        &self.0.email
    }
    fn available(&self) -> bool {
        self.0.available
    }
}
