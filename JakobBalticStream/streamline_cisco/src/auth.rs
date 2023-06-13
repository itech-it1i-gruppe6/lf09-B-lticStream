use anyhow::anyhow;
use hyper::{Response, Body, body};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginRespone {
    pub response: CiscoToken,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CiscoToken {
    pub idle_timeout: usize,
    pub service_ticket: String,
    pub session_timeout: usize,
}

impl LoginRespone {
    pub async fn try_from_async(value: Response<Body>) -> anyhow::Result<Self> {
        let buffer = body::to_bytes(value.into_body()).await?;
        let token_result: Result<LoginRespone, serde_json::Error> = serde_json::from_slice(&buffer);
        return match token_result {
            Err(e) => Err(anyhow!(e)),
            Ok(token) => Ok(token),
        }
    }
}
