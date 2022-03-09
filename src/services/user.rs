use crate::{
    models::user::{CreateUserDto, User},
    repositories::user,
    utils::{error::CustomError, validator::Validator},
};
use argon2::Config;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use sqlx::PgPool;

pub async fn register(data: CreateUserDto, db: &PgPool) -> Result<User, CustomError> {
    data.validate()?;

    let config = Config::default();

    let salt: Vec<u8> = OsRng.sample_iter(Alphanumeric).take(10).collect();

    let hash = argon2::hash_encoded(data.password.as_bytes(), &salt, &config)?;

    let new_user = User {
        id: None,
        username: data.username,
        email: data.email,
        hashed_password: hash,
    };

    Ok(user::save(&new_user, db).await?)
}
