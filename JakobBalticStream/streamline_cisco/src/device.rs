use anyhow::anyhow;
use hyper::{Response, Body, body};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AllDevicesResponse {
    pub response: Vec<Device>,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub collection_status: String,
    pub error_description: String,
    pub global_credential_id: String,
    pub id: String,
    pub interface_count: String,
    pub inventory_status_detail: String,
    pub last_update_time: String,
    pub last_updated: String,
    pub mac_address: String,
    pub management_ip_address: String,
    pub platform_id: String,
    pub product_id: String,
    pub reachability_failure_reason: String,
    pub reachability_status: String,
    pub serial_number: String,
    #[serde(alias = "type")]
    pub typ: String,
    pub up_time: String,
}

impl AllDevicesResponse {
    pub async fn try_from_async(value: Response<Body>) -> anyhow::Result<Self> {
        let buffer = body::to_bytes(value.into_body()).await?;
        let token_result: Result<AllDevicesResponse, serde_json::Error> = serde_json::from_slice(&buffer);
        return match token_result {
            Err(e) => Err(anyhow!(e)),
            Ok(token) => Ok(token),
        }
    }
}
