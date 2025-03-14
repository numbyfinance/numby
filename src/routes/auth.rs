use axum::{
    Form, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
};
use maud::html;

use crate::{AppState, layout::Layout, session::AuthSession};

#[derive(Debug, serde::Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
}

mod get {
    use super::*;

    #[tracing::instrument]
    pub async fn login(session: AuthSession) -> impl IntoResponse {
        Layout::new(session).render(
            "Login",
            html! {
               div {
                   "hello"
               }
            },
        )
    }

    #[tracing::instrument]
    pub async fn logout(mut session: AuthSession) -> impl IntoResponse {
        match session.logout().await {
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
