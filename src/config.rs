//! src/config.rs
#[derive(serde::Deserialize)]

pub struct DatabaseConfig {
    pub url: String,
}

pub struct Config {
    pub port: u16,
    pub db_config: DatabaseConfig
}

impl Config {
    pub fn new() -> Config {
        let url = "postgres://admin:admin@localhost:5432/newsletter".to_string();

        let db_config = DatabaseConfig {
            url
        };
        
        return Config {
            port: 3000,
            db_config
        }
    }
}



