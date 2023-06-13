use anyhow::anyhow;
use hyper::{Client, Request, Method, client::HttpConnector, Uri, header, Response, Body};
use crate::{auth::{LoginCredentials, CiscoToken, self}, device::{Device, self, DeviceWithIp, HostCount}};

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
        return match response_result {
            Ok(response) => {
                let token = auth::LoginRespone::try_from_async(response).await?;
                self.token = Some(token.response);
                Ok(())
            },
            Err(e) => {
                Err(anyhow!(e))
            }
        }
    }

    pub fn logout(&mut self) {
        self.token = None;
    }

    pub async fn get_all_devices(&self) -> anyhow::Result<Vec<Device>> {
        match &self.token {
            Some(t) => {
                let request_builder = Request::builder()
                    .method(Method::GET)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("X-Auth-Token", &t.service_ticket)
                    .uri(Uri::from_static("http://localhost:58000/api/v1/network-device"))
                    .body(String::new());
                let response_result = self.send_request(request_builder).await;
                return match response_result {
                    Ok(result) => {
                        Ok(device::AllDevicesResponse::try_from_async(result).await?.response)
                    },
                    Err(e) => {
                        Err(anyhow!(e))
                    }
                }
            },
            None => Err(anyhow!("Authentication required")),
        }
    }

    pub async fn get_device_by_ipv4(&self, ipv4_address: &String) -> anyhow::Result<DeviceWithIp> {
        match &self.token {
            Some(t) => {
                let request_builder = Request::builder()
                    .method(Method::GET)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("X-Auth-Token", &t.service_ticket)
                    .uri(format!("http://localhost:58000/api/v1/network-device/ip-address/{}", ipv4_address))
                    .body(String::new());
                let response_result = self.send_request(request_builder).await;
                return match response_result {
                    // TODO: check for 404 befor parsing
                    Ok(result) => Ok(DeviceWithIp::try_from_async(result).await?),
                    Err(e) => Err(anyhow!(e)),
                }
            },
            None => Err(anyhow!("GET_DEVICE_BY_IPV4: authentication required")),

        }
    }

    pub async fn get_all_clients(&self) -> anyhow::Result<Vec<Device>> {
        match &self.token {
            Some(t) => {
                let request_builder = Request::builder()
                    .method(Method::GET)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("X-Auth-Token", &t.service_ticket)
                    .uri(Uri::from_static("http://localhost:58000/api/v1/network-device?hostType=Pc"))
                    .body(String::new());
                let response_result = self.send_request(request_builder).await;
                return match response_result {
                    Ok(result) => {
                        Ok(device::AllDevicesResponse::try_from_async(result).await?.response)
                    },
                    Err(e) => {
                        Err(anyhow!(e))
                    }
                }
            },
            None => Err(anyhow!("Authentication required")),
        }
    }

    pub async fn get_host_count(&self) -> anyhow::Result<HostCount> {
        match &self.token {
            Some(t) => {
                let request_builder = Request::builder()
                    .method(Method::GET)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("X-Auth-Token", &t.service_ticket)
                    .uri(Uri::from_static("http://localhost:58000/api/v1/host/count?hostType=Pc"))
                    .body(String::new());
                let response_result = self.send_request(request_builder).await;
                return match response_result {
                    Ok(result) => Ok(HostCount::try_from_async(result).await?),
                    Err(e) => Err(anyhow!(e)),
                }
            },
            None => Err(anyhow!("GET_HOST_COUNT: authentication required")),
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
        let login = client.login( LoginCredentials {
            username: "test".into(),
            password: "test".into(),
        }).await;
        assert!(login.is_ok());
        let res = client.get_all_devices().await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn get_all_clients_with_login() {
        let mut client = CiscoClient::new();
        let login = client.login( LoginCredentials {
            username: "test".into(),
            password: "test".into(),
        }).await;
        assert!(login.is_ok());
        let res = client.get_all_devices().await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn get_device_by_ipv4_with_login() {
        let test_ip: String = String::from("192.168.1.1");
        let mut client = CiscoClient::new();
        let login = client.login( LoginCredentials {
            username: "test".into(),
            password: "test".into(),
        }).await;
        assert!(login.is_ok());
        let res = client.get_device_by_ipv4(&test_ip).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn get_host_count_with_login() {
        let mut client = CiscoClient::new();
        let login = client.login( LoginCredentials {
            username: "test".into(),
            password: "test".into(),
        }).await;
        assert!(login.is_ok());
        let res = client.get_host_count().await;
        assert!(res.is_ok());
    }
}
