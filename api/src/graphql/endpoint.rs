//! Axum adapter for GraphQL execution; routing belongs to the application.
use crate::{
    di::Di,
    graphql::{ApplicationSchema, GraphqlContext},
    middleware::authentication::VerifiedAccess,
};
use axum::{
    Extension, Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use juniper::http::{GraphQLBatchResponse, GraphQLRequest};
use juniper_axum::response::JuniperResponse;

pub async fn handle(
    State(schema): State<ApplicationSchema>,
    Extension(identity): Extension<VerifiedAccess>,
    Extension(di): Extension<&'static Di>,
    request: Result<Json<GraphQLRequest>, JsonRejection>,
) -> Response {
    match request {
        Ok(Json(request)) => {
            let context = GraphqlContext::new(di, identity);
            JuniperResponse(GraphQLBatchResponse::Single(
                request.execute(schema.as_ref(), &context).await,
            ))
            .into_response()
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
    (
        status,
        Json(serde_json::json!({
            "errors": [{ "message": message, "extensions": { "code": code } }]
        })),
    )
        .into_response()
}
