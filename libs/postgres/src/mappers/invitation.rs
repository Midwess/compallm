impl TryFrom<crate::schema::OrganizationMemberInvitationSchema>
    for compallm_core::organization::invitation::Invitation
{
    type Error = compallm_core::authentication::AuthError;
    fn try_from(
        value: crate::schema::OrganizationMemberInvitationSchema,
    ) -> Result<Self, Self::Error> {
        fn id(value: i64) -> Result<u64, compallm_core::authentication::AuthError> {
            u64::try_from(value)
                .map_err(|_| compallm_core::authentication::AuthError::InvalidInvitation)
        }
        Ok(Self {
            id: id(value.id)?,
            organization_id: id(value.organization_id)?,
            email: value.email,
            inviter_id: id(value.inviter_id)?,
            status: value.status,
            expires_at: id(value.expires_at)?,
            accepted_at: value.accepted_at.map(id).transpose()?,
            accepted_by: value.accepted_by.map(id).transpose()?,
            revoked_at: value.revoked_at.map(id).transpose()?,
        })
    }
}
