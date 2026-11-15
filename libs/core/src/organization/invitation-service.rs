use crate::authentication::{
    AuthError, AuthService, EmailOtpAccessToken, UserTokens,
    token::{AccessToken, InvitationToken, TokenValidity},
};
use crate::{
    authorization::{MemberPermission, condition::CheckPermission, policy::Action},
    organization::{
        OrganizationMember,
        invitation::{Invitation, InvitationStatus},
    },
    persistent::{
        db_context, invitation::InvitationRepository, organization::OrganizationRepository,
        user::UserRepository,
    },
    user::UserStatus,
};
use compallm_utils::time::unix_timestamp;
use std::time::Duration;

pub struct IssuedInvitation {
    pub invitation: Invitation,
    pub token: String,
    pub email_admitted: bool,
}
pub struct InvitationContext {
    pub invitation: Invitation,
    pub organization_name: String,
    pub signup: bool,
}

pub struct InvitationService {
    invitations: &'static dyn InvitationRepository,
    organizations: &'static Box<dyn OrganizationRepository>,
    users: &'static Box<dyn UserRepository>,
    registration: &'static crate::user::RegistrationService,
    preferences: &'static crate::preference::UserPreferenceService,
    email: Option<&'static Box<dyn crate::mail::EmailService>>,
    auth: &'static AuthService,
}

impl InvitationService {
    pub fn new(
        invitations: &'static dyn InvitationRepository,
        organizations: &'static Box<dyn OrganizationRepository>,
        users: &'static Box<dyn UserRepository>,
        registration: &'static crate::user::RegistrationService,
        preferences: &'static crate::preference::UserPreferenceService,
        email: Option<&'static Box<dyn crate::mail::EmailService>>,
        auth: &'static AuthService,
    ) -> Self {
        Self {
            invitations,
            organizations,
            users,
            registration,
            preferences,
            email,
            auth,
        }
    }

    #[db_context::root]
    pub async fn invitation_context(
        &self,
        claims: &InvitationToken,
    ) -> Result<InvitationContext, AuthError> {
        let invitation = self
            .invitations
            .find(claims.organization_id, claims.invitation_id, false)
            .await?
            .ok_or(AuthError::InvalidInvitation)?;
        invitation.validate(claims, unix_timestamp())?;
        let organization = self
            .organizations
            .find_by_id(claims.organization_id)
            .await?
            .ok_or(AuthError::InvalidInvitation)?;
        if organization.status != crate::organization::OrganizationStatus::Active {
            return Err(AuthError::InvalidInvitation);
        }
        let user = self.users.find_by_email(&claims.email).await?;
        if user
            .as_ref()
            .is_some_and(|user| user.status != UserStatus::Active)
        {
            return Err(AuthError::UserDisabled);
        }
        Ok(InvitationContext {
            invitation,
            organization_name: organization.name,
            signup: user.is_none(),
        })
    }

    pub async fn invitations_create(
        &self,
        permission: &MemberPermission,
        emails: &[String],
    ) -> Result<Vec<IssuedInvitation>, AuthError> {
        Action::MemberInvite
            .check_permission(permission)
            .map_err(|_| AuthError::OrganizationAccessDenied)?;
        if emails.is_empty() || emails.len() > 100 {
            return Err(AuthError::InvalidEmail);
        }
        let mut unique = std::collections::BTreeSet::new();
        for email in emails {
            let email = compallm_utils::email::normalize_email(email);
            compallm_utils::email::validate_email(&email).map_err(|_| AuthError::InvalidEmail)?;
            if !unique.insert(email) {
                return Err(AuthError::InvitationConflict);
            }
        }
        self.create_invitation_batch(permission, unique.into_iter().collect())
            .await
    }
    #[db_context::root]
    async fn create_invitation_batch(
        &self,
        permission: &MemberPermission,
        emails: Vec<String>,
    ) -> Result<Vec<IssuedInvitation>, AuthError> {
        self.auth.latest_dictionary().await?;
        crate::transaction!(db, {
            let mut issued = Vec::with_capacity(emails.len());
            for email in emails {
                issued.push(self.invitation_create(permission, &email, None).await?);
            }
            Ok(issued)
        })
    }

    pub async fn invitation_create(
        &self,
        permission: &MemberPermission,
        email: &str,
        resend: Option<u64>,
    ) -> Result<IssuedInvitation, AuthError> {
        Action::MemberInvite
            .check_permission(permission)
            .map_err(|_| AuthError::OrganizationAccessDenied)?;
        let email = compallm_utils::email::normalize_email(email);
        compallm_utils::email::validate_email(&email).map_err(|_| AuthError::InvalidEmail)?;
        self.issue_invitation(
            permission.organization_id(),
            permission.user_id(),
            email,
            resend,
        )
        .await
    }

    #[db_context::root]
    async fn issue_invitation(
        &self,
        organization_id: u64,
        inviter_id: u64,
        email: String,
        resend: Option<u64>,
    ) -> Result<IssuedInvitation, AuthError> {
        let repository = self.invitations;
        repository.member_role(organization_id).await?;
        let validity = TokenValidity::new(Duration::from_secs(InvitationToken::TTL_SECONDS))?;
        let invitation_id = hero::distributed_id::try_gen_id().await?;
        let claims = InvitationToken {
            invitation_id,
            organization_id,
            email: email.clone(),
            validity,
        };
        let token = self
            .auth
            .encrypt_token(
                AccessToken::Invitation(claims.clone()),
                Duration::from_secs(InvitationToken::TTL_SECONDS),
            )
            .await?;
        let invitation = crate::transaction!(db, {
            repository.lock_organization(organization_id).await?;
            if let Some(previous) = resend {
                let old = repository
                    .find(organization_id, previous, true)
                    .await?
                    .ok_or(AuthError::InvalidInvitation)?;
                if old.email != email {
                    return Err(AuthError::InvalidInvitation);
                }
                repository
                    .revoke(organization_id, previous, unix_timestamp())
                    .await?;
            }
            if let Some(user) = self.users.find_by_email(&email).await? {
                if user.status != UserStatus::Active {
                    return Err(AuthError::UserDisabled);
                }
                if self
                    .organizations
                    .find_membership(organization_id, user.id)
                    .await?
                    .is_some()
                {
                    return Err(AuthError::InvitationConflict);
                }
                if repository
                    .membership_exists(organization_id, user.id)
                    .await?
                {
                    return Err(AuthError::InvalidInvitation);
                }
            }
            if repository
                .has_pending(organization_id, &email, unix_timestamp())
                .await?
            {
                return Err(AuthError::InvitationConflict);
            }
            repository
                .create(Invitation {
                    id: invitation_id,
                    organization_id,
                    email: email.clone(),
                    inviter_id,
                    status: InvitationStatus::Pending,
                    expires_at: claims.validity.expires_at,
                    accepted_at: None,
                    accepted_by: None,
                    revoked_at: None,
                })
                .await
        })?;
        Ok(IssuedInvitation {
            invitation,
            token,
            email_admitted: false,
        })
    }

    pub async fn invitation_resend(
        &self,
        permission: &MemberPermission,
        id: u64,
    ) -> Result<IssuedInvitation, AuthError> {
        Action::MemberInvite
            .check_permission(permission)
            .map_err(|_| AuthError::OrganizationAccessDenied)?;
        self.resend_invitation(permission.organization_id(), permission.user_id(), id)
            .await
    }

    #[db_context::root]
    async fn resend_invitation(
        &self,
        organization_id: u64,
        inviter_id: u64,
        id: u64,
    ) -> Result<IssuedInvitation, AuthError> {
        let old = self
            .invitations
            .find(organization_id, id, false)
            .await?
            .ok_or(AuthError::InvalidInvitation)?;
        self.issue_invitation(organization_id, inviter_id, old.email, Some(id))
            .await
    }

    #[db_context::root]
    pub async fn invitation_send(
        &self,
        issued: &mut IssuedInvitation,
        url: &str,
    ) -> Result<(), AuthError> {
        let organization = self
            .organizations
            .find_by_id(issued.invitation.organization_id)
            .await?
            .ok_or(AuthError::InvalidInvitation)?;
        let inviter = self
            .users
            .find_by_id(issued.invitation.inviter_id)
            .await?
            .ok_or(AuthError::UserNotFound)?;
        let request = crate::mail::EmailRequest {
            to: issued.invitation.email.clone(),
            subject: format!("Join {} on Compallm", organization.name),
            text: crate::mail::EmailTemplate::Invitation(
                crate::mail::template::invitation::InvitationEmail {
                    organization: organization.name,
                    inviter: inviter.email,
                    recipient: issued.invitation.email.clone(),
                    url: url.into(),
                },
            ),
        };
        issued.email_admitted = match self.email {
            Some(sender) => sender.send(request).await.is_ok(),
            None => false,
        };
        Ok(())
    }

    pub async fn invitation_revoke(
        &self,
        permission: &MemberPermission,
        id: u64,
    ) -> Result<Invitation, AuthError> {
        Action::MemberInvite
            .check_permission(permission)
            .map_err(|_| AuthError::OrganizationAccessDenied)?;
        self.revoke_invitation(permission.organization_id(), id)
            .await
    }
    #[db_context::root]
    async fn revoke_invitation(
        &self,
        organization_id: u64,
        id: u64,
    ) -> Result<Invitation, AuthError> {
        crate::transaction!(db, {
            self.invitations.lock_organization(organization_id).await?;
            self.invitations
                .revoke(organization_id, id, unix_timestamp())
                .await
        })
    }

    #[db_context::root]
    pub async fn invitation_email(
        &self,
        invitation: &InvitationToken,
        email: &str,
        signup: bool,
    ) -> Result<crate::authentication::IssuedEmailOtp, AuthError> {
        let context = self.invitation_context(invitation).await?;
        if compallm_utils::email::normalize_email(email) != invitation.email
            || signup != context.signup
        {
            return Err(AuthError::InvalidInvitation);
        }
        let mut claims = EmailOtpAccessToken::new(invitation.email.clone())?;
        claims.invitation_id = Some(invitation.invitation_id);
        self.auth.issue_email_otp(claims).await
    }

    #[db_context::root]
    pub async fn invitation_confirm(
        &self,
        invitation: &InvitationToken,
        otp: &EmailOtpAccessToken,
        code: &str,
        signup: bool,
    ) -> Result<UserTokens, AuthError> {
        otp.verify_code(code, unix_timestamp())?;
        if otp.invitation_id != Some(invitation.invitation_id) || otp.email != invitation.email {
            return Err(AuthError::InvalidInvitation);
        }
        let context = self.invitation_context(invitation).await?;
        if context.signup != signup {
            return Err(AuthError::InvalidInvitation);
        }
        let dictionary = self.auth.latest_dictionary().await?;
        let repository = self.invitations;
        crate::transaction!(db, {
            repository
                .lock_organization(invitation.organization_id)
                .await?;
            let row = repository
                .find(invitation.organization_id, invitation.invitation_id, true)
                .await?
                .ok_or(AuthError::InvalidInvitation)?;
            row.validate(invitation, unix_timestamp())?;
            let user = if signup {
                let user = self
                    .registration
                    .register_invited(&invitation.email)
                    .await?;
                self.preferences.create_defaults(user.id).await?;
                user
            } else {
                self.users
                    .find_by_email(&invitation.email)
                    .await?
                    .ok_or(AuthError::UserNotFound)?
            };
            if user.status != UserStatus::Active {
                return Err(AuthError::UserDisabled);
            }
            let membership = match self
                .organizations
                .find_membership(invitation.organization_id, user.id)
                .await?
            {
                Some((member, _)) => member,
                None => {
                    if repository
                        .membership_exists(invitation.organization_id, user.id)
                        .await?
                    {
                        return Err(AuthError::InvalidInvitation);
                    }
                    let role_id = repository.member_role(invitation.organization_id).await?;
                    let role = self
                        .organizations
                        .find_role(invitation.organization_id, role_id)
                        .await?
                        .ok_or(AuthError::InvalidInvitation)?;
                    self.organizations
                        .add_member(OrganizationMember {
                            organization_id: invitation.organization_id,
                            user_id: user.id,
                            role,
                            groups: vec![],
                        })
                        .await?
                }
            };
            repository
                .accept(
                    invitation.organization_id,
                    invitation.invitation_id,
                    user.id,
                    unix_timestamp(),
                )
                .await?;
            self.auth
                .start_session(user.id, membership, dictionary)
                .await
        })
    }
}
