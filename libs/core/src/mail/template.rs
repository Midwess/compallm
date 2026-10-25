use std::fmt;

pub mod otp_code;
pub use otp_code::OtpCode;

/// Every supported email template. Intentionally does not implement Debug.
pub enum EmailTemplate {
    OtpCode(OtpCode),
}

impl EmailTemplate {
    pub fn plain_text(&self) -> String {
        match self {
            Self::OtpCode(template) => template.plain_text(),
        }
    }
}

/// `to_string()` renders the HTML body using the inner template.
impl fmt::Display for EmailTemplate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OtpCode(template) => template.fmt(formatter),
        }
    }
}
