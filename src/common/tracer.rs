// I'm already Tracer

use super::session::AuthSession;

/// Format an AuthSession to apply to the span tags, should be used on every endpoint
pub fn fmt(session: &AuthSession) -> String {
    session
        .user
        .as_ref()
        .map_or("anonymous".to_string(), |u| u.to_string())
}
