use minijinja::context;
use std::{fmt, time::Duration};

/// Verification email content. Keep the code out of logs and preview text.
pub struct OtpCode {
    pub code: String,
    pub expires_in: Duration,
}

impl OtpCode {
    pub fn new(code: impl Into<String>, expires_in: Duration) -> Self {
        Self {
            code: code.into(),
            expires_in,
        }
    }

    pub fn plain_text(&self) -> String {
        format!(
            "Compallm\n\nConfirm your email\n\nYour verification code is:\n\n{}\n\nThis code expires in {}. Enter it in the app to verify your identity and continue in Compallm.\n\nNever share this code. Compallm will never ask you to send it by email or chat.\n\nIf you did not request this email, you can safely ignore it.",
            self.code,
            self.expiration_label(),
        )
    }

    fn expiration_label(&self) -> String {
        let minutes = self.expires_in.as_secs().div_ceil(60);
        format!(
            "{minutes} {}",
            if minutes == 1 { "minute" } else { "minutes" }
        )
    }

    pub fn render(&self) -> Result<String, minijinja::Error> {
        super::render(
            "otp.html",
            context! {
                code => self.code.as_str(),
                expires_in => self.expiration_label(),
            },
        )
    }
}

/// The standard `ToString` implementation renders HTML.
impl fmt::Display for OtpCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.render().map_err(|_| fmt::Error)?)
    }
}
