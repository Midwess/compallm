use crate::authorization::condition::check::CheckPermission;
use crate::{
    authentication::UserAccessToken,
    authorization::{MemberPermission, policy::Action},
    persistent::{
        db_context,
        onboarding::{OnboardingRepository, OnboardingRepositoryError},
        user::UserRepository,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Destination {
    #[default]
    Chat,
    Models,
    Providers,
}
#[derive(Clone, Debug, Serialize, Deserialize, validify::Validify)]
pub struct UsefulLink {
    #[validate(length(min = 1, max = 128))]
    pub label: String,
    #[validate(custom(function = compallm_utils::input::web_url), length(max = 2048))]
    pub href: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, validify::Validify)]
pub struct Settings {
    #[validate(length(max = 4096))]
    pub welcome: String,
    #[validate(length(max = 20))]
    #[validify]
    pub links: Vec<UsefulLink>,
    pub destination: Destination,
    #[validate(custom(function = compallm_utils::input::hex_color))]
    pub artwork_color: Option<String>,
    pub require_full_name: bool,
    pub require_avatar: bool,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Flow {
    Personal,
    InvitedNew,
    InvitedExisting,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Step {
    Profile,
    Workspace,
    Work,
    Preferences,
    Introduction,
    Ready,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Progress {
    pub flow: Flow,
    pub step: Step,
    pub settings: Settings,
    #[serde(default)]
    pub presented_session_ids: std::collections::BTreeSet<u64>,
}
impl Progress {
    pub fn new(flow: Flow, settings: Settings) -> Self {
        Self {
            flow,
            step: if flow == Flow::InvitedExisting {
                Step::Introduction
            } else {
                Step::Profile
            },
            settings,
            presented_session_ids: Default::default(),
        }
    }
    pub fn steps(&self) -> &'static [Step] {
        match self.flow {
            Flow::Personal => &[
                Step::Profile,
                Step::Workspace,
                Step::Work,
                Step::Preferences,
                Step::Ready,
            ],
            Flow::InvitedNew => &[
                Step::Profile,
                Step::Work,
                Step::Preferences,
                Step::Introduction,
                Step::Ready,
            ],
            Flow::InvitedExisting => &[Step::Introduction, Step::Ready],
        }
    }
    pub fn require_profile(&self, user: &crate::user::User) -> Result<(), OnboardingError> {
        if self.flow == Flow::InvitedNew
            && ((self.settings.require_full_name && user.display_name.trim().is_empty())
                || (self.settings.require_avatar && user.avatar_path.is_none()))
        {
            return Err(OnboardingError::Requirements);
        }
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct Context {
    pub required: bool,
    pub progress: Progress,
}
#[derive(Debug, thiserror::Error)]
pub enum OnboardingError {
    #[error(transparent)]
    Persistence(#[from] OnboardingRepositoryError),
    #[error(transparent)]
    User(#[from] crate::persistent::user::UserRepositoryError),
    #[error(transparent)]
    Authentication(#[from] crate::authentication::AuthError),
    #[error(transparent)]
    Permission(#[from] crate::authorization::MemberPermissionError),
    #[error(transparent)]
    Invalid(#[from] validify::ValidationErrors),
    #[error("onboarding requirements are not satisfied")]
    Requirements,
    #[error("invalid onboarding transition")]
    Transition,
}
impl From<crate::persistent::ConnectionError> for OnboardingError {
    fn from(e: crate::persistent::ConnectionError) -> Self {
        Self::Persistence(e.into())
    }
}
pub struct OnboardingService {
    repository: &'static dyn OnboardingRepository,
    users: &'static dyn UserRepository,
    organizations: &'static dyn crate::persistent::organization::OrganizationRepository,
}
impl OnboardingService {
    pub fn new(
        repository: &'static dyn OnboardingRepository,
        users: &'static dyn UserRepository,
        organizations: &'static dyn crate::persistent::organization::OrganizationRepository,
    ) -> Self {
        Self {
            repository,
            users,
            organizations,
        }
    }
    async fn require_member(&self, actor: &UserAccessToken) -> Result<(), OnboardingError> {
        let user = self
            .users
            .find_by_id(actor.session.user_id)
            .await?
            .ok_or(OnboardingError::Requirements)?;
        if user.status != crate::user::UserStatus::Active {
            return Err(crate::authentication::AuthError::UserDisabled.into());
        }
        if self
            .organizations
            .find_membership(actor.session.organization_id, actor.session.user_id)
            .await
            .map_err(crate::authentication::AuthError::from)?
            .is_none()
        {
            return Err(crate::authentication::AuthError::OrganizationAccessDenied.into());
        }
        Ok(())
    }
    pub async fn update_settings(
        &self,
        permission: &MemberPermission,
        mut settings: Settings,
    ) -> Result<Settings, OnboardingError> {
        use validify::Validify;
        Action::OrganizationOnboardingUpdate.check_permission(permission)?;
        settings.validify()?;
        Ok(self
            .repository
            .save_settings(permission.organization_id(), settings)
            .await?)
    }
    #[db_context::current]
    pub async fn advance(
        &self,
        actor: &UserAccessToken,
        back: bool,
    ) -> Result<Context, OnboardingError> {
        actor.validate(compallm_utils::time::unix_timestamp())?;
        crate::transaction!(db, {
            let mut context = self
                .repository
                .find(actor.session.organization_id, actor.session.user_id, true)
                .await?
                .ok_or(OnboardingError::Transition)?;
            if !context.required {
                return Ok(context);
            }
            let progress = &mut context.progress;
            if !back && progress.step == Step::Profile {
                let user = self
                    .users
                    .find_by_id(actor.session.user_id)
                    .await?
                    .ok_or(OnboardingError::Requirements)?;
                progress.require_profile(&user)?;
            }
            let steps = progress.steps();
            let index = steps
                .iter()
                .position(|s| *s == progress.step)
                .ok_or(OnboardingError::Transition)?;
            let target = if back {
                index.saturating_sub(1)
            } else {
                (index + 1).min(steps.len() - 1)
            };
            progress.step = steps[target];
            self.repository
                .save_progress(
                    actor.session.organization_id,
                    actor.session.user_id,
                    progress.clone(),
                )
                .await?;
            Ok(context)
        })
    }
    #[db_context::root]
    pub async fn complete(&self, actor: &UserAccessToken) -> Result<Context, OnboardingError> {
        actor.validate(compallm_utils::time::unix_timestamp())?;
        self.require_member(actor).await?;
        crate::transaction!(db, {
            let mut context = self
                .repository
                .find(actor.session.organization_id, actor.session.user_id, true)
                .await?
                .ok_or(OnboardingError::Transition)?;
            if !context.required {
                return Ok(context);
            }
            if context.progress.step != Step::Ready {
                return Err(OnboardingError::Transition);
            }
            let user = self
                .users
                .find_by_id(actor.session.user_id)
                .await?
                .ok_or(OnboardingError::Requirements)?;
            context.progress.require_profile(&user)?;
            self.repository
                .complete(actor.session.organization_id, actor.session.user_id)
                .await?;
            context.required = false;
            Ok(context)
        })
    }
    #[db_context::root]
    pub async fn present(
        &self,
        actor: &UserAccessToken,
        manual: bool,
    ) -> Result<Context, OnboardingError> {
        actor.validate(compallm_utils::time::unix_timestamp())?;
        self.require_member(actor).await?;
        crate::transaction!(db, {
            let org = actor.session.organization_id;
            let user = actor.session.user_id;
            if self.repository.find(org, user, false).await?.is_none() {
                if !manual {
                    return Err(OnboardingError::Transition);
                }
                let settings = self.repository.settings(org).await?.unwrap_or_default();
                self.repository
                    .initialize(org, user, Progress::new(Flow::InvitedExisting, settings))
                    .await?;
            }
            let mut context = self
                .repository
                .find(org, user, true)
                .await?
                .ok_or(OnboardingError::Transition)?;
            context
                .progress
                .presented_session_ids
                .insert(actor.session.id);
            self.repository
                .save_progress(org, user, context.progress.clone())
                .await?;
            Ok(context)
        })
    }
}
