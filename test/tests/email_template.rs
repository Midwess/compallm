use compallm_core::mail::{EmailTemplate, OtpCode};
use std::time::Duration;

#[test]
fn otp_template_renders_html_and_plain_text_with_the_same_code_and_expiry() {
    let template = EmailTemplate::OtpCode(OtpCode::new("012345", Duration::from_secs(600)));
    let html = template.to_string();
    let text = template.plain_text();
    assert!(html.starts_with("<!DOCTYPE html>"));
    assert!(html.contains("012345"));
    assert!(html.contains("10 minutes"));
    assert!(html.contains("role=\"presentation\""));
    assert!(html.contains("lang=\"en\""));
    assert!(!html.contains("<script"));
    assert!(!html.contains("<img"));
    assert!(!html.contains("<link"));
    assert!(!html.contains("https://"));
    let preheader = html
        .split("<body")
        .nth(1)
        .unwrap()
        .split("</div>")
        .next()
        .unwrap();
    assert!(!preheader.contains("012345"));
    assert!(text.contains("012345"));
    assert!(text.contains("10 minutes"));
    assert!(!text.contains("<html"));
}

#[test]
fn dynamic_template_values_are_html_escaped() {
    let template = OtpCode::new("<script>alert('code')</script>&", Duration::from_secs(60));
    let html = template.to_string();
    assert!(!html.contains("<script>"));
    assert!(html.contains("&lt;script&gt;"));
    assert!(html.contains("&amp;"));
    assert!(template.plain_text().contains("1 minute."));
}
