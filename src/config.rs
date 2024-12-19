use std::path::Path;
use serde::{ Serialize, Deserialize };

use crate::zerotier::ZeroTier;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    database_url: String,

    pub zerotier: ZeroTier,
}

impl AppConfig {
    pub fn init<T: AsRef<Path>>(path: T) -> Self {
        let conifg = std::fs::read_to_string(path).expect("Failed to read config file");

        serde_json::from_str(&conifg).expect("Failed to parse config file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let file = std::fs::File::create("test.json").unwrap();

        let config = AppConfig {
            database_url: "sqlite://test.db".to_string(),
            zerotier: ZeroTier {
                auth_token: "test_token".to_string(),
                address: "12312".to_string(),
            },
        };

        serde_json::to_writer(file, &config).unwrap();

        let config = AppConfig::init("test.json");

        assert_eq!(config.database_url, "sqlite://test.db");
        assert_eq!(config.zerotier.auth_token, "test_token");
        assert_eq!(config.zerotier.address, "12312");

        assert_ne!(config.database_url, "sqlite://test.db2");
        assert_ne!(config.zerotier.auth_token, "test_token2");
        assert_ne!(config.zerotier.address, "123122");

        std::fs::remove_file("test.json").unwrap();
    }
}
