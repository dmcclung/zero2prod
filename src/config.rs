//! src/config.rs

use std::env;
#[derive(serde::Deserialize)]

pub struct DatabaseConfig {
    pub url: String,
}

pub struct Config {
    pub port: u16,
    pub db_config: DatabaseConfig,
}

impl Config {
    pub fn new() -> Config {
        dotenv::dotenv().ok();

        let psql_user = env::var("PSQL_USER").unwrap_or("admin".into());
        let psql_password = env::var("PSQL_PASSWORD").unwrap_or("admin".into());
        let psql_database = env::var("PSQL_DATABASE").unwrap_or("newsletter".into());
        let psql_host = env::var("PSQL_HOST").unwrap_or("localhost".into());
        let psql_port = env::var("PSQL_PORT").unwrap_or("5432".into());

        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            psql_user, psql_password, psql_host, psql_port, psql_database
        );

        let db_config = DatabaseConfig { url };

        Config {
            port: 3000,
            db_config,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
