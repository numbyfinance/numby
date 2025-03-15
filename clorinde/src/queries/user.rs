// This file was generated with `clorinde`. Do not modify.

#[derive(serde::Serialize, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
    pub created_at: crate::types::time::TimestampTz,
    pub updated_at: crate::types::time::TimestampTz,
    pub tombstone: bool,
}
pub struct UserBorrowed<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub name: &'a str,
    pub password: &'a str,
    pub created_at: crate::types::time::TimestampTz,
    pub updated_at: crate::types::time::TimestampTz,
    pub tombstone: bool,
}
impl<'a> From<UserBorrowed<'a>> for User {
    fn from(
        UserBorrowed {
            id,
            email,
            name,
            password,
            created_at,
            updated_at,
            tombstone,
        }: UserBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            email: email.into(),
            name: name.into(),
            password: password.into(),
            created_at,
            updated_at,
            tombstone,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct UserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> UserBorrowed,
    mapper: fn(UserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(UserBorrowed) -> R) -> UserQuery<'c, 'a, 's, C, R, N> {
        UserQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| (self.mapper)((self.extractor)(&row))))
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
            .into_stream();
        Ok(it)
    }
}
pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> &str,
    mapper: fn(&str) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
        StringQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| (self.mapper)((self.extractor)(&row))))
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
            .into_stream();
        Ok(it)
    }
}
pub fn get_user_by_email() -> GetUserByEmailStmt {
    GetUserByEmailStmt(crate::client::async_::Stmt::new(
        "SELECT * FROM users WHERE email = $1",
    ))
}
pub struct GetUserByEmailStmt(crate::client::async_::Stmt);
impl GetUserByEmailStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        email: &'a T1,
    ) -> UserQuery<'c, 'a, 's, C, User, 1> {
        UserQuery {
            client,
            params: [email],
            stmt: &mut self.0,
            extractor: |row| UserBorrowed {
                id: row.get(0),
                email: row.get(1),
                name: row.get(2),
                password: row.get(3),
                created_at: row.get(4),
                updated_at: row.get(5),
                tombstone: row.get(6),
            },
            mapper: |it| User::from(it),
        }
    }
}
pub fn get_user_by_id() -> GetUserByIdStmt {
    GetUserByIdStmt(crate::client::async_::Stmt::new(
        "SELECT * FROM users WHERE id = $1",
    ))
}
pub struct GetUserByIdStmt(crate::client::async_::Stmt);
impl GetUserByIdStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> UserQuery<'c, 'a, 's, C, User, 1> {
        UserQuery {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor: |row| UserBorrowed {
                id: row.get(0),
                email: row.get(1),
                name: row.get(2),
                password: row.get(3),
                created_at: row.get(4),
                updated_at: row.get(5),
                tombstone: row.get(6),
            },
            mapper: |it| User::from(it),
        }
    }
}
/// Gets all permissions from all groups a user is in.
pub fn get_permissions() -> GetPermissionsStmt {
    GetPermissionsStmt(crate::client::async_::Stmt::new(
        "SELECT DISTINCT permissions.name FROM users JOIN users_groups ON users.id = users_groups.user_id JOIN groups_permissions ON users_groups.group_id = groups_permissions.group_id JOIN permissions ON groups_permissions.permission_id = permissions.id WHERE users.id = $1",
    ))
}
pub struct GetPermissionsStmt(crate::client::async_::Stmt);
impl GetPermissionsStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it.into(),
        }
    }
}
