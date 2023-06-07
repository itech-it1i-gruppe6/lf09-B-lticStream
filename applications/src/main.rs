use std::collections::HashMap;

use anyhow::Result;
use reqwest;
use serde_json;
use serde_json::Value;

use crate::cisco::TicketResponse;
mod cisco;

#[tokio::main]
async fn main() -> Result<()> {
    let mut map = HashMap::new();
    map.insert("password", "test");
    map.insert("username", "test");
    let client = reqwest::Client::new();
    let res: Value = client
        .post("http://localhost:58000/api/v1/ticket")
        .json(&map)
        .send()
        .await?
        .json()
        .await?;
    let json: TicketResponse = serde_json::from_value(res)?;
    println!("ticket = {}", &json.response.service_ticket);

    let devices: Value = client
        .get("http://localhost:58000/api/v1/network-device")
        .header("X-Auth-Token", &json.response.service_ticket)
        .send()
        .await?
        .json()
        .await?;

    println!("devices = {:?}", devices);

    Ok(())
}
