use crate::model;
use sqlx::PgPool;

pub struct UserRepository {}
impl UserRepository {
    pub async fn find_many(pool: &PgPool) -> Result<Vec<model::User>, sqlx::Error> {
        sqlx::query_as!(model::User, "SELECT id, email, username FROM users")
            .fetch_all(pool)
            .await
    }

    pub async fn find_one(pool: &PgPool, id_user: i32) -> Result<model::User, sqlx::Error> {
        sqlx::query_as!(
            model::User,
            "SELECT id, email, username FROM users WHERE id = $1",
            id_user
        )
        .fetch_one(pool)
        .await
    }
}
