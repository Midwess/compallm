//! Axum adapter for GraphQL execution; routing belongs to the application.
use crate::{
    di::Di,
    graphql::{ApplicationSchema, GraphqlContext},
    middleware::authentication::{CookieDelivery, RequestIdentity},
};
use axum::{
    Extension,
    extract::State,
    response::{IntoResponse, Response},
};
use juniper::http::{GraphQLBatchResponse, GraphQLRequest};
use juniper_axum::response::JuniperResponse;

pub async fn handle(
    State(schema): State<ApplicationSchema>,
    Extension(identity): Extension<RequestIdentity>,
    Extension(delivery): Extension<CookieDelivery>,
    Extension(di): Extension<&'static Di>,
    Extension(request): Extension<GraphQLRequest>,
) -> Response {
    let context = GraphqlContext::new(di, identity, delivery);
    JuniperResponse(GraphQLBatchResponse::Single(
        request.execute(schema.as_ref(), &context).await,
    ))
    .into_response()
}
