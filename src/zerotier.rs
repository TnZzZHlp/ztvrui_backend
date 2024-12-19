use reqwest::{ Method, Response };
use serde::{ Serialize, Deserialize };

use crate::config::AppConfig;

#[derive(Serialize, Deserialize)]
pub struct ZeroTier {
    pub auth_token: String,
    pub address: String,
}

impl ZeroTier {
    pub fn new(config: &AppConfig) -> Self {
        ZeroTier {
            auth_token: config.zerotier.auth_token.clone(),
            address: config.zerotier.address.clone(),
        }
    }

    // Forward a request to the ZeroTier API
    pub async fn forward(
        &self,
        endpoint: &str,
        method: Method
    ) -> Result<Response, reqwest::Error> {
        let url = format!("{}/{}", self.address, endpoint);

        reqwest::Client
            ::new()
            .request(method, &url)
            .header("X-ZT1-AUTH", self.auth_token.clone())
            .send().await
    }
}