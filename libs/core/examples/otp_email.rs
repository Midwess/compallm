use compallm_core::mail::OtpCode;
use std::time::Duration;

/// Render a sample email for local preview; no email is sent.
fn main() {
    println!("{}", OtpCode::new("482619", Duration::from_secs(600)));
}
