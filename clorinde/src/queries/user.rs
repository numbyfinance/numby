// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub created_at: crate::types::time::TimestampTz,
    pub updated_at: crate::types::time::TimestampTz,
    pub is_active: bool,
}
pub struct UserBorrowed<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub name: &'a str,
    pub password_hash: &'a str,
    pub created_at: crate::types::time::TimestampTz,
    pub updated_at: crate::types::time::TimestampTz,
    pub is_active: bool,
}
impl<'a> From<UserBorrowed<'a>> for User {
    fn from(
        UserBorrowed {
            id,
            email,
            name,
            password_hash,
            created_at,
            updated_at,
            is_active,
        }: UserBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            email: email.into(),
            name: name.into(),
            password_hash: password_hash.into(),
            created_at,
            updated_at,
            is_active,
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
pub fn select_all_users() -> SelectAllUsersStmt {
    SelectAllUsersStmt(crate::client::async_::Stmt::new("select * from users"))
}
pub struct SelectAllUsersStmt(crate::client::async_::Stmt);
impl SelectAllUsersStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> UserQuery<'c, 'a, 's, C, User, 0> {
        UserQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor: |row| UserBorrowed {
                id: row.get(0),
                email: row.get(1),
                name: row.get(2),
                password_hash: row.get(3),
                created_at: row.get(4),
                updated_at: row.get(5),
                is_active: row.get(6),
            },
            mapper: |it| User::from(it),
        }
    }
}
