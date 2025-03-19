use juniper::graphql_object;

use super::{GraphQLContext, object::User};

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = GraphQLContext)]
impl Query {
    async fn me(#[graphql(context)] context: &GraphQLContext) -> User {
        User::from(context.user.clone())
    }
}
