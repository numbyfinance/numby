use std::sync::Arc;

use axum::{
    Extension,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use clorinde::queries::user::{User, get_user_from_api_token};
use juniper::{Context, EmptyMutation, EmptySubscription, RootNode};
use juniper_axum::{extract::JuniperRequest, response::JuniperResponse};

use crate::AppState;

pub mod object;
pub mod query;

#[allow(dead_code)]
pub struct GraphQLContext {
    pub state: AppState,
    pub user: User,
}

impl Context for GraphQLContext {}

pub type Schema = RootNode<
    'static,
    query::Query,
    EmptyMutation<GraphQLContext>,
    EmptySubscription<GraphQLContext>,
>;

#[derive(Clone, Debug)]
pub struct ApiUser(User);

pub async fn route(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(state): Extension<AppState>,
    Extension(user): Extension<ApiUser>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    let context = GraphQLContext {
        state,
        user: user.0,
    };

    JuniperResponse(request.execute(&*schema, &context).await)
}

pub async fn middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let auth_header = match req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
    {
        Some(header) => header,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(token) => token,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let client = state.pool.get().await.unwrap();

    match get_user_from_api_token()
        .bind(&client, &token)
        .opt()
        .await
        .unwrap()
    {
        Some(user) => {
            let mut req = req;
            req.extensions_mut().insert(ApiUser(user));
            let response = next.run(req).await;
            response
        }
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}
