//! Axum adapter for GraphQL execution; routing belongs to the application.
use crate::graphql::{ApplicationSchema, GraphqlContext};
use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use juniper::http::GraphQLBatchResponse;
pub use juniper::http::GraphQLRequest;
use juniper_axum::response::JuniperResponse;

pub async fn handle(
    schema: ApplicationSchema,
    context: GraphqlContext,
    request: Result<Json<GraphQLRequest>, JsonRejection>,
) -> Response {
    let request = match request {
        Ok(Json(request)) => request,
        Err(error) => {
            let status = match error.status() {
                status @ (StatusCode::PAYLOAD_TOO_LARGE | StatusCode::UNSUPPORTED_MEDIA_TYPE) => {
                    status
                }
                _ => StatusCode::BAD_REQUEST,
            };
            return failure(status, "Invalid GraphQL request");
        }
    };
    if super::operation::validate(&request, &schema).is_err() {
        return failure(
            StatusCode::BAD_REQUEST,
            "Invalid operation or combined mutations",
        );
    }
    JuniperResponse(GraphQLBatchResponse::Single(
        request.execute(schema.as_ref(), &context).await,
    ))
    .into_response()
}

fn failure(status: StatusCode, message: &'static str) -> Response {
    (
        status,
        Json(
            serde_json::json!({"errors":[{"message":message,"extensions":{"code":"BAD_REQUEST"}}]}),
        ),
    )
        .into_response()
}
