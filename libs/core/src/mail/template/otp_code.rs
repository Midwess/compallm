use maud::{DOCTYPE, Markup, html};
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
            "Compallm\n\nConfirm your email\n\nYour verification code is:\n\n{}\n\nThis code expires in {}. Enter it in the app to finish creating your account.\n\nNever share this code. Compallm will never ask you to send it by email or chat.\n\nIf you did not request this email, you can safely ignore it.",
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

    pub fn render(&self) -> Markup {
        let expires_in = self.expiration_label();
        html! {
            (DOCTYPE)
            html lang="en" dir="ltr" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    meta name="color-scheme" content="light";
                    meta name="supported-color-schemes" content="light";
                    title { "Confirm your Compallm email" }
                }
                body style="margin:0;padding:0;background-color:#f3f5f2;color:#182a25;font-family:Arial,Helvetica,sans-serif;-webkit-text-size-adjust:100%;" {
                    div style="display:none;max-height:0;overflow:hidden;opacity:0;mso-hide:all;" {
                        "Your Compallm verification code is ready. It expires in " (expires_in) "."
                    }
                    table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background-color:#f3f5f2;" {
                        tr {
                            td align="center" style="padding:40px 16px;" {
                                table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="max-width:520px;" {
                                    tr {
                                        td style="padding:0 8px 24px;font-size:22px;line-height:28px;font-weight:700;letter-spacing:-0.8px;color:#183c30;" {
                                            "Compallm" span style="color:#619c80;" { "." }
                                        }
                                    }
                                    tr {
                                        td style="padding:32px 28px;background-color:#ffffff;border:1px solid #dfe7df;border-top:4px solid #1e654c;border-radius:16px;" {
                                            p style="margin:0 0 16px;font-size:11px;line-height:16px;font-weight:700;letter-spacing:1.8px;color:#47735f;" {
                                                "EMAIL VERIFICATION"
                                            }
                                            h1 style="margin:0 0 16px;font-size:30px;line-height:38px;font-weight:700;letter-spacing:-0.8px;color:#182a25;" {
                                                "Confirm your email."
                                            }
                                            p style="margin:0 0 28px;font-size:15px;line-height:24px;color:#52665d;" {
                                                "One quick step before you get started. Enter this code in the app to finish creating your Compallm account."
                                            }
                                            table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background-color:#f0f6f1;border:1px solid #dce9de;border-radius:12px;" {
                                                tr {
                                                    td align="center" style="padding:22px 12px;" {
                                                        p style="margin:0 0 12px;font-size:12px;line-height:18px;color:#52665d;" { "Your verification code" }
                                                        p dir="ltr" aria-label="Your verification code" style="margin:0;font-family:'Courier New',Courier,monospace;font-size:38px;line-height:48px;font-weight:700;letter-spacing:6px;color:#183c30;white-space:nowrap;" {
                                                            (self.code)
                                                        }
                                                        p style="margin:12px 0 0;font-size:12px;line-height:18px;color:#52665d;" {
                                                            "Expires in " strong style="color:#183c30;" { (expires_in) }
                                                        }
                                                    }
                                                }
                                            }
                                            p style="margin:24px 0 0;font-size:13px;line-height:21px;color:#52665d;" {
                                                strong style="color:#183c30;" { "Keep this code private." }
                                                " Compallm will never ask you to send it by email or chat."
                                            }
                                        }
                                    }
                                    tr {
                                        td style="padding:22px 8px 0;font-size:12px;line-height:20px;color:#687a70;" {
                                            p style="margin:0 0 8px;" { "Didn't request this? You can safely ignore this email." }
                                            p style="margin:0;" { "Compallm · Your organization workspace" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The standard `ToString` implementation renders HTML.
impl fmt::Display for OtpCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.render().into_string())
    }
}
