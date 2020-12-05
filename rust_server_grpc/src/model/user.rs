#[derive(sqlx::FromRow, Clone, Debug)]
#[sqlx(rename = "users")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub person: Option<Person>,
}

#[derive(sqlx::FromRow, sqlx::Decode, Clone, Debug)]
#[sqlx(rename = "people")]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}
