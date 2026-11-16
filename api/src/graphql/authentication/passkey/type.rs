use compallm_core::authentication::{
    AuthError,
    passkey::{CeremonyStart, PasskeyRecord},
};
use juniper::ID;

pub struct PasskeySummary {
    record: PasskeyRecord,
}
impl PasskeySummary {
    pub(super) fn cursor_position(&self) -> Vec<Option<String>> {
        vec![
            Some(self.record.registered_at.to_string()),
            Some(self.record.id.to_string()),
        ]
    }
}
impl From<PasskeyRecord> for PasskeySummary {
    fn from(record: PasskeyRecord) -> Self {
        Self { record }
    }
}
#[juniper::graphql_object]
impl PasskeySummary {
    fn id(&self) -> ID {
        ID::new(self.record.id.to_string())
    }
    fn credential_id(&self) -> String {
        use base64::Engine;
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(self.record.credential.cred_id().as_ref())
    }
    fn name(&self) -> &str {
        &self.record.name
    }
    fn registered_at(&self) -> String {
        self.record.registered_at.to_string()
    }
    fn last_used_at(&self) -> Option<String> {
        self.record.last_used_at.map(|v| v.to_string())
    }
}
pub struct PasskeyCeremony {
    pub token: String,
    pub expires_at: String,
    pub options: PasskeyOptions,
}
pub struct PasskeyOptions {
    pub challenge: String,
    pub rp_id: String,
    pub rp_name: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub user_display_name: Option<String>,
    pub algorithms: Vec<i32>,
    pub credentials: Vec<PasskeyDescriptor>,
    pub timeout: i32,
}
pub struct PasskeyDescriptor {
    pub id: String,
    pub transports: Vec<String>,
}
#[juniper::graphql_object]
impl PasskeyDescriptor {
    fn id(&self) -> &str {
        &self.id
    }
    fn transports(&self) -> &[String] {
        &self.transports
    }
}
#[juniper::graphql_object]
impl PasskeyOptions {
    fn challenge(&self) -> &str {
        &self.challenge
    }
    fn rp_id(&self) -> &str {
        &self.rp_id
    }
    fn rp_name(&self) -> Option<&str> {
        self.rp_name.as_deref()
    }
    fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }
    fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    fn user_display_name(&self) -> Option<&str> {
        self.user_display_name.as_deref()
    }
    fn algorithms(&self) -> &[i32] {
        &self.algorithms
    }
    fn credentials(&self) -> &[PasskeyDescriptor] {
        &self.credentials
    }
    fn timeout(&self) -> i32 {
        self.timeout
    }
    fn user_verification(&self) -> &str {
        "required"
    }
}
#[juniper::graphql_object]
impl PasskeyCeremony {
    fn token(&self) -> &str {
        &self.token
    }
    fn expires_at(&self) -> &str {
        &self.expires_at
    }
    fn options(&self) -> &PasskeyOptions {
        &self.options
    }
}
impl TryFrom<CeremonyStart> for PasskeyCeremony {
    type Error = AuthError;
    fn try_from(c: CeremonyStart) -> Result<Self, AuthError> {
        let p = &c.options["publicKey"];
        let text = |v: &serde_json::Value| v.as_str().map(str::to_owned);
        let credentials = p
            .get("excludeCredentials")
            .or_else(|| p.get("allowCredentials"))
            .and_then(|v| v.as_array())
            .map(|items| {
                items
                    .iter()
                    .map(|d| PasskeyDescriptor {
                        id: text(&d["id"]).unwrap_or_default(),
                        transports: d["transports"]
                            .as_array()
                            .map(|a| a.iter().filter_map(text).collect())
                            .unwrap_or_default(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        Ok(Self {
            token: c.token,
            expires_at: c.expires_at.to_string(),
            options: PasskeyOptions {
                challenge: text(&p["challenge"]).unwrap_or_default(),
                rp_id: text(&p["rp"]["id"])
                    .or_else(|| text(&p["rpId"]))
                    .unwrap_or_default(),
                rp_name: text(&p["rp"]["name"]),
                user_id: text(&p["user"]["id"]),
                user_name: text(&p["user"]["name"]),
                user_display_name: text(&p["user"]["displayName"]),
                algorithms: p["pubKeyCredParams"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v["alg"].as_i64().and_then(|n| i32::try_from(n).ok()))
                            .collect()
                    })
                    .unwrap_or_default(),
                credentials,
                timeout: 300_000,
            },
        })
    }
}
