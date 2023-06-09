use std::collections::HashMap;

use anyhow::Ok;
use anyhow::Result;
use reqwest;
use reqwest::Client;
use serde_json;
use serde_json::Value;

use crate::cisco::AllDevicesResponse;
use crate::cisco::DeviceIp;
use crate::cisco::HostCount;
use crate::cisco::TicketResponse;
mod cisco;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let ticket = get_ticket(&client).await.unwrap();
    get_all_devices(&client, &ticket).await.unwrap();
    get_all_clients(&client, &ticket).await.unwrap();
    get_device_ip(&client, &ticket).await.unwrap();
    get_host_count(&client, &ticket).await.unwrap();
    Ok(())
}

async fn get_ticket(client: &Client) -> Result<String> {
    let mut map = HashMap::new();
    map.insert("password", "test");
    map.insert("username", "test");
    let res: Value = client
        .post("http://localhost:58000/api/v1/ticket")
        .json(&map)
        .send()
        .await?
        .json()
        .await?;
    let json: TicketResponse = serde_json::from_value(res)?;
    println!("ticket = {}\n", &json.response.service_ticket);

    Ok(json.response.service_ticket)
}

async fn get_all_devices(client: &Client, ticket: &String) -> Result<()> {
    let devices: Value = client
        .get("http://localhost:58000/api/v1/network-device")
        .header("X-Auth-Token", ticket)
        .send()
        .await?
        .json()
        .await?;
    let vec_devices: AllDevicesResponse = serde_json::from_value(devices).unwrap();

    println!("device = {:?}\n", &vec_devices.response[0]);

    Ok(())
}

async fn get_all_clients(client: &Client, ticket: &String) -> Result<()> {
    let clients: Value = client
        .get("http://localhost:58000/api/v1/network-device?hostType=Pc")
        .header("X-Auth-Token", ticket)
        //.header("hostType", "Pc")
        .send()
        .await?
        .json()
        .await?;
    let vec_clients: AllDevicesResponse = serde_json::from_value(clients).unwrap();

    println!("client = {:?}\n", &vec_clients.response);

    Ok(())
}

async fn get_device_ip(client: &Client, ticket: &String) -> Result<()> {
    let device_ip: Value = client
        .get("http://localhost:58000/api/v1/network-device/ip-address/192.168.1.1")
        .header("X-Auth-Token", ticket)
        .send()
        .await?
        .json()
        .await?;
    let vec_device_ip: DeviceIp = serde_json::from_value(device_ip).unwrap();

    println!("deviceIp = {:?}\n", &vec_device_ip);

    Ok(())
}

async fn get_host_count(client: &Client, ticket: &String) -> Result<()> {
    let host_count: Value = client
        .get("http://localhost:58000/api/v1/host/count?hostType=Pc")
        .header("X-Auth-Token", ticket)
        .send()
        .await?
        .json()
        .await?;
    let vec_host_count: HostCount = serde_json::from_value(host_count).unwrap();

    println!("hostCount = {:?}\n", &vec_host_count.response);

    Ok(())
}
