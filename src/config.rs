use serde::{Deserialize, Serialize};
use dirs::home_dir;
use std::io::Write;
use std::net::Ipv4Addr;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(rename = "Address")]
    ip_addr: Ipv4Addr,
    #[serde(rename = "Token")]
    auth_token: String,
    #[serde(rename = "Friendly Name")]
    friendly_name: String,
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