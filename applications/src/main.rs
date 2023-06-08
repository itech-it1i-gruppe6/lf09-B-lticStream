use std::collections::HashMap;

use anyhow::Result;
use reqwest;
use reqwest::header;
use serde_json;
use serde_json::Value;

use crate::cisco::AllDevicesResponse;
use crate::cisco::DeviceIp;
use crate::cisco::HostCount;
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
    println!("ticket = {}\n", &json.response.service_ticket);

    let devices: Value = client
        .get("http://localhost:58000/api/v1/network-device")
        .header("X-Auth-Token", &json.response.service_ticket)
        .send()
        .await?
        .json()
        .await?;
    let vec_devices: AllDevicesResponse = serde_json::from_value(devices).unwrap();

    println!("device = {:?}\n", &vec_devices.response[0]);

    let clients: Value = client
        .get("http://localhost:58000/api/v1/network-device?hostType=Pc")
        .header("X-Auth-Token", &json.response.service_ticket)
        //.header("hostType", "Pc")
        .send()
        .await?
        .json()
        .await?;
    let vec_clients: AllDevicesResponse = serde_json::from_value(clients).unwrap();

    println!("client = {:?}\n", &vec_clients.response);

    let deviceIp: Value = client
        .get("http://localhost:58000/api/v1/network-device/ip-address/192.168.1.1")
        .header("X-Auth-Token", &json.response.service_ticket)
        .send()
        .await?
        .json()
        .await?;
    let vec_deviceIp: DeviceIp = serde_json::from_value(deviceIp).unwrap();

    println!("deviceIp = {:?}\n", &vec_deviceIp);

    let hostCount: Value = client
        .get("http://localhost:58000/api/v1/host/count?hostType=Pc")
        .header("X-Auth-Token", &json.response.service_ticket)
        .send()
        .await?
        .json()
        .await?;
    let vec_host_count: HostCount = serde_json::from_value(hostCount).unwrap();

    println!("hostCount = {:?}\n", &vec_host_count.response);

    Ok(())
}
