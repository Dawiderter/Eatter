use std::sync::Arc;

use axum::extract::FromRef;
use tower_cookies::Key;

use crate::{posts::Post, db::{self, Database}};

#[derive(Clone, FromRef)]
pub struct GlobalState {
    pub master_key: Key,
    pub database: Database,
}