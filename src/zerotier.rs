use reqwest::{ Method, Response };
use salvo::http::body;
use serde::{ Serialize, Deserialize };
use tokio::time::sleep;

use crate::config::AppConfig;

#[derive(Serialize, Deserialize, Default)]
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
        method: Method,
        body: Option<serde_json::Value>
    ) -> Result<Response, reqwest::Error> {
        let url = format!("{}/{}", self.address, endpoint);

        reqwest::Client
            ::new()
            .request(method, &url)
            .header("X-ZT1-AUTH", self.auth_token.clone())
            .header("Content-Type", "application/json")
            .json(&body)
            .send().await
    }
}
