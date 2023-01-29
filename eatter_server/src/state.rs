use argon2::Argon2;
use axum::extract::FromRef;
use sqlx::MySqlPool;

#[derive(Clone, FromRef)]
pub struct GlobalState {
    pub database: MySqlPool,
    pub hash_fn: Argon2<'static>,
}