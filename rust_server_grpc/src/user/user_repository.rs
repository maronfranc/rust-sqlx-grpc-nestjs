use crate::model;
use crate::model::user::Person;
use sqlx::PgPool;

pub struct UserRepository {}
impl UserRepository {
    pub async fn find_many(pool: &PgPool) -> Result<Vec<model::User>, sqlx::Error> {
        sqlx::query_as!(model::User, r#"SELECT users.id, users.email, users.username, (people.id,people.first_name, people.last_name) as "person: Person" FROM users INNER JOIN people ON users.id_person = people.id"#)
            .fetch_all(pool)
            .await
    }

    pub async fn find_one(pool: &PgPool, id_user: i32) -> Result<model::User, sqlx::Error> {
        sqlx::query_as!(
            model::User,
            r#"SELECT users.id, users.email, users.username, (people.id,people.first_name, people.last_name) as "person: Person" FROM users INNER JOIN people ON users.id_person = people.id where users.id = $1"#,
            id_user
        )
        .fetch_one(pool)
        .await
    }
}
