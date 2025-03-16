use async_stream::stream;
use axum::{
    Router,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use datastar::{
    Sse,
    prelude::{ExecuteScript, MergeFragments, ReadSignals},
};
use maud::html;

use crate::r#static::file;
use crate::{
    AppState,
    layout::Layout,
    session::{AuthSession, Credentials},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Meta {
    next: Option<String>,
}

mod post {
    use super::*;

    #[tracing::instrument]
    pub async fn login(
        mut session: AuthSession,
        meta: Query<Meta>,
        ReadSignals(creds): ReadSignals<Credentials>,
    ) -> impl IntoResponse {
        // NOTE: This can't all be wrapped in an Sse() because it needs to set a cookie
        let user = match session.authenticate(creds).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Sse(stream! {
                    yield MergeFragments::new(html! { p { "Incorrect email or password." } })
                        .selector("#error")
                        .merge_mode(datastar::prelude::FragmentMergeMode::Inner).into();
                })
                .into_response();
            }
            Err(e) => {
                tracing::error!("{}", e.to_string());
                return Sse(stream! {
                    yield MergeFragments::new(html! { p { "Unknown error (1), please try again later." } })
                        .selector("#error")
                        .merge_mode(datastar::prelude::FragmentMergeMode::Inner).into();
                }).into_response();
            }
        };

        match session.login(&user).await {
            Ok(_) => {
                let redirect_url = meta.next.as_deref().unwrap_or("/tp");
                let js_command = format!("window.location.assign('{}')", redirect_url);
                Sse(stream! {
                    yield ExecuteScript::new(js_command).into()
                })
                .into_response()
            }
            Err(e) => {
                tracing::error!("{}", e.to_string());
                Sse(stream! {
                    yield MergeFragments::new(html! { p { "Unknown error (2), please try again later." } })
                        .selector("#error")
                        .merge_mode(datastar::prelude::FragmentMergeMode::Inner).into()
                }).into_response()
            }
        }
    }
}

mod get {
    use super::*;

    #[tracing::instrument]
    pub async fn login(session: AuthSession, meta: Query<Meta>) -> impl IntoResponse {
        let next_param = meta
            .next
            .as_ref()
            .map_or("".to_string(), |next| format!("?next={}", next));

        let login_path = format!("@post('/login{}')", next_param);

        Layout::new(session).no_content().no_sidebar().render(
            "Login",
            html! {
               main class="bg-white dark:bg-gray-900" {
                   div class="relative flex" {
                       div class="w-full md:w-1/2" {
                           div class="min-h-[100dvh] flex h-screen" {
                               div class="max-w-sm m-auto w-full px-4 py-8" {
                                   div class="flex flex-col" {
                                       img class="align-center w-24 h-24" src=(file::numby_png) alt="Logo";
                                       h1 class="text-3xl text-gray-800 dark:text-gray-100 font-bold mb-6" { "Welcome back!" }
                                   }
                                   form {
                                       div class="space-y-4" {
                                           div {
                                               label class="block text-sm font-medium mb-1" for="email" { "Email Address" }
                                               input data-bind-email id="email" class="form-input w-full" type="email";
                                           }
                                           div {
                                               label class="block text-sm font-medium mb-1" for="password" { "Password" }
                                               input data-bind-password id="password" class="form-input w-full" type="password" autocomplete="on";
                                           }
                                       }

                                       div #error class="mt-2 text-sm text-red-400" {}

                                       div class="flex items-center justify-between mt-6" {
                                           div class="mr-1" {
                                               a class="text-sm underline hover:no-underline" href="reset-password.html" { "Forgot Password?" }
                                           }

                                           a
                                               data-on-click=(login_path)
                                               class="btn bg-gray-900 text-gray-100 hover:bg-gray-800 dark:bg-gray-100 dark:text-gray-800 dark:hover:bg-white ml-3 cursor-pointer"
                                               { "Sign In" }
                                       }
                                   }

                                   div class="pt-5 mt-6 border-t border-gray-100 dark:border-gray-700/60" {
                                       div class="text-sm" {
                                           "Don't have an account? "
                                           a class="font-medium text-violet-500 hover:text-violet-600 dark:hover:text-violet-400" href="/signup"
                                               { "Sign Up" }
                                       }
                                   }
                               }
                           }
                       }

                       div class="hidden md:block absolute top-0 bottom-0 right-0 md:w-1/2" aria-hidden="true" {
                           img class="object-contain object-center w-full h-full" src=(file::kafka_webp) alt="Authentication image";
                       }
                   }
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
