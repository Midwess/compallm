use maud::html;
use std::fmt;

pub struct InvitationEmail {
    pub organization: String,
    pub inviter: String,
    pub recipient: String,
    pub url: String,
}
impl InvitationEmail {
    pub fn plain_text(&self) -> String {
        format!(
            "{} invited {} to join {} on Compallm.\n\nAccept invitation: {}\n\nThis invitation expires seven days after issuance. Continue to receive an email verification code.\n\nIf you did not expect this invitation, ignore this email.",
            self.inviter, self.recipient, self.organization, self.url
        )
    }
}
impl fmt::Display for InvitationEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&html! {
            html lang="en" { body {
                h1 { "Join " (self.organization) }
                p { (self.inviter) " invited " (self.recipient) " to join their organization on Compallm." }
                p { a href=(self.url) { "Accept invitation" } }
                p { "This invitation expires seven days after issuance. Continue to receive an email verification code." }
                p { "If you did not expect this invitation, ignore this email." }
            } }
        }.into_string())
    }
}
