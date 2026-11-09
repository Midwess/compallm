//! Axum adapter for GraphQL execution; routing belongs to the application.
use crate::{graphql::ApplicationSchema, middleware::authentication::VerifiedAccess};
use async_graphql::{Error, ErrorExtensions};
use async_graphql_axum::GraphQLResponse;
use axum::{
    Extension, Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn handle(
    State(schema): State<ApplicationSchema>,
    Extension(identity): Extension<VerifiedAccess>,
    request: Result<Json<async_graphql::Request>, JsonRejection>,
) -> Response {
    match request {
        Ok(Json(request)) => {
            let result = schema.execute(request.data(identity)).await;
            let request_failed = !result.errors.is_empty()
                && result.errors.iter().all(|error| error.path.is_empty());
            let mut response = GraphQLResponse::from(result).into_response();
            if request_failed {
                *response.status_mut() = StatusCode::BAD_REQUEST;
            }
            response
        }
        Err(error) => {
            let status = match error.status() {
                status @ (StatusCode::PAYLOAD_TOO_LARGE | StatusCode::UNSUPPORTED_MEDIA_TYPE) => {
                    status
                }
                _ => StatusCode::BAD_REQUEST,
            };
            let message = match status {
                StatusCode::PAYLOAD_TOO_LARGE => "GraphQL request body is too large",
                StatusCode::UNSUPPORTED_MEDIA_TYPE => "Unsupported GraphQL request content type",
                _ => "Invalid GraphQL request",
            };
            error_response(status, "BAD_REQUEST", message)
        }
    }
}

fn error_response(status: StatusCode, code: &'static str, message: &'static str) -> Response {
    let error = Error::new(message).extend_with(|_, extensions| extensions.set("code", code));
    let result = async_graphql::Response::from_errors(vec![
        error.into_server_error(async_graphql::Pos::default()),
    ]);
    let mut response = GraphQLResponse::from(result).into_response();
    *response.status_mut() = status;
    response
}
