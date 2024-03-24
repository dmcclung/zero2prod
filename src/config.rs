//! src/config.rs

use std::env;

#[derive(Clone, Debug)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub default_sender: String,
}

impl SmtpConfig {
    pub fn new(
        host: String,
        port: String,
        user: String,
        password: String,
        default_sender: String,
    ) -> Self {
        Self {
            host,
            port: port.parse::<u16>().unwrap(),
            user,
            password,
            default_sender,
        }
    }

    pub fn parse_from_env() -> Self {
        dotenv::dotenv().ok();

        let host = env::var("EMAIL_HOST").unwrap();
        let port = env::var("EMAIL_PORT").unwrap();
        let user = env::var("EMAIL_USER").unwrap();
        let password = env::var("EMAIL_PASSWORD").unwrap();
        let default_sender = env::var("EMAIL_DEFAULT_SENDER").unwrap();

        Self {
            host,
            port: port.parse::<u16>().unwrap(),
            user,
            password,
            default_sender,
        }
    }
}

// #[derive(serde::Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

pub struct Config {
    pub port: u16,
    pub db_config: DatabaseConfig,
    pub smtp_config: SmtpConfig,
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

        let smtp_config = SmtpConfig::parse_from_env();

        Config {
            port: 3000,
            db_config,
            smtp_config,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use fake::faker::internet::en::{DomainSuffix, Password, SafeEmail, Username};
    use fake::faker::number::en::NumberWithFormat;
    use fake::Fake;
    use std::env::{remove_var, set_var};

    use crate::config::SmtpConfig;

    #[test]
    fn smtp_config_from_env() {
        let hostname = generate_hostname();
        let username: String = Username().fake();
        let port: String = NumberWithFormat("###").fake();
        let password: String = Password(8..16).fake();
        let default_sender: String = SafeEmail().fake();

        set_var("EMAIL_HOST", hostname.clone());
        set_var("EMAIL_USER", username.clone());
        set_var("EMAIL_PORT", port.clone());
        set_var("EMAIL_PASSWORD", password.clone());
        set_var("EMAIL_DEFAULT_SENDER", default_sender.clone());

        let smtp_config = SmtpConfig::parse_from_env();

        remove_var("EMAIL_HOST");
        remove_var("EMAIL_USER");
        remove_var("EMAIL_PORT");
        remove_var("EMAIL_PASSWORD");
        remove_var("EMAIL_DEFAULT_SENDER");

        assert_eq!(hostname, smtp_config.host);
        assert_eq!(username, smtp_config.user);
        assert_eq!(port.parse::<u16>().unwrap(), smtp_config.port);
        assert_eq!(password, smtp_config.password);
        assert_eq!(default_sender, smtp_config.default_sender);
    }

    fn generate_hostname() -> String {
        let domain: String = Username().fake();
        let domain_suffix: String = DomainSuffix().fake();
        format!("smtp.{}.{}", domain, domain_suffix)
    }
}
