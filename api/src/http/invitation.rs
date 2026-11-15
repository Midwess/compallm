use super::error::RequestError;
use crate::di::Di;
use axum::{
    extract::{Query, State},
    http::{
        HeaderMap,
        header::{CACHE_CONTROL, REFERRER_POLICY},
    },
    response::{IntoResponse, Redirect, Response},
};
use compallm_core::authentication::token::InvitationToken;

#[derive(Clone, serde::Deserialize)]
pub struct OpenInvitation {
    token: String,
}

pub async fn open(
    State(di): State<&'static Di>,
    axum::Extension(query): axum::Extension<OpenInvitation>,
) -> Response {
    let result = async {
        let claims =
            InvitationToken::try_from(di.auth_service().authenticate_token(&query.token).await?)?;
        di.auth_service().invitation_context(&claims).await?;
        let headers = di.invitation_settings().invitation_headers(
            query.token,
            claims
                .validity
                .expires_at
                .saturating_sub(compallm_utils::time::unix_timestamp()),
        )?;
        Ok::<HeaderMap, RequestError>(headers)
    }
    .await;
    let mut response = match result {
        Ok(headers) => (headers, Redirect::to("/invitation")).into_response(),
        Err(_) => Redirect::to("/invitation?unavailable=1").into_response(),
    };
    response
        .headers_mut()
        .insert(CACHE_CONTROL, "no-store".parse().unwrap());
    response
        .headers_mut()
        .insert(REFERRER_POLICY, "no-referrer".parse().unwrap());
    response
}

pub async fn redact_query(
    mut request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    if request.uri().path() == "/api/invitations/open" {
        let query = Query::<OpenInvitation>::try_from_uri(request.uri())
            .map(|Query(value)| value)
            .unwrap_or(OpenInvitation {
                token: String::new(),
            });
        request.extensions_mut().insert(query);
        *request.uri_mut() = "/api/invitations/open".parse().unwrap();
    }
    next.run(request).await
}
