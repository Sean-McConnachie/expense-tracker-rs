pub mod category;
pub mod expense;
pub mod initialize;
pub mod user;

use sqlx::postgres;
use sqlx::ConnectOptions;
use std::str::FromStr;

pub async fn connect_db(db_conf: &DbConfig) -> Result<sqlx::PgPool, sqlx::Error> {
    let options = postgres::PgConnectOptions::from_str(db_conf.database_url())?
        .disable_statement_logging()
        .clone();
    let pool = postgres::PgPoolOptions::new()
        .max_connections(db_conf.max_connections)
        .connect_with(options)
        .await?;

    Ok(pool)
}

#[derive(serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DbConfig {
    database_url: String,
    max_connections: u32,
}

impl DbConfig {
    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }
}
