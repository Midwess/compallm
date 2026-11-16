use minijinja::{AutoEscape, Environment, UndefinedBehavior, Value};
use std::{fmt, sync::LazyLock};

pub mod invitation;
pub mod otp_code;
pub use otp_code::OtpCode;

static TEMPLATES: LazyLock<Result<Environment<'static>, minijinja::Error>> = LazyLock::new(|| {
    let mut environment = Environment::new();
    environment.set_auto_escape_callback(|_| AutoEscape::Html);
    environment.set_undefined_behavior(UndefinedBehavior::Strict);
    environment.add_template(
        "otp.html",
        include_str!("../../../email-templates/otp.html"),
    )?;
    environment.add_template(
        "organization-invitation.html",
        include_str!("../../../email-templates/organization-invitation.html"),
    )?;
    Ok(environment)
});

fn render(name: &str, context: Value) -> Result<String, minijinja::Error> {
    TEMPLATES
        .as_ref()
        .map_err(|error| {
            minijinja::Error::new(error.kind(), "embedded email template failed to compile")
        })?
        .get_template(name)?
        .render(context)
}

/// Every supported email template. Intentionally does not implement Debug.
pub enum EmailTemplate {
    OtpCode(OtpCode),
    Invitation(invitation::InvitationEmail),
}

impl EmailTemplate {
    pub fn render(&self) -> Result<String, minijinja::Error> {
        match self {
            Self::OtpCode(template) => template.render(),
            Self::Invitation(template) => template.render(),
        }
    }

    pub fn plain_text(&self) -> String {
        match self {
            Self::OtpCode(template) => template.plain_text(),
            Self::Invitation(template) => template.plain_text(),
        }
    }
}

/// `to_string()` renders the HTML body using the inner template.
impl fmt::Display for EmailTemplate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OtpCode(template) => template.fmt(formatter),
            Self::Invitation(template) => template.fmt(formatter),
        }
    }
}
