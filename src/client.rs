use reqwest;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
use std::net::Ipv4Addr;
use std::time::Duration;
use serde_json::Value;

const AURORA_PORT: u32 = 16021;

pub struct Aurora {
    client: Client,
    base_url: String,
    auth_token: String,
}

#[derive(Deserialize)]
struct AddUserResponseBody {
    pub auth_token: String,
}

pub async fn generate_auth_token(
    addr: Ipv4Addr,
    port: Option<u32>,
) -> Result<String, reqwest::Error> {
    let port: u32 = port.unwrap_or(AURORA_PORT);
    let url = format!("http://{}:{}/api/v1/new", addr, port);
    let client = reqwest::Client::new();
    let response = client.post(&url).send().await?;

    assert_eq!(reqwest::StatusCode::OK, response.status());
    let add_user_body: AddUserResponseBody = response.json().await?;
    Ok(add_user_body.auth_token)
}

impl Aurora {
    pub fn new(addr: Ipv4Addr, port: Option<u32>, auth: String) -> Result<Aurora, Box<dyn Error>> {
        let port: u32 = port.unwrap_or(AURORA_PORT);
        Ok(Aurora {
            client: reqwest::blocking::Client::new(),
            base_url: format!("http://{}:{}/api/v1", addr, port),
            auth_token: auth,
        })
    }

    pub fn get_firmware_version(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/{}/", &self.base_url, &self.auth_token);
        let response = self.client.get(&url).send()?;

        assert_eq!(reqwest::StatusCode::OK, response.status());
        let response_body: serde_json::Value = response.json().unwrap();
        Ok(response_body["firmwareVersion"].as_str().unwrap().to_string())
    }

    pub fn set_brightness(&self, value: u16, duration: Option<Duration>) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/{}/state", &self.base_url, &self.auth_token);
        let duration = duration.unwrap_or(Duration::from_secs(0)).as_secs();
        let request_body = serde_json::json!({"brightness": {"value": value, "duration": duration}});
        let _response = self.client.put(&url).json(&request_body).send()?;

        Ok(())
    }

    pub fn get_effects(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let url = format!("{}/{}/effects/effectsList", &self.base_url, &self.auth_token);
        let response = self.client.get(&url).send()?;

        let mut effects: Vec<String> = vec![];
        if response.status() == reqwest::StatusCode::OK {
            let response_body: serde_json::Value = response.json().unwrap();
            for val in response_body.as_array().unwrap() {
                effects.push(val.as_str().unwrap().to_string());
            }
        }
        Ok(effects)
    }

    pub fn set_effect(&self, effect: String) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/{}/effects/select", &self.base_url, &self.auth_token);
        let request_body = serde_json::json!({"select": effect});
        let _response = self.client.put(&url).json(&request_body).send()?;

        Ok(())
    }
}
