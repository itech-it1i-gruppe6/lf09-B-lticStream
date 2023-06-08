use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TicketResponse {
    pub response: Ticket,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub idle_timeout: usize,
    pub service_ticket: String,
    pub session_timeout: usize,
}

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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceIp {
    pub collection_status: String,
    pub error_description: String,
    pub global_credential_id: String,
    pub id: String,
    pub interface_count: String,
    pub inventory_status_detail: String,
    pub ip_addresses: Vec<String>,
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HostCount {
    pub response: usize,
    pub version: String,
}
