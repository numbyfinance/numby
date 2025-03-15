use axum::{body::Body, http::Request};

use crate::{routes::auth, session::Credentials, tests::utils::TextContext};

#[tokio::test]
async fn test_login_success() {
    let context = TextContext::new(auth::router()).await;

    let credentials = Credentials {
        email: "topaz@ipc.org".to_string(),
        password: "topaz".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/login?next=/dashboard")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&credentials).unwrap()))
        .unwrap();

    let response = context.send_request(request).await;

    let has_redirect = context
        .assert_sse_contains(
            response,
            "datastar-execute-script",
            &format!("window.location = '/dashboard'"),
        )
        .await;

    assert!(
        has_redirect,
        "Expected redirect to /dashboard but didn't find it"
    );
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let context = TextContext::new(auth::router()).await;

    let credentials = Credentials {
        email: "jade@ipc.org".to_string(),
        password: "badpassword".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&credentials).unwrap()))
        .unwrap();

    let response = context.send_request(request).await;

    let has_error = context
        .assert_sse_contains(
            response,
            "datastar-merge-fragments",
            "Incorrect email or password.",
        )
        .await;

    assert!(has_error, "Couldn't find error message");
}
