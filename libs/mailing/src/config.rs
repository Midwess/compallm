use serde::Deserialize;

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SmtpTls {
    None,
    Starttls,
    Tls,
}

/// Credentials intentionally do not implement Debug.
#[derive(Clone, Deserialize)]
pub struct SmtpSettings {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub from_address: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    pub tls: SmtpTls,
    #[serde(default = "SmtpSettings::default_concurrency")]
    pub concurrency: usize,
    #[serde(default = "SmtpSettings::default_queue_capacity")]
    pub queue_capacity: usize,
    #[serde(default = "SmtpSettings::default_admission_timeout_seconds")]
    pub admission_timeout_seconds: u64,
    #[serde(default = "SmtpSettings::default_send_timeout_seconds")]
    pub send_timeout_seconds: u64,
    #[serde(default = "SmtpSettings::default_recipient_cooldown_seconds")]
    pub recipient_cooldown_seconds: u64,
    #[serde(default = "SmtpSettings::default_shutdown_timeout_seconds")]
    pub shutdown_timeout_seconds: u64,
}

impl SmtpSettings {
    fn default_concurrency() -> usize {
        10
    }
    fn default_queue_capacity() -> usize {
        1024
    }
    fn default_admission_timeout_seconds() -> u64 {
        30
    }
    fn default_send_timeout_seconds() -> u64 {
        60
    }
    fn default_recipient_cooldown_seconds() -> u64 {
        30
    }
    fn default_shutdown_timeout_seconds() -> u64 {
        60
    }
}
