use anyhow::anyhow;
use hyper::{Client, Request, Method, client::HttpConnector, Uri, body, header, Response, Body};
use crate::{auth::{LoginCredentials, CiscoToken, LoginRespone, self}, device::{AllDevicesResponse, Device, self}};

pub struct CiscoClient {
    client: Client<HttpConnector, String>,
    token: Option<CiscoToken>,
}


impl CiscoClient {
    pub fn new() -> CiscoClient {
        return CiscoClient {
            client: Client::builder()
                .build_http(),
            token: None,
        };
    }

    pub async fn login(&mut self, credentials: LoginCredentials) -> anyhow::Result<()> {
        let req_builder = Request::builder()
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .uri(Uri::from_static("http://localhost:58000/api/v1/ticket"))
            .body(serde_json::to_string(&credentials).unwrap_or("".into()));
        let response_result = self.send_request(req_builder).await;
        match response_result {
            Ok(response) => {
                let token = auth::LoginRespone::try_from_async(response).await?;
                println!("{:?}", token);
                return Ok(());
            },
            Err(e) => {
                return Err(anyhow!(e));
            }
        }
    }

    pub fn logout(&mut self) {
        self.token = None;
    }

    pub async fn get_all_devices(&self) -> anyhow::Result<Vec<Device>> {
        match self.token.clone() {
            Some(t) => {
                let request_builder = Request::builder()
                    .method(Method::GET)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("X-Auth-Token", t.service_ticket)
                    .uri(Uri::from_static("http://localhost:58000/api/v1/network-device"))
                    .body(String::new());
                let response_result = self.send_request(request_builder).await;
                match response_result {
                    Ok(result) => {
                        println!("Request: recived answer");
                        return Ok(device::AllDevicesResponse::try_from_async(result).await?.response);
                    },
                    Err(e) => {
                        println!("An Error: {}", e);
                        return Err(anyhow!(e));
                    }
                }
            },
            None => Err(anyhow!("Authentication required")),
        }
    }

    async fn send_request(&self, req: Result<Request<String>, hyper::http::Error>) -> anyhow::Result<Response<Body>> {
        if req.is_err() {
            return Err(anyhow!("Request is err"));
        }
        return match self.client.request(req.unwrap()).await {
            Ok(res) => Ok(res),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn loggin_with_correct_creds() {
        let mut client = CiscoClient::new();
        let login_result = client.login(LoginCredentials {
            username: "test".into(),
            password: "test".into(),
        }).await;
        assert!(login_result.is_ok());
    }

    #[tokio::test]
    async fn get_all_devices_with_login() {
        let mut client = CiscoClient::new();
        let _ = client.login( LoginCredentials {
            username: "test".into(),
            password: "test".into(),
        }).await;
        let res = client.get_all_devices().await;
        assert!(res.is_ok());
    }
}
