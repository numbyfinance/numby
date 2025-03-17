use std::collections::HashSet;

use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use clorinde::queries::user::{User, get_permissions, get_user_by_email, get_user_by_id};
use password_auth::verify_password;
use tokio::task;

#[derive(Clone)]
pub struct AuthenticatedUser(User);

impl AuthUser for AuthenticatedUser {
    type Id = uuid::Uuid;

    fn id(&self) -> Self::Id {
        self.0.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.0.password.as_bytes()
    }
}

impl std::fmt::Debug for AuthenticatedUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.0.id)
            .field("email", &self.0.email)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl std::fmt::Display for AuthenticatedUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User(id={}, email={})", self.0.id, self.0.email)
    }
}

impl From<User> for AuthenticatedUser {
    fn from(user: User) -> Self {
        AuthenticatedUser(user)
    }
}

impl From<AuthenticatedUser> for User {
    fn from(auth_user: AuthenticatedUser) -> Self {
        auth_user.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Permission {
    pub name: String,
}

impl From<String> for Permission {
    fn from(name: String) -> Self {
        Permission { name }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Backend {
    pool: clorinde::deadpool_postgres::Pool,
}

impl Backend {
    pub fn new(pool: clorinde::deadpool_postgres::Pool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Clorinde(#[from] clorinde::tokio_postgres::Error),
    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

impl AuthnBackend for Backend {
    type User = AuthenticatedUser;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let client = self.pool.get().await.unwrap();

        let user = match get_user_by_email().bind(&client, &creds.email).one().await {
            Ok(user) => user,
            Err(_) => return Ok(None),
        };

        task::spawn_blocking(|| match verify_password(creds.password, &user.password) {
            Ok(_) => Ok(Some(AuthenticatedUser::from(user))),
            Err(_) => Ok(None),
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let client = self.pool.get().await.unwrap();
        let user = get_user_by_id().bind(&client, user_id).one().await?;

        Ok(Some(AuthenticatedUser::from(user)))
    }
}

impl AuthzBackend for Backend {
    type Permission = Permission;

    async fn get_group_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        let client = self.pool.get().await.unwrap();
        let permissions = get_permissions().bind(&client, &user.0.id).all().await?;

        Ok(permissions.into_iter().map(Permission::from).collect())
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;
