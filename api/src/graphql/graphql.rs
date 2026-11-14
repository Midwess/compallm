use super::{
    GraphqlContext, authentication,
    base_type::{Collection, Payload},
    error::Error,
    organization, policy,
};
use crate::http::{
    authentication::{AuthenticationContext, CookieDelivery, UserContext},
    authorization::AuthorizationContext,
};
use compallm_core::{
    authentication::AuthService, authorization::rbac::RbacService,
    persistent::organization::OrganizationRepository,
};
use juniper::{
    Arguments, DefaultScalarValue, EmptySubscription, ExecutionResult, Executor, GraphQLType,
    GraphQLValue, GraphQLValueAsync, ID, Registry, RootNode, graphql_object,
};
use schemars::JsonSchema;
use std::sync::Arc;

pub struct Query;

#[graphql_object(name = "Query", context = GraphqlContext, scalar = DefaultScalarValue)]
impl Query {
    async fn organization_profile(
        id: ID,
        #[graphql(context)] repository: &Box<dyn OrganizationRepository>,
        #[graphql(context)] database: &compallm_core::persistent::DbContext,
    ) -> Result<Option<Payload<organization::Organization>>, Error> {
        organization::query::profile(id, repository.as_ref(), database).await
    }

    /// Read an active user profile using verified user identity.
    async fn user_profile(
        id: ID,
        #[graphql(context)] user: &UserContext,
        #[graphql(context)] repository: &Box<dyn compallm_core::persistent::user::UserRepository>,
        #[graphql(context)] database: &compallm_core::persistent::DbContext,
    ) -> Result<Option<Payload<super::user::User>>, Error> {
        super::user::query::profile(id, user, repository.as_ref(), database).await
    }

    /// Return the signed-in user's personal preferences; missing rows use Core defaults without a write.
    async fn user_preferences(
        #[graphql(context)] user: &UserContext,
        #[graphql(context)] repository: &Box<
            dyn compallm_core::persistent::preference::user::UserPreferenceRepository,
        >,
        #[graphql(context)] database: &compallm_core::persistent::DbContext,
    ) -> Result<Payload<super::preference::user::UserPreferences>, Error> {
        super::preference::user::query::get(user, repository.as_ref(), database).await
    }

    /// Return the system-owned organization policy action catalog.
    async fn organization_policy_actions(
        #[graphql(context)] user: &UserContext,
    ) -> Result<Collection<policy::PolicyAction>, Error> {
        policy::query::list(user).await
    }

    /// Return the verified current session identity.
    async fn session(
        #[graphql(context)] user: &UserContext,
        #[graphql(context)] repository: &Box<dyn OrganizationRepository>,
        #[graphql(context)] database: &compallm_core::persistent::DbContext,
    ) -> Result<Payload<authentication::Session>, Error> {
        authentication::query::session(user, repository.as_ref(), database).await
    }

    /// Return the active organization from verified user authority.
    async fn organization(
        #[graphql(context)] user: &UserContext,
        #[graphql(context)] repository: &Box<dyn OrganizationRepository>,
        #[graphql(context)] database: &compallm_core::persistent::DbContext,
    ) -> Result<Payload<organization::Organization>, Error> {
        organization::query::get(user, repository.as_ref(), database).await
    }

    async fn members(
        filter: Option<organization::member::input::MemberFilterInput>,
        order_by: Option<Vec<organization::member::input::MemberOrderInput>>,
        first: Option<i32>,
        last: Option<i32>,
        before: Option<String>,
        after: Option<String>,
        offset: Option<i32>,
        #[graphql(context)] repository: &Box<dyn OrganizationRepository>,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Collection<organization::member::Member>, Error> {
        organization::member::query::list(
            filter,
            order_by,
            compallm_query::Pagination {
                first: first.or(if last.is_none() && before.is_none() {
                    Some(30)
                } else {
                    None
                }),
                last: last.or(if first.is_none() && before.is_some() {
                    Some(30)
                } else {
                    None
                }),
                before: before.map(compallm_query::Cursor),
                after: after.map(compallm_query::Cursor),
                offset,
            },
            repository,
            authority,
        )
        .await
    }

    /// Find visible role metadata in the active organization.
    async fn role(
        id: ID,
        #[graphql(context)] repository: &Box<dyn OrganizationRepository>,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<Payload<organization::role::Role>>, Error> {
        organization::role::query::get(id, repository, authority).await
    }

    /// Return the complete visible role metadata collection, ordered by ID.
    async fn roles(
        #[graphql(context)] repository: &Box<dyn OrganizationRepository>,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Collection<organization::role::Role>, Error> {
        organization::role::query::list(repository, authority).await
    }

    /// Find visible group metadata in the active organization.
    async fn group(
        id: ID,
        #[graphql(context)] repository: &Box<dyn OrganizationRepository>,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Option<Payload<organization::group::Group>>, Error> {
        organization::group::query::get(id, repository, authority).await
    }

    /// Return the complete visible group metadata collection, ordered by ID.
    async fn groups(
        #[graphql(context)] repository: &Box<dyn OrganizationRepository>,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Collection<organization::group::Group>, Error> {
        organization::group::query::list(repository, authority).await
    }
}

pub struct Mutation;

#[graphql_object(context = GraphqlContext, scalar = DefaultScalarValue)]
impl Mutation {
    async fn organization_profile_update(
        input: organization::input::OrganizationProfileUpdateInput,
        #[graphql(context)] authority: &AuthorizationContext,
        #[graphql(context)] service: &compallm_core::organization::profile_service::ProfileService,
        #[graphql(context)] database: &compallm_core::persistent::DbContext,
    ) -> Result<Payload<organization::Organization>, Error> {
        organization::mutation::update(input, authority, service, database).await
    }
    async fn generate_org_avatar_url(
        input: super::user::input::ImageUploadInput,
        #[graphql(context)] authority: &AuthorizationContext,
        #[graphql(context)] service: &compallm_core::organization::profile_service::ProfileService,
        #[graphql(context)] database: &compallm_core::persistent::DbContext,
    ) -> Result<Payload<super::user::AvatarUpload>, Error> {
        organization::mutation::generate(input, authority, service, database).await
    }

    async fn group_update(
        id: ID,
        input: organization::group::input::GroupUpdateInput,
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Payload<organization::group::Group>, Error> {
        organization::group::mutation::update(id, input, service, authority).await
    }
    async fn group_delete(
        id: ID,
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Payload<bool>, Error> {
        organization::group::mutation::delete(id, service, authority).await
    }

    /// Update the current user's display name and confirm an uploaded avatar.
    async fn user_profile_update(
        input: super::user::input::UserProfileUpdateInput,
        #[graphql(context)] user: &UserContext,
        #[graphql(context)] service: &compallm_core::user::ProfileService,
        #[graphql(context)] database: &compallm_core::persistent::DbContext,
    ) -> Result<Payload<super::user::User>, Error> {
        super::user::mutation::update(input, user, service, database).await
    }
    /// Authorize a direct JPEG/PNG/WebP avatar upload up to 5 MiB for five minutes.
    async fn generate_url_avatar_url(
        input: super::user::input::ImageUploadInput,
        #[graphql(context)] user: &UserContext,
        #[graphql(context)] service: &compallm_core::user::ProfileService,
        #[graphql(context)] database: &compallm_core::persistent::DbContext,
    ) -> Result<Payload<super::user::AvatarUpload>, Error> {
        super::user::mutation::generate_url_avatar_url(input, user, service, database).await
    }

    /// Atomically patch the signed-in user's preferences, preserving omitted fields.
    async fn user_preferences_update(
        input: super::preference::user::input::UserPreferencesPatchInput,
        #[graphql(context)] user: &UserContext,
        #[graphql(context)] service: &compallm_core::preference::UserPreferenceService,
        #[graphql(context)] database: &compallm_core::persistent::DbContext,
    ) -> Result<Payload<super::preference::user::UserPreferences>, Error> {
        super::preference::user::mutation::update(input, user, service, database).await
    }
    /// Restore selected preference groups, or ALL alone, to Core defaults.
    async fn user_preferences_reset(
        groups: Vec<super::preference::user::input::UserPreferenceGroup>,
        #[graphql(context)] user: &UserContext,
        #[graphql(context)] service: &compallm_core::preference::UserPreferenceService,
        #[graphql(context)] database: &compallm_core::persistent::DbContext,
    ) -> Result<Payload<super::preference::user::UserPreferences>, Error> {
        super::preference::user::mutation::reset(groups, user, service, database).await
    }

    /// Request a signup email challenge; reject existing accounts before sending.
    async fn email_signup_request(
        input: authentication::input::EmailInput,
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] identity: &AuthenticationContext,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<Payload<authentication::EmailChallenge>, Error> {
        authentication::mutation::signup_email_request(input, auth, identity, cookies).await
    }

    /// Confirm the verified signup challenge and create the account and session.
    async fn email_signup_confirm(
        input: authentication::input::EmailCodeInput,
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] identity: &AuthenticationContext,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<Payload<authentication::SessionCredentials>, Error> {
        authentication::mutation::signup_email_confirm(input, auth, identity, cookies).await
    }

    /// Request a signin email challenge for an existing active account.
    async fn email_signin_request(
        input: authentication::input::EmailInput,
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] identity: &AuthenticationContext,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<Payload<authentication::EmailChallenge>, Error> {
        authentication::mutation::signin_email_request(input, auth, identity, cookies).await
    }

    /// Confirm the verified signin challenge and create a scoped session.
    async fn email_signin_confirm(
        input: authentication::input::EmailCodeInput,
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] identity: &AuthenticationContext,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<Payload<authentication::SessionCredentials>, Error> {
        authentication::mutation::signin_email_confirm(input, auth, identity, cookies).await
    }

    /// Refresh using explicit refresh Bearer authority; leave browser cookies unchanged.
    async fn session_refresh(
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] identity: &AuthenticationContext,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<Payload<authentication::SessionCredentials>, Error> {
        authentication::mutation::session_refresh(auth, identity, cookies).await
    }
    /// Revoke the verified session and clear its browser cookies after success.
    async fn session_revoke(
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] identity: &AuthenticationContext,
        #[graphql(context)] user: &UserContext,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<Payload<bool>, Error> {
        authentication::mutation::session_revoke(auth, identity, user, cookies).await
    }

    /// Create a role in the verified active organization.
    async fn role_create(
        input: organization::role::input::RoleCreateInput,
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Payload<organization::role::Role>, Error> {
        organization::role::mutation::create(input, service, authority).await
    }

    async fn role_delete(
        id: ID,
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Payload<bool>, Error> {
        organization::role::mutation::delete(id, service, authority).await
    }

    async fn role_update(
        id: ID,
        input: organization::role::input::RoleUpdateInput,
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Payload<organization::role::Role>, Error> {
        organization::role::mutation::update(id, input, service, authority).await
    }

    /// Create a group and its initial role assignments in the verified active organization.
    async fn group_create(
        input: organization::group::input::GroupCreateInput,
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Payload<organization::group::Group>, Error> {
        organization::group::mutation::create(input, service, authority).await
    }
}

pub type ApplicationSchema =
    Arc<RootNode<RegisteredQuery, Mutation, EmptySubscription<GraphqlContext>>>;

/// Construct the shared schema without storing any caller or service state.
pub fn build_schema(max_depth: usize, max_fields: usize, max_tokens: usize) -> ApplicationSchema {
    Arc::new(
        RootNode::new(RegisteredQuery, Mutation, EmptySubscription::new())
            .limit_depth(max_depth)
            .limit_fields(max_fields)
            .limit_tokens(max_tokens),
    )
}

pub struct RegisteredQuery;

// Juniper discovers types through root fields. Register reusable contracts in
// that same schema without inventing executable operations to make them reachable.
impl GraphQLType for RegisteredQuery {
    fn name(info: &()) -> Option<juniper::ArcStr> {
        Query::name(info)
    }

    fn meta(info: &(), registry: &mut Registry) -> juniper::meta::MetaType {
        use super::{
            authentication,
            provider::{catalog, input},
        };
        registry.get_type::<authentication::input::EmailInput>(&());
        registry.get_type::<authentication::input::EmailCodeInput>(&());
        registry.get_type::<input::ProvidersCreateInput>(&());
        registry.get_type::<input::ProviderReplaceInput>(&());
        registry.get_type::<input::ProviderPatchInput>(&());
        registry.get_type::<catalog::ProviderTemplate>(&());
        registry.get_type::<super::user::input::ProfileUpdateInput>(&());
        registry.get_type::<super::user::input::ImageUploadInput>(&());
        registry.get_type::<super::organization::input::OrganizationSettingsUpdateInput>(&());
        registry.get_type::<super::chat::input::ChatCreateInput>(&());
        registry.get_type::<super::user::input::CredentialCreateInput>(&());
        registry.get_type::<super::organization::input::SpendingLimitInput>(&());
        registry.get_type::<super::organization::input::ReportingScheduleInput>(&());
        Query::meta(info, registry)
    }
}

impl GraphQLValue for RegisteredQuery {
    type Context = GraphqlContext;
    type TypeInfo = ();

    fn type_name(&self, info: &()) -> Option<juniper::ArcStr> {
        Query.type_name(info)
    }

    fn concrete_type_name(&self, context: &Self::Context, info: &()) -> String {
        Query.concrete_type_name(context, info)
    }

    fn resolve_field(
        &self,
        info: &(),
        field_name: &str,
        arguments: &Arguments,
        executor: &Executor<Self::Context>,
    ) -> ExecutionResult {
        Query.resolve_field(info, field_name, arguments, executor)
    }
}

impl GraphQLValueAsync for RegisteredQuery {
    fn resolve_field_async<'a>(
        &'a self,
        info: &'a (),
        field_name: &'a str,
        arguments: &'a Arguments,
        executor: &'a Executor<Self::Context>,
    ) -> juniper::BoxFuture<'a, ExecutionResult> {
        Query.resolve_field_async(info, field_name, arguments, executor)
    }
}

#[derive(JsonSchema)]
#[allow(dead_code)]
struct Inputs {
    user_preferences_patch: super::preference::user::input::UserPreferencesPatchInput,
    credential_create: super::user::input::CredentialCreateInput,
    spending_limit: super::organization::input::SpendingLimitInput,
    reporting_schedule: super::organization::input::ReportingScheduleInput,
    organization_profile_update: organization::input::OrganizationProfileUpdateInput,
    user_profile_update: super::user::input::UserProfileUpdateInput,
    form_0: super::user::input::ProfileUpdateInput,
    form_2: super::user::input::ImageUploadInput,
    form_3: super::organization::input::OrganizationSettingsUpdateInput,
    form_4: super::chat::input::ChatCreateInput,

    role_create: super::organization::role::input::RoleCreateInput,
    role_update: super::organization::role::input::RoleUpdateInput,
    group_create: super::organization::group::input::GroupCreateInput,
    group_update: super::organization::group::input::GroupUpdateInput,
    member_filter: organization::member::input::MemberFilterInput,
    member_order: organization::member::input::MemberOrderInput,
    email: super::authentication::input::EmailInput,
    code: super::authentication::input::EmailCodeInput,
    provider_create: super::provider::input::ProvidersCreateInput,
    provider_replace: super::provider::input::ProviderReplaceInput,
    provider_patch: super::provider::input::ProviderPatchInput,
}

pub(crate) fn validation_schema() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(Inputs))
        .expect("serializable GraphQL validation schema")
}
