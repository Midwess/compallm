//! Select a credential policy for one bounded GraphQL operation before execution.
use axum::{
    Extension, Json,
    extract::{FromRequest, Request, State},
    http::{HeaderValue, StatusCode, header::CACHE_CONTROL},
    middleware::Next,
    response::{IntoResponse, Response},
};
use juniper::{Definition, OperationType, Selection, http::GraphQLRequest};

use super::ApplicationSchema;
use crate::middleware::authentication::{Authentication, CredentialPolicy};

pub async fn authenticate(
    State(schema): State<ApplicationSchema>,
    Extension(authentication): Extension<Authentication>,
    request: Request,
    next: Next,
) -> Response {
    let (parts, body) = request.into_parts();
    let request =
        match Json::<GraphQLRequest>::from_request(Request::from_parts(parts.clone(), body), &())
            .await
        {
            Ok(Json(request)) => request,
            Err(error) => {
                let status = match error.status() {
                    status @ (StatusCode::PAYLOAD_TOO_LARGE
                    | StatusCode::UNSUPPORTED_MEDIA_TYPE) => status,
                    _ => StatusCode::BAD_REQUEST,
                };
                return failure(status, "Invalid GraphQL request");
            }
        };
    let policy = match policy(&request, &schema) {
        Ok(policy) => policy,
        Err(()) => {
            return failure(
                StatusCode::BAD_REQUEST,
                "Invalid operation or combined authentication mutations",
            );
        }
    };
    let mut http = Request::from_parts(parts, axum::body::Body::empty());
    http.extensions_mut().insert(request);
    authentication.run_policy(http, next, policy).await
}

fn policy(request: &GraphQLRequest, schema: &ApplicationSchema) -> Result<CredentialPolicy, ()> {
    let document =
        juniper::parser::parse_document_source(&request.query, &schema.schema).map_err(|_| ())?;
    let operations = document
        .iter()
        .filter_map(|definition| match definition {
            Definition::Operation(operation) => Some(&operation.item),
            _ => None,
        })
        .collect::<Vec<_>>();
    let selected = if let Some(name) = &request.operation_name {
        let matches = operations
            .iter()
            .filter(|op| op.name.as_ref().is_some_and(|value| value.item == name))
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            return Err(());
        }
        *matches[0]
    } else {
        if operations.len() != 1 {
            return Err(());
        }
        operations[0]
    };
    if selected.operation_type == OperationType::Query {
        return Ok(CredentialPolicy::User);
    }
    if selected.operation_type != OperationType::Mutation {
        return Err(());
    }
    let mut fields = Vec::new();
    root_fields(
        &selected.selection_set,
        &document,
        &mut Vec::new(),
        &mut fields,
    )?;
    match fields.as_slice() {
        ["signupEmailRequest" | "signinEmailRequest"] => Ok(CredentialPolicy::Public),
        ["signupEmailConfirm" | "signinEmailConfirm"] => Ok(CredentialPolicy::EmailOtp),
        ["sessionRefresh"] => Ok(CredentialPolicy::Refresh),
        ["sessionRevoke"] => Ok(CredentialPolicy::User),
        _ => Err(()),
    }
}

fn root_fields<'a>(
    selections: &[Selection<'a>],
    document: &[Definition<'a, juniper::DefaultScalarValue>],
    visiting: &mut Vec<&'a str>,
    fields: &mut Vec<&'a str>,
) -> Result<(), ()> {
    for selection in selections {
        match selection {
            Selection::Field(field) => fields.push(field.item.name.item),
            Selection::InlineFragment(fragment) => {
                root_fields(&fragment.item.selection_set, document, visiting, fields)?
            }
            Selection::FragmentSpread(spread) => {
                let name = spread.item.name.item;
                if visiting.contains(&name) {
                    return Err(());
                }
                let fragment = document
                    .iter()
                    .find_map(|definition| match definition {
                        Definition::Fragment(fragment) if fragment.item.name.item == name => {
                            Some(&fragment.item)
                        }
                        _ => None,
                    })
                    .ok_or(())?;
                visiting.push(name);
                root_fields(&fragment.selection_set, document, visiting, fields)?;
                visiting.pop();
            }
        }
        if fields.len() > 1 {
            return Err(());
        }
    }
    Ok(())
}

fn failure(status: StatusCode, message: &'static str) -> Response {
    let mut response = (
        status,
        Json(
            serde_json::json!({"errors":[{"message":message,"extensions":{"code":"BAD_REQUEST"}}]}),
        ),
    )
        .into_response();
    response
        .headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}
