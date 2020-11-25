use sqlx::PgPool;

#[derive(sqlx::FromRow, Debug)]
pub struct UserRepository {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl UserRepository {
    pub async fn find_many(pool: &PgPool) -> Result<Vec<UserRepository>, sqlx::Error> {
        sqlx::query_as!(UserRepository, "SELECT id, email, username FROM users")
            .fetch_all(pool)
            .await
    }

    pub async fn find_one(pool: &PgPool, id_user: i32) -> Result<UserRepository, sqlx::Error> {
        sqlx::query_as!(
            UserRepository,
            "SELECT id, email, username FROM users WHERE id = $1",
            id_user
        )
        .fetch_one(pool)
        .await
    }
}
