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
