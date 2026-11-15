use minijinja::context;
use std::fmt;

pub struct InvitationEmail {
    pub organization: String,
    pub inviter: String,
    pub recipient: String,
    pub url: String,
}
impl InvitationEmail {
    pub fn render(&self) -> Result<String, minijinja::Error> {
        super::render(
            "organization-invitation.html",
            context! {
                organization => self.organization.as_str(),
                inviter => self.inviter.as_str(),
                recipient => self.recipient.as_str(),
                invitation_url => self.url.as_str(),
            },
        )
    }

    pub fn plain_text(&self) -> String {
        format!(
            "{} invited {} to join {} on Compallm.\n\nAccept invitation: {}\n\nThis invitation expires seven days after issuance. Continue to receive an email verification code.\n\nIf you did not expect this invitation, ignore this email.",
            self.inviter, self.recipient, self.organization, self.url
        )
    }
}
impl fmt::Display for InvitationEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.render().map_err(|_| fmt::Error)?)
    }
}
