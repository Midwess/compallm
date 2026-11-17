use compallm_api::di::Di;
use std::sync::Arc;
pub fn browser_router(di: &'static Di) -> axum::Router {
    use axum::{
        Extension, Json, Router,
        extract::State,
        middleware::{from_fn, from_fn_with_state},
        routing::post,
    };
    use compallm_api::{
        graphql::{ApplicationSchema, GraphqlContext, endpoint},
        http::{
            authentication::{AuthenticationContext, CookieDelivery, UserContext},
            authorization::AuthorizationContext,
            middleware::{AuthenticationMiddleware, AuthorizationMiddleware},
        },
    };
    Router::new().route(
        "/api/graphql",
        post(
            |State(schema): State<ApplicationSchema>,
             Extension(di): Extension<&'static Di>,
             Extension(user): Extension<UserContext>,
             Extension(authentication): Extension<Arc<AuthenticationContext>>,
             Extension(delivery): Extension<CookieDelivery>,
             Extension(authorization): Extension<AuthorizationContext>,
             request: Result<
                Json<endpoint::GraphQLRequest>,
                axum::extract::rejection::JsonRejection,
            >| async move {
                endpoint::handle(
                    schema,
                    GraphqlContext::new(di, user, authentication, delivery, authorization),
                    request,
                )
                .await
            },
        )
        .with_state(di.graphql_schema().clone())
        .layer(Extension(di))
        .route_layer(from_fn(AuthorizationMiddleware::handle))
        .route_layer(from_fn_with_state(
            di.authentication().clone(),
            AuthenticationMiddleware::handle,
        )),
    )
}

pub async fn execute(
    router: &axum::Router,
    bearer: &str,
    query: &str,
) -> anyhow::Result<serde_json::Value> {
    use tower::ServiceExt;
    let request = axum::http::Request::builder()
        .method("POST")
        .uri("/api/graphql")
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {bearer}"))
        .body(axum::body::Body::from(
            serde_json::json!({"query":query}).to_string(),
        ))?;
    let response = router.clone().oneshot(request).await?;
    Ok(serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), 65536).await?,
    )?)
}
