//! src/config.rs
#[derive(serde::Deserialize)]
pub struct Settings {
    pub port: u16
}

pub fn get_settings() -> Settings {
    return Settings {
        port: 3000,
    }
}



