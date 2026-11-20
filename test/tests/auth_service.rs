#[path = "support/onboarding.rs"]
mod onboarding_support;
#[path = "support/permission.rs"]
mod organization_permission_fixture;
#[path = "support/roles.rs"]
mod role_fixture;
#[path = "support/token_codec.rs"]
mod test_codec;
#[path = "support/token_pair.rs"]
mod token_pair;
use role_fixture::default_roles;

#[path = "support/session.rs"]
mod session_fixture;
use session_fixture::stored_session;

use compallm_postgres::postgres_connection::PostgresConnection;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use compallm_core::{
    authentication::{
        AccessToken, AuthError, AuthService, EmailOtpAccessToken, EmailOtpChallenge, RefreshToken,
        Session, TokenLifetimes, UserAccessToken, UserTokens,
    },
    authorization::PolicySnapshot,
    organization::{CreationService, Organization, OrganizationMember, Role},
    persistent::{
        DbContext,
        organization::{OrganizationRepository, OrganizationRepositoryError},
        session::{SessionRepository, SessionRepositoryError},
        user::{UserRepository, UserRepositoryError},
    },
    user::{RegistrationService, User, UserStatus},
};
use sea_orm::{DatabaseConnection, DbBackend, DbErr, MockDatabase, Statement, Transaction};

thread_local! {
    static ROOT_CONNECTION: std::cell::RefCell<Option<Arc<dyn compallm_core::persistent::Connection>>> = const { std::cell::RefCell::new(None) };
}

struct TestRoot;

impl TestRoot {
    fn set(connection: Arc<dyn compallm_core::persistent::Connection>) {
        static INITIALIZE: std::sync::Once = std::sync::Once::new();
        INITIALIZE.call_once(|| DbContext::initialize_default(Arc::new(Self)).unwrap());
        ROOT_CONNECTION.with(|root| *root.borrow_mut() = Some(connection));
    }
}

#[async_trait::async_trait]
impl compallm_core::persistent::Connection for TestRoot {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn is_transaction(&self) -> bool {
        false
    }
    fn scoped(&self) -> Arc<dyn compallm_core::persistent::Connection> {
        ROOT_CONNECTION.with(|root| root.borrow().as_ref().unwrap().scoped())
    }
    fn restrict(
        &self,
        _: compallm_core::persistent::PermissionLoader,
    ) -> Result<(), compallm_core::persistent::ConnectionError> {
        unreachable!()
    }
    fn ensure_open(&self) -> Result<(), compallm_core::persistent::ConnectionError> {
        Ok(())
    }
    async fn begin(
        &self,
    ) -> Result<
        Arc<dyn compallm_core::persistent::Connection>,
        compallm_core::persistent::ConnectionError,
    > {
        unreachable!()
    }
    async fn commit(&self) -> Result<(), compallm_core::persistent::ConnectionError> {
        unreachable!()
    }
    async fn rollback(&self) -> Result<(), compallm_core::persistent::ConnectionError> {
        unreachable!()
    }
    fn cancel(&self) {
        unreachable!()
    }
}

const SECRET: &str = "test-signing-secret-at-least-32-bytes";
const REFRESH_LIFETIME: Duration = Duration::from_secs(30 * 24 * 60 * 60);

struct TestDictionary {
    dictionary: compallm_core::compression::DCompression,
    newer: Option<compallm_core::compression::DCompression>,
}

struct CountingDictionary {
    inner: TestDictionary,
    reads: Arc<std::sync::atomic::AtomicUsize>,
}

#[async_trait::async_trait]
impl compallm_core::persistent::dictionary::DictionaryRepository for CountingDictionary {
    async fn latest(
        &self,
        kind: compallm_core::compression::DictionaryType,
    ) -> anyhow::Result<Option<&compallm_core::compression::DCompression>> {
        self.reads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        compallm_core::persistent::dictionary::DictionaryRepository::latest(&self.inner, kind).await
    }
    async fn get(
        &self,
        kind: compallm_core::compression::DictionaryType,
        version: u32,
    ) -> anyhow::Result<Option<&compallm_core::compression::DCompression>> {
        self.reads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        compallm_core::persistent::dictionary::DictionaryRepository::get(&self.inner, kind, version)
            .await
    }
}

struct TestEmail;

#[async_trait::async_trait]
impl compallm_core::mail::EmailService for TestEmail {
    async fn send(
        &self,
        _: compallm_core::mail::EmailRequest,
    ) -> Result<(), compallm_core::mail::EmailError> {
        Ok(())
    }
}

trait TestTokenEncoding {
    fn encode_compressed(&self, secret: &str, lifetime: Duration) -> Result<String, AuthError>;
}

fn sign_unchecked(token: &AccessToken, lifetime: Duration) -> String {
    test_codec::sign_unchecked(token, SECRET, lifetime)
}

fn make_oversized(role: &mut Role) {
    let mut state = 0x123456789abcdefu64;
    let mut random = String::new();
    for _ in 0..500 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        random.push_str(&format!("{state:016x}"));
    }
    role.cedar_source = format!("@note(\"{random}\")\n{}", role.cedar_source);
}

impl TestTokenEncoding for AccessToken {
    fn encode_compressed(&self, secret: &str, lifetime: Duration) -> Result<String, AuthError> {
        test_codec::encode(self, secret, lifetime)
    }
}

#[async_trait::async_trait]
impl compallm_core::persistent::dictionary::DictionaryRepository for TestDictionary {
    async fn latest(
        &self,
        _: compallm_core::compression::DictionaryType,
    ) -> anyhow::Result<Option<&compallm_core::compression::DCompression>> {
        Ok(Some(self.newer.as_ref().unwrap_or(&self.dictionary)))
    }
    async fn get(
        &self,
        _: compallm_core::compression::DictionaryType,
        version: u32,
    ) -> anyhow::Result<Option<&compallm_core::compression::DCompression>> {
        Ok(self
            .newer
            .as_ref()
            .filter(|dictionary| dictionary.version == version)
            .or_else(|| (version == self.dictionary.version).then_some(&self.dictionary)))
    }
}

fn decode_token(encoded: &str, secret: &str) -> Result<AccessToken, AuthError> {
    test_codec::decode(encoded, secret)
}

fn test_challenge(email: &str, secret: &str) -> Result<EmailOtpChallenge, AuthError> {
    test_codec::challenge(email, secret)
}

#[derive(Default)]
struct State {
    taken_usernames: std::collections::BTreeSet<String>,
    created_user: Option<User>,
    user: Option<User>,
    preferences: std::collections::BTreeMap<u64, compallm_core::preference::UserPreferences>,
    contexts: Vec<usize>,
    created: Option<(String, String)>,
    fail_create: bool,
    fail_session: bool,
    fail_preferences: bool,
    change_after_authority_read: bool,
    full_default_policies: bool,
    fail_authority_read: bool,
    freshness_reads: usize,
    authority_reads: usize,
    fail_rotation: bool,
    competing_rotation: bool,
    logins: Vec<u64>,
    sessions: std::collections::BTreeMap<u64, Session>,
    organization: Option<Organization>,
    owner: Option<OrganizationMember>,
    fail_organization: bool,
    fail_membership: bool,
    memberships: Vec<OrganizationMember>,
    disabled_organizations: Vec<u64>,
}

#[derive(Clone)]
struct Repository(Arc<Mutex<State>>, DbContext);

#[async_trait::async_trait]
impl compallm_core::persistent::preference::user::UserPreferenceRepository for Repository {
    async fn find(
        &self,
        user_id: u64,
    ) -> Result<
        Option<compallm_core::preference::UserPreferences>,
        compallm_core::persistent::preference::user::UserPreferenceRepositoryError,
    > {
        Ok(self.0.lock().unwrap().preferences.get(&user_id).cloned())
    }
    async fn create_defaults(
        &self,
        user_id: u64,
    ) -> Result<(), compallm_core::persistent::preference::user::UserPreferenceRepositoryError>
    {
        let mut state = self.0.lock().unwrap();
        if state.fail_preferences {
            return Err(
                compallm_core::persistent::preference::user::UserPreferenceRepositoryError(
                    anyhow::anyhow!("unavailable"),
                ),
            );
        }
        state.preferences.entry(user_id).or_default();
        Ok(())
    }
    async fn update(
        &self,
        _: u64,
        _: &compallm_core::preference::patch::UserPreferencesPatch,
    ) -> Result<
        compallm_core::preference::UserPreferences,
        compallm_core::persistent::preference::user::UserPreferenceRepositoryError,
    > {
        unreachable!()
    }
    async fn reset(
        &self,
        _: u64,
        _: &[compallm_core::preference::patch::UserPreferenceGroup],
    ) -> Result<
        compallm_core::preference::UserPreferences,
        compallm_core::persistent::preference::user::UserPreferenceRepositoryError,
    > {
        unreachable!()
    }
}

struct OrganizationRepositoryStub(Arc<Mutex<State>>);

#[async_trait::async_trait]
impl OrganizationRepository for OrganizationRepositoryStub {
    async fn owner_role(&self, _: u64) -> Result<Role, OrganizationRepositoryError> {
        Err(OrganizationRepositoryError::NotFound)
    }
    async fn active_owner_ids(&self, _: u64) -> Result<Vec<u64>, OrganizationRepositoryError> {
        Ok(vec![])
    }
    async fn find_member_identity(
        &self,
        _: u64,
        _: u64,
    ) -> Result<Option<OrganizationMember>, OrganizationRepositoryError> {
        unreachable!()
    }
    async fn list_members(
        &self,
        _: Box<dyn compallm_query::IntoQuery>,
        _: compallm_query::Pagination,
        _: Vec<compallm_query::OrderBy>,
        _: &str,
        _: Option<u64>,
    ) -> Result<
        compallm_query::Connection<compallm_core::organization::group::MemberInfo>,
        OrganizationRepositoryError,
    > {
        unreachable!()
    }
    async fn group_member_ids(
        &self,
        _: u64,
        _: u64,
    ) -> Result<Vec<u64>, OrganizationRepositoryError> {
        unreachable!()
    }
    async fn group_member_count(&self, _: u64, _: u64) -> Result<u64, OrganizationRepositoryError> {
        unreachable!()
    }
    async fn save_group_changes(
        &self,
        _: &compallm_core::organization::Group,
        _: &compallm_core::organization::group::GroupChanges,
    ) -> Result<Vec<u64>, OrganizationRepositoryError> {
        unreachable!()
    }
    async fn delete_group(&self, _: u64, _: u64) -> Result<(), OrganizationRepositoryError> {
        unreachable!()
    }

    async fn find_by_id(
        &self,
        organization_id: u64,
    ) -> Result<Option<Organization>, OrganizationRepositoryError> {
        let state = self.0.lock().unwrap();
        if state.disabled_organizations.contains(&organization_id) {
            return Ok(None);
        }
        if let Some(org) = state
            .organization
            .as_ref()
            .filter(|org| org.id == organization_id)
        {
            return Ok(Some(org.clone()));
        }
        Ok(state
            .memberships
            .iter()
            .find(|member| member.organization_id == organization_id)
            .map(|_| Organization {
                id: organization_id,
                name: if organization_id == 84 {
                    "Research team".into()
                } else {
                    format!("Organization {organization_id}")
                },
                avatar_path: None,
                status: compallm_core::organization::OrganizationStatus::Active,
            }))
    }
    async fn update_profile(
        &self,
        organization_id: u64,
        name: Option<String>,
        avatar_path: Option<String>,
    ) -> Result<Organization, OrganizationRepositoryError> {
        let mut state = self.0.lock().unwrap();
        let org = state
            .organization
            .as_mut()
            .filter(|org| org.id == organization_id)
            .ok_or(OrganizationRepositoryError::NotFound)?;
        if let Some(name) = name {
            org.name = name;
        }
        if let Some(path) = avatar_path {
            org.avatar_path = Some(path);
        }
        Ok(org.clone())
    }

    fn supports_role_policy(&self, _: &str) -> bool {
        true
    }
    async fn update_role(
        &self,
        _: u64,
        _: u64,
        _: Option<String>,
        _: Option<String>,
        _: Option<String>,
    ) -> Result<(), OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn lock_role(&self, _: u64, _: u64) -> Result<Role, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn lock_role_groups(&self, _: u64, _: u64) -> Result<bool, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn lock_group(&self, _: u64, _: u64) -> Result<(), OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn role_assignees(
        &self,
        _: u64,
        _: u64,
    ) -> Result<Vec<u64>, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn delete_role(&self, _: u64, _: u64) -> Result<(), OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn find_roles_for_assignment(
        &self,
        organization_id: u64,
        role_ids: &[u64],
    ) -> Result<Vec<Role>, OrganizationRepositoryError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .memberships
            .iter()
            .filter(|member| member.organization_id == organization_id)
            .map(|member| &member.role)
            .filter(|role| role_ids.contains(&role.id))
            .cloned()
            .collect())
    }
    async fn create_group(
        &self,
        _: compallm_core::organization::Group,
    ) -> Result<compallm_core::organization::Group, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn find_role(&self, _: u64, _: u64) -> Result<Option<Role>, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn list_role_info(&self) -> Result<Vec<Role>, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn list_group_info(
        &self,
    ) -> Result<Vec<compallm_core::organization::group::GroupInfo>, OrganizationRepositoryError>
    {
        unimplemented!()
    }
    async fn find_group_info(
        &self,
        _: u64,
    ) -> Result<Option<compallm_core::organization::group::GroupInfo>, OrganizationRepositoryError>
    {
        unimplemented!()
    }
    async fn find_group(
        &self,
        _: u64,
        _: u64,
    ) -> Result<Option<compallm_core::organization::Group>, OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn save_member(
        &self,
        _member: &OrganizationMember,
        _previous: &OrganizationMember,
    ) -> Result<(), OrganizationRepositoryError> {
        unimplemented!()
    }
    async fn find_membership(
        &self,
        organization_id: u64,
        user_id: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError> {
        DbContext::current()?;
        let state = self.0.lock().unwrap();
        Ok(state
            .memberships
            .iter()
            .find(|member| {
                member.user_id == user_id
                    && member.organization_id == organization_id
                    && !state
                        .disabled_organizations
                        .contains(&member.organization_id)
          })
            .cloned()
            .map(|mut member| {
                if !state.full_default_policies {
                    // Synthetic bounded policies for transport/lifecycle tests; full defaults have dedicated rejection tests.
                    for role in std::iter::once(&mut member.role) {
                        role.cedar_source = format!(r#"permit(principal in Compallm::Role::"{}:{}", action == Compallm::Action::"model.read", resource == Compallm::Organization::"{}");"#, role.organization_id, role.id, role.organization_id);
                  }
              }
                let organization = state
                    .organization
                    .clone()
                    .filter(|o| o.id == member.organization_id)
                    .unwrap_or(Organization {
            avatar_path: None,
                        id: member.organization_id,
                        name: if member.organization_id == 84 {
                            "Research team".into()
                      } else {
                            format!("Organization {}", member.organization_id)
                      },
                        status: compallm_core::organization::OrganizationStatus::Active,
                  });
                (member, organization)
          }))
    }

    async fn find_first_membership(
        &self,
        user_id: u64,
    ) -> Result<Option<(OrganizationMember, Organization)>, OrganizationRepositoryError> {
        DbContext::current()?;
        let state = self.0.lock().unwrap();
        Ok(state
            .memberships
            .iter()
            .find(|member| {
                member.user_id == user_id
                    && !state
                        .disabled_organizations
                        .contains(&member.organization_id)
          })
            .cloned()
            .map(|mut member| {
                if !state.full_default_policies {
                    // Synthetic bounded policies for transport/lifecycle tests; full defaults have dedicated rejection tests.
                    for role in std::iter::once(&mut member.role) {
                        role.cedar_source = format!(r#"permit(principal in Compallm::Role::"{}:{}", action == Compallm::Action::"model.read", resource == Compallm::Organization::"{}");"#, role.organization_id, role.id, role.organization_id);
                  }
              }
                let organization = state
                    .organization
                    .clone()
                    .filter(|o| o.id == member.organization_id)
                    .unwrap_or(Organization {
            avatar_path: None,
                        id: member.organization_id,
                        name: if member.organization_id == 84 {
                            "Research team".into()
                      } else {
                            format!("Organization {}", member.organization_id)
                      },
                        status: compallm_core::organization::OrganizationStatus::Active,
                  });
                (member, organization)
          }))
    }

    async fn update_name(
        &self,
        organization_id: u64,
        name: &str,
    ) -> Result<Organization, OrganizationRepositoryError> {
        let mut state = self.0.lock().unwrap();
        let organization = state
            .organization
            .as_mut()
            .filter(|organization| organization.id == organization_id)
            .ok_or(OrganizationRepositoryError::NotFound)?;
        organization.name = name.into();
        Ok(organization.clone())
    }
    async fn create(
        &self,
        organization: Organization,
    ) -> Result<Organization, OrganizationRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        if state.fail_organization {
            return Err(OrganizationRepositoryError::database(anyhow::anyhow!(
                "organization insert failed"
            )));
        }
        state.organization = Some(organization.clone());
        Ok(organization)
    }

    async fn create_roles(&self, _roles: Vec<Role>) -> Result<(), OrganizationRepositoryError> {
        Ok(())
    }
    async fn set_default_member_role(
        &self,
        _: u64,
        _: u64,
    ) -> Result<(), OrganizationRepositoryError> {
        Ok(())
    }
    async fn add_member(
        &self,
        member: OrganizationMember,
    ) -> Result<OrganizationMember, OrganizationRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        if state.fail_membership {
            return Err(OrganizationRepositoryError::database(anyhow::anyhow!(
                "owner insert failed"
            )));
        }
        state.owner = Some(member.clone());
        state.memberships.push(member.clone());
        Ok(member)
    }
}

#[async_trait::async_trait]
impl UserRepository for Repository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        let mut state = self.0.lock().unwrap();
        state
            .contexts
            .push(PostgresConnection::from_context(&DbContext::current()?)? as *const _ as usize);
        Ok(state.user.clone().filter(|u| u.email == email))
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<User>, UserRepositoryError> {
        let mut state = self.0.lock().unwrap();
        state
            .contexts
            .push(PostgresConnection::from_context(&DbContext::current()?)? as *const _ as usize);
        Ok(state.user.clone().filter(|u| u.id == id))
    }

    async fn update_profile(
        &self,
        user_id: u64,
        display_name: Option<String>,
        username: Option<String>,
        avatar_path: Option<String>,
    ) -> Result<User, UserRepositoryError> {
        let mut state = self.0.lock().unwrap();
        if username
            .as_ref()
            .is_some_and(|name| state.taken_usernames.contains(name))
        {
            return Err(UserRepositoryError::UsernameAlreadyExists);
        }
        let user = state
            .user
            .as_mut()
            .filter(|user| user.id == user_id && user.status == UserStatus::Active)
            .ok_or(UserRepositoryError::NotFound)?;
        if let Some(name) = display_name {
            user.display_name = name;
        }
        if let Some(username) = username {
            user.username = username;
        }
        if let Some(path) = avatar_path {
            user.avatar_path = Some(path);
        }
        Ok(user.clone())
    }

    async fn create(&self, user: User) -> Result<User, UserRepositoryError> {
        let mut state = self.0.lock().unwrap();
        state
            .contexts
            .push(PostgresConnection::from_context(&DbContext::current()?)? as *const _ as usize);
        if state.fail_create {
            return Err(UserRepositoryError::database(DbErr::Custom(
                "test storage failure".into(),
            )));
        }
        if state.taken_usernames.contains(&user.username) {
            return Err(UserRepositoryError::UsernameAlreadyExists);
        }
        state.created_user = Some(user.clone());
        state.created = Some((user.email.clone(), user.username.clone()));
        assert!(user.display_name.is_empty());
        Ok(user)
    }
}

struct ProfileStorage {
    uploaded: std::sync::atomic::AtomicBool,
    calls: Mutex<Vec<u64>>,
}

#[async_trait::async_trait]
impl compallm_core::persistent::storage::org_space::OrganizationStorage for ProfileStorage {
    fn avatar_path(&self, id: u64) -> String {
        compallm_core::persistent::storage::avatar_path(id)
    }
    async fn avatar_uploaded(
        &self,
        id: u64,
    ) -> Result<bool, compallm_core::persistent::storage::StorageError> {
        compallm_core::persistent::storage::user_space::UserStorageSpace::avatar_uploaded(self, id)
            .await
    }
    async fn generate_avatar_upload(
        &self,
        id: u64,
        constraints: &compallm_core::persistent::storage::AvatarUploadConstraints,
    ) -> Result<
        compallm_core::persistent::storage::AvatarUpload,
        compallm_core::persistent::storage::StorageError,
    > {
        compallm_core::persistent::storage::user_space::UserStorageSpace::generate_avatar_upload(
            self,
            id,
            constraints,
        )
        .await
    }
}

#[tokio::test]
async fn organization_profile_service_requires_settings_permission_and_checks_upload_before_writes()
{
    use compallm_core::authorization::{
        MemberPermission, condition::CheckPermission, policy::Action,
    };
    use compallm_core::organization::profile_service::{
        ProfileError, ProfilePatch, ProfileService,
    };
    use compallm_core::persistent::storage::StorageError;
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().organization = Some(Organization {
        id: 84,
        name: "Original".into(),
        status: compallm_core::organization::OrganizationStatus::Active,
        avatar_path: None,
    });
    let repository = Box::leak(Box::new(OrganizationRepositoryStub(fixture.repo.0.clone())));
    let storage = Box::leak(Box::new(ProfileStorage {
        uploaded: false.into(),
        calls: Mutex::new(Vec::new()),
    }));
    let service = ProfileService::new(repository, Some(storage));
    let denied = MemberPermission::try_from(
        fixture.access_for_session(stored_session(1, 7, 84, "Original")),
    )
    .unwrap();
    assert!(
        Action::OrganizationSettingsUpdate
            .check_permission(&denied)
            .is_err()
    );
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.update_profile(
                &denied,
                ProfilePatch {
                    name: Some("Denied".into()),
                    avatar_uploaded: true
                }
            ))
            .await,
        Err(ProfileError::Permission(_))
    ));
    assert!(storage.calls.lock().unwrap().is_empty());
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.generate_avatar_url(
                &denied,
                compallm_core::persistent::storage::AvatarMediaType::Png,
                1024
            ))
            .await,
        Err(ProfileError::Permission(_))
    ));
    let foreign =
        organization_permission_fixture::permission(85, &[Action::OrganizationSettingsUpdate]);
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.update_profile(
                &foreign,
                ProfilePatch {
                    name: Some("Foreign".into()),
                    avatar_uploaded: true
                }
            ))
            .await,
        Err(ProfileError::Persistence(
            OrganizationRepositoryError::NotFound
        ))
    ));
    assert!(storage.calls.lock().unwrap().is_empty());
    let permission =
        organization_permission_fixture::permission(84, &[Action::OrganizationSettingsUpdate]);
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.update_profile(
                &permission,
                ProfilePatch {
                    name: Some("NotSaved".into()),
                    avatar_uploaded: true
                }
            ))
            .await,
        Err(ProfileError::Storage(StorageError::NotFound))
    ));
    assert_eq!(
        fixture
            .repo
            .0
            .lock()
            .unwrap()
            .organization
            .as_ref()
            .unwrap()
            .name,
        "Original"
    );
    storage
        .uploaded
        .store(true, std::sync::atomic::Ordering::SeqCst);
    let saved = fixture
        .repo
        .1
        .scope(service.update_profile(
            &permission,
            ProfilePatch {
                name: Some(" Renamed ".into()),
                avatar_uploaded: true,
            },
        ))
        .await
        .unwrap();
    assert_eq!(saved.name, "Renamed");
    assert_eq!(saved.avatar_path.as_deref(), Some("/avatars/84"));
    let saved = fixture
        .repo
        .1
        .scope(service.update_profile(
            &permission,
            ProfilePatch {
                name: Some("Other".into()),
                avatar_uploaded: false,
            },
        ))
        .await
        .unwrap();
    assert_eq!(saved.avatar_path.as_deref(), Some("/avatars/84"));
    assert_eq!(*storage.calls.lock().unwrap(), vec![84, 84]);
}

#[async_trait::async_trait]
impl compallm_core::persistent::storage::user_space::UserStorageSpace for ProfileStorage {
    fn avatar_path(&self, id: u64) -> String {
        Self::get_avatar_path(id)
    }
    async fn avatar_uploaded(
        &self,
        user_id: u64,
    ) -> Result<bool, compallm_core::persistent::storage::user_space::StorageError> {
        assert!(!DbContext::current().unwrap().postgres().is_transaction());
        self.calls.lock().unwrap().push(user_id);
        Ok(self.uploaded.load(std::sync::atomic::Ordering::SeqCst))
    }
    async fn generate_avatar_upload(
        &self,
        user_id: u64,
        constraints: &compallm_core::persistent::storage::user_space::AvatarUploadConstraints,
    ) -> Result<
        compallm_core::persistent::storage::user_space::AvatarUpload,
        compallm_core::persistent::storage::user_space::StorageError,
    > {
        assert!(!DbContext::current().unwrap().postgres().is_transaction());
        self.calls.lock().unwrap().push(user_id);
        assert_eq!(
            (constraints.min_bytes, constraints.max_bytes),
            (1, 5 * 1024 * 1024)
        );
        assert_eq!(constraints.content_type, "image/png");
        assert!(
            constraints
                .expires_at
                .duration_since(std::time::SystemTime::now())
                .unwrap()
                <= Duration::from_secs(300)
        );
        Ok(
            compallm_core::persistent::storage::user_space::AvatarUpload {
                url: "https://storage.example.com/assets".into(),
                fields: std::collections::BTreeMap::from([(
                    "key".into(),
                    Self::get_avatar_path(user_id),
                )]),
                expires_at: constraints.expires_at,
            },
        )
    }
}

#[tokio::test]
async fn profile_service_saves_confirmed_path_and_preserves_omitted_fields() {
    use compallm_core::user::{ProfilePatch, ProfileService};
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let repository = Box::leak(Box::new(fixture.repo.clone()));
    let storage = Box::leak(Box::new(ProfileStorage {
        uploaded: true.into(),
        calls: Mutex::new(Vec::new()),
    }));
    let service = ProfileService::new(repository, Some(storage));
    let actor = fixture.access_for_session(stored_session(1, 10, 84, "Workspace"));
    let saved = fixture
        .repo
        .1
        .scope(service.update_profile(
            &actor,
            ProfilePatch {
                display_name: Some("  New Name  ".into()),
                username: None,
                avatar_uploaded: true,
            },
        ))
        .await
        .unwrap();
    assert_eq!(saved.avatar_path.as_deref(), Some("/avatars/1"));
    assert_eq!(saved.display_name, "New Name");
    assert_eq!(saved.email, "james@midwess.com");
    let saved = fixture
        .repo
        .1
        .scope(service.update_profile(
            &actor,
            ProfilePatch {
                display_name: Some("Other Name".into()),
                username: None,
                avatar_uploaded: false,
            },
        ))
        .await
        .unwrap();
    assert_eq!(saved.avatar_path.as_deref(), Some("/avatars/1"));
    assert_eq!(*storage.calls.lock().unwrap(), vec![1]);
}

#[tokio::test]
async fn profile_service_missing_avatar_and_denied_actor_do_not_write() {
    use compallm_core::persistent::storage::user_space::StorageError;
    use compallm_core::user::{ProfileError, ProfilePatch, ProfileService};
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let repository = Box::leak(Box::new(fixture.repo.clone()));
    let storage = Box::leak(Box::new(ProfileStorage {
        uploaded: false.into(),
        calls: Mutex::new(Vec::new()),
    }));
    let service = ProfileService::new(repository, Some(storage));
    let mut actor = fixture.access_for_session(stored_session(1, 10, 84, "Workspace"));
    let patch = || ProfilePatch {
        display_name: Some("Changed".into()),
        username: None,
        avatar_uploaded: true,
    };
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.update_profile(&actor, patch()))
            .await,
        Err(ProfileError::Storage(StorageError::NotFound))
    ));
    assert_eq!(
        fixture
            .repo
            .0
            .lock()
            .unwrap()
            .user
            .as_ref()
            .unwrap()
            .display_name,
        "James"
    );
    actor.session.user_id = 2;
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.update_profile(&actor, patch()))
            .await,
        Err(ProfileError::Denied)
    ));
    actor.session.user_id = 1;
    fixture.repo.0.lock().unwrap().user.as_mut().unwrap().status = UserStatus::Disabled;
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.update_profile(&actor, patch()))
            .await,
        Err(ProfileError::Denied)
    ));
    assert_eq!(*storage.calls.lock().unwrap(), vec![1]);
}

#[tokio::test]
async fn profile_service_upload_authorization_uses_the_actor_and_core_limits() {
    use compallm_core::user::{AvatarMediaType, ProfileError, ProfileService};
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let repository = Box::leak(Box::new(fixture.repo.clone()));
    let storage = Box::leak(Box::new(ProfileStorage {
        uploaded: false.into(),
        calls: Mutex::new(Vec::new()),
    }));
    let service = ProfileService::new(repository, Some(storage));
    let mut actor = fixture.access_for_session(stored_session(1, 10, 84, "Workspace"));
    fixture
        .repo
        .1
        .scope(service.generate_url_avatar_url(&actor, AvatarMediaType::Png, 1024))
        .await
        .unwrap();
    actor.validity.issued_at = 0;
    actor.validity.expires_at = 1;
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.generate_url_avatar_url(&actor, AvatarMediaType::Png, 1024))
            .await,
        Err(ProfileError::Authentication(_))
    ));
    assert_eq!(*storage.calls.lock().unwrap(), vec![1]);
}

#[async_trait::async_trait]
impl SessionRepository for Repository {
    async fn list_owned(
        &self,
        user_id: u64,
        pagination: compallm_query::Pagination,
    ) -> Result<compallm_query::Connection<Session>, SessionRepositoryError> {
        let boundary = pagination
            .after
            .as_ref()
            .map(|cursor| cursor.decode(&format!("test-sessions:{user_id}")))
            .transpose()
            .map_err(anyhow::Error::from)?
            .and_then(|values| {
                Some((
                    values[0].as_ref()?.parse::<u64>().ok()?,
                    values[1].as_ref()?.parse::<u64>().ok()?,
                ))
            });
        let state = self.0.lock().unwrap();
        let mut nodes: Vec<_> = state
            .sessions
            .values()
            .filter(|session| {
                session.user_id == user_id
                    && session.revoked_at.is_none()
                    && session.expires_at > compallm_utils::time::unix_timestamp()
                    && boundary.is_none_or(|key| (session.last_seen_at, session.id) < key)
            })
            .cloned()
            .collect();
        nodes.sort_by_key(|session| std::cmp::Reverse((session.last_seen_at, session.id)));
        let size = pagination.page_size(50).map_err(anyhow::Error::from)? as usize;
        let has_next_page = nodes.len() > size;
        nodes.truncate(size);
        let edges: Vec<_> = nodes
            .into_iter()
            .map(|node| {
                Ok(compallm_query::Edge {
                    cursor: compallm_query::Cursor::encode(
                        vec![
                            Some(node.last_seen_at.to_string()),
                            Some(node.id.to_string()),
                        ],
                        &format!("test-sessions:{user_id}"),
                    )
                    .map_err(anyhow::Error::from)?
                    .0,
                    node,
                })
            })
            .collect::<Result<_, SessionRepositoryError>>()?;
        Ok(compallm_query::Connection {
            page_info: compallm_query::PageInfo {
                start_cursor: edges.first().map(|e| e.cursor.clone()),
                end_cursor: edges.last().map(|e| e.cursor.clone()),
                has_next_page,
                has_previous_page: boundary.is_some(),
            },
            edges,
        })
    }
    async fn revoke_owned(
        &self,
        user_id: u64,
        current_session_id: u64,
        target_id: Option<u64>,
    ) -> Result<u64, SessionRepositoryError> {
        let mut state = self.0.lock().unwrap();
        let mut changed = 0;
        for session in state.sessions.values_mut() {
            if session.user_id == user_id
                && session.id != current_session_id
                && session.revoked_at.is_none()
                && target_id.is_none_or(|id| id == session.id)
            {
                session.revoke();
                changed += 1;
            }
        }
        Ok(changed)
    }

    async fn authorization_version(
        &self,
        presented: &Session,
    ) -> Result<Option<u32>, SessionRepositoryError> {
        assert!(
            PostgresConnection::from_context(&DbContext::current()?)?
                .rls()
                .is_none()
        );
        let mut state = self.0.lock().unwrap();
        state.freshness_reads += 1;
        if state.fail_authority_read {
            return Err(anyhow::anyhow!("freshness query failed").into());
        }
        Ok(state
            .sessions
            .get(&presented.id)
            .filter(|s| s.is_active(presented))
            .map(|s| s.authorization_version))
    }

    async fn find_policy_membership(
        &self,
        presented: &Session,
    ) -> Result<Option<(Session, OrganizationMember)>, SessionRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        state.authority_reads += 1;
        if state.fail_authority_read {
            return Err(anyhow::anyhow!("authority query failed").into());
        }
        let Some(session) = state
            .sessions
            .get(&presented.id)
            .filter(|s| s.is_active(presented))
            .cloned()
        else {
            return Ok(None);
        };
        if state
            .user
            .as_ref()
            .is_none_or(|u| u.id != session.user_id || u.status != UserStatus::Active)
            || state
                .disabled_organizations
                .contains(&session.organization_id)
        {
            return Ok(None);
        }
        let Some(member) = state
            .memberships
            .iter()
            .find(|m| m.organization_id == session.organization_id && m.user_id == session.user_id)
            .cloned()
        else {
            return Ok(None);
        };
        if state.change_after_authority_read {
            state
                .sessions
                .get_mut(&presented.id)
                .unwrap()
                .authorization_version += 1;
            state.memberships.clear();
            state.change_after_authority_read = false;
        }
        Ok(Some((session, member)))
    }

    async fn invalidate_permissions(
        &self,
        organization_id: u64,
        user_ids: &[u64],
    ) -> Result<u64, SessionRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        if state.fail_rotation {
            return Err(anyhow::anyhow!("invalidation failed").into());
        }
        let mut count = 0;
        for session in state.sessions.values_mut() {
            if session.organization_id == organization_id
                && user_ids.contains(&session.user_id)
                && session.revoked_at.is_none()
            {
                session.authorization_version += 1;
                count += 1;
            }
        }
        Ok(count)
    }

    async fn create(&self, session: Session) -> Result<(), SessionRepositoryError> {
        let mut state = self.0.lock().unwrap();
        state
            .contexts
            .push(PostgresConnection::from_context(&DbContext::current()?)? as *const _ as usize);
        if state.fail_session {
            return Err(anyhow::anyhow!("session insert failed").into());
        }
        state.logins.push(session.user_id);
        state.sessions.insert(session.id, session);
        Ok(())
    }

    async fn find(&self, session: &Session) -> Result<Option<Session>, SessionRepositoryError> {
        DbContext::current()?;
        Ok(self
            .0
            .lock()
            .unwrap()
            .sessions
            .get(&session.id)
            .filter(|stored| stored.has_same_identity(session))
            .cloned())
    }

    async fn update(
        &self,
        session: &Session,
        previous: &Session,
    ) -> Result<bool, SessionRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        if state.fail_rotation {
            return Ok(false);
        }
        if state.competing_rotation {
            state.sessions.get_mut(&session.id).unwrap().organization_id = 85;
            state.competing_rotation = false;
            return Ok(false);
        }
        let Some(current) = state.sessions.get_mut(&session.id) else {
            return Ok(false);
        };
        if !current.has_same_identity(previous) || current.revoked_at != previous.revoked_at {
            return Ok(false);
        }
        if session.organization_id != previous.organization_id {
            current.organization_id = session.organization_id;
        }
        if session.expires_at != previous.expires_at
            || session.last_seen_at != previous.last_seen_at
        {
            if current.expires_at <= compallm_utils::time::unix_timestamp() {
                return Ok(false);
            }
            current.expires_at = current.expires_at.max(session.expires_at);
            current.last_seen_at = current.last_seen_at.max(session.last_seen_at);
        }
        current.revoked_at = session.revoked_at;
        Ok(true)
    }
}

struct Fixture {
    db: Arc<DatabaseConnection>,
    repo: Repository,
}

impl Fixture {
    fn graphql(&self) -> axum::Router {
        self.graphql_with_database(self.repo.1.clone())
    }

    fn graphql_with_database(&self, db_context: DbContext) -> axum::Router {
        self.graphql_with_storage(db_context, None)
    }

    fn graphql_with_storage(
        &self,
        db_context: DbContext,
        storage: Option<Arc<compallm_storage::S3Storage>>,
    ) -> axum::Router {
        self.graphql_with_profile_storage(db_context, storage, None)
    }

    fn graphql_with_profile_storage(
        &self,
        db_context: DbContext,
        storage: Option<Arc<compallm_storage::S3Storage>>,
        profile_storage: Option<
            Box<dyn compallm_core::persistent::storage::user_space::UserStorageSpace>,
        >,
    ) -> axum::Router {
        self.graphql_with_org_storage(db_context, storage, profile_storage, None)
    }

    fn graphql_with_org_storage(
        &self,
        db_context: DbContext,
        storage: Option<Arc<compallm_storage::S3Storage>>,
        profile_storage: Option<
            Box<dyn compallm_core::persistent::storage::user_space::UserStorageSpace>,
        >,
        org_storage: Option<
            Box<dyn compallm_core::persistent::storage::org_space::OrganizationStorage>,
        >,
    ) -> axum::Router {
        self.graphql_with_org_repository(db_context, storage, profile_storage, org_storage, false)
    }

    fn graphql_with_org_repository(
        &self,
        db_context: DbContext,
        storage: Option<Arc<compallm_storage::S3Storage>>,
        profile_storage: Option<
            Box<dyn compallm_core::persistent::storage::user_space::UserStorageSpace>,
        >,
        org_storage: Option<
            Box<dyn compallm_core::persistent::storage::org_space::OrganizationStorage>,
        >,
        real_organizations: bool,
    ) -> axum::Router {
        use axum::{
            Extension, Json, Router,
            extract::{DefaultBodyLimit, State, rejection::JsonRejection},
            middleware::{from_fn, from_fn_with_state},
            routing::post,
        };
        use compallm_api::{
            di::{AuthSettings, Di, GraphqlSettings},
            graphql::{ApplicationSchema, GraphqlContext, build_schema, endpoint},
            http::authentication::{
                AuthenticationContext, AuthenticationSettings, CookieDelivery, UserContext,
            },
            http::authorization::AuthorizationContext,
            http::middleware::{AuthenticationMiddleware, AuthorizationMiddleware},
        };
        let email = Box::leak(Box::new(
            Box::new(TestEmail) as Box<dyn compallm_core::mail::EmailService>
        ));
        let service = self.service_with_dependencies(Box::new(TestDictionary {
            dictionary: compallm_core::compression::DCompression::new(
                compallm_core::compression::DictionaryType::AccessToken, 1,
                "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source",
            ).unwrap(), newer: None,
      }), Some(email));
        let limits = GraphqlSettings::default();
        let schema = build_schema(limits.max_depth, limits.max_fields, limits.max_tokens);
        let mut di = Di::new(
            db_context.clone(),
            AuthSettings {
                jwt_secret: SECRET.into(),
                user_token_ttl_seconds: 1800,
                refresh_token_ttl_seconds: 2_592_000,
            },
            GraphqlSettings {
                cookie_secure: false,
                public_origin: "http://localhost".into(),
                ..GraphqlSettings::default()
            },
            None,
        )
        .with_auth_service(service)
        .with_session_repository(Box::new(self.repo.clone()))
        .with_onboarding_repository(Box::new(onboarding_support::MemoryOnboarding::default()))
        .with_user_repository(Box::new(self.repo.clone()));
        if !real_organizations {
            di = di.with_organization_repository(Box::new(OrganizationRepositoryStub(
                self.repo.0.clone(),
            )));
        }
        if let Some(storage) = storage {
            di = di.with_s3_storage(storage);
        }
        if let Some(storage) = profile_storage {
            di = di.with_user_storage_space(storage);
        }
        if let Some(storage) = org_storage {
            di = di.with_org_storage(storage);
        }
        let di = Box::leak(Box::new(di));
        di.initialize_services().unwrap();
        let db = Box::leak(Box::new(db_context));
        let authentication = AuthenticationMiddleware::optional(
            db,
            di.auth_service(),
            AuthenticationSettings {
                cookie_secure: false,
                public_origin: "http://localhost".into(),
            },
        )
        .unwrap();
        Router::new().route(
            "/api/graphql",
            post(
                |State(schema): State<ApplicationSchema>,
                 Extension(di): Extension<&'static Di>,
                 Extension(user): Extension<UserContext>,
                 Extension(authentication): Extension<Arc<AuthenticationContext>>,
                 Extension(delivery): Extension<CookieDelivery>,
                 Extension(authorization): Extension<AuthorizationContext>,
                 request: Result<Json<endpoint::GraphQLRequest>, JsonRejection>| async move {
                    let context =
                        GraphqlContext::new(di, user, authentication, delivery, authorization);
                    endpoint::handle(schema, context, request).await
                },
            )
            .with_state(schema.clone())
            .layer(axum::Extension(di as &'static Di))
            .route_layer(from_fn(AuthorizationMiddleware::handle))
            .route_layer(from_fn_with_state(
                authentication,
                AuthenticationMiddleware::handle,
            ))
            .layer(DefaultBodyLimit::max(65_536)),
        )
    }

    fn access_for_session(&self, session: Session) -> UserAccessToken {
        UserAccessToken::new(session, empty_snapshot(), Duration::from_secs(60)).unwrap()
    }

    fn otp(challenge: &EmailOtpChallenge) -> compallm_core::authentication::EmailOtpAccessToken {
        compallm_core::authentication::EmailOtpAccessToken::try_from(
            decode_token(&challenge.access_token, SECRET).unwrap(),
        )
        .unwrap()
    }

    fn new(user: Option<User>) -> Self {
        match hero::distributed_id::init_scoped_id_generator("auth-tests") {
            Ok(())
            | Err(hero::distributed_id::generator::DistributedIdError::AlreadyInitialized) => {}
            Err(error) => panic!("{error}"),
        }
        let db = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
        TestRoot::set(Arc::new(PostgresConnection::new(db.clone())));
        let memberships = user
            .as_ref()
            .map(|user| {
                vec![OrganizationMember {
                    organization_id: 84,
                    user_id: user.id,
                    groups: Vec::new(),
                    role: Fixture::role(84, 0),
                }]
            })
            .unwrap_or_default();
        let repo = Repository(
            Arc::new(Mutex::new(State {
                user,
                memberships,
                ..State::default()
            })),
            DbContext::new(Arc::new(PostgresConnection::new(db.clone()))),
        );
        Self { db, repo }
    }

    fn service(&self) -> AuthService {
        self.service_with_dictionaries(Box::new(TestDictionary {
            dictionary: compallm_core::compression::DCompression::new(
                compallm_core::compression::DictionaryType::AccessToken,
                1,
                "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source",
            ).unwrap(),
            newer: None,
      }))
    }

    fn service_with_dictionaries(
        &self,
        dictionary: Box<dyn compallm_core::persistent::dictionary::DictionaryRepository>,
    ) -> AuthService {
        self.service_with_dependencies(dictionary, None)
    }

    fn service_with_dependencies(
        &self,
        dictionary: Box<dyn compallm_core::persistent::dictionary::DictionaryRepository>,
        email: Option<&'static Box<dyn compallm_core::mail::EmailService>>,
    ) -> AuthService {
        // Static dependencies mirror the application container. Test-only
        // allocations live for this test process and never touch a real DB.
        let users = Box::leak(Box::new(
            Box::new(self.repo.clone()) as Box<dyn UserRepository>
        ));
        let organizations = Box::leak(Box::new(Box::new(OrganizationRepositoryStub(
            self.repo.0.clone(),
        )) as Box<dyn OrganizationRepository>));
        let creation = Box::leak(Box::new(CreationService::new(
            organizations,
            onboarding_support::repository(),
        )));
        let registration = Box::leak(Box::new(RegistrationService::new(users, creation)));
        let dictionaries = Box::leak(Box::new(dictionary));
        AuthService::new(
            users,
            organizations,
            Box::leak(Box::new(
                Box::new(self.repo.clone()) as Box<dyn SessionRepository>
            )),
            dictionaries,
            registration,
            Box::leak(Box::new(
                compallm_core::preference::UserPreferenceService::new(Box::leak(Box::new(
                    Box::new(self.repo.clone())
                        as Box<dyn compallm_core::persistent::preference::user::UserPreferenceRepository>,
                ))),
            )),
            SECRET.into(),
            TokenLifetimes {
                user: Duration::from_secs(60),
                refresh: REFRESH_LIFETIME,
            },
            email,
        )
        .unwrap()
    }

    async fn refresh(&self, token: &str) -> Result<UserTokens, AuthError> {
        self.repo.1.scope(self.service().refresh(token)).await
    }

    async fn signin(
        &self,
        claims: &EmailOtpAccessToken,
        code: &str,
    ) -> Result<UserTokens, AuthError> {
        self.repo
            .1
            .scope(self.service().signin(claims, code, Default::default()))
            .await
    }

    async fn signup_in_transaction(
        &self,
        claims: &EmailOtpAccessToken,
        code: &str,
    ) -> Result<UserTokens, AuthError> {
        self.service()
            .signup(claims, code, Default::default())
            .await
    }

    async fn register(&self, email: &str) -> Result<User, compallm_core::user::RegistrationError> {
        self.repo.1.scope(self.registration().register(email)).await
    }

    async fn switch_organization(
        &self,
        token: &str,
        organization_id: u64,
    ) -> Result<UserTokens, AuthError> {
        self.repo
            .1
            .scope(self.service().switch_organization(token, organization_id))
            .await
    }

    fn remember(&self, token: &str) {
        let refresh = RefreshToken::try_from(decode_token(token, SECRET).unwrap()).unwrap();
        self.repo
            .0
            .lock()
            .unwrap()
            .sessions
            .insert(refresh.session.id, refresh.session.clone());
    }

    fn registration(&self) -> RegistrationService {
        let users = Box::leak(Box::new(
            Box::new(self.repo.clone()) as Box<dyn UserRepository>
        ));
        let organizations = Box::leak(Box::new(Box::new(OrganizationRepositoryStub(
            self.repo.0.clone(),
        )) as Box<dyn OrganizationRepository>));
        let creation = Box::leak(Box::new(CreationService::new(
            organizations,
            onboarding_support::repository(),
        )));
        RegistrationService::new(users, creation)
    }

    fn role(organization_id: u64, index: usize) -> Role {
        let mut role = Role {
            id: (index + 1) as u64,
            organization_id,
            name: ["Owner", "Admin", "Member"][index].into(),
            description: String::new(),
            cedar_source: String::new(),
            org_owner: index == 0,
        };
        role.cedar_source = format!(
            r#"permit(principal in Compallm::Role::"{}:{}", action == Compallm::Action::"model.read", resource == Compallm::Organization::"{}");"#,
            organization_id, role.id, organization_id
        );
        role
    }

    fn user(status: UserStatus) -> User {
        User {
            id: 1,
            email: "james@midwess.com".into(),
            display_name: "James".into(),
            username: "james".into(),
            avatar_path: None,
            status,
        }
    }

    fn assert_transaction(self, finish: &str) {
        drop(self.repo);
        let mut statements = vec![Statement::from_string(DbBackend::Postgres, "BEGIN")];
        statements.push(Statement::from_string(DbBackend::Postgres, finish));
        assert_eq!(
            self.db.as_ref().clone().into_transaction_log(),
            vec![Transaction::many(statements)]
        );
    }

    fn assert_no_transaction(&self) {
        assert!(self.db.as_ref().clone().into_transaction_log().is_empty());
    }
}

#[tokio::test]
async fn direct_registration_commits_user_organization_and_owner_together() {
    let fixture = Fixture::new(None);
    let user = fixture.register("james@midwess.com").await.unwrap();
    {
        let state = fixture.repo.0.lock().unwrap();
        assert!(state.created.is_some());
        let organization = state.organization.as_ref().unwrap();
        let owner = state.owner.as_ref().unwrap();
        assert_eq!(owner.user_id, user.id);
        assert_eq!(owner.organization_id, organization.id);
    }
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn direct_registration_rolls_back_when_default_organization_or_owner_fails() {
    for fail_membership in [false, true] {
        let fixture = Fixture::new(None);
        {
            let mut state = fixture.repo.0.lock().unwrap();
            state.fail_organization = !fail_membership;
            state.fail_membership = fail_membership;
        }
        assert!(matches!(
            fixture.register("james@midwess.com").await,
            Err(compallm_core::user::RegistrationError::Organization(_))
        ));
        assert!(fixture.repo.0.lock().unwrap().created.is_some());
        assert!(fixture.repo.0.lock().unwrap().owner.is_none());
        fixture.assert_transaction("ROLLBACK");
    }
}

#[tokio::test]
async fn signin_checks_user_and_records_session_without_starting_a_transaction() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let issued = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    assert_eq!(
        decode_token(&issued.access_token, SECRET)
            .and_then(UserAccessToken::try_from)
            .unwrap()
            .session
            .user_id,
        1
    );
    let contexts = fixture.repo.0.lock().unwrap().contexts.clone();
    let access =
        UserAccessToken::try_from(decode_token(&issued.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(access.session.organization_id, 84);
    assert_eq!(contexts.len(), 2);
    assert_eq!(fixture.repo.0.lock().unwrap().logins, [1]);
    assert!(contexts.iter().all(|context| *context == contexts[0]));
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn issued_tokens_use_signed_versioned_compression() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let service = fixture.service();
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    for (encoded, expected) in [
        (&challenge.access_token, "otp"),
        (&tokens.access_token, "user"),
        (&tokens.refresh_token, "refresh"),
    ] {
        use base64::Engine;
        use coset::{
            CborSerializable, CoseMac0,
            cwt::{ClaimName, ClaimsSet, Timestamp},
            iana,
        };
        use hmac::Mac;
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(encoded)
            .unwrap();
        let cose = CoseMac0::from_slice(&bytes).unwrap();
        assert_eq!(
            cose.protected.header.alg,
            Some(iana::Algorithm::HMAC_256_256.into())
        );
        cose.verify_payload_tag(
            b"",
            || (),
            |tag, message| {
                let mut mac =
                    <hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(SECRET.as_bytes()).unwrap();
                mac.update(message);
                mac.verify_slice(tag).map_err(|_| ())
            },
        )
        .unwrap();
        let claims = ClaimsSet::from_slice(cose.payload.as_deref().unwrap()).unwrap();
        assert!(matches!(
            claims.expiration_time,
            Some(Timestamp::WholeSeconds(_))
        ));
        assert!(matches!(claims.issued_at, Some(Timestamp::WholeSeconds(_))));
        assert!(matches!(
            &claims.rest[..],
            [(
                ClaimName::PrivateUse(-65537),
                ciborium::value::Value::Bytes(_)
            )]
        ));
        let token = fixture
            .repo
            .1
            .scope(service.authenticate_token(encoded))
            .await
            .unwrap();
        assert!(matches!(
            (expected, token),
            ("otp", AccessToken::EmailOtp(_))
                | ("user", AccessToken::User(_))
                | ("refresh", AccessToken::Refresh(_))
        ));
        let mut tampered = encoded.as_bytes().to_vec();
        let index = tampered.len() - 4;
        tampered[index] = if tampered[index] == b'a' { b'b' } else { b'a' };
        assert!(
            fixture
                .repo
                .1
                .scope(service.authenticate_token(std::str::from_utf8(&tampered).unwrap()))
                .await
                .is_err()
        );
    }
    let missing = AccessToken::Compressed(compallm_core::authentication::token::CompressedToken {
        dictionary_version: 999,
        data: vec![1],
    });
    let encoded = test_codec::sign_raw(&missing, SECRET, Duration::from_secs(60));
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(&encoded))
            .await,
        Err(AuthError::DictionaryUnavailable)
    ));
    let legacy = test_codec::sign_raw(
        &AccessToken::Refresh(
            RefreshToken::from_session(&stored_session(1, 7, 84, "Team"), REFRESH_LIFETIME)
                .unwrap(),
        ),
        SECRET,
        REFRESH_LIFETIME,
    );
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(&legacy))
            .await,
        Ok(AccessToken::Refresh(_))
    ));
    let wrong = compallm_core::compression::DCompression::new(
        compallm_core::compression::DictionaryType::AccessToken,
        1,
        "a different dictionary",
    )
    .unwrap();
    let payload = AccessToken::Compressed(compallm_core::authentication::token::CompressedToken {
        dictionary_version: 1,
        data: wrong.compress_bytes(b"{}").await.unwrap(),
    });
    let encoded = test_codec::sign_raw(&payload, SECRET, Duration::from_secs(60));
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(&encoded))
            .await,
        Err(AuthError::InvalidToken)
    ));
}

#[tokio::test]
async fn email_challenge_uses_compressed_wrapper_and_keeps_otp_encrypted() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let dictionary = Box::new(TestDictionary {
        dictionary: compallm_core::compression::DCompression::new(
            compallm_core::compression::DictionaryType::AccessToken, 1,
            "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source",
        ).unwrap(),
        newer: None,
  });
    let email = Box::leak(Box::new(
        Box::new(TestEmail) as Box<dyn compallm_core::mail::EmailService>
    ));
    let service = fixture.service_with_dependencies(dictionary, Some(email));
    let issued = fixture
        .repo
        .1
        .scope(service.signin_email("james@midwess.com"))
        .await
        .unwrap();
    let token = fixture
        .repo
        .1
        .scope(service.authenticate_token(&issued.token))
        .await
        .unwrap();
    assert!(matches!(token, AccessToken::EmailOtp(_)));
    let envelope = test_codec::envelope(&issued.token);
    let AccessToken::Compressed(envelope) = envelope else {
        panic!("expected compressed token")
    };
    let dictionary = compallm_core::compression::DCompression::new(
        compallm_core::compression::DictionaryType::AccessToken,
        1,
        "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source",
    )
    .unwrap();
    let source = dictionary.decompress_bytes(&envelope.data).await.unwrap();
    assert!(
        !source
            .windows("james@midwess.com".len())
            .any(|part| part == b"james@midwess.com")
    );
    assert!(matches!(
        ciborium::de::from_reader::<AccessToken, _>(source.as_slice()).unwrap(),
        AccessToken::Encrypted(_)
    ));
}

#[tokio::test]
async fn refresh_reads_old_dictionary_and_issues_with_latest() {
    use compallm_core::compression::{DCompression, DictionaryType};
    let original =
        "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source";
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let rotated = fixture.service_with_dictionaries(Box::new(TestDictionary {
        dictionary: DCompression::new(DictionaryType::AccessToken, 1, original).unwrap(),
        newer: Some(
            DCompression::new(
                DictionaryType::AccessToken,
                2,
                &format!("{original} more vocabulary"),
            )
            .unwrap(),
        ),
    }));
    let renewed = fixture
        .repo
        .1
        .scope(rotated.refresh(&tokens.refresh_token))
        .await
        .unwrap();
    for token in [&renewed.access_token, &renewed.refresh_token] {
        let payload = test_codec::envelope(token);
        let AccessToken::Compressed(payload) = payload else {
            panic!("expected compressed token")
        };
        assert_eq!(payload.dictionary_version, 2);
        assert!(
            fixture
                .repo
                .1
                .scope(rotated.authenticate_token(token))
                .await
                .is_ok()
        );
    }
    assert!(
        fixture
            .repo
            .1
            .scope(rotated.authenticate_token(&tokens.access_token))
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn invalid_signature_is_rejected_before_dictionary_lookup() {
    use base64::Engine;
    use coset::{
        CborSerializable, CoseMac0Builder, HeaderBuilder,
        cwt::{ClaimsSetBuilder, Timestamp},
        iana,
    };
    use hmac::Mac;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let reads = Arc::new(AtomicUsize::new(0));
    let service = fixture.service_with_dictionaries(Box::new(CountingDictionary {
        inner: TestDictionary { dictionary: compallm_core::compression::DCompression::new(
            compallm_core::compression::DictionaryType::AccessToken, 1,
            "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source",
        ).unwrap(), newer: None },
        reads: reads.clone(),
  }));
    let mut tampered = challenge.access_token.as_bytes().to_vec();
    let index = tampered.len() - 4;
    tampered[index] = if tampered[index] == b'a' { b'b' } else { b'a' };
    assert!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(std::str::from_utf8(&tampered).unwrap()))
            .await
            .is_err()
    );
    assert_eq!(reads.load(Ordering::SeqCst), 0);
    let now = compallm_utils::time::unix_timestamp() as i64;
    let claims = ClaimsSetBuilder::new()
        .issued_at(Timestamp::WholeSeconds(now - 2))
        .expiration_time(Timestamp::WholeSeconds(now - 1))
        .private_claim(-65537, ciborium::value::Value::Bytes(vec![0]))
        .build()
        .to_vec()
        .unwrap();
    let expired = CoseMac0Builder::new()
        .protected(
            HeaderBuilder::new()
                .algorithm(iana::Algorithm::HMAC_256_256)
                .build(),
        )
        .payload(claims)
        .create_tag(b"", |message| {
            let mut mac =
                <hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(SECRET.as_bytes()).unwrap();
            mac.update(message);
            mac.finalize().into_bytes().to_vec()
        })
        .build()
        .to_vec()
        .unwrap();
    let expired = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(expired);
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(&expired))
            .await,
        Err(AuthError::ExpiredToken)
    ));
    assert_eq!(reads.load(Ordering::SeqCst), 0);
    let user = fixture.access_for_session(stored_session(1, 7, 84, "Team"));
    let raw = test_codec::sign_raw(&AccessToken::User(user), SECRET, Duration::from_secs(60));
    assert!(matches!(
        fixture.repo.1.scope(service.authenticate_token(&raw)).await,
        Ok(AccessToken::User(_))
    ));
    let otp = EmailOtpAccessToken::new("james@midwess.com".into()).unwrap();
    let encrypted = AccessToken::EmailOtp(otp)
        .encrypt(&compallm_core::authentication::EncryptedAccessToken::new(SECRET).unwrap())
        .await
        .unwrap();
    let raw = test_codec::sign_raw(&encrypted, SECRET, Duration::from_secs(60));
    assert!(matches!(
        fixture.repo.1.scope(service.authenticate_token(&raw)).await,
        Ok(AccessToken::EmailOtp(_))
    ));
    assert_eq!(reads.load(Ordering::SeqCst), 0);
    assert!(
        fixture
            .repo
            .1
            .scope(service.authenticate_token(&challenge.access_token))
            .await
            .is_ok()
    );
    assert_eq!(reads.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn signup_requires_verified_email_and_uses_one_root_transaction() {
    let fixture = Fixture::new(None);
    let challenge = test_challenge("James@MIDWESS.COM", SECRET).unwrap();
    let issued = fixture
        .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    assert!(
        decode_token(&issued.access_token, SECRET)
            .and_then(UserAccessToken::try_from)
            .unwrap()
            .session
            .user_id
            > 0
    );
    {
        let state = fixture.repo.0.lock().unwrap();
        assert_eq!(
            state.created,
            Some(("james@midwess.com".into(), "james".into()))
        );
        assert_eq!(state.contexts.len(), 3);
        assert_eq!(state.logins.len(), 1);
        assert_eq!(state.preferences.len(), 1);
        let organization = state.organization.as_ref().unwrap();
        let owner = state.owner.as_ref().unwrap();
        assert_eq!(organization.name, "james's organization");
        assert!(organization.id > 0);
        assert_eq!(organization.id, owner.organization_id);
        let access =
            UserAccessToken::try_from(decode_token(&issued.access_token, SECRET).unwrap()).unwrap();
        assert_eq!(access.session.organization_id, organization.id);
        assert!(
            state
                .contexts
                .iter()
                .all(|context| *context == state.contexts[0])
        );
    }
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn organization_or_owner_failure_rolls_back_the_entire_signup() {
    for fail_membership in [false, true] {
        let fixture = Fixture::new(None);
        {
            let mut state = fixture.repo.0.lock().unwrap();
            state.fail_organization = !fail_membership;
            state.fail_membership = fail_membership;
        }
        let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
        assert!(matches!(
            fixture
                .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
                .await,
            Err(AuthError::Organization(_))
        ));
        assert!(fixture.repo.0.lock().unwrap().owner.is_none());
        fixture.assert_transaction("ROLLBACK");
    }
}

#[tokio::test]
async fn duplicate_signup_and_disabled_signin_are_rejected() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
            .await,
        Err(AuthError::Persistence(
            UserRepositoryError::EmailAlreadyExists
        ))
    ));
    assert!(fixture.repo.0.lock().unwrap().created.is_none());
    fixture.assert_transaction("ROLLBACK");

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Disabled)));
    assert!(matches!(
        fixture
            .signin(&Fixture::otp(&challenge), &challenge.code)
            .await,
        Err(AuthError::UserDisabled)
    ));
    assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn failed_session_insert_rejects_signin_without_returning_tokens() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().fail_session = true;
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .signin(&Fixture::otp(&challenge), &challenge.code)
            .await,
        Err(AuthError::SessionPersistence(
            SessionRepositoryError::Database(_)
        ))
    ));
    assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn failed_repository_operation_rolls_back_and_returns_no_tokens() {
    let fixture = Fixture::new(None);
    fixture.repo.0.lock().unwrap().fail_create = true;
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
            .await,
        Err(AuthError::Persistence(UserRepositoryError::Database(_)))
    ));
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn invalid_otp_does_not_reach_persistence() {
    let fixture = Fixture::new(None);
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture.signin(&Fixture::otp(&challenge), "wrong").await,
        Err(AuthError::IncorrectCode)
    ));
    assert!(fixture.repo.0.lock().unwrap().contexts.is_empty());
    assert!(
        fixture
            .db
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn signup_derives_user_name_and_default_organization_from_the_email() {
    for local_part in [
        "James.Dang+dev".to_string(),
        "!+~".to_string(),
        "a".repeat(64),
    ] {
        let fixture = Fixture::new(None);
        let email = format!("{local_part}@midwess.com");
        let challenge = test_challenge(&email, SECRET).unwrap();
        fixture
            .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
            .await
            .unwrap();
        {
            let state = fixture.repo.0.lock().unwrap();
            let name = local_part.to_ascii_lowercase();
            assert_eq!(
                state.created.as_ref().unwrap().0,
                email.to_ascii_lowercase()
            );
            assert_eq!(state.created.as_ref().unwrap().1, name);
            assert!(state.created_user.as_ref().unwrap().display_name.is_empty());
            let prefix: String = name
                .chars()
                .take(Organization::MAX_NAME_LENGTH - "'s organization".len())
                .collect();
            let organization = state.organization.as_ref().unwrap();
            assert_eq!(organization.name, format!("{prefix}'s organization"));
            assert!(organization.name.chars().count() <= Organization::MAX_NAME_LENGTH);
        }
        fixture.assert_transaction("COMMIT");
    }
}

#[tokio::test]
async fn signup_username_uses_domain_then_number_when_taken() {
    for (taken, expected) in [
        (vec!["alex"], "alex-midwess"),
        (
            vec!["alex", "alex-midwess", "alex-midwess-2"],
            "alex-midwess-3",
        ),
    ] {
        let fixture = Fixture::new(None);
        fixture.repo.0.lock().unwrap().taken_usernames =
            taken.into_iter().map(str::to_owned).collect();
        let challenge = test_challenge("alex@midwess.com", SECRET).unwrap();
        fixture
            .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
            .await
            .unwrap();
        let user = fixture.repo.0.lock().unwrap().created_user.clone().unwrap();
        assert_eq!(user.username, expected);
        assert!(user.display_name.is_empty());
        fixture.assert_transaction("COMMIT");
    }
}

#[tokio::test]
async fn refresh_uses_root_without_a_caller_scope() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let service = fixture.service();
    let tokens = token_pair::compressed_new(
        &stored_session(1, 7, 84, "Research team"),
        empty_snapshot(),
        SECRET,
        Duration::from_secs(60),
        REFRESH_LIFETIME,
    )
    .unwrap();

    fixture.remember(&tokens.refresh_token);
    service.refresh(&tokens.refresh_token).await.unwrap();
    assert_eq!(fixture.repo.0.lock().unwrap().contexts.len(), 0);
    assert!(DbContext::current().is_err());
    assert!(
        fixture
            .db
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn refresh_checks_current_user_and_renews_both_tokens_for_another_month() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let original = token_pair::compressed_new(
        &stored_session(1, 7, 84, "Research team"),
        empty_snapshot(),
        SECRET,
        Duration::from_secs(60),
        REFRESH_LIFETIME,
    )
    .unwrap();
    let mut previous =
        RefreshToken::try_from(decode_token(&original.refresh_token, SECRET).unwrap()).unwrap();
    // Simulate a credential issued 29 days ago without waiting or replacing the clock.
    let elapsed = 29 * 24 * 60 * 60;
    previous.validity.issued_at -= elapsed;
    previous.validity.expires_at -= elapsed;
    let original_session = previous.session.clone();
    let previous_expiry = previous.validity.expires_at;
    let old_refresh = AccessToken::Refresh(previous)
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&old_refresh);
    let before_refresh = compallm_utils::time::unix_timestamp();
    let issued = fixture.refresh(&old_refresh).await.unwrap();
    let after_refresh = compallm_utils::time::unix_timestamp();
    let renewed =
        RefreshToken::try_from(decode_token(&issued.refresh_token, SECRET).unwrap()).unwrap();
    assert_ne!(issued.refresh_token, old_refresh);
    assert_eq!(renewed.session, original_session);
    assert_eq!(renewed.session.user_id, 1);
    assert!(renewed.validity.issued_at >= before_refresh);
    assert!(renewed.validity.issued_at <= after_refresh);
    assert_eq!(
        renewed.validity.expires_at - renewed.validity.issued_at,
        REFRESH_LIFETIME.as_secs()
    );
    assert!(renewed.validity.expires_at > previous_expiry);
    assert_eq!(renewed.session.id, 7);
    {
        let state = fixture.repo.0.lock().unwrap();
        let session = state.sessions.get(&7).unwrap();
        assert_eq!(session, &renewed.session);
        assert_eq!(state.sessions.len(), 1);
        assert!(state.logins.is_empty(), "refresh is not a new login");
    }
    assert_eq!(
        decode_token(&issued.access_token, SECRET)
            .and_then(UserAccessToken::try_from)
            .unwrap()
            .session
            .user_id,
        1
    );
    fixture.repo.0.lock().unwrap().user.as_mut().unwrap().status = UserStatus::Disabled;
    assert!(matches!(
        fixture.refresh(&issued.refresh_token).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(matches!(
        fixture.refresh(&original.access_token).await,
        Err(AuthError::WrongTokenVariant)
    ));
}

#[tokio::test]
async fn expired_refresh_requires_signin_before_reading_the_user() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let mut expired =
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap();
    let elapsed = REFRESH_LIFETIME.as_secs() + 1;
    expired.validity.issued_at -= elapsed;
    expired.validity.expires_at -= elapsed;
    // A valid JWT envelope must not bypass the expired inner claims.
    let encoded = sign_unchecked(&AccessToken::Refresh(expired), REFRESH_LIFETIME);
    assert!(matches!(
        fixture.refresh(&encoded).await,
        Err(AuthError::ExpiredToken)
    ));
    assert!(fixture.repo.0.lock().unwrap().contexts.is_empty());
}

#[tokio::test]
async fn signin_selects_first_active_membership_independent_of_roles() {
    let fixture_roles_0 = default_roles(80, [1, 2, 3]).await;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().memberships = vec![
        OrganizationMember {
            organization_id: 80,
            user_id: 1,
            groups: Vec::new(),
            role: fixture_roles_0.clone().remove(2),
        },
        OrganizationMember {
            organization_id: 85,
            user_id: 1,
            groups: Vec::new(),
            role: Fixture::role(85, 1),
        },
    ];
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let claims =
        UserAccessToken::try_from(decode_token(&tokens.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.session.organization_id, 80);
    assert_eq!(claims.policy_snapshot.role_id, 3);
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn organization_switch_is_separate_and_uses_current_membership_roles() {
    let fixture_roles_0 = default_roles(85, [1, 2, 3]).await;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .memberships
        .push(OrganizationMember {
            organization_id: 85,
            user_id: 1,
            groups: Vec::new(),
            role: fixture_roles_0.clone().remove(2),
        });
    let refresh = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&refresh);
    let tokens = fixture.switch_organization(&refresh, 85).await.unwrap();
    let claims =
        UserAccessToken::try_from(decode_token(&tokens.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.session.organization_id, 85);
    let refreshed =
        RefreshToken::try_from(decode_token(&tokens.refresh_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.session, refreshed.session);
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&7].organization_id,
        85
    );
    assert!(matches!(
        fixture.switch_organization(&refresh, 84).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    // Membership changes are checked again instead of copying role from an old JWT.
    fixture.repo.0.lock().unwrap().memberships[1].role = Fixture::role(85, 1);
    let tokens = fixture
        .switch_organization(&tokens.refresh_token, 85)
        .await
        .unwrap();
    let claims =
        UserAccessToken::try_from(decode_token(&tokens.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.policy_snapshot.role_id, 2);
    assert!(matches!(
        fixture
            .switch_organization(&tokens.refresh_token, 999)
            .await,
        Err(AuthError::OrganizationAccessDenied)
    ));
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .disabled_organizations
        .push(85);
    assert!(matches!(
        fixture.switch_organization(&tokens.refresh_token, 85).await,
        Err(AuthError::OrganizationAccessDenied)
    ));
}

#[tokio::test]
async fn signin_rejects_missing_or_disabled_memberships() {
    for disabled in [false, true] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        {
            let mut state = fixture.repo.0.lock().unwrap();
            if disabled {
                state.disabled_organizations.push(84);
            } else {
                state.memberships.clear();
            }
        }
        let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
        assert!(matches!(
            fixture
                .signin(&Fixture::otp(&challenge), &challenge.code)
                .await,
            Err(AuthError::OrganizationAccessDenied)
        ));
        assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
        fixture.assert_no_transaction();
    }
}

#[tokio::test]
async fn refresh_without_selection_keeps_the_session_organization() {
    let fixture_roles_0 = default_roles(80, [1, 2, 3]).await;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    fixture.repo.0.lock().unwrap().memberships.insert(
        0,
        OrganizationMember {
            organization_id: 80,
            user_id: 1,
            groups: Vec::new(),
            role: fixture_roles_0.clone().remove(2),
        },
    );
    let refresh = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&refresh);
    let tokens = fixture.refresh(&refresh).await.unwrap();
    let claims =
        UserAccessToken::try_from(decode_token(&tokens.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.session.organization_id, 84);
}

#[tokio::test]
async fn renewal_requires_persistence_and_preserves_unexpired_prior_credentials() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let original = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&original);
    let before = fixture.repo.0.lock().unwrap().sessions[&7].clone();
    fixture.repo.0.lock().unwrap().fail_rotation = true;
    assert!(matches!(
        fixture.refresh(&original).await,
        Err(AuthError::InvalidSession)
    ));
    assert_eq!(fixture.repo.0.lock().unwrap().sessions[&7], before);
    fixture.repo.0.lock().unwrap().fail_rotation = false;
    let replacement = fixture.refresh(&original).await.unwrap();
    assert!(fixture.refresh(&original).await.is_ok());
    assert!(fixture.refresh(&replacement.refresh_token).await.is_ok());
    let stored = fixture.repo.0.lock().unwrap().sessions[&7].clone();
    assert!(stored.has_same_identity(&before));
    assert!(stored.expires_at >= before.expires_at);
    assert!(stored.last_seen_at >= before.last_seen_at);
    assert!(
        fixture
            .db
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn session_insert_failure_rolls_back_signup_and_returns_no_credentials() {
    let fixture = Fixture::new(None);
    fixture.repo.0.lock().unwrap().fail_session = true;
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
            .await,
        Err(AuthError::SessionPersistence(_))
    ));
    assert!(fixture.repo.0.lock().unwrap().sessions.is_empty());
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn each_successful_signin_creates_an_independent_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let one = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let two = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let one = RefreshToken::try_from(decode_token(&one.refresh_token, SECRET).unwrap()).unwrap();
    let two = RefreshToken::try_from(decode_token(&two.refresh_token, SECRET).unwrap()).unwrap();
    assert_ne!(one.session.id, two.session.id);
    assert_eq!(fixture.repo.0.lock().unwrap().sessions.len(), 2);
    fixture
        .repo
        .1
        .scope(async {
            let previous = SessionRepository::find(&fixture.repo, &one.session)
                .await
                .unwrap()
                .unwrap();
            let mut revoked = previous.clone();
            revoked.revoke();
            assert!(
                SessionRepository::update(&fixture.repo, &revoked, &previous)
                    .await
                    .unwrap()
            );

            Ok::<_, compallm_core::persistent::ConnectionError>(())
        })
        .await
        .unwrap();
    let one = AccessToken::Refresh(one)
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
    let two = AccessToken::Refresh(two)
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
    assert!(matches!(
        fixture.refresh(&one).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(fixture.refresh(&two).await.is_ok());
}

#[tokio::test]
async fn missing_revoked_or_foreign_sessions_cannot_refresh() {
    for state in ["missing", "revoked", "foreign"] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        let refresh = AccessToken::Refresh(
            RefreshToken::from_session(
                &stored_session(1, 7, 84, "Research team"),
                REFRESH_LIFETIME,
            )
            .unwrap(),
        )
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
        fixture.remember(&refresh);
        {
            let mut data = fixture.repo.0.lock().unwrap();
            match state {
                "missing" => {
                    data.sessions.clear();
                }
                "revoked" => {
                    data.sessions.get_mut(&7).unwrap().revoked_at =
                        Some(compallm_utils::time::unix_timestamp());
                }
                "foreign" => {
                    data.sessions.get_mut(&7).unwrap().user_id = 2;
                }
                _ => unreachable!(),
            }
        }
        assert!(
            matches!(
                fixture.refresh(&refresh).await,
                Err(AuthError::InvalidSession)
            ),
            "accepted {state} session"
        );
        assert!(
            fixture.repo.0.lock().unwrap().contexts.is_empty(),
            "invalid session must fail before user lookup"
        );
        assert!(
            fixture
                .db
                .as_ref()
                .clone()
                .into_transaction_log()
                .is_empty()
        );
    }
}

#[tokio::test]
async fn invalid_signature_cannot_revoke_a_valid_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let claims =
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap();
    let valid = AccessToken::Refresh(claims.clone())
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&valid);
    let forged = AccessToken::Refresh(claims)
        .encode_compressed("wrong-key-at-least-32-bytes-long", REFRESH_LIFETIME)
        .unwrap();
    assert!(matches!(
        fixture.refresh(&forged).await,
        Err(AuthError::InvalidToken)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    assert!(
        fixture
            .db
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn refresh_persists_activity_in_an_enclosing_root_transaction() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let token = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&token);
    fixture
        .repo
        .1
        .transaction(async {
            fixture.service().refresh(&token).await?;
            Ok::<_, AuthError>(())
        })
        .await
        .unwrap();
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn failed_organization_switch_returns_no_credentials_and_preserves_the_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let claims =
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap();
    let token = AccessToken::Refresh(claims.clone())
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&token);
    fixture.repo.0.lock().unwrap().fail_rotation = true;
    assert!(matches!(
        fixture.switch_organization(&token, 84).await,
        Err(AuthError::InvalidSession)
    ));
    assert_eq!(fixture.repo.0.lock().unwrap().sessions[&7], claims.session);
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn competing_organization_switch_rejects_the_loser_without_revoking_the_winner() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let token = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&token);
    fixture.repo.0.lock().unwrap().competing_rotation = true;
    assert!(matches!(
        fixture.switch_organization(&token, 84).await,
        Err(AuthError::InvalidSession)
    ));
    let stored = fixture.repo.0.lock().unwrap().sessions[&7].clone();
    assert!(stored.revoked_at.is_none());
    assert_eq!(stored.organization_id, 85);
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn token_for_another_organization_cannot_read_or_revoke_an_owned_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let valid = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&valid);
    let foreign = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 85, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    assert!(matches!(
        fixture.refresh(&foreign).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    assert!(fixture.refresh(&valid).await.is_ok());
}

#[tokio::test]
async fn organization_switch_uses_root_with_an_enclosing_transaction() {
    let fixture_roles_0 = default_roles(85, [1, 2, 3]).await;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let original = AccessToken::Refresh(
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap(),
    )
    .encode_compressed(SECRET, REFRESH_LIFETIME)
    .unwrap();
    fixture.remember(&original);
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .memberships
        .push(OrganizationMember {
            organization_id: 85,
            user_id: 1,
            groups: Vec::new(),
            role: fixture_roles_0.clone().remove(2),
        });
    let service = fixture.service();
    let tokens = fixture
        .repo
        .1
        .transaction(service.switch_organization(&original, 85))
        .await
        .unwrap();
    let claims =
        RefreshToken::try_from(decode_token(&tokens.refresh_token, SECRET).unwrap()).unwrap();
    assert_eq!(claims.session.organization_id, 85);
    assert!(matches!(
        fixture.refresh(&original).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    fixture.assert_transaction("COMMIT");
}

#[tokio::test]
async fn expired_refresh_never_writes_or_revokes_a_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let mut token =
        RefreshToken::from_session(&stored_session(1, 7, 84, "Research team"), REFRESH_LIFETIME)
            .unwrap();
    let at = token.validity.issued_at;
    let stored = token.session.clone();
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .insert(7, stored.clone());
    token.validity.issued_at = at - REFRESH_LIFETIME.as_secs() - 1;
    token.validity.expires_at = at - 1;
    let expired = sign_unchecked(&AccessToken::Refresh(token), REFRESH_LIFETIME);
    assert!(matches!(
        fixture.refresh(&expired).await,
        Err(AuthError::ExpiredToken)
    ));
    assert!(
        fixture.repo.0.lock().unwrap().sessions[&7]
            .revoked_at
            .is_none()
    );
    assert!(
        fixture
            .db
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn signout_revokes_only_the_selected_session_and_is_repeatable() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let selected = stored_session(1, 7, 84, "Research team");
    let other = stored_session(1, 8, 84, "Research team");
    {
        let mut state = fixture.repo.0.lock().unwrap();
        state.sessions.insert(7, selected.clone());
        state.sessions.insert(8, other);
    }
    let claims =
        UserAccessToken::new(selected.clone(), empty_snapshot(), Duration::from_secs(60)).unwrap();
    let service = fixture.service();
    fixture
        .repo
        .1
        .scope(service.signout(&claims))
        .await
        .unwrap();
    fixture
        .repo
        .1
        .scope(service.signout(&claims))
        .await
        .unwrap();
    {
        let state = fixture.repo.0.lock().unwrap();
        assert!(state.sessions[&7].revoked_at.is_some());
        assert!(state.sessions[&8].revoked_at.is_none());
    }
    let refresh =
        AccessToken::Refresh(RefreshToken::from_session(&selected, REFRESH_LIFETIME).unwrap())
            .encode_compressed(SECRET, REFRESH_LIFETIME)
            .unwrap();
    assert!(matches!(
        fixture.refresh(&refresh).await,
        Err(AuthError::InvalidSession)
    ));
}

#[tokio::test]
async fn signout_cannot_revoke_a_foreign_user_or_organization_session() {
    for presented in [
        stored_session(2, 7, 84, "Research team"),
        stored_session(1, 7, 85, "Research team"),
    ] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        fixture
            .repo
            .0
            .lock()
            .unwrap()
            .sessions
            .insert(7, stored_session(1, 7, 84, "Research team"));
        let claims =
            UserAccessToken::new(presented, empty_snapshot(), Duration::from_secs(60)).unwrap();
        assert!(matches!(
            fixture
                .repo
                .1
                .scope(fixture.service().signout(&claims))
                .await,
            Err(AuthError::InvalidSession)
        ));
        assert!(
            fixture.repo.0.lock().unwrap().sessions[&7]
                .revoked_at
                .is_none()
        );
    }
}

#[tokio::test]
async fn signin_email_rejects_unknown_accounts_before_mail_admission() {
    let fixture = Fixture::new(None);
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(fixture.service().signin_email("new@example.com"))
            .await,
        Err(AuthError::UserNotFound)
    ));
}

#[tokio::test]
async fn signin_email_rejects_disabled_accounts_before_mail_admission() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Disabled)));
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(fixture.service().signin_email("james@midwess.com"))
            .await,
        Err(AuthError::UserDisabled)
    ));
}

#[tokio::test]
async fn signin_email_normalizes_existing_accounts_before_mail_admission() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    // This fixture has no mail service: only an accepted existing user reaches it.
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(fixture.service().signin_email(" James@MIDWESS.COM "))
            .await,
        Err(AuthError::Email(
            compallm_core::mail::EmailError::Unavailable
        ))
    ));
}

#[tokio::test]
async fn refresh_preserves_coherent_session_and_authority_without_restamping() {
    let fixture_roles_0 = default_roles(84, [1, 2, 3]).await;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let presented = stored_session(1, 7, 84, "Research team");
    {
        let mut state = fixture.repo.0.lock().unwrap();
        let mut stored = presented.clone();
        stored.authorization_version = 4;
        state.sessions.insert(7, stored);
        state.memberships[0].role.cedar_source.clear();
        state.memberships[0].groups.push(
            compallm_core::organization::Group::new(
                9,
                84,
                "Team".into(),
                String::new(),
                vec![fixture_roles_0.clone().remove(2)],
            )
            .unwrap(),
        );
        state.change_after_authority_read = true;
    }
    let service = fixture.service();
    let refresh =
        AccessToken::Refresh(RefreshToken::from_session(&presented, REFRESH_LIFETIME).unwrap())
            .encode_compressed(SECRET, REFRESH_LIFETIME)
            .unwrap();
    let issued = fixture
        .repo
        .1
        .scope(service.refresh(&refresh))
        .await
        .unwrap();
    let claims =
        UserAccessToken::try_from(decode_token(&issued.access_token, SECRET).unwrap()).unwrap();
    let session = claims.session;
    let snapshot = claims.policy_snapshot;
    assert_eq!(session.authorization_version, 4);
    assert!(snapshot.role_id > 0);
    assert_eq!(snapshot.group_roles[&9], vec![3]);
    assert!(!snapshot.cedar_source.is_empty());
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&7].authorization_version,
        5
    );
    assert!(fixture.repo.0.lock().unwrap().memberships.is_empty());
    assert!(matches!(
        fixture.repo.1.scope(service.refresh(&refresh)).await,
        Err(AuthError::InvalidSession)
    ));
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn refresh_rejects_unavailable_invalid_or_oversized_authority() {
    let fixture_roles_0 = default_roles(84, [1, 2, 3]).await;

    for case in [
        "missing user",
        "disabled user",
        "missing member",
        "disabled organization",
        "missing session",
        "revoked session",
        "foreign organization",
        "foreign user",
        "future version",
        "oversized",
        "database failure",
    ] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        let mut presented = stored_session(1, 7, 84, "Research team");
        {
            let mut state = fixture.repo.0.lock().unwrap();
            state.sessions.insert(7, presented.clone());
            match case {
                "missing user" => state.user = None,
                "disabled user" => state.user.as_mut().unwrap().status = UserStatus::Disabled,
                "missing member" => state.memberships.clear(),
                "disabled organization" => state.disabled_organizations.push(84),
                "missing session" => state.sessions.clear(),
                "revoked session" => state.sessions.get_mut(&7).unwrap().revoke(),
                "foreign organization" => presented.organization_id = 99,
                "foreign user" => presented.user_id = 99,
                "future version" => presented.authorization_version = 2,
                "oversized" => {
                    state.memberships[0].role = fixture_roles_0.clone().remove(0);
                    make_oversized(&mut state.memberships[0].role);
                    state.full_default_policies = true;
                }
                "database failure" => state.fail_authority_read = true,
                _ => unreachable!(),
            }
        }
        let refresh =
            AccessToken::Refresh(RefreshToken::from_session(&presented, REFRESH_LIFETIME).unwrap())
                .encode_compressed(SECRET, REFRESH_LIFETIME)
                .unwrap();
        let result = fixture.refresh(&refresh).await;
        match case {
            "oversized" => assert!(matches!(result, Err(AuthError::TokenTooLarge { .. }))),
            "database failure" => assert!(matches!(result, Err(AuthError::SessionPersistence(_)))),
            _ => assert!(matches!(result, Err(AuthError::InvalidSession)), "{case}"),
        }
        fixture.assert_no_transaction();
    }
}

#[tokio::test]
async fn snapshot_creation_uses_only_loaded_membership_without_session_or_context() {
    let fixture_roles_0 = default_roles(84, [1, 2, 3]).await;

    let fixture = Fixture::new(None);
    let membership = OrganizationMember {
        organization_id: 84,
        user_id: 1,
        groups: vec![],
        role: (fixture_roles_0.clone())
            .first()
            .cloned()
            .expect("fixture requires a direct role"),
    };
    let snapshot = compallm_core::authorization::PolicySnapshot::new(&membership).unwrap();
    assert_eq!(snapshot.role_id, 1);
    assert!(!snapshot.cedar_source.is_empty());
    assert!(DbContext::current().is_err());
    assert!(fixture.repo.0.lock().unwrap().sessions.is_empty());
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn renewal_copies_current_session_version_and_keeps_stale_refresh_usable() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let session = stored_session(1, 7, 84, "Research team");
    assert_eq!(session.authorization_version, 1);
    let token =
        AccessToken::Refresh(RefreshToken::from_session(&session, REFRESH_LIFETIME).unwrap())
            .encode_compressed(SECRET, REFRESH_LIFETIME)
            .unwrap();
    fixture.remember(&token);
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .get_mut(&7)
        .unwrap()
        .authorization_version = 42;
    let renewed = fixture.refresh(&token).await.unwrap();
    let access =
        UserAccessToken::try_from(decode_token(&renewed.access_token, SECRET).unwrap()).unwrap();
    let refresh =
        RefreshToken::try_from(decode_token(&renewed.refresh_token, SECRET).unwrap()).unwrap();
    assert_eq!(access.session.authorization_version, 42);
    assert_eq!(refresh.session.authorization_version, 42);
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&7].authorization_version,
        42
    );
    assert!(fixture.refresh(&token).await.is_ok());
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn explicit_permission_invalidation_scopes_users_and_sessions() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    {
        let mut state = fixture.repo.0.lock().unwrap();
        for session in [
            stored_session(1, 7, 84, "Team"),
            stored_session(1, 8, 84, "Team"),
            stored_session(2, 9, 84, "Team"),
            stored_session(1, 10, 85, "Other"),
        ] {
            state.sessions.insert(session.id, session);
        }
        let mut revoked = stored_session(1, 11, 84, "Team");
        revoked.revoke();
        state.sessions.insert(11, revoked);
    }
    let service = fixture.service();
    for expected in [2, 3] {
        assert_eq!(
            fixture
                .repo
                .1
                .scope(service.invalidate_permissions(84, &[1, 1]))
                .await
                .unwrap(),
            2
        );
        let state = fixture.repo.0.lock().unwrap();
        for id in [7, 8] {
            assert_eq!(state.sessions[&id].authorization_version, expected);
        }
        for id in [9, 10, 11] {
            assert_eq!(state.sessions[&id].authorization_version, 1);
        }
        assert!(state.sessions[&7].revoked_at.is_none());
    }
    assert_eq!(
        fixture
            .repo
            .1
            .scope(service.invalidate_permissions(84, &[]))
            .await
            .unwrap(),
        0
    );
    fixture.repo.0.lock().unwrap().fail_rotation = true;
    assert!(matches!(
        fixture
            .repo
            .1
            .scope(service.invalidate_permissions(84, &[1]))
            .await,
        Err(AuthError::SessionPersistence(_))
    ));
    fixture.assert_no_transaction();
}

fn empty_snapshot() -> compallm_core::authorization::PolicySnapshot {
    compallm_core::authorization::PolicySnapshot {
        cedar_source: String::new(),
        group_roles: std::collections::BTreeMap::new(),
        role_id: 3,
    }
}

#[tokio::test]
async fn access_issuance_includes_direct_and_group_authority_and_refresh_rebuilds_it() {
    let fixture_roles_0 = default_roles(84, [1, 2, 3]).await;

    use compallm_core::{authorization::PolicySnapshot, organization::Group};
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    {
        let mut state = fixture.repo.0.lock().unwrap();
        state.memberships[0].groups.push(
            Group::new(
                9,
                84,
                "Team".into(),
                String::new(),
                vec![fixture_roles_0.clone().remove(2)],
            )
            .unwrap(),
        );
    }
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let issued = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let access =
        UserAccessToken::try_from(decode_token(&issued.access_token, SECRET).unwrap()).unwrap();
    let expected = PolicySnapshot::new(&fixture.repo.0.lock().unwrap().memberships[0]).unwrap();
    assert_eq!(access.policy_snapshot, expected);
    assert_eq!(access.policy_snapshot.role_id, 1);
    assert_eq!(access.policy_snapshot.group_roles[&9], vec![3]);
    {
        let mut state = fixture.repo.0.lock().unwrap();
        state.memberships[0].role.cedar_source.clear();
        state.memberships[0].groups.clear();
        state
            .sessions
            .get_mut(&access.session.id)
            .unwrap()
            .authorization_version = 2;
    }

    let renewed = fixture.refresh(&issued.refresh_token).await.unwrap();
    let new_access =
        UserAccessToken::try_from(decode_token(&renewed.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(new_access.session.authorization_version, 2);
    assert!(new_access.policy_snapshot.cedar_source.is_empty());
    assert_eq!(new_access.policy_snapshot.role_id, 1);
    assert!(new_access.policy_snapshot.group_roles.is_empty());
    let refresh = decode_token(&renewed.refresh_token, SECRET).unwrap();
    assert!(
        serde_json::to_value(refresh).unwrap()["r"]
            .get("policy_snapshot")
            .is_none()
    );
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn default_policies_fit_with_compression() {
    let roles = default_roles(84, [1, 2, 3]).await;
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    {
        let mut state = fixture.repo.0.lock().unwrap();
        state.full_default_policies = true;
        state.memberships[0].role = roles[0].clone();
    }
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    assert!(tokens.access_token.len() <= AccessToken::MAX_ENCODED_BYTES);
    assert!(fixture.refresh(&tokens.refresh_token).await.is_ok());
}

#[tokio::test]
async fn signin_accepts_active_memberships_with_custom_or_default_role() {
    for custom in [false, true] {
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        {
            let mut state = fixture.repo.0.lock().unwrap();
            state.memberships[0].role = if custom {
                Role {
                    id: 77,
                    ..Fixture::role(84, 0)
                }
            } else {
                Fixture::role(84, 0)
            };
        }
        let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
        let tokens = fixture
            .signin(&Fixture::otp(&challenge), &challenge.code)
            .await
            .unwrap();
        let access =
            UserAccessToken::try_from(decode_token(&tokens.access_token, SECRET).unwrap()).unwrap();
        assert_eq!(
            access.policy_snapshot.role_id,
            if custom { 77 } else { Fixture::role(84, 0).id }
        );
        assert!(
            serde_json::to_value(access)
                .unwrap()
                .get("organization_role")
                .is_none()
        );
    }
}

async fn graphql_request(
    router: &axum::Router,
    bearer: Option<&str>,
    cookies: Option<&str>,
    trusted_origin: bool,
) -> axum::response::Response {
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    let mut request = Request::builder()
        .method("POST")
        .uri("/api/graphql")
        .header("content-type", "application/json")
        .header("host", "localhost");
    if let Some(token) = bearer {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    if let Some(cookies) = cookies {
        request = request.header("cookie", cookies);
    }
    if trusted_origin {
        request = request.header("origin", "http://localhost");
    }
    let body = serde_json::json!({
          "query": "query { __typename first: session {node{ id userId organizationId organizationName }} second: session {node{ id }}  }"
    });
    router
        .clone()
        .oneshot(request.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn invalid_authorization_clears_cookies_without_revoking_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let session = fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .values()
        .next()
        .unwrap()
        .clone();
    let invalid = token_pair::new(
        &session,
        PolicySnapshot {
            cedar_source:
                "permit(principal, action == Compallm::Action::\"retired.action\", resource);"
                    .into(),
            group_roles: Default::default(),
            role_id: 3,
        },
        SECRET,
        Duration::from_secs(60),
        REFRESH_LIFETIME,
    )
    .unwrap();
    let cookies = format!(
        "compallm-access={}; compallm-refresh={}",
        invalid.access_token, tokens.refresh_token
    );
    let response = graphql_operation(
        &fixture.graphql(),
        "{ roles { edges { node { id } } } }",
        serde_json::json!({}),
        None,
        Some(&cookies),
        true,
    )
    .await;
    let headers: Vec<_> = response
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|value| value.to_str().unwrap().to_owned())
        .collect();
    assert_eq!(headers.len(), 4);
    assert!(headers.iter().all(|value| value.contains("Max-Age=0")));
    assert_eq!(
        graphql_body(response).await["errors"][0]["extensions"]["code"],
        "INVALID_AUTHORIZATION_STATE"
    );
    assert!(
        fixture
            .repo
            .0
            .lock()
            .unwrap()
            .sessions
            .get(&session.id)
            .unwrap()
            .revoked_at
            .is_none()
    );
}

#[tokio::test]
async fn policy_catalog_requires_a_current_user_without_a_role_grant() {
    use compallm_core::authorization::policy::Action;
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let session = fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .values()
        .next()
        .unwrap()
        .clone();
    let query = "{ organizationPolicyActions { edges { node { name entityType resourceType } } pageInfo { hasNextPage hasPreviousPage } } }";
    let snapshot = |source: &str| PolicySnapshot {
        cedar_source: source.into(),
        group_roles: Default::default(),
        role_id: 3,
    };
    let authorized = token_pair::new(
        &session,
        snapshot(""),
        SECRET,
        Duration::from_secs(60),
        REFRESH_LIFETIME,
    )
    .unwrap();
    let router = fixture.graphql();
    let body = graphql_body(
        graphql_operation(
            &router,
            query,
            serde_json::json!({}),
            Some(&authorized.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(body["errors"].is_null(), "{body}");
    let edges = body["data"]["organizationPolicyActions"]["edges"]
        .as_array()
        .unwrap();
    assert_eq!(edges.len(), Action::ALL.len());
    assert!(edges.iter().any(|edge| edge["node"] == serde_json::json!({"name":"role.create", "entityType":"Role", "resourceType":"Organization"})));
    assert_eq!(
        body["data"]["organizationPolicyActions"]["pageInfo"]["hasNextPage"],
        false
    );
    let body = graphql_body(
        graphql_operation(&router, query, serde_json::json!({}), None, None, false).await,
    )
    .await;
    assert_eq!(body["errors"][0]["extensions"]["code"], "UNAUTHENTICATED");
}

#[tokio::test]
async fn graphql_identity_uses_one_freshness_read_and_rejects_non_access_or_revoked_credentials() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let router = fixture.graphql();

    let introspection = graphql_operation(
        &router,
        "{ __schema { queryType { name } } }",
        serde_json::json!({}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    assert_eq!(introspection.status(), 200);
    assert!(graphql_body(introspection).await["errors"].is_null());
    assert_eq!(fixture.repo.0.lock().unwrap().freshness_reads, 0);
    let response = graphql_request(&router, Some(&tokens.access_token), None, false).await;
    assert_eq!(response.status(), 200);
    assert!(!response.headers().contains_key("set-cookie"));
    let bytes = axum::body::to_bytes(response.into_body(), 8192)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(body.get("errors").is_none());
    assert!(body["data"]["first"]["node"].is_object());
    assert_eq!(body["data"]["__typename"], "Query");
    assert_eq!(fixture.repo.0.lock().unwrap().freshness_reads, 1);

    for token in [
        None,
        Some(tokens.refresh_token.as_str()),
        Some(challenge.access_token.as_str()),
    ] {
        let response = graphql_request(&router, token, None, false).await;
        assert_graphql_denial(response, 401).await;
    }
    assert_eq!(fixture.repo.0.lock().unwrap().freshness_reads, 1);
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .values_mut()
        .next()
        .unwrap()
        .revoke();
    let response = graphql_request(&router, Some(&tokens.access_token), None, false).await;
    assert_graphql_denial(response, 401).await;
    let state = fixture.repo.0.lock().unwrap();
    assert_eq!(state.authority_reads, 0);
    assert_eq!(state.logins.len(), 1);
}

#[tokio::test]
async fn graphql_avatar_url_uses_shared_storage_without_storage_requests() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let storage = Arc::new(
        compallm_storage::S3Storage::new(compallm_storage::S3Settings {
            enabled: true,
            region: "us-east-1".into(),
            endpoint: "http://127.0.0.1:1".into(),
            upload_endpoint: "http://127.0.0.1:1".into(),
            access_key: "test".into(),
            secret_key: "test-secret".into(),
            force_path_style: true,
            bucket: "assets".into(),
            public_base_url: "https://cdn.example.com/assets".into(),
            timeout_seconds: 1,
            allowed_origins: vec![],
        })
        .unwrap(),
    );
    let router = fixture.graphql_with_storage(fixture.repo.1.clone(), Some(storage));
    let response = graphql_operation(
        &router,
        "{ session { node { userId avatarUrl } } }",
        serde_json::json!({}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    let body = graphql_body(response).await;
    assert!(body["errors"].is_null());
    let user = body["data"]["session"]["node"]["userId"].as_str().unwrap();
    let url = body["data"]["session"]["node"]["avatarUrl"]
        .as_str()
        .unwrap();
    assert!(url.starts_with(&format!("https://cdn.example.com/assets/avatars/{user}?v=")));
}

#[tokio::test]
async fn graphql_organization_profiles_are_public_and_writes_use_active_permission() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    {
        let mut state = fixture.repo.0.lock().unwrap();
        state.organization = Some(Organization {
            id: 84,
            name: "Original".into(),
            avatar_path: None,
            status: compallm_core::organization::OrganizationStatus::Active,
        });
        state.full_default_policies = true;
        state.memberships[0].role.cedar_source = r#"permit(principal, action == Compallm::Action::"organization.settings.update", resource == Compallm::Organization::"84");"#.into();
    }
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let router = fixture.graphql_with_org_storage(
        fixture.repo.1.clone(),
        None,
        None,
        Some(Box::new(ProfileStorage {
            uploaded: true.into(),
            calls: Mutex::new(Vec::new()),
        })),
    );
    let query =
        "{ organizationProfile(id: \"84\") { node { id name avatarUrl canUpdateProfile } } }";
    let body = graphql_body(
        graphql_operation(&router, query, serde_json::json!({}), None, None, false).await,
    )
    .await;
    assert!(body["errors"].is_null(), "{body}");
    assert_eq!(
        body["data"]["organizationProfile"]["node"]["canUpdateProfile"],
        false
    );
    let body = graphql_body(
        graphql_operation(
            &router,
            query,
            serde_json::json!({}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert_eq!(
        body["data"]["organizationProfile"]["node"]["canUpdateProfile"],
        true
    );
    let update = "mutation { organizationProfileUpdate(input: {name: \"Renamed\", avatarUploaded: true}) { node { id name } } }";
    let denied = graphql_operation(&router, update, serde_json::json!({}), None, None, false).await;
    assert_graphql_denial(denied, 401).await;
    let body = graphql_body(
        graphql_operation(
            &router,
            update,
            serde_json::json!({}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(body["errors"].is_null(), "{body}");
    assert_eq!(
        body["data"]["organizationProfileUpdate"]["node"]["name"],
        "Renamed"
    );
    assert_eq!(
        fixture
            .repo
            .0
            .lock()
            .unwrap()
            .organization
            .as_ref()
            .unwrap()
            .avatar_path
            .as_deref(),
        Some("/avatars/84")
    );
    let body = graphql_body(
        graphql_operation(
            &router,
            "{ session { node { organizationName } } }",
            serde_json::json!({}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert_eq!(
        body["data"]["session"]["node"]["organizationName"],
        "Renamed"
    );
    let upload = "mutation { generateOrgAvatarUrl(input: {mediaType: PNG, size: 1024}) { node { url fields { name value } } } }";
    let body = graphql_body(
        graphql_operation(
            &router,
            upload,
            serde_json::json!({}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(body["errors"].is_null(), "{body}");
    assert_eq!(
        body["data"]["generateOrgAvatarUrl"]["node"]["fields"][0]["value"],
        "/avatars/84"
    );
}

#[tokio::test]
async fn graphql_profile_requires_authentication_and_preserves_owner_updates() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let storage = Arc::new(
        compallm_storage::S3Storage::new(compallm_storage::S3Settings {
            enabled: true,
            region: "us-east-1".into(),
            endpoint: "http://127.0.0.1:1".into(),
            upload_endpoint: "http://127.0.0.1:1".into(),
            access_key: "test".into(),
            secret_key: "test-secret".into(),
            force_path_style: true,
            bucket: "assets".into(),
            public_base_url: "https://cdn.example.com/assets".into(),
            timeout_seconds: 1,
            allowed_origins: vec![],
        })
        .unwrap(),
    );
    let router = fixture.graphql_with_profile_storage(
        fixture.repo.1.clone(),
        Some(storage),
        Some(Box::new(ProfileStorage {
            uploaded: true.into(),
            calls: Mutex::new(Vec::new()),
        })),
    );
    let query = "{ userProfile(id: \"1\") { node { id displayName username avatarUrl } } }";
    let body = graphql_body(
        graphql_operation(
            &router,
            query,
            serde_json::json!({}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert_eq!(body["data"]["userProfile"]["node"]["displayName"], "James");
    assert!(body["data"]["userProfile"]["node"]["avatarUrl"].is_null());
    assert_eq!(fixture.repo.0.lock().unwrap().freshness_reads, 1);
    assert_eq!(body["data"]["userProfile"]["node"]["username"], "james");
    let mutation = "mutation($input: UserProfileUpdateInput!) { userProfileUpdate(input: $input) { node { id displayName avatarUrl } } }";
    let body = graphql_body(
        graphql_operation(
            &router,
            mutation,
            serde_json::json!({"input":{"displayName":"  New Name  ","avatarUploaded":true}}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(body["errors"].is_null(), "{body}");
    assert_eq!(
        body["data"]["userProfileUpdate"]["node"]["displayName"],
        "New Name"
    );
    assert!(
        body["data"]["userProfileUpdate"]["node"]["avatarUrl"]
            .as_str()
            .unwrap()
            .starts_with("https://cdn.example.com/assets/avatars/1?v=")
    );
    assert_eq!(
        fixture
            .repo
            .0
            .lock()
            .unwrap()
            .user
            .as_ref()
            .unwrap()
            .avatar_path
            .as_deref(),
        Some("/avatars/1")
    );
    let body = graphql_body(
        graphql_operation(
            &router,
            query,
            serde_json::json!({}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert_eq!(
        body["data"]["userProfile"]["node"]["displayName"],
        "New Name"
    );
    let body = graphql_body(
        graphql_operation(
            &router,
            mutation,
            serde_json::json!({"input":{"displayName":"Name Only"}}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(body["errors"].is_null(), "{body}");
    assert_eq!(
        body["data"]["userProfileUpdate"]["node"]["displayName"],
        "Name Only"
    );
    assert_eq!(
        fixture
            .repo
            .0
            .lock()
            .unwrap()
            .user
            .as_ref()
            .unwrap()
            .avatar_path
            .as_deref(),
        Some("/avatars/1")
    );
    let denied = graphql_operation(
        &router,
        mutation,
        serde_json::json!({"input":{"displayName":"Denied"}}),
        None,
        None,
        false,
    )
    .await;
    assert_graphql_denial(denied, 401).await;
    let upload = "mutation { generateUrlAvatarUrl(input: {mediaType: PNG, size: 1024}) { node { url fields { name value } expiresAt } } }";
    assert_graphql_denial(
        graphql_operation(&router, upload, serde_json::json!({}), None, None, false).await,
        401,
    )
    .await;
    let body = graphql_body(
        graphql_operation(
            &router,
            upload,
            serde_json::json!({}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(body["errors"].is_null(), "{body}");
    assert_eq!(
        body["data"]["generateUrlAvatarUrl"]["node"]["fields"][0]["value"],
        "/avatars/1"
    );
    let body = graphql_body(
        graphql_operation(
            &router,
            "{ userProfile(id: \"1\") { node { email } } }",
            serde_json::json!({}),
            None,
            None,
            false,
        )
        .await,
    )
    .await;
    assert_eq!(body["errors"][0]["extensions"]["status"], 401);
}

#[tokio::test]
async fn graphql_username_updates_and_authenticated_email_leave_full_name_independent() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let router = fixture.graphql();
    let query = "{ userProfile(id: \"1\") { node { username displayName email } } }";
    let body = graphql_body(
        graphql_operation(
            &router,
            query,
            serde_json::json!({}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(body["errors"].is_null());
    assert_eq!(
        body["data"]["userProfile"]["node"]["email"],
        "james@midwess.com"
    );
    let mutation = "mutation($input: UserProfileUpdateInput!) { userProfileUpdate(input: $input) { node { username displayName email } } }";
    let body = graphql_body(
        graphql_operation(
            &router,
            mutation,
            serde_json::json!({"input":{"username":"  New.User  ","displayName":""}}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(body["errors"].is_null(), "{body}");
    assert_eq!(
        body["data"]["userProfileUpdate"]["node"]["username"],
        "new.user"
    );
    assert_eq!(body["data"]["userProfileUpdate"]["node"]["displayName"], "");
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .taken_usernames
        .insert("taken".into());
    let body = graphql_body(
        graphql_operation(
            &router,
            mutation,
            serde_json::json!({"input":{"username":"taken","displayName":"Should not save"}}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert_eq!(body["errors"][0]["extensions"]["code"], "USERNAME_TAKEN");
    let user = fixture.repo.0.lock().unwrap().user.clone().unwrap();
    assert_eq!(user.username, "new.user");
    assert!(user.display_name.is_empty());
    let response = graphql_operation(
        &router,
        mutation,
        serde_json::json!({"input":{"email":"changed@example.com"}}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    assert_eq!(response.status(), 400);
    assert_eq!(
        fixture.repo.0.lock().unwrap().user.as_ref().unwrap().email,
        "james@midwess.com"
    );
    fixture.repo.0.lock().unwrap().user.as_mut().unwrap().id = 2;
    let body = graphql_body(
        graphql_operation(
            &router,
            "{ userProfile(id: \"2\") { node { email } } }",
            serde_json::json!({}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert_eq!(
        body["data"]["userProfile"]["node"]["email"],
        "james@midwess.com"
    );
    assert!(body["errors"].is_null(), "{body}");
}

#[tokio::test]
async fn graphql_stale_cookie_rebuilds_current_authority_but_bearer_and_unavailable_state_fail_closed()
 {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let router = fixture.graphql();
    {
        let mut state = fixture.repo.0.lock().unwrap();
        state.memberships[0].role.cedar_source.clear();
        state
            .sessions
            .values_mut()
            .next()
            .unwrap()
            .authorization_version += 1;
    }
    let cookies = format!(
        "compallm-access={}; compallm-refresh={}",
        tokens.access_token, tokens.refresh_token
    );
    let response = graphql_request(&router, Some(&tokens.access_token), Some(&cookies), true).await;
    assert!(!response.headers().contains_key("set-cookie"));
    assert_graphql_denial(response, 401).await;
    assert_eq!(fixture.repo.0.lock().unwrap().authority_reads, 0);

    let response = graphql_request(&router, None, Some(&cookies), false).await;
    assert!(!response.headers().contains_key("set-cookie"));
    assert_graphql_denial(response, 403).await;
    assert_eq!(fixture.repo.0.lock().unwrap().authority_reads, 0);

    let response = graphql_request(&router, None, Some(&cookies), true).await;
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get_all("set-cookie").iter().count(), 2);
    let renewed = response
        .headers()
        .get_all("set-cookie")
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find_map(|value| value.split(';').next()?.strip_prefix("compallm-access="))
        .unwrap();
    let renewed = UserAccessToken::try_from(decode_token(renewed, SECRET).unwrap()).unwrap();
    assert_eq!(renewed.session.authorization_version, 2);
    assert!(renewed.policy_snapshot.cedar_source.is_empty());
    {
        let mut state = fixture.repo.0.lock().unwrap();
        assert_eq!(state.authority_reads, 1);
        assert_eq!(state.freshness_reads, 2);
        state.fail_authority_read = true;
    }
    let response = graphql_request(&router, None, Some(&cookies), true).await;
    assert!(!response.headers().contains_key("set-cookie"));
    assert_graphql_denial(response, 503).await;
    let state = fixture.repo.0.lock().unwrap();
    assert_eq!(state.authority_reads, 1);
    assert_eq!(state.logins.len(), 1);
    assert_eq!(
        state
            .sessions
            .values()
            .next()
            .unwrap()
            .authorization_version,
        2
    );
}

#[tokio::test]
async fn authorization_context_requires_current_verified_user() {
    use axum::{
        Extension, Router,
        body::Body,
        http::Request,
        middleware::{from_fn, from_fn_with_state},
        routing::get,
    };
    use compallm_api::http::{
        authentication::AuthenticationSettings,
        authorization::{self, AuthorizationContext},
        middleware::{AuthenticationMiddleware, AuthorizationMiddleware},
    };
    use tower::ServiceExt;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let authentication = AuthenticationMiddleware::optional(
        Box::leak(Box::new(fixture.repo.1.clone())),
        Box::leak(Box::new(fixture.service())),
        AuthenticationSettings {
            cookie_secure: false,
            public_origin: "http://localhost".into(),
        },
    )
    .unwrap();
    let router = Router::new().route(
        "/",
        get(
            |Extension(context): Extension<AuthorizationContext>,
             Extension(authentication): Extension<
                Arc<compallm_api::http::authentication::AuthenticationContext>,
            >| async move {
                let db = DbContext::current().unwrap();
                let first = Arc::as_ptr(&context.member_permission().await?) as usize;
                let permission = context.member_permission().await?;
                assert_eq!(first, Arc::as_ptr(&permission) as usize);
                assert_eq!(first, db.permission().await.unwrap() as *const _ as usize);
                let rls = PostgresConnection::from_context(&db)
                    .unwrap()
                    .rls()
                    .unwrap();
                assert_eq!(
                    rls.get().await.unwrap().organization_id(),
                    permission.organization_id()
                );
                let trusted = authentication
                    .execute(async {
                        let current = DbContext::current()?;
                        Ok::<_, AuthError>(
                            PostgresConnection::from_context(&current)?.rls().is_none(),
                        )
                    })
                    .await
                    .unwrap();
                assert!(trusted);
                assert_eq!(
                    first,
                    DbContext::current().unwrap().permission().await.unwrap() as *const _ as usize
                );
                Ok::<_, authorization::AuthorizationError>(axum::Json((
                    permission.user_id(),
                    permission.organization_id(),
                )))
            },
        )
        .route_layer(from_fn(AuthorizationMiddleware::handle))
        .route_layer(from_fn_with_state(
            authentication,
            AuthenticationMiddleware::handle,
        )),
    );

    let anonymous = router
        .clone()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(anonymous.status(), 401);
    let request = || {
        Request::builder()
            .uri("/")
            .header("authorization", format!("Bearer {}", tokens.access_token))
            .body(Body::empty())
            .unwrap()
    };
    let response = router.clone().oneshot(request()).await.unwrap();
    assert_eq!(response.status(), 200);
    let body = axum::body::to_bytes(response.into_body(), 8192)
        .await
        .unwrap();
    let (user_id, organization_id): (u64, u64) = serde_json::from_slice(&body).unwrap();
    assert_eq!(organization_id, 84);
    assert_eq!(
        user_id,
        fixture.repo.0.lock().unwrap().user.as_ref().unwrap().id
    );
    assert_eq!(fixture.repo.0.lock().unwrap().freshness_reads, 1);
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .values_mut()
        .next()
        .unwrap()
        .revoke();
    assert_eq!(router.oneshot(request()).await.unwrap().status(), 401);
}

#[tokio::test]
async fn shared_http_authentication_passes_current_identity_and_blocks_revoked_requests() {
    use axum::{
        Extension, Router, body::Body, http::Request, middleware::from_fn_with_state, routing::post,
    };
    use compallm_api::http::{
        authentication::{AuthenticationSettings, VerifiedAccess},
        middleware::AuthenticationMiddleware,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tower::ServiceExt;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let authentication = AuthenticationMiddleware::required(
        Box::leak(Box::new(fixture.repo.1.clone())),
        Box::leak(Box::new(fixture.service())),
        AuthenticationSettings {
            cookie_secure: false,
            public_origin: "http://localhost".into(),
        },
    )
    .unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let handler_calls = calls.clone();
    let router = Router::new().route(
        "/protected",
        post(move |Extension(identity): Extension<VerifiedAccess>| {
            let calls = handler_calls.clone();
            async move {
                calls.fetch_add(1, Ordering::SeqCst);
                axum::Json(identity.access().session.user_id)
            }
        })
        .route_layer(from_fn_with_state(
            authentication,
            AuthenticationMiddleware::handle,
        )),
    );
    let request = || {
        Request::builder()
            .method("POST")
            .uri("/protected")
            .header("authorization", format!("Bearer {}", tokens.access_token))
            .body(Body::empty())
            .unwrap()
    };
    let response = router.clone().oneshot(request()).await.unwrap();
    assert_eq!(response.status(), 200);
    let bytes = axum::body::to_bytes(response.into_body(), 8192)
        .await
        .unwrap();
    let user_id: u64 = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        user_id,
        fixture.repo.0.lock().unwrap().user.as_ref().unwrap().id
    );
    assert_eq!(fixture.repo.0.lock().unwrap().freshness_reads, 1);

    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .values_mut()
        .next()
        .unwrap()
        .revoke();
    let response = router.oneshot(request()).await.unwrap();
    assert_eq!(response.status(), 401);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn graphql_shared_schema_keeps_concurrent_session_identities_separate() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let first_challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let first = fixture
        .signin(&Fixture::otp(&first_challenge), &first_challenge.code)
        .await
        .unwrap();
    let second_challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let second = fixture
        .signin(&Fixture::otp(&second_challenge), &second_challenge.code)
        .await
        .unwrap();
    let first_claims =
        UserAccessToken::try_from(decode_token(&first.access_token, SECRET).unwrap()).unwrap();
    let second_claims =
        UserAccessToken::try_from(decode_token(&second.access_token, SECRET).unwrap()).unwrap();
    assert_ne!(first_claims.session.id, second_claims.session.id);
    let router = fixture.graphql();
    let (first_response, second_response) = tokio::join!(
        graphql_request(&router, Some(&first.access_token), None, false),
        graphql_request(&router, Some(&second.access_token), None, false),
    );
    for (response, expected) in [
        (first_response, first_claims.session),
        (second_response, second_claims.session),
    ] {
        assert_eq!(response.status(), 200);
        let bytes = axum::body::to_bytes(response.into_body(), 8192)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert!(body.get("errors").is_none());
        assert_eq!(body["data"]["first"]["node"]["id"], expected.id.to_string());
        assert_eq!(
            body["data"]["second"]["node"]["id"],
            expected.id.to_string()
        );
        assert_eq!(
            body["data"]["first"]["node"]["userId"],
            expected.user_id.to_string()
        );
        assert_eq!(
            body["data"]["first"]["node"]["organizationId"],
            expected.organization_id.to_string()
        );
    }
    assert_eq!(fixture.repo.0.lock().unwrap().freshness_reads, 2);
}

async fn graphql_operation(
    router: &axum::Router,
    query: &str,
    variables: serde_json::Value,
    bearer: Option<&str>,
    cookies: Option<&str>,
    origin: bool,
) -> axum::response::Response {
    use tower::ServiceExt;
    let mut request = axum::http::Request::builder()
        .method("POST")
        .uri("/api/graphql")
        .header("content-type", "application/json")
        .header("host", "localhost");
    if let Some(token) = bearer {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    if let Some(cookies) = cookies {
        request = request.header("cookie", cookies);
    }
    if origin {
        request = request.header("origin", "http://localhost");
    }
    router
        .clone()
        .oneshot(
            request
                .body(axum::body::Body::from(
                    serde_json::json!({"query":query,"variables":variables}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn graphql_rejects_multiple_root_writes() {
    let fixture = Fixture::new(None);
    let router = fixture.graphql();
    let query = r#"mutation { first: roleCreate(input: {name: "one", description: "", cedarSource: ""}) {node{ id }} second: roleCreate(input: {name: "two", description: "", cedarSource: ""}) {node{ id }} }"#;
    let response =
        graphql_operation(&router, query, serde_json::json!({}), None, None, false).await;
    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    let fragment_query = r#"mutation { ...Writes } fragment Writes on Mutation { first: groupCreate(input: {name: "one", description: "", roleIds: []}) {node{ id }} second: groupCreate(input: {name: "two", description: "", roleIds: []}) {node{ id }} }"#;
    let response = graphql_operation(
        &router,
        fragment_query,
        serde_json::json!({}),
        None,
        None,
        false,
    )
    .await;
    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    let single = r#"mutation { ...Write } fragment Write on Mutation { groupCreate(input: {name: "one", description: "", roleIds: []}) {node{ id }} }"#;
    let response =
        graphql_operation(&router, single, serde_json::json!({}), None, None, false).await;
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    assert_eq!(
        graphql_body(response).await["errors"][0]["extensions"]["status"],
        401
    );
    let response = graphql_operation(
        &router,
        "query { session { node { id } } organization { node { id } } role(id: \"1\") { node { id } } roles { edges { node { id } } } group(id: \"1\") { node { id } } groups { edges { node { id } } } }",
        serde_json::json!({}), None, None, false,
    ).await;
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = graphql_body(response).await;
    assert!(
        body["errors"]
            .as_array()
            .unwrap()
            .iter()
            .all(|error| error["extensions"]["status"] == 401)
    );
}

async fn graphql_body(response: axum::response::Response) -> serde_json::Value {
    let bytes = axum::body::to_bytes(response.into_body(), 1_048_576)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

fn response_cookies(response: &axum::response::Response) -> String {
    response
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|value| {
            value
                .to_str()
                .unwrap()
                .split(';')
                .next()
                .unwrap()
                .to_owned()
        })
        .collect::<Vec<_>>()
        .join("; ")
}

#[tokio::test]
async fn graphql_token_signin_refresh_and_revoke_preserve_session_authority() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let router = fixture.graphql().layer(axum::middleware::from_fn(
        |mut request: axum::extract::Request, next: axum::middleware::Next| async move {
            request.extensions_mut().insert(axum::extract::ConnectInfo(
                "203.0.113.42:12345"
                    .parse::<std::net::SocketAddr>()
                    .unwrap(),
            ));
            request
                .headers_mut()
                .insert("user-agent", "recognizable-test-browser".parse().unwrap());
            request
                .headers_mut()
                .insert("x-forwarded-for", "198.51.100.1".parse().unwrap());
            next.run(request).await
        },
    ));
    let response = graphql_operation(&router,
        "mutation($input:EmailInput!){emailSigninRequest(input:$input){node{token lifetime{expiresInSeconds}}}}",
        serde_json::json!({"input":{"email":"james@midwess.com","delivery":"TOKEN"}}),Some("invalid-bearer"),None,false).await;
    assert_eq!(response.status(), 200);
    assert!(!response.headers().contains_key("set-cookie"));
    let body = graphql_body(response).await;
    let challenge = body["data"]["emailSigninRequest"]["node"]["token"]
        .as_str()
        .unwrap();
    let claims = EmailOtpAccessToken::try_from(decode_token(challenge, SECRET).unwrap()).unwrap();
    assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
    let response = graphql_operation(&router,
        "mutation($input:EmailCodeInput!){emailSigninConfirm(input:$input){node{accessToken refreshToken}}}",
        serde_json::json!({"input":{"code":claims.code,"delivery":"TOKEN"}}),Some(challenge),None,false).await;
    assert_eq!(response.status(), 200);
    assert!(!response.headers().contains_key("set-cookie"));
    let body = graphql_body(response).await;
    let tokens = &body["data"]["emailSigninConfirm"]["node"];
    let access = tokens["accessToken"].as_str().unwrap();
    let refresh = tokens["refreshToken"].as_str().unwrap();
    assert_eq!(fixture.repo.0.lock().unwrap().logins.len(), 1);
    let signed = UserAccessToken::try_from(decode_token(access, SECRET).unwrap()).unwrap();
    let metadata = signed.session.metadata.clone();
    assert_eq!(metadata.ip_address, Some("203.0.113.42".parse().unwrap()));
    assert_eq!(
        metadata.user_agent.as_deref(),
        Some("recognizable-test-browser")
    );
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&signed.session.id].metadata,
        metadata
    );
    assert_eq!(
        RefreshToken::try_from(decode_token(refresh, SECRET).unwrap())
            .unwrap()
            .session
            .metadata,
        metadata
    );
    let info = graphql_body(
        graphql_operation(
            &router,
            "{session{node{ipAddress userAgent}}}",
            serde_json::json!({}),
            Some(access),
            None,
            false,
        )
        .await,
    )
    .await;
    assert_eq!(info["data"]["session"]["node"]["ipAddress"], "203.0.113.42");
    assert_eq!(
        info["data"]["session"]["node"]["userAgent"],
        "recognizable-test-browser"
    );

    assert_eq!(
        graphql_request(&router, Some(access), None, false)
            .await
            .status(),
        200
    );
    let response = graphql_operation(
        &router,
        "mutation{sessionRefresh{node{accessToken refreshToken}}}",
        serde_json::json!({}),
        Some(refresh),
        None,
        false,
    )
    .await;
    assert!(!response.headers().contains_key("set-cookie"));
    let renewed = graphql_body(response).await;
    let renewed_refresh = renewed["data"]["sessionRefresh"]["node"]["refreshToken"]
        .as_str()
        .unwrap();
    assert_eq!(
        RefreshToken::try_from(decode_token(renewed_refresh, SECRET).unwrap())
            .unwrap()
            .session
            .metadata,
        metadata
    );
    let access = renewed["data"]["sessionRefresh"]["node"]["accessToken"]
        .as_str()
        .unwrap();
    assert_eq!(fixture.repo.0.lock().unwrap().logins.len(), 1);
    let signed = UserAccessToken::try_from(decode_token(access, SECRET).unwrap()).unwrap();
    let metadata = signed.session.metadata.clone();
    assert_eq!(metadata.ip_address, Some("203.0.113.42".parse().unwrap()));
    assert_eq!(
        metadata.user_agent.as_deref(),
        Some("recognizable-test-browser")
    );
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&signed.session.id].metadata,
        metadata
    );
    assert_eq!(
        RefreshToken::try_from(decode_token(refresh, SECRET).unwrap())
            .unwrap()
            .session
            .metadata,
        metadata
    );
    let info = graphql_body(
        graphql_operation(
            &router,
            "{session{node{ipAddress userAgent}}}",
            serde_json::json!({}),
            Some(access),
            None,
            false,
        )
        .await,
    )
    .await;
    assert_eq!(info["data"]["session"]["node"]["ipAddress"], "203.0.113.42");
    assert_eq!(
        info["data"]["session"]["node"]["userAgent"],
        "recognizable-test-browser"
    );

    let response = graphql_operation(
        &router,
        "mutation{sessionRevoke{node}}",
        serde_json::json!({}),
        Some(access),
        None,
        false,
    )
    .await;
    assert!(!response.headers().contains_key("set-cookie"));
    assert_eq!(
        graphql_body(response).await["data"]["sessionRevoke"]["node"],
        true
    );
    assert_graphql_denial(
        graphql_request(&router, Some(access), None, false).await,
        401,
    )
    .await;
    fixture.assert_no_transaction();
}

#[tokio::test]
async fn graphql_cookie_signup_and_signout_reject_combined_authentication_before_effects() {
    let fixture = Fixture::new(None);
    let router = fixture.graphql();
    let response = graphql_operation(&router,
        "mutation($input:EmailInput!){emailSignupRequest(input:$input){node{token lifetime{expiresInSeconds}}}}",
        serde_json::json!({"input":{"email":"james@midwess.com"}}),None,None,true).await;
    let cookies = response_cookies(&response);
    assert_eq!(response.status(), 200);
    assert!(graphql_body(response).await["data"]["emailSignupRequest"]["node"]["token"].is_null());
    let challenge = cookies
        .split(';')
        .find_map(|value| value.trim().strip_prefix("compallm-access="))
        .unwrap();
    let claims = EmailOtpAccessToken::try_from(decode_token(challenge, SECRET).unwrap()).unwrap();
    let input = serde_json::json!({"input":{"code":claims.code}});
    // Aliases and fragments cannot conceal a second auth effect, even if skipped.
    let response = graphql_operation(&router,
        "mutation($input:EmailCodeInput!){...A second:emailSignupConfirm(input:$input) @skip(if:true){node{accessToken}}} fragment A on Mutation{first:emailSignupConfirm(input:$input){node{accessToken}}}",
        input.clone(),None,Some(&cookies),true).await;
    assert_eq!(response.status(), 400);
    assert!(fixture.repo.0.lock().unwrap().created.is_none());
    let query = "mutation($input:EmailCodeInput!){emailSignupConfirm(input:$input){node{accessToken refreshToken accessLifetime{expiresInSeconds}}}}";
    let response =
        graphql_operation(&router, query, input.clone(), None, Some(&cookies), false).await;
    assert_graphql_denial(response, 403).await;
    assert!(fixture.repo.0.lock().unwrap().created.is_none());
    let response = graphql_operation(&router, query, input, None, Some(&cookies), true).await;
    let cookies = response_cookies(&response);
    let body = graphql_body(response).await;
    assert!(body["data"]["emailSignupConfirm"]["node"].is_object());
    assert!(body["data"]["emailSignupConfirm"]["node"]["accessToken"].is_null());
    assert!(body["data"]["emailSignupConfirm"]["node"]["refreshToken"].is_null());
    assert!(fixture.repo.0.lock().unwrap().created.is_some());
    assert_eq!(fixture.repo.0.lock().unwrap().logins.len(), 1);
    assert_eq!(
        graphql_request(&router, None, Some(&cookies), true)
            .await
            .status(),
        200
    );
    let response = graphql_operation(
        &router,
        "mutation{sessionRefresh{node{accessToken}}}",
        serde_json::json!({}),
        None,
        Some(&cookies),
        true,
    )
    .await;
    assert_graphql_denial(response, 401).await;
    let response = graphql_operation(
        &router,
        "mutation{sessionRevoke{node}}",
        serde_json::json!({}),
        None,
        Some(&cookies),
        true,
    )
    .await;
    assert_eq!(response.headers().get_all("set-cookie").iter().count(), 4);
    assert_eq!(
        graphql_body(response).await["data"]["sessionRevoke"]["node"],
        true
    );
    assert_graphql_denial(
        graphql_request(&router, None, Some(&cookies), true).await,
        401,
    )
    .await;
}

async fn assert_graphql_denial(response: axum::response::Response, status: u64) {
    assert_eq!(response.status(), 200);
    let body = graphql_body(response).await;
    assert!(body["data"].is_null());
    assert_eq!(body["errors"][0]["extensions"]["status"], status);
}

#[tokio::test]
async fn request_membership_is_lazy_and_concurrent_request_scopes_are_isolated() {
    use axum::{
        Extension, Router,
        body::Body,
        http::Request,
        middleware::{from_fn, from_fn_with_state},
        routing::get,
    };
    use compallm_api::http::{
        authentication::AuthenticationSettings,
        authorization::AuthorizationContext,
        middleware::{AuthenticationMiddleware, AuthorizationMiddleware},
    };
    use tower::ServiceExt;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let middleware = AuthenticationMiddleware::optional(
        Box::leak(Box::new(fixture.repo.1.clone())),
        Box::leak(Box::new(fixture.service())),
        AuthenticationSettings {
            cookie_secure: false,
            public_origin: "http://localhost".into(),
        },
    )
    .unwrap();
    let barrier = Arc::new(tokio::sync::Barrier::new(2));
    let contexts = Arc::new(Mutex::new(Vec::new()));
    let handler_contexts = contexts.clone();
    let router = Router::new()
        .route(
            "/public",
            get(|| async { axum::http::StatusCode::NO_CONTENT }),
        )
        .route(
            "/protected",
            get(
                move |Extension(authority): Extension<AuthorizationContext>| {
                    let barrier = barrier.clone();
                    let contexts = handler_contexts.clone();
                    async move {
                        let db = DbContext::current().unwrap();
                        assert!(
                            PostgresConnection::from_context(&db)
                                .unwrap()
                                .rls()
                                .is_some()
                        );
                        barrier.wait().await;
                        let permission = db.permission().await.unwrap();
                        assert_eq!(permission.organization_id(), 84);
                        let shared = authority.member_permission().await.unwrap();
                        assert!(std::ptr::eq(permission, shared.as_ref()));
                        contexts.lock().unwrap().push(db.clone());
                        axum::http::StatusCode::OK
                    }
                },
            ),
        )
        .route_layer(from_fn(AuthorizationMiddleware::handle))
        .route_layer(from_fn_with_state(
            middleware,
            AuthenticationMiddleware::handle,
        ));
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/public")
                .header("authorization", "Bearer intentionally-unused")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 204);
    assert_eq!(fixture.repo.0.lock().unwrap().freshness_reads, 0);
    let request = || {
        Request::builder()
            .uri("/protected")
            .header("authorization", format!("Bearer {}", tokens.access_token))
            .body(Body::empty())
            .unwrap()
    };
    let (left, right) = tokio::join!(router.clone().oneshot(request()), router.oneshot(request()));
    assert_eq!(left.unwrap().status(), 200);
    assert_eq!(right.unwrap().status(), 200);
    assert_eq!(fixture.repo.0.lock().unwrap().freshness_reads, 2);
    let contexts = contexts.lock().unwrap();
    assert!(!std::ptr::eq(
        contexts[0].postgres(),
        contexts[1].postgres()
    ));
    assert!(DbContext::current().is_err());
}

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL with the RLS migration applied"]
async fn http_request_membership_drives_postgres_rls_and_returns_a_clean_connection() {
    use axum::{
        Router,
        body::Body,
        http::Request,
        middleware::{from_fn, from_fn_with_state},
        routing::get,
    };
    use compallm_api::http::{
        authentication::AuthenticationSettings,
        middleware::{AuthenticationMiddleware, AuthorizationMiddleware},
    };
    use sea_orm::{ConnectOptions, ConnectionTrait, Database};
    use tower::ServiceExt;

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let mut options = ConnectOptions::new(std::env::var("COMPALLM_TEST_DATABASE_URL").unwrap());
    options.max_connections(1).min_connections(1);
    let pool = Arc::new(Database::connect(options).await.unwrap());
    let baseline = pool
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "SELECT pg_backend_pid() AS pid, current_user::text AS role",
        ))
        .await
        .unwrap()
        .unwrap();
    let middleware = AuthenticationMiddleware::optional(
        Box::leak(Box::new(DbContext::new(Arc::new(PostgresConnection::new(
            pool.clone(),
        ))))),
        Box::leak(Box::new(fixture.service())),
        AuthenticationSettings {
            cookie_secure: false,
            public_origin: "http://localhost".into(),
        },
    )
    .unwrap();
    let router = Router::new().route("/", get(|| async {
        let db = DbContext::current().unwrap();
        let result = PostgresConnection::from_context(&db).unwrap().query_one_raw(
            Statement::from_string(DbBackend::Postgres,
                "SELECT pg_backend_pid() AS pid, current_user::text AS role, compallm_rls.organization_id() AS organization, compallm_rls.organization_allowed('model.read') AS model_read, compallm_rls.organization_allowed('provider.read') AS provider_read, (SELECT count(*) FROM providers) AS providers")
        ).await.unwrap().unwrap();
        assert_eq!(result.try_get::<String>("", "role").unwrap(), "compallm_restricted");
        assert_eq!(result.try_get::<i64>("", "organization").unwrap(), 84);
        assert!(result.try_get::<bool>("", "model_read").unwrap());
        assert!(!result.try_get::<bool>("", "provider_read").unwrap());
        assert_eq!(result.try_get::<i64>("", "providers").unwrap(), 0);
        axum::http::StatusCode::OK
  }))
    .route_layer(from_fn(AuthorizationMiddleware::handle))
    .route_layer(from_fn_with_state(middleware, AuthenticationMiddleware::handle));
    let response = tokio::time::timeout(
        Duration::from_secs(10),
        router.oneshot(
            Request::builder()
                .header("authorization", format!("Bearer {}", tokens.access_token))
                .body(Body::empty())
                .unwrap(),
        ),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(fixture.repo.0.lock().unwrap().freshness_reads, 1);
    let clean = pool.query_one_raw(Statement::from_string(DbBackend::Postgres,
        "SELECT pg_backend_pid() AS pid, current_user::text AS role, coalesce(current_setting('app.organization_id',true),'') AS organization, coalesce(current_setting('app.model.read',true),'') AS model_read"))
        .await.unwrap().unwrap();
    assert_eq!(
        clean.try_get::<i32>("", "pid").unwrap(),
        baseline.try_get::<i32>("", "pid").unwrap()
    );
    assert_eq!(
        clean.try_get::<String>("", "role").unwrap(),
        baseline.try_get::<String>("", "role").unwrap()
    );
    assert!(
        clean
            .try_get::<String>("", "organization")
            .unwrap()
            .is_empty()
    );
    assert!(
        clean
            .try_get::<String>("", "model_read")
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL with the RLS migration applied"]
async fn restricted_query_errors_and_cancellation_discard_dirty_connections() {
    use sea_orm::{ConnectOptions, ConnectionTrait, Database};
    use std::convert::Infallible;

    let url = std::env::var("COMPALLM_TEST_DATABASE_URL").unwrap();
    let mut options = ConnectOptions::new(&url);
    options.max_connections(1).min_connections(1);
    let pool = Arc::new(Database::connect(options).await.unwrap());
    let baseline = pool
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "SELECT pg_backend_pid() AS pid, current_user::text AS role",
        ))
        .await
        .unwrap()
        .unwrap();
    let member = OrganizationMember {
        organization_id: 84,
        user_id: 1,
        groups: Vec::new(),
        role: compallm_core::organization::Role {
            id: 3,
            organization_id: 84,
            name: "Member".into(),
            description: String::new(),
            cedar_source: String::new(),
            org_owner: false,
        },
    };
    let access = UserAccessToken::new(
        stored_session(1, 1, 84, "RLS fixture"),
        compallm_core::authorization::PolicySnapshot::new(&member).unwrap(),
        Duration::from_secs(60),
    )
    .unwrap();
    let permission =
        Arc::new(compallm_core::authorization::MemberPermission::try_from(access).unwrap());
    let restricted = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
    restricted
        .restrict(async move { Ok::<_, Infallible>(permission) })
        .unwrap();
    assert!(
        PostgresConnection::from_context(&restricted)
            .unwrap()
            .query_one_raw(Statement::from_string(DbBackend::Postgres, "SELECT 1 / 0"))
            .await
            .is_err()
    );
    let clean = pool.query_one_raw(Statement::from_string(DbBackend::Postgres,
        "SELECT pg_backend_pid() AS pid, current_user::text AS role, coalesce(current_setting('app.organization_id',true),'') AS organization"))
        .await.unwrap().unwrap();
    let pid = clean.try_get::<i32>("", "pid").unwrap();
    assert_ne!(pid, baseline.try_get::<i32>("", "pid").unwrap());
    assert_eq!(
        clean.try_get::<String>("", "role").unwrap(),
        baseline.try_get::<String>("", "role").unwrap()
    );
    assert!(
        clean
            .try_get::<String>("", "organization")
            .unwrap()
            .is_empty()
    );

    let waiter = tokio::spawn(async move {
        PostgresConnection::from_context(&restricted)
            .unwrap()
            .query_all_raw(Statement::from_string(
                DbBackend::Postgres,
                "SELECT pg_sleep(20)",
            ))
            .await
    });
    let observer = Database::connect(&url).await.unwrap();
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let row = observer.query_one_raw(Statement::from_sql_and_values(DbBackend::Postgres,
                "SELECT EXISTS (SELECT 1 FROM pg_stat_activity WHERE pid = $1 AND state = 'active' AND query = 'SELECT pg_sleep(20)') AS started",
                [pid.into()])).await.unwrap().unwrap();
            if row.try_get::<bool>("", "started").unwrap() { break; }
            tokio::time::sleep(Duration::from_millis(10)).await;
      }
  }).await.unwrap();
    waiter.abort();
    assert!(waiter.await.unwrap_err().is_cancelled());
    let clean = tokio::time::timeout(Duration::from_secs(5), pool.query_one_raw(Statement::from_string(DbBackend::Postgres,
        "SELECT pg_backend_pid() AS pid, current_user::text AS role, coalesce(current_setting('app.organization_id',true),'') AS organization")))
        .await.unwrap().unwrap().unwrap();
    assert_ne!(clean.try_get::<i32>("", "pid").unwrap(), pid);
    assert_eq!(
        clean.try_get::<String>("", "role").unwrap(),
        baseline.try_get::<String>("", "role").unwrap()
    );
    assert!(
        clean
            .try_get::<String>("", "organization")
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL with the RLS migration applied"]
async fn graphql_role_and_group_creation_enforces_scope_and_denies_without_writes() {
    use compallm_postgres::{OrganizationSchema, OrganizationStatusSchema};
    use sea_orm::{ActiveModelTrait, Database, EntityTrait, IntoActiveModel};

    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let pool = Arc::new(
        Database::connect(std::env::var("COMPALLM_TEST_DATABASE_URL").unwrap())
            .await
            .unwrap(),
    );
    let organization = Organization::new("Role and group API fixture")
        .await
        .unwrap();
    let foreign = Organization::new("Foreign role API fixture").await.unwrap();
    for org in [&organization, &foreign] {
        OrganizationSchema {
            avatar_path: None,
            id: i64::try_from(org.id).unwrap(),
            name: org.name.clone(),
            status: OrganizationStatusSchema::Active,
        }
        .into_active_model()
        .insert(pool.as_ref())
        .await
        .unwrap();
    }
    let connection = Arc::new(PostgresConnection::new(pool.clone()));
    TestRoot::set(connection.clone());
    let db = DbContext::new(connection);
    let repository = compallm_postgres::repository::organization::OrganizationRepository;
    let foreign_role = Role::new(foreign.id, "Foreign role", "", "").await.unwrap();
    db.scope(repository.create_roles(vec![foreign_role.clone()]))
        .await
        .unwrap();

    let session = stored_session(1, 7, organization.id, &organization.name);
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .insert(session.id, session.clone());
    let source = format!(
        r#"permit(principal, action in [Compallm::Action::"role.read", Compallm::Action::"role.create", Compallm::Action::"role.assign", Compallm::Action::"role.delete", Compallm::Action::"group.read", Compallm::Action::"group.create", Compallm::Action::"member.read", Compallm::Action::"group.metadata.update", Compallm::Action::"group.delete", Compallm::Action::"role.unassign"], resource in Compallm::Organization::"{org}");
permit(principal, action in [Compallm::Action::"group.role.assign", Compallm::Action::"group.role.unassign"], resource is Compallm::Group in Compallm::Organization::"{org}") when {{ context.role.organization == Compallm::Organization::"{org}" }};
permit(principal, action in [Compallm::Action::"group.member.add", Compallm::Action::"group.member.remove"], resource is Compallm::Group in Compallm::Organization::"{org}") when {{ context.member.organization == Compallm::Organization::"{org}" }};"#,
        org = organization.id
    );
    let snapshot = PolicySnapshot {
        cedar_source: source,
        group_roles: Default::default(),
        role_id: 3,
    };
    let tokens = token_pair::new(
        &session,
        snapshot,
        SECRET,
        Duration::from_secs(60),
        REFRESH_LIFETIME,
    )
    .unwrap();
    let access = db
        .scope(fixture.service().authenticate_access(&tokens.access_token))
        .await
        .unwrap();
    let permission =
        Arc::new(compallm_core::authorization::MemberPermission::try_from(access.clone()).unwrap());
    let scoped = db
        .fresh()
        .unwrap()
        .restrict(async move { Ok::<_, std::convert::Infallible>(permission) })
        .unwrap();
    scoped
        .scope(async {
            assert!(fixture.service().validate_access(&access).await.unwrap());
            assert!(
                PostgresConnection::from_context(&DbContext::current().unwrap())
                    .unwrap()
                    .rls()
                    .is_some()
            );
            assert_eq!(
                DbContext::current()
                    .unwrap()
                    .permission()
                    .await
                    .unwrap()
                    .organization_id(),
                organization.id
            );

            Ok::<_, compallm_core::persistent::ConnectionError>(())
        })
        .await
        .unwrap();
    assert!(
        scoped
            .transaction(fixture.service().validate_access(&access))
            .await
            .is_err()
    );
    let router = fixture.graphql_with_org_repository(db.clone(), None, None, None, true);
    let policy = format!(
        r#"permit(principal, action == Compallm::Action::"model.read", resource == Compallm::Organization::"{}");"#,
        organization.id
    );
    let create_role =
        "mutation($input:RoleCreateInput!){roleCreate(input:$input){node{id name description}}}";
    let response = graphql_operation(&router, create_role, serde_json::json!({"input":{"name":"  API reader  ","description":"Read models", "cedarSource":policy}}), Some(&tokens.access_token), None, false).await;
    let body = graphql_body(response).await;
    assert!(body["errors"].is_null(), "{body}");
    let role = &body["data"]["roleCreate"]["node"];
    assert_eq!(role["name"], "API reader");
    let role_id = role["id"].as_str().unwrap().to_owned();
    let stored = db
        .scope(repository.find_role(organization.id, role_id.parse().unwrap()))
        .await
        .unwrap()
        .unwrap();
    let stored_policy: cedar_policy::PolicySet = stored.cedar_source.parse().unwrap();
    let submitted_policy: cedar_policy::PolicySet = policy.parse().unwrap();
    assert_eq!(
        stored_policy.policies().next().unwrap().to_json().unwrap(),
        submitted_policy
            .policies()
            .next()
            .unwrap()
            .to_json()
            .unwrap()
    );
    assert!(!stored.org_owner);

    let response = graphql_operation(
        &router,
        "mutation($input:GroupCreateInput!){groupCreate(input:$input){node{id name roles{edges{node{id}}}}}}",
        serde_json::json!({"input":{"name":"API readers", "description":"", "roleIds":[role_id]}}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    let body = graphql_body(response).await;
    assert!(body["errors"].is_null(), "{body}");
    let group = &body["data"]["groupCreate"]["node"];
    assert_eq!(group["roles"]["edges"][0]["node"]["id"], role_id);
    let group_id = group["id"].as_str().unwrap().to_owned();
    let stored = db
        .scope(repository.find_group(organization.id, group_id.parse().unwrap()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stored.roles().len(), 1);
    assert_eq!(stored.roles()[0].id.to_string(), role_id);

    let response = graphql_operation(&router,
        "query($role:ID!, $group:ID!, $foreign:ID!){roles{edges{node{id}}} role(id:$role){node{id}} foreign:role(id:$foreign){node{id}} groups{edges{node{id}}} group(id:$group){node{id}}}",
        serde_json::json!({"role":role_id,"group":group_id,"foreign":foreign_role.id.to_string()}), Some(&tokens.access_token), None, false).await;
    let body = graphql_body(response).await;
    assert!(body["errors"].is_null(), "{body}");
    assert_eq!(
        body["data"]["roles"]["edges"],
        serde_json::json!([{"node":{"id":role_id}}])
    );
    assert_eq!(
        body["data"]["groups"]["edges"],
        serde_json::json!([{"node":{"id":group_id}}])
    );
    assert_eq!(body["data"]["role"]["node"]["id"], role_id);
    assert_eq!(body["data"]["group"]["node"]["id"], group_id);
    assert!(body["data"]["foreign"].is_null());

    let duplicate = graphql_operation(
        &router,
        create_role,
        serde_json::json!({"input":{"name":"API reader", "description":"", "cedarSource":""}}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    assert_graphql_denial(duplicate, 409).await;
    let unsupported = format!(
        r#"permit(principal, action == Compallm::Action::"model.read", resource == Compallm::Organization::"{}") when {{ principal in [Compallm::User::"1"] }};"#,
        organization.id
    );
    let response = graphql_operation(&router, create_role, serde_json::json!({"input":{"name":"Unsupported policy", "description":"", "cedarSource":unsupported}}), Some(&tokens.access_token), None, false).await;
    assert_graphql_denial(response, 400).await;

    let denied = token_pair::new(
        &session,
        empty_snapshot(),
        SECRET,
        Duration::from_secs(60),
        REFRESH_LIFETIME,
    )
    .unwrap();
    for (query, input) in [
        (
            create_role,
            serde_json::json!({"name":"Denied role", "description":"", "cedarSource":""}),
        ),
        (
            "mutation($input:GroupCreateInput!){groupCreate(input:$input){node{id}}}",
            serde_json::json!({"name":"Denied group", "description":"", "roleIds":[]}),
        ),
    ] {
        let response = graphql_operation(
            &router,
            query,
            serde_json::json!({"input":input}),
            Some(&denied.access_token),
            None,
            false,
        )
        .await;
        assert_graphql_denial(response, 403).await;
    }
    let response = graphql_operation(&router,
        "mutation($input:GroupCreateInput!){groupCreate(input:$input){node{id}}}",
        serde_json::json!({"input":{"name":"Foreign assignment", "description":"", "roleIds":[foreign_role.id.to_string()]}}), Some(&tokens.access_token), None, false).await;
    assert_graphql_denial(response, 404).await;
    assert_eq!(
        scoped
            .scope(repository.list_role_info())
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        scoped
            .scope(repository.list_group_info())
            .await
            .unwrap()
            .len(),
        1
    );

    let delete_role = "mutation($id:ID!){roleDelete(id:$id){node}}";
    let assigned = graphql_operation(
        &router,
        delete_role,
        serde_json::json!({"id":role_id}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    assert_graphql_denial(assigned, 409).await;
    assert!(
        db.scope(repository.find_role(organization.id, role_id.parse().unwrap()))
            .await
            .unwrap()
            .is_some()
    );
    type GroupRoles =
        <compallm_postgres::OrganizationGroupRoleSchema as sea_orm::ModelTrait>::Entity;
    GroupRoles::delete_by_id((
        i64::try_from(organization.id).unwrap(),
        group_id.parse::<i64>().unwrap(),
        role_id.parse::<i64>().unwrap(),
    ))
    .exec(pool.as_ref())
    .await
    .unwrap();
    let assignee = User::from_email(&format!("role-delete-{}@example.com", organization.id))
        .await
        .unwrap();
    db.scope(compallm_postgres::repository::user::UserRepository.create(assignee.clone()))
        .await
        .unwrap();
    let assigned_member = OrganizationMember {
        organization_id: organization.id,
        user_id: assignee.id,
        groups: vec![],
        role: db
            .scope(repository.find_role(organization.id, role_id.parse().unwrap()))
            .await
            .unwrap()
            .unwrap(),
    };
    db.scope(repository.add_member(assigned_member))
        .await
        .unwrap();
    let assigned = graphql_operation(
        &router,
        delete_role,
        serde_json::json!({"id":role_id}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    assert_graphql_denial(assigned, 409).await;
    type MemberRoles = <compallm_postgres::OrganizationMemberSchema as sea_orm::ModelTrait>::Entity;
    use sea_orm::{ColumnTrait as _, QueryFilter as _};
    let fallback_role = Role::new(organization.id, "Remaining membership role", "", "")
        .await
        .unwrap();
    db.scope(repository.create_roles(vec![fallback_role.clone()]))
        .await
        .unwrap();
    let fallback_role_record = fallback_role.clone();
    let fallback_role = fallback_role.id;
    type MemberRoleColumn = <MemberRoles as EntityTrait>::Column;
    MemberRoles::update_many()
        .col_expr(
            MemberRoleColumn::RoleId,
            sea_orm::sea_query::Expr::value(i64::try_from(fallback_role).unwrap()),
        )
        .filter(MemberRoleColumn::OrganizationId.eq(i64::try_from(organization.id).unwrap()))
        .filter(MemberRoleColumn::UserId.eq(i64::try_from(assignee.id).unwrap()))
        .exec(pool.as_ref())
        .await
        .unwrap();
    let deleted = graphql_operation(
        &router,
        delete_role,
        serde_json::json!({"id":role_id}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    let body = graphql_body(deleted).await;
    assert!(body["errors"].is_null(), "{body}");
    assert_eq!(body["data"]["roleDelete"]["node"], true);
    assert!(
        db.scope(repository.find_role(organization.id, role_id.parse().unwrap()))
            .await
            .unwrap()
            .is_none()
    );

    let mut owner = Role::new(organization.id, "Owner", "", "").await.unwrap();
    owner.org_owner = true;
    db.scope(repository.create_roles(vec![owner.clone()]))
        .await
        .unwrap();
    let owner_detail = graphql_operation(
        &router,
        "query($id:ID!){role(id:$id,includeUnseen:true){node{id owner}}}",
        serde_json::json!({"id": owner.id.to_string()}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    let body = graphql_body(owner_detail).await;
    assert!(body["errors"].is_null(), "{body}");
    assert_eq!(body["data"]["role"]["node"]["owner"], true);
    let denied = graphql_operation(
        &router,
        delete_role,
        serde_json::json!({"id": owner.id.to_string()}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    assert_graphql_denial(denied, 403).await;

    fixture.repo.0.lock().unwrap().sessions.insert(
        12345,
        stored_session(assignee.id, 12345, organization.id, &organization.name),
    );
    let update_group = "mutation($id:ID!,$input:GroupUpdateInput!){groupUpdate(id:$id,input:$input){node{id name memberIds roles{edges{node{id}}}}}}";
    let owner_group = graphql_operation(&router, "mutation($input:GroupCreateInput!){groupCreate(input:$input){node{id}}}",
        serde_json::json!({"input": {"name": "Owner group prohibited", "description": "", "roleIds": [owner.id.to_string()]}}),
        Some(&tokens.access_token), None, false).await;
    assert_graphql_denial(owner_group, 400).await;
    let rejected = graphql_operation(
        &router,
        update_group,
        serde_json::json!({"id": group_id, "input": {"addRoleIds": [owner.id.to_string()]}}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    assert_graphql_denial(rejected, 400).await;
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&12345].authorization_version,
        1
    );
    let result=graphql_body(graphql_operation(&router,update_group,serde_json::json!({"id":group_id,"input":{"addMemberIds":[assignee.id.to_string()],"addRoleIds":[fallback_role_record.id.to_string()],"name":"Updated group"}}),Some(&tokens.access_token),None,false).await).await;
    assert!(result["errors"].is_null(), "{result}");
    assert_eq!(
        result["data"]["groupUpdate"]["node"]["memberIds"],
        serde_json::json!([assignee.id.to_string()])
    );
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&12345].authorization_version,
        2
    );
    let repeated = graphql_body(
        graphql_operation(
            &router,
            update_group,
            serde_json::json!({"id":group_id,"input":{"addMemberIds":[assignee.id.to_string()],"addRoleIds":[fallback_role_record.id.to_string()]}}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(repeated["errors"].is_null(), "{repeated}");
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&12345].authorization_version,
        2
    );
    let other_member = User::from_email(&format!("group-other-{}@example.com", organization.id))
        .await
        .unwrap();
    db.scope(compallm_postgres::repository::user::UserRepository.create(other_member.clone()))
        .await
        .unwrap();
    db.scope(repository.add_member(OrganizationMember {
        organization_id: organization.id,
        user_id: other_member.id,
        groups: vec![],
        role: fallback_role_record.clone(),
    }))
    .await
    .unwrap();
    let foreign_member = User::from_email(&format!("group-foreign-{}@example.com", foreign.id))
        .await
        .unwrap();
    db.scope(compallm_postgres::repository::user::UserRepository.create(foreign_member.clone()))
        .await
        .unwrap();
    db.scope(repository.add_member(OrganizationMember {
        organization_id: foreign.id,
        user_id: foreign_member.id,
        groups: vec![],
        role: foreign_role.clone(),
    }))
    .await
    .unwrap();
    let read = "query($id:ID!){group(id:$id){node{memberIds}} members(first:1){edges{node{id email effectiveRoleIds groupIds joinedAt role{id name} groups{id name}}}pageInfo{hasNextPage endCursor}}}";
    let result = graphql_body(
        graphql_operation(
            &router,
            read,
            serde_json::json!({"id":group_id}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(result["errors"].is_null(), "{result}");
    assert!(
        result["data"]["group"]["node"]["memberIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!(assignee.id.to_string()))
    );
    assert_eq!(
        result["data"]["members"]["edges"][0]["node"]["email"],
        assignee.email
    );
    assert_eq!(
        result["data"]["members"]["edges"][0]["node"]["effectiveRoleIds"],
        serde_json::json!([fallback_role_record.id.to_string()])
    );
    assert_eq!(
        result["data"]["members"]["edges"][0]["node"]["groupIds"],
        serde_json::json!([group_id])
    );
    assert_eq!(result["data"]["members"]["pageInfo"]["hasNextPage"], true);
    let cursor = result["data"]["members"]["pageInfo"]["endCursor"]
        .as_str()
        .unwrap();
    let page = graphql_body(graphql_operation(&router, "query($after:String!){members(first:1,after:$after){edges{node{id}}pageInfo{hasNextPage}}}", serde_json::json!({"after":cursor}), Some(&tokens.access_token), None, false).await).await;
    assert!(page["errors"].is_null(), "{page}");
    assert_eq!(
        page["data"]["members"]["edges"][0]["node"]["id"],
        other_member.id.to_string()
    );
    assert_eq!(page["data"]["members"]["pageInfo"]["hasNextPage"], false);
    let no_authority = token_pair::new(
        &session,
        empty_snapshot(),
        SECRET,
        Duration::from_secs(60),
        REFRESH_LIFETIME,
    )
    .unwrap();
    let denied_read = graphql_operation(
        &router,
        "{members(first:1){edges{node{id email}}}}",
        serde_json::json!({}),
        Some(&no_authority.access_token),
        None,
        false,
    )
    .await;
    assert_graphql_denial(denied_read, 403).await;
    let foreign_change = graphql_operation(
        &router,
        update_group,
        serde_json::json!({"id":group_id,"input":{"addMemberIds":[foreign_member.id.to_string()]}}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    assert_graphql_denial(foreign_change, 404).await;
    fixture.repo.0.lock().unwrap().fail_rotation = true;
    let failure=graphql_operation(&router,update_group,serde_json::json!({"id":group_id,"input":{"addMemberIds":[other_member.id.to_string()],"name":"Should roll back"}}),Some(&tokens.access_token),None,false).await;
    assert_graphql_denial(failure, 503).await;
    fixture.repo.0.lock().unwrap().fail_rotation = false;
    assert_eq!(
        db.scope(repository.group_member_count(organization.id, group_id.parse().unwrap()))
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        scoped
            .scope(repository.find_group_info(group_id.parse().unwrap()))
            .await
            .unwrap()
            .unwrap()
            .name,
        "Updated group"
    );
    let result = graphql_operation(
        &router,
        "mutation($id:ID!){groupDelete(id:$id){node}}",
        serde_json::json!({"id":group_id}),
        Some(&tokens.access_token),
        None,
        false,
    )
    .await;
    assert_graphql_denial(result, 409).await;
    let result = graphql_body(
        graphql_operation(
            &router,
            update_group,
            serde_json::json!({"id":group_id,"input":{"removeRoleIds":[fallback_role_record.id.to_string()]}}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(result["errors"].is_null(), "{result}");
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&12345].authorization_version,
        3
    );
    let restored = graphql_body(
        graphql_operation(
            &router,
            update_group,
            serde_json::json!({"id":group_id,"input":{"addRoleIds":[fallback_role_record.id.to_string()]}}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(restored["errors"].is_null(), "{restored}");
    type Memberships = <compallm_postgres::OrganizationMemberSchema as sea_orm::ModelTrait>::Entity;
    type MembershipColumn = <Memberships as EntityTrait>::Column;
    Memberships::update_many()
        .col_expr(
            MembershipColumn::Status,
            sea_orm::sea_query::Expr::value(compallm_postgres::UserStatusSchema::Disabled),
        )
        .filter(MembershipColumn::OrganizationId.eq(i64::try_from(organization.id).unwrap()))
        .filter(MembershipColumn::UserId.eq(i64::try_from(assignee.id).unwrap()))
        .exec(pool.as_ref())
        .await
        .unwrap();
    let result=graphql_body(graphql_operation(&router,update_group,serde_json::json!({"id":group_id,"input":{"removeMemberIds":[assignee.id.to_string()]}}),Some(&tokens.access_token),None,false).await).await;
    assert!(result["errors"].is_null(), "{result}");
    assert_eq!(
        fixture.repo.0.lock().unwrap().sessions[&12345].authorization_version,
        5
    );
    let competing_database = Arc::new(
        sea_orm::Database::connect(std::env::var("COMPALLM_TEST_DATABASE_URL").unwrap())
            .await
            .unwrap(),
    );
    let competing_context = DbContext::new(Arc::new(PostgresConnection::new(competing_database)));
    db.transaction(async {
        repository
            .lock_group(organization.id, group_id.parse().unwrap())
            .await?;
        let group = repository
            .find_group(organization.id, group_id.parse().unwrap())
            .await?
            .unwrap();
        let competing_group_id = group.id;
        let competing_member_id = other_member.id;
        let mut competing = tokio::spawn(async move {
            let repository = compallm_postgres::repository::organization::OrganizationRepository;
            competing_context.transaction(async {
                repository.lock_group(organization.id, competing_group_id).await?;
                repository.save_group_changes(
                    &group,
                    &compallm_core::organization::group::GroupChanges {
                        add_member_ids: vec![competing_member_id],
                        ..Default::default()
                    },
                ).await?;
                Ok::<_, compallm_core::persistent::organization::OrganizationRepositoryError>(())
            }).await
        });
        assert!(
            tokio::time::timeout(Duration::from_millis(50), &mut competing)
                .await
                .is_err()
        );
        competing.abort();
        let _ = competing.await;
        assert_eq!(
            repository
                .group_member_count(organization.id, competing_group_id)
                .await?,
            0
        );
        Ok::<_, compallm_core::persistent::organization::OrganizationRepositoryError>(())
    })
    .await
    .unwrap();
    let result = graphql_body(
        graphql_operation(
            &router,
            "mutation($id:ID!){groupDelete(id:$id){node}}",
            serde_json::json!({"id":group_id}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(result["errors"].is_null(), "{result}");
    assert!(
        db.scope(repository.find_group(organization.id, group_id.parse().unwrap()))
            .await
            .unwrap()
            .is_none()
    );

    type Organizations = <OrganizationSchema as sea_orm::ModelTrait>::Entity;
    for org in [organization, foreign] {
        Organizations::delete_by_id(i64::try_from(org.id).unwrap())
            .exec(pool.as_ref())
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn graphql_full_schema_introspection_is_available_without_credentials() {
    let router = Fixture::new(None).graphql();
    let response = graphql_operation(
        &router,
        include_str!("fixtures/graphql-introspection.graphql"),
        serde_json::json!({}),
        None,
        None,
        false,
    )
    .await;
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = graphql_body(response).await;
    assert!(body["errors"].is_null(), "{body}");
    assert!(body["data"]["__schema"]["types"].as_array().unwrap().len() > 10);
}

#[tokio::test]
async fn preference_creation_failure_rolls_back_signup() {
    let fixture = Fixture::new(None);
    fixture.repo.0.lock().unwrap().fail_preferences = true;
    let challenge = test_challenge("preferences@example.com", SECRET).unwrap();
    assert!(matches!(
        fixture
            .signup_in_transaction(&Fixture::otp(&challenge), &challenge.code)
            .await,
        Err(AuthError::Preferences(_))
    ));
    assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
    fixture.assert_transaction("ROLLBACK");
}

#[tokio::test]
async fn preference_graphql_requires_authentication_and_rejects_null_before_writes() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let router = fixture.graphql();
    let denied = graphql_body(graphql_operation(&router, "mutation { userPreferencesUpdate(input: {accessibility: {pointerCursors: true}}) { node { accessibility { pointerCursors } } } }", serde_json::json!({}), None, None, false).await).await;
    assert_eq!(denied["errors"][0]["extensions"]["code"], "UNAUTHENTICATED");
    for input in [
        serde_json::json!({"theme":null}),
        serde_json::json!({"theme":{"palettes":{"dark":null}}}),
        serde_json::json!({"sidebar":{"order":["CHAT"]}}),
    ] {
        let invalid = graphql_body(graphql_operation(&router, "mutation($input: UserPreferencesPatchInput!) { userPreferencesUpdate(input: $input) { node { navigation { home } } } }", serde_json::json!({"input":input}), Some(&tokens.access_token), None, false).await).await;
        assert_eq!(invalid["errors"][0]["extensions"]["code"], "BAD_USER_INPUT");
    }
}

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL with all migrations applied; creates a disposable database"]
async fn preference_graphql_persists_grouped_updates_and_resets_for_current_user()
-> anyhow::Result<()> {
    use sea_orm::{ConnectionTrait, Database, EntityTrait, ModelTrait, Set, TransactionTrait};
    use sea_orm_migration::{MigratorTrait, SchemaManager};
    let url = std::env::var("COMPALLM_TEST_DATABASE_URL")?;
    let name = format!(
        "preference_http_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos()
    );
    let admin = Database::connect(&url).await?;
    admin
        .execute_unprepared(&format!("CREATE DATABASE {name}"))
        .await?;
    let result = async {
        let (base, parameters) = url.split_once('?').unwrap_or((&url, ""));
        let (server, _) = base.rsplit_once('/').ok_or_else(|| anyhow::anyhow!("database URL"))?;
        let pool = Arc::new(Database::connect(format!("{server}/{name}?{parameters}")).await?);
        for migration in compallm_postgres::Migrator::migrations() {
            if migration.name().ends_with("restricted_reads") {
                // Reuse the cluster-wide role installed in the parent test database.
                let setup = include_str!("../../libs/postgres/src/migration/m20261110_000028_restricted_reads/up.sql");
                let (_, setup) = setup.split_once('\n').unwrap();
                pool.execute_unprepared(setup).await?;
                pool.execute_unprepared(include_str!("../../libs/postgres/src/migration/m20261110_000028_restricted_reads/policies.sql")).await?;
                pool.execute_unprepared(include_str!("../../libs/postgres/src/migration/m20261110_000028_restricted_reads/guards.sql")).await?;
            } else {
                let transaction = pool.begin().await?;
                migration.up(&SchemaManager::new(&transaction)).await?;
                transaction.commit().await?;
            }
        }
        type Users = <compallm_postgres::UserSchema as ModelTrait>::Entity;
        type ActiveUser = <Users as EntityTrait>::ActiveModel;
        Users::insert(ActiveUser { id: Set(1), email: Set("james@midwess.com".into()), display_name: Set("James".into()), username: Set("james".into()), status: Set(compallm_postgres::schema::UserStatusSchema::Active), email_verified_at: Set(sea_orm::prelude::ChronoUtc::now().fixed_offset()), ..Default::default() }).exec(pool.as_ref()).await?;
        let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
        let challenge = test_challenge("james@midwess.com", SECRET)?;
        let tokens = fixture.signin(&Fixture::otp(&challenge), &challenge.code).await?;
        let context = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
        let router = fixture.graphql_with_database(context.clone());
        let initial = graphql_body(graphql_operation(&router, "{ userPreferences { node { navigation { home } codeAppearance { theme } } } }", serde_json::json!({}), Some(&tokens.access_token), None, false).await).await;
        assert!(initial["errors"].is_null(), "{initial}");
        assert_eq!(initial["data"]["userPreferences"]["node"]["navigation"]["home"], "CHAT");
        assert_eq!(initial["data"]["userPreferences"]["node"]["codeAppearance"]["theme"], "FOLLOW_INTERFACE");
        let update = "mutation($input: UserPreferencesPatchInput!) { userPreferencesUpdate(input: $input) { node { navigation { home } theme { palettes { dark } } codeAppearance { font { family size } } sidebar { entries { item visibility } } } } }";
        let response = graphql_body(graphql_operation(&router, update, serde_json::json!({"input": {
            "navigation":{"home":"MODELS"}, "calendar":{"firstDay":"SUNDAY"},
            "theme":{"palettes":{"dark":"BLUEBERRY_DARK"}},
            "codeAppearance":{"font":{"family":"MONOSPACE","size":"PX16"}},
            "sidebar":{"order":["PLUGINS","PROVIDERS"],"visibility":[{"item":"PLUGINS","visibility":"HIDDEN"}]}
        }}), Some(&tokens.access_token), None, false).await).await;
        assert!(response.get("errors").is_none(), "{response}");
        assert_eq!(response["data"]["userPreferencesUpdate"]["node"]["navigation"]["home"], "MODELS");
        let read = "query { userPreferences { node { calendar { firstDay } theme { mode palettes { light dark } } codeAppearance { font { family size } } sidebar { entries { item visibility } } } } }";
        let saved = graphql_body(graphql_operation(&router, read, serde_json::json!({}), Some(&tokens.access_token), None, false).await).await;
        assert_eq!(saved["data"]["userPreferences"]["node"]["theme"]["palettes"]["dark"], "BLUEBERRY_DARK");
        assert_eq!(saved["data"]["userPreferences"]["node"]["theme"]["palettes"]["light"], "BASIC_WHITE");
        assert_eq!(saved["data"]["userPreferences"]["node"]["sidebar"]["entries"][0]["visibility"], "HIDDEN");
        for groups in [serde_json::json!([]), serde_json::json!(["ALL","THEME"])] {
            let invalid = graphql_body(graphql_operation(&router, "mutation($groups: [UserPreferenceGroup!]!) { userPreferencesReset(groups: $groups) { node { calendar { firstDay } } } }", serde_json::json!({"groups":groups}), Some(&tokens.access_token), None, false).await).await;
            assert_eq!(invalid["errors"][0]["extensions"]["code"], "BAD_USER_INPUT");
        }
        let reset = graphql_body(graphql_operation(&router, "mutation { userPreferencesReset(groups: [THEME]) { node { theme { palettes { dark } } calendar { firstDay } } } }", serde_json::json!({}), Some(&tokens.access_token), None, false).await).await;
        assert_eq!(reset["data"]["userPreferencesReset"]["node"]["theme"]["palettes"]["dark"], "BASIC_DARK");
        assert_eq!(reset["data"]["userPreferencesReset"]["node"]["calendar"]["firstDay"], "SUNDAY");
        let defaults = graphql_body(graphql_operation(&router, "mutation { userPreferencesReset(groups: [ALL]) { node { calendar { firstDay } } } }", serde_json::json!({}), Some(&tokens.access_token), None, false).await).await;
        assert_eq!(defaults["data"]["userPreferencesReset"]["node"]["calendar"]["firstDay"], "MONDAY");
        let preferences = compallm_core::persistent::preference::user::UserPreferenceRepository::find(&compallm_postgres::repository::preference::user::UserPreferenceRepository,1);
        assert_eq!(context.scope(preferences).await?.unwrap(), compallm_core::preference::UserPreferences::default());
        Ok::<_,anyhow::Error>(())
    }.await;
    let cleanup = admin
        .execute_unprepared(&format!("DROP DATABASE {name} WITH (FORCE)"))
        .await;
    result?;
    cleanup?;
    Ok(())
}

#[tokio::test]
async fn graphql_me_returns_only_the_verified_users_effective_cedar() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let session = fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .values()
        .next()
        .unwrap()
        .clone();
    let source = r#"permit(principal, action == Compallm::Action::"provider.create", resource == Compallm::Organization::"84"); forbid(principal, action == Compallm::Action::"model.create", resource == Compallm::Organization::"84");"#;
    let authorized = token_pair::new(
        &session,
        PolicySnapshot {
            cedar_source: source.into(),
            group_roles: Default::default(),
            role_id: 3,
        },
        SECRET,
        Duration::from_secs(60),
        REFRESH_LIFETIME,
    )
    .unwrap();
    let router = fixture.graphql();
    let query = "{me{node{id sessionId organizationId cedarSource cedarEntities}}}";
    let before = fixture.repo.0.lock().unwrap().freshness_reads;
    let body = graphql_body(
        graphql_operation(
            &router,
            query,
            serde_json::json!({}),
            Some(&authorized.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(body["errors"].is_null(), "{body}");
    assert_eq!(
        body["data"]["me"]["node"]["id"],
        session.user_id.to_string()
    );
    assert_eq!(body["data"]["me"]["node"]["organizationId"], "84");
    let parsed: cedar_policy::PolicySet = body["data"]["me"]["node"]["cedarSource"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(parsed.policies().count(), 2);
    let entities: serde_json::Value = serde_json::from_str(
        body["data"]["me"]["node"]["cedarEntities"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    let user_id = session.user_id.to_string();
    assert!(
        entities
            .as_array()
            .unwrap()
            .iter()
            .any(|entity| entity["uid"]["type"] == "Compallm::User"
                && entity["uid"]["id"].as_str() == Some(user_id.as_str()))
    );
    assert_eq!(fixture.repo.0.lock().unwrap().freshness_reads, before + 1);
    let anonymous = graphql_body(
        graphql_operation(&router, query, serde_json::json!({}), None, None, false).await,
    )
    .await;
    assert_eq!(
        anonymous["errors"][0]["extensions"]["code"],
        "UNAUTHENTICATED"
    );
}

#[tokio::test]
async fn ordinary_signin_ignores_stale_invitation_cookie_and_clears_it() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let router = fixture.graphql();
    let response = graphql_operation(&router,
        "mutation($input:EmailInput!){emailSigninRequest(input:$input){node{lifetime{expiresInSeconds}}}}",
        serde_json::json!({"input":{"email":"james@midwess.com"}}), None,
        Some("compallm-invitation=stale"), true).await;
    let cookies = response_cookies(&response);
    let body = graphql_body(response).await;
    assert!(body.get("errors").is_none(), "{body}");
    assert!(cookies.contains("compallm-invitation="));
    let challenge = cookies
        .split(';')
        .find_map(|value| value.trim().strip_prefix("compallm-access="))
        .unwrap();
    let claims = EmailOtpAccessToken::try_from(decode_token(challenge, SECRET).unwrap()).unwrap();
    assert!(claims.invitation_id.is_none());
    let response = graphql_operation(&router,
        "mutation($input:EmailCodeInput!){emailSigninConfirm(input:$input){node{accessLifetime{expiresInSeconds}}}}",
        serde_json::json!({"input":{"code":claims.code}}), None,
        Some(&format!("compallm-access={challenge}; compallm-invitation=stale")), true).await;
    let body = graphql_body(response).await;
    assert!(body.get("errors").is_none(), "{body}");
    assert_eq!(fixture.repo.0.lock().unwrap().logins.len(), 1);
}

#[tokio::test]
async fn explicit_invitation_request_without_cookie_never_falls_back_to_signin() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let router = fixture.graphql();
    let response = graphql_operation(&router,
        "mutation($input:EmailInput!){emailSigninRequest(input:$input){node{lifetime{expiresInSeconds}}}}",
        serde_json::json!({"input":{"email":"james@midwess.com","invitation":true}}), None, None, true).await;
    assert!(!response.headers().contains_key("set-cookie"));
    let body = graphql_body(response).await;
    assert!(body.get("errors").is_some());
    assert!(fixture.repo.0.lock().unwrap().logins.is_empty());
}

fn management_grant(actor: &UserAccessToken) -> String {
    use compallm_core::authentication::{AccessToken, PasskeyChangeToken};
    AccessToken::PasskeyChange(PasskeyChangeToken::new(&actor.session).unwrap())
        .encode(SECRET, std::time::Duration::from_secs(180))
        .unwrap()
}

#[path = "support/passkeys.rs"]
mod passkey_support;

#[tokio::test]
async fn passkey_enrollment_signin_reauthentication_and_removal_use_verified_session_evidence() {
    use compallm_core::{
        authentication::passkey::PasskeyService, persistent::passkey::PasskeyRepository,
    };
    use webauthn_authenticator_rs::WebauthnAuthenticator;
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let repository = Box::leak(Box::new(passkey_support::MemoryPasskeys::default()));
    let auth = Box::leak(Box::new(
        fixture.service().with_passkey_repository(repository),
    ));
    let service = PasskeyService::new(
        repository,
        auth,
        SECRET.into(),
        "localhost",
        "http://localhost",
    )
    .unwrap();
    let otp = EmailOtpAccessToken::new(Fixture::user(UserStatus::Active).email).unwrap();
    let tokens = auth
        .signin(&otp, &otp.code, Default::default())
        .await
        .unwrap();
    let actor = auth
        .authenticate_access(&tokens.access_token)
        .await
        .unwrap();
    let grant = management_grant(&actor);
    let origin = url::Url::parse("http://localhost").unwrap();
    let mut authenticator =
        WebauthnAuthenticator::new(passkey_support::ResidentAuthenticator::new());
    let start = service.registration_start(&actor, &grant).await.unwrap();
    assert_eq!(
        start.options["publicKey"]["authenticatorSelection"]["residentKey"],
        "required"
    );
    assert_eq!(start.options["publicKey"]["user"]["name"], otp.email);
    assert!(
        repository
            .list(actor.session.user_id)
            .await
            .unwrap()
            .is_empty()
    );
    let response = authenticator
        .do_registration(
            origin.clone(),
            serde_json::from_value(start.options).unwrap(),
        )
        .unwrap();
    let record = service
        .registration_finish(&actor, &grant, &start.token, response.clone(), "Laptop")
        .await
        .unwrap();
    assert!(
        service
            .registration_finish(&actor, &grant, &start.token, response, "Laptop")
            .await
            .is_err()
    );
    assert_eq!(
        repository.list(actor.session.user_id).await.unwrap().len(),
        1
    );

    let renamed = service
        .rename(&actor, &grant, record.id, "Work Macbook")
        .await
        .unwrap();
    assert_eq!(renamed.name, "Work Macbook");
    assert_eq!(
        repository
            .session_passkey(&actor.session)
            .await
            .unwrap()
            .unwrap()
            .id,
        record.id
    );
    let start = service.authentication_start().await.unwrap();
    assert!(
        start.options["publicKey"]["allowCredentials"]
            .as_array()
            .is_some_and(|a| a.is_empty())
    );
    let assertion = authenticator
        .do_authentication(
            origin.clone(),
            serde_json::from_value(start.options).unwrap(),
        )
        .unwrap();
    let mut missing_handle = assertion.clone();
    missing_handle.response.user_handle = None;
    assert!(
        service
            .authentication_finish(&start.token, missing_handle, Default::default())
            .await
            .is_err()
    );
    let mut wrong_handle = assertion.clone();
    wrong_handle.response.user_handle = Some(uuid::Uuid::new_v4().as_bytes().to_vec().into());
    assert!(
        service
            .authentication_finish(&start.token, wrong_handle, Default::default())
            .await
            .is_err()
    );
    let mut unknown_credential = assertion.clone();
    unknown_credential.raw_id = vec![1, 2, 3, 4].into();
    assert!(
        service
            .authentication_finish(&start.token, unknown_credential, Default::default())
            .await
            .is_err()
    );
    let login = service
        .authentication_finish(&start.token, assertion.clone(), Default::default())
        .await
        .unwrap();
    assert!(
        service
            .authentication_finish(&start.token, assertion, Default::default())
            .await
            .is_err()
    );
    let login_actor = auth.authenticate_access(&login.access_token).await.unwrap();
    assert_ne!(actor.session.id, login_actor.session.id);
    assert_eq!(actor.session.user_id, login_actor.session.user_id);
    assert!(
        repository
            .state
            .lock()
            .unwrap()
            .evidence
            .contains_key(&(login_actor.session.id, "passkey".into()))
    );
    let reauth = service.reauthentication_start(&actor).await.unwrap();
    let assertion = authenticator
        .do_authentication(origin, serde_json::from_value(reauth.options).unwrap())
        .unwrap();
    assert!(
        service
            .authentication_finish(&reauth.token, assertion.clone(), Default::default())
            .await
            .is_err()
    );
    let renewed_grant = service
        .reauthentication_finish(&actor, &reauth.token, assertion)
        .await
        .unwrap();
    assert!(
        auth.verify_passkey_change(&renewed_grant.token, &actor.session)
            .is_ok()
    );
    assert!(
        auth.verify_passkey_change(&renewed_grant.token, &login_actor.session)
            .is_err()
    );
    let pending = service.authentication_start().await.unwrap();
    let pending_assertion = authenticator
        .do_authentication(
            url::Url::parse("http://localhost").unwrap(),
            serde_json::from_value(pending.options).unwrap(),
        )
        .unwrap();
    service.remove(&actor, &grant, record.id).await.unwrap();
    assert!(
        repository
            .list(actor.session.user_id)
            .await
            .unwrap()
            .is_empty()
    );

    assert!(auth.validate_access(&actor).await.unwrap());
    assert!(auth.validate_access(&login_actor).await.unwrap());
    assert!(
        repository
            .state
            .lock()
            .unwrap()
            .challenges
            .values()
            .any(|c| c.purpose == "authentication")
    );
    assert!(
        service
            .authentication_finish(&pending.token, pending_assertion, Default::default())
            .await
            .is_err()
    );
}

#[tokio::test]
async fn passkey_management_requires_grant_and_rejects_foreign_or_expired_ceremonies() {
    use compallm_core::authentication::passkey::PasskeyService;
    use webauthn_authenticator_rs::WebauthnAuthenticator;
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let repository = Box::leak(Box::new(passkey_support::MemoryPasskeys::default()));
    let auth = Box::leak(Box::new(
        fixture.service().with_passkey_repository(repository),
    ));
    let service = PasskeyService::new(
        repository,
        auth,
        SECRET.into(),
        "localhost",
        "http://localhost",
    )
    .unwrap();
    let otp = EmailOtpAccessToken::new(Fixture::user(UserStatus::Active).email).unwrap();
    let tokens = auth
        .signin(&otp, &otp.code, Default::default())
        .await
        .unwrap();
    let actor = auth
        .authenticate_access(&tokens.access_token)
        .await
        .unwrap();
    let grant = String::new();
    repository.state.lock().unwrap().evidence.clear();
    assert!(matches!(
        service.registration_start(&actor, &grant).await,
        Err(AuthError::ReauthenticationRequired)
    ));
    assert!(repository.state.lock().unwrap().challenges.is_empty());
    assert!(matches!(
        service.remove(&actor, &grant, 1).await,
        Err(AuthError::ReauthenticationRequired)
    ));
    let _ = auth.refresh(&tokens.refresh_token).await.unwrap();
    assert!(repository.state.lock().unwrap().evidence.is_empty());
    repository.state.lock().unwrap().evidence.insert(
        (actor.session.id, "email_code".into()),
        compallm_utils::time::unix_timestamp(),
    );
    let grant = management_grant(&actor);
    let start = service.registration_start(&actor, &grant).await.unwrap();
    let mut authenticator =
        WebauthnAuthenticator::new(passkey_support::ResidentAuthenticator::new());
    let response = authenticator
        .do_registration(
            url::Url::parse("http://localhost").unwrap(),
            serde_json::from_value(start.options).unwrap(),
        )
        .unwrap();
    let mut foreign = actor.clone();
    foreign.session.user_id += 1;
    assert!(
        service
            .registration_finish(&foreign, &grant, &start.token, response.clone(), "Laptop")
            .await
            .is_err()
    );
    for ceremony in repository.state.lock().unwrap().challenges.values_mut() {
        ceremony.expires_at = 0;
    }
    assert!(
        service
            .registration_finish(&actor, &grant, &start.token, response, "Laptop")
            .await
            .is_err()
    );
    assert!(repository.state.lock().unwrap().records.is_empty());
}

#[tokio::test]
async fn email_passkey_change_is_purpose_bound_session_bound_and_preserves_login() {
    struct Email(Arc<Mutex<String>>);
    #[async_trait::async_trait]
    impl compallm_core::mail::EmailService for Email {
        async fn send(
            &self,
            request: compallm_core::mail::EmailRequest,
        ) -> Result<(), compallm_core::mail::EmailError> {
            if let compallm_core::mail::EmailTemplate::OtpCode(code) = request.text {
                *self.0.lock().unwrap() = code.code;
            }
            Ok(())
        }
    }
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let repository = Box::leak(Box::new(passkey_support::MemoryPasskeys::default()));
    let email = Arc::new(Mutex::new(String::new()));
    let sender = Box::leak(Box::new(
        Box::new(Email(email.clone())) as Box<dyn compallm_core::mail::EmailService>
    ));
    let auth = Box::leak(Box::new(
        fixture
            .service_with_dependencies(
                Box::new(TestDictionary {
                    dictionary: compallm_core::compression::DCompression::new(
                        compallm_core::compression::DictionaryType::AccessToken,
                        1,
                        "passkey session user policy",
                    )
                    .unwrap(),
                    newer: None,
                }),
                Some(sender),
            )
            .with_passkey_repository(repository),
    ));
    let otp = EmailOtpAccessToken::new(Fixture::user(UserStatus::Active).email).unwrap();
    let tokens = auth
        .signin(&otp, &otp.code, Default::default())
        .await
        .unwrap();
    let actor = auth
        .authenticate_access(&tokens.access_token)
        .await
        .unwrap();
    let challenge = auth.passkey_change_email(&actor, &otp.email).await.unwrap();
    let proof: EmailOtpAccessToken = auth
        .authenticate_token(&challenge.token)
        .await
        .unwrap()
        .try_into()
        .unwrap();
    let code = email.lock().unwrap().clone();
    assert!(
        auth.signin(&proof, &code, Default::default())
            .await
            .is_err()
    );
    assert!(
        auth.passkey_change_confirm(&actor, &otp, &otp.code)
            .await
            .is_err()
    );
    let mut other = actor.clone();
    other.session.id += 1;
    assert!(
        auth.passkey_change_confirm(&other, &proof, &code)
            .await
            .is_err()
    );
    let wrong = if code == "000000" { "111111" } else { "000000" };
    assert!(
        auth.passkey_change_confirm(&actor, &proof, wrong)
            .await
            .is_err()
    );
    let grant = auth
        .passkey_change_confirm(&actor, &proof, &code)
        .await
        .unwrap();
    assert_eq!(
        grant.claims.validity.expires_at - grant.claims.validity.issued_at,
        180
    );
    assert!(
        auth.verify_passkey_change(&grant.token, &actor.session)
            .is_ok()
    );
    assert!(
        auth.verify_passkey_change(&grant.token, &actor.session)
            .is_ok()
    );
    assert!(
        auth.verify_passkey_change(&grant.token, &other.session)
            .is_err()
    );
    assert!(auth.authenticate_access(&grant.token).await.is_err());
    assert!(
        auth.refresh(&tokens.refresh_token)
            .await
            .unwrap()
            .passkey_change
            .is_none()
    );
    assert!(auth.validate_access(&actor).await.unwrap());
}

#[tokio::test]
async fn discoverable_passkey_ceremony_accepts_either_account_without_an_email_binding() {
    use compallm_core::authentication::passkey::PasskeyService;
    use webauthn_authenticator_rs::WebauthnAuthenticator;
    let first = Fixture::user(UserStatus::Active);
    let fixture = Fixture::new(Some(first.clone()));
    let repository = Box::leak(Box::new(passkey_support::MemoryPasskeys::default()));
    let auth = Box::leak(Box::new(
        fixture.service().with_passkey_repository(repository),
    ));
    let service = PasskeyService::new(
        repository,
        auth,
        SECRET.into(),
        "localhost",
        "http://localhost",
    )
    .unwrap();
    let origin = url::Url::parse("http://localhost").unwrap();
    let mut authenticators = [
        WebauthnAuthenticator::new(passkey_support::ResidentAuthenticator::new()),
        WebauthnAuthenticator::new(passkey_support::ResidentAuthenticator::new()),
    ];
    let mut second = first.clone();
    second.id += 1;
    second.email = "second@example.com".into();
    for (index, account) in [first.clone(), second.clone()].into_iter().enumerate() {
        {
            let mut state = fixture.repo.0.lock().unwrap();
            state.user = Some(account.clone());
            for membership in &mut state.memberships {
                membership.user_id = account.id;
            }
        }
        let otp = EmailOtpAccessToken::new(account.email.clone()).unwrap();
        let login = auth
            .signin(&otp, &otp.code, Default::default())
            .await
            .unwrap();
        let actor = auth.authenticate_access(&login.access_token).await.unwrap();
        let grant = management_grant(&actor);
        let enrollment = service.registration_start(&actor, &grant).await.unwrap();
        let response = authenticators[index]
            .do_registration(
                origin.clone(),
                serde_json::from_value(enrollment.options).unwrap(),
            )
            .unwrap();
        service
            .registration_finish(&actor, &grant, &enrollment.token, response, "Macbook")
            .await
            .unwrap();
    }
    for (index, account) in [first, second].into_iter().enumerate() {
        let start = service.authentication_start().await.unwrap();
        {
            let mut state = fixture.repo.0.lock().unwrap();
            state.user = Some(account.clone());
            for membership in &mut state.memberships {
                membership.user_id = account.id;
            }
        }
        let assertion = authenticators[index]
            .do_authentication(
                origin.clone(),
                serde_json::from_value(start.options).unwrap(),
            )
            .unwrap();
        let login = service
            .authentication_finish(&start.token, assertion, Default::default())
            .await
            .unwrap();
        let actor = auth.authenticate_access(&login.access_token).await.unwrap();
        assert_eq!(actor.session.user_id, account.id);
    }
}

#[tokio::test]
async fn graphql_passkey_change_cookie_preserves_login_and_cannot_authorize_session() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let auth = fixture.service();
    let otp = EmailOtpAccessToken::new(Fixture::user(UserStatus::Active).email).unwrap();
    let tokens = auth
        .signin(&otp, &otp.code, Default::default())
        .await
        .unwrap();
    let router = fixture.graphql();
    let cookies = format!(
        "compallm-access={}; compallm-refresh={}",
        tokens.access_token, tokens.refresh_token
    );
    let response = graphql_operation(&router,
        "mutation($input:EmailInput!){emailSigninRequest(input:$input){node{token lifetime{expiresInSeconds}}}}",
        serde_json::json!({"input":{"email":otp.email,"purpose":"PASSKEY_CHANGE"}}), None, Some(&cookies), true).await;
    let issued = response_cookies(&response);
    assert!(issued.starts_with("compallm-passkey-otp="));
    assert!(!issued.contains("compallm-access="));
    assert!(!issued.contains("compallm-refresh="));
    let proof_token = issued.strip_prefix("compallm-passkey-otp=").unwrap();
    let proof = EmailOtpAccessToken::try_from(decode_token(proof_token, SECRET).unwrap()).unwrap();
    assert_eq!(
        graphql_body(response).await["data"]["emailSigninRequest"]["node"]["token"],
        serde_json::Value::Null
    );
    let response = graphql_operation(&router,
        "mutation($input:EmailCodeInput!){emailSigninConfirm(input:$input){node{purpose accessToken refreshToken passkeyChangeToken passkeyChangeLifetime{expiresInSeconds}}}}",
        serde_json::json!({"input":{"code":proof.code,"purpose":"PASSKEY_CHANGE"}}), None, Some(&format!("{cookies}; {issued}")), true).await;
    let headers = response
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|v| v.to_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    let grant_cookie = headers
        .iter()
        .find(|v| v.starts_with("compallm-passkey-change="))
        .unwrap();
    assert!(grant_cookie.contains("HttpOnly") && grant_cookie.contains("Max-Age=180"));
    assert!(
        headers
            .iter()
            .all(|v| !v.starts_with("compallm-access=") && !v.starts_with("compallm-refresh="))
    );
    let grant = grant_cookie
        .split(';')
        .next()
        .unwrap()
        .strip_prefix("compallm-passkey-change=")
        .unwrap()
        .to_owned();
    let body = graphql_body(response).await;
    let result = &body["data"]["emailSigninConfirm"]["node"];
    assert_eq!(result["purpose"], "PASSKEY_CHANGE");
    assert!(
        result["accessToken"].is_null()
            && result["refreshToken"].is_null()
            && result["passkeyChangeToken"].is_null()
    );
    assert_eq!(fixture.repo.0.lock().unwrap().logins.len(), 1);
    assert_graphql_denial(
        graphql_request(&router, Some(&grant), None, false).await,
        401,
    )
    .await;
    assert_eq!(graphql_body(graphql_request(&router, None, Some(&cookies), true).await).await["data"]["first"]["node"]["id"].as_str().unwrap(), auth.authenticate_access(&tokens.access_token).await.unwrap().session.id.to_string());
}

#[tokio::test]
async fn account_sessions_list_both_logins_and_revoke_only_the_owned_target() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let first = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let second = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let actor =
        UserAccessToken::try_from(decode_token(&first.access_token, SECRET).unwrap()).unwrap();
    let other =
        UserAccessToken::try_from(decode_token(&second.access_token, SECRET).unwrap()).unwrap();
    {
        let mut state = fixture.repo.0.lock().unwrap();
        let mut foreign = actor.session.clone();
        foreign.id += 100;
        foreign.user_id += 1;
        state.sessions.insert(foreign.id, foreign);
        let mut revoked = actor.session.clone();
        revoked.id += 101;
        revoked.revoke();
        state.sessions.insert(revoked.id, revoked);
    }
    let router = fixture.graphql();
    let query = "{sessions(first:1){edges{node{id}}pageInfo{hasNextPage endCursor}}}";
    let page = graphql_body(
        graphql_operation(
            &router,
            query,
            serde_json::json!({}),
            Some(&first.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(page["errors"].is_null(), "{page}");
    assert_eq!(
        page["data"]["sessions"]["edges"].as_array().unwrap().len(),
        1
    );
    assert_eq!(page["data"]["sessions"]["pageInfo"]["hasNextPage"], true);
    let cursor = page["data"]["sessions"]["pageInfo"]["endCursor"]
        .as_str()
        .unwrap();
    let next = graphql_body(graphql_operation(&router,
        "query($after:String!){sessions(first:1,after:$after){edges{node{id}}pageInfo{hasNextPage}}}",
        serde_json::json!({"after":cursor}), Some(&first.access_token), None,false).await).await;
    assert!(next["errors"].is_null(), "{next}");
    let ids = [
        page["data"]["sessions"]["edges"][0]["node"]["id"]
            .as_str()
            .unwrap(),
        next["data"]["sessions"]["edges"][0]["node"]["id"]
            .as_str()
            .unwrap(),
    ];
    assert!(ids.contains(&actor.session.id.to_string().as_str()));
    assert!(ids.contains(&other.session.id.to_string().as_str()));
    let denied = graphql_body(
        graphql_operation(
            &router,
            "mutation($id:ID!){sessionRevokeOther(id:$id){node}}",
            serde_json::json!({"id":(actor.session.id+100).to_string()}),
            Some(&first.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert_eq!(denied["data"]["sessionRevokeOther"]["node"], false);
    let done = graphql_body(
        graphql_operation(
            &router,
            "mutation($id:ID!){sessionRevokeOther(id:$id){node}}",
            serde_json::json!({"id":other.session.id.to_string()}),
            Some(&first.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert_eq!(done["data"]["sessionRevokeOther"]["node"], true);
    assert!(fixture.service().validate_access(&actor).await.is_ok());
    assert!(fixture.service().validate_access(&other).await.is_err());
    assert!(fixture.refresh(&second.refresh_token).await.is_err());
}

#[tokio::test]
async fn revoke_other_sessions_preserves_current_and_foreign_accounts() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET).unwrap();
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await
        .unwrap();
    let actor =
        UserAccessToken::try_from(decode_token(&tokens.access_token, SECRET).unwrap()).unwrap();
    {
        let mut state = fixture.repo.0.lock().unwrap();
        for offset in [1, 2] {
            let mut other = actor.session.clone();
            other.id += offset;
            other.organization_id += offset;
            state.sessions.insert(other.id, other);
        }
        let mut foreign = actor.session.clone();
        foreign.id += 3;
        foreign.user_id += 1;
        state.sessions.insert(foreign.id, foreign);
    }
    assert_eq!(
        fixture
            .service()
            .revoke_other_sessions(&actor, None)
            .await
            .unwrap(),
        2
    );
    let state = fixture.repo.0.lock().unwrap();
    assert!(state.sessions[&actor.session.id].revoked_at.is_none());
    assert!(state.sessions[&(actor.session.id + 3)].revoked_at.is_none());
}

#[tokio::test]
async fn session_activity_changes_only_on_renewal_and_expiry_blocks_access_and_refresh() {
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let mut stored = stored_session(1, 7, 84, "Workspace");
    stored.last_seen_at -= 3600;
    stored.expires_at -= 3600;
    let refresh = RefreshToken::from_session(&stored, REFRESH_LIFETIME).unwrap();
    let token = AccessToken::Refresh(refresh)
        .encode_compressed(SECRET, REFRESH_LIFETIME)
        .unwrap();
    fixture.remember(&token);
    let actor = fixture.access_for_session(stored.clone());
    assert!(fixture.service().validate_access(&actor).await.unwrap());
    assert_eq!(fixture.repo.0.lock().unwrap().sessions[&7], stored);
    let renewed = fixture.service().refresh(&token).await.unwrap();
    let credential =
        RefreshToken::try_from(decode_token(&renewed.refresh_token, SECRET).unwrap()).unwrap();
    let current = fixture.repo.0.lock().unwrap().sessions[&7].clone();
    assert_eq!(current.expires_at, credential.validity.expires_at);
    assert_eq!(current.last_seen_at, credential.validity.issued_at);
    assert!(current.last_seen_at > stored.last_seen_at);
    fixture
        .repo
        .0
        .lock()
        .unwrap()
        .sessions
        .get_mut(&7)
        .unwrap()
        .expires_at = compallm_utils::time::unix_timestamp();
    assert!(matches!(
        fixture.service().validate_access(&actor).await,
        Err(AuthError::InvalidSession)
    ));
    assert!(matches!(
        fixture.service().refresh(&token).await,
        Err(AuthError::InvalidSession)
    ));
}

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL with catalog migrations applied"]
async fn graphql_template_reads_require_membership_and_return_complete_model_defaults()
-> anyhow::Result<()> {
    use sea_orm::Database;
    let fixture = Fixture::new(Some(Fixture::user(UserStatus::Active)));
    let challenge = test_challenge("james@midwess.com", SECRET)?;
    let tokens = fixture
        .signin(&Fixture::otp(&challenge), &challenge.code)
        .await?;
    let pool = Arc::new(Database::connect(std::env::var("COMPALLM_TEST_DATABASE_URL")?).await?);
    let context = DbContext::new(Arc::new(PostgresConnection::new(pool)));
    let router = fixture.graphql_with_database(context);
    let query = "{ providerTemplates(first: 2) { edges { cursor node { id name logoPath } } pageInfo { endCursor hasNextPage } } }";
    let denied = graphql_body(
        graphql_operation(&router, query, serde_json::json!({}), None, None, false).await,
    )
    .await;
    assert_eq!(denied["errors"][0]["extensions"]["code"], "UNAUTHENTICATED");
    let page = graphql_body(
        graphql_operation(
            &router,
            query,
            serde_json::json!({}),
            Some(&tokens.access_token),
            None,
            false,
        )
        .await,
    )
    .await;
    assert!(page["errors"].is_null(), "{page}");
    assert_eq!(
        page["data"]["providerTemplates"]["edges"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        page["data"]["providerTemplates"]["pageInfo"]["hasNextPage"],
        true
    );
    let cursor = page["data"]["providerTemplates"]["pageInfo"]["endCursor"]
        .as_str()
        .unwrap();
    let next = graphql_body(graphql_operation(&router, "query($after: String!) { providerTemplates(first: 2, after: $after) { edges { node { id } } pageInfo { hasPreviousPage } } }", serde_json::json!({"after": cursor}), Some(&tokens.access_token), None, false).await).await;
    assert!(next["errors"].is_null(), "{next}");
    assert_eq!(
        next["data"]["providerTemplates"]["pageInfo"]["hasPreviousPage"],
        true
    );
    assert_ne!(
        next["data"]["providerTemplates"]["edges"][0]["node"]["id"],
        page["data"]["providerTemplates"]["edges"][0]["node"]["id"]
    );
    let search = graphql_body(graphql_operation(&router, "{ providerTemplates(filter: {name: {ilike: \"%oPeNaI%\"}}) { edges { node { name } } } }", serde_json::json!({}), Some(&tokens.access_token), None, false).await).await;
    assert!(search["errors"].is_null(), "{search}");
    assert!(
        !search["data"]["providerTemplates"]["edges"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let mismatch = graphql_body(graphql_operation(&router, "query($after: String!) { providerTemplates(filter: {name: {ilike: \"%openai%\"}}, after: $after) { edges { node { id } } } }", serde_json::json!({"after":cursor}), Some(&tokens.access_token), None, false).await).await;
    assert_eq!(
        mismatch["errors"][0]["extensions"]["code"],
        "BAD_USER_INPUT"
    );
    let detail = graphql_body(graphql_operation(&router, "{ providerTemplate(id: \"openai\") { node { id headers { key value } authentication { method } models { edges { node { id upstreamModelId operations { operation path } capabilities } } pageInfo { hasNextPage hasPreviousPage } } } } }", serde_json::json!({}), Some(&tokens.access_token), None, false).await).await;
    assert!(detail["errors"].is_null(), "{detail}");
    let models = &detail["data"]["providerTemplate"]["node"]["models"];
    assert_eq!(models["pageInfo"]["hasNextPage"], false);
    let aliased = graphql_body(graphql_operation(&router, "{ providerTemplate(id: \"openai\") { node { models { entries: edges { item: node { id context: maxContextTokens inputPrice outputPrice } } } } } }", serde_json::json!({}), Some(&tokens.access_token), None, false).await).await;
    assert!(aliased["errors"].is_null(), "{aliased}");
    let id = models["edges"][0]["node"]["id"]
        .as_str()
        .expect("eligible OpenAI model");
    let model = graphql_body(graphql_operation(&router, "query($id: ID!) { modelTemplate(providerTemplateId: \"openai\", id: $id) { node { id upstreamModelId maxContextTokens maxInputTokens maxOutputTokens effortMapping { level value } inputPrice outputPrice cachedPrice billingModel pricingMode cacheBilling } } }", serde_json::json!({"id": id}), Some(&tokens.access_token), None, false).await).await;
    assert!(model["errors"].is_null(), "{model}");
    assert_eq!(model["data"]["modelTemplate"]["node"]["id"], id);
    let missing = graphql_body(graphql_operation(&router, "{ modelTemplate(providerTemplateId: \"absent\", id: \"absent\") { node { id } } providerTemplate(id: \"absent\") { node { id } } }", serde_json::json!({}), Some(&tokens.access_token), None, false).await).await;
    assert!(missing["errors"].is_null(), "{missing}");
    assert!(missing["data"]["modelTemplate"].is_null());
    assert!(missing["data"]["providerTemplate"].is_null());
    Ok(())
}
