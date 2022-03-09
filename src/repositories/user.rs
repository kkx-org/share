use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_by_id<'a>(id: Uuid, db: &PgPool) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&id)
        .fetch_one(&*db)
        .await
}

pub async fn get_by_username<'a>(username: String, db: &PgPool) -> Result<User, sqlx::Error> {
    let username = username.to_lowercase();
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&username)
        .fetch_one(&*db)
        .await
}

pub async fn save(user: &User, db: &PgPool) -> Result<User, sqlx::Error> {
    match user.id {
        Some(id) => {
            sqlx::query_as(
                "UPDATE users SET username = $1, email = $2, hashed_password = $3 WHERE id = $4 RETURNING *",
            )
                .bind(&user.username.to_lowercase())
                .bind(&user.email)
                .bind(&user.hashed_password)
                .bind(id)
                .fetch_one(&*db)
            .await
        },
        None => {
            sqlx::query_as(
                "INSERT INTO users(username, email, hashed_password) VALUES ($1, $2, $3) RETURNING *",
            )
                .bind(&user.username.to_lowercase())
                .bind(&user.email)
                .bind(&user.hashed_password)
                .fetch_one(&*db)
            .await
        }
    }
}
