use dirs::home_dir;
use reqwest;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Write;
use std::net::Ipv4Addr;
use std::time::Duration;

const AURORA_PORT: u32 = 16021;

pub struct Aurora {
    client: Client,
    base_url: String,
    auth_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(rename = "Address")]
    ip_addr: Ipv4Addr,
    #[serde(rename = "Token")]
    auth_token: String,
    #[serde(rename = "Friendly Name")]
    friendly_name: String,
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
            client: reqwest::Client::new(),
            base_url: format!("http://{}:{}/api/v1", addr, port),
            auth_token: auth,
        })
    }

    pub async fn get_firmware_version(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/{}/", &self.base_url, &self.auth_token);
        let response = self.client.get(&url).send().await?;

        assert_eq!(reqwest::StatusCode::OK, response.status());
        let response_body: serde_json::Value = response.json().await.unwrap();
        Ok(response_body["firmwareVersion"]
            .as_str()
            .unwrap()
            .to_string())
    }

    pub async fn set_brightness(
        &self,
        value: u16,
        duration: Option<Duration>,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/{}/state", &self.base_url, &self.auth_token);
        let duration = duration.unwrap_or(Duration::from_secs(0)).as_secs();
        let request_body =
            serde_json::json!({"brightness": {"value": value, "duration": duration}});
        let _response = self.client.put(&url).json(&request_body).send().await?;

        Ok(())
    }

    pub async fn get_effects(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let url = format!(
            "{}/{}/effects/effectsList",
            &self.base_url, &self.auth_token
        );
        let response = self.client.get(&url).send().await?;

        let mut effects: Vec<String> = vec![];
        if response.status() == reqwest::StatusCode::OK {
            let response_body: serde_json::Value = response.json().await.unwrap();
            for val in response_body.as_array().unwrap() {
                effects.push(val.as_str().unwrap().to_string());
            }
        }
        Ok(effects)
    }

    pub async fn set_effect(&self, effect: &String) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/{}/effects/select", &self.base_url, &self.auth_token);
        let request_body = serde_json::json!({ "select": effect });
        let _response = self.client.put(&url).json(&request_body).send().await?;

        Ok(())
    }

    pub async fn get_name(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/{}/", &self.base_url, &self.auth_token);
        let response = self.client.get(&url).send().await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let response_body: serde_json::Value = response.json().await.unwrap();
                Ok(response_body["name"].as_str().unwrap().to_string())
            }
            _ => Ok("".to_string()),
        }
    }
}

impl ConfigFile {
    pub fn new(ip_addr: Ipv4Addr, token: String, name: String) -> ConfigFile {
        ConfigFile {
            ip_addr,
            auth_token: token,
            friendly_name: name,
        }
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let mut config_path = home_dir().unwrap();
        config_path.push(".borealis");
        let mut config_file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(config_path)?;
        let content: String = serde_json::to_string(&self)?;
        config_file.write_all(content.as_bytes())?;
        Ok(())
    }
}
