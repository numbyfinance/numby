use clorinde::queries::user::User as ClorindeUser;
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
}

impl From<ClorindeUser> for User {
    fn from(user: ClorindeUser) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
        }
    }
}
