use sqlx::{Pool, Postgres};

use crate::config::Config;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: Config,
}
