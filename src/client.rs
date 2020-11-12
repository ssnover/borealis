use crate::nanoleaf_json_messages::*;
use reqwest;
use std::error::Error;
use std::net::Ipv4Addr;
use std::time::Duration;

const AURORA_PORT: u32 = 16021;

/// Data required for the Aurora client to make API requests.
pub struct Aurora {
    client: reqwest::Client,
    base_url: String,
    auth_token: String,
}

/// Requests a new authorization token from the Nanoleaf Aurora gateway.
/// This function will fail if the user has not used the Aurora's power
/// button to indicate that they have physical access to the device.
/// See the Nanoleaf API documentation for more information: https://forum.nanoleaf.me/docs/openapi
///
/// # Arguments
/// * `addr` - Local IP address of your Nanoleaf Aurora gateway.
/// * `port` - The port the Aurora is listening on, defaults to `16021` if `None` is passed.
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
    /// Constructs a new Nanoleaf Aurora client.
    ///
    /// # Arguments
    /// * `addr` - Local IP address of your Nanoleaf Aurora gateway.
    /// * `port` - The port the Aurora is listening on, defaults to `16021` if `None` is passed.
    /// * `auth` - The authorization token required for calling API methods.
    pub fn new(addr: Ipv4Addr, port: Option<u32>, auth: &String) -> Aurora {
        let port: u32 = port.unwrap_or(AURORA_PORT);
        Aurora {
            client: reqwest::Client::new(),
            base_url: format!("http://{}:{}/api/v1", addr, port),
            auth_token: auth.clone(),
        }
    }

    /// Get the firmware revision of the Nanoleaf Aurora.
    pub async fn get_firmware_version(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/{}/", &self.base_url, &self.auth_token);
        let response = self.client.get(&url).send().await?;

        assert_eq!(reqwest::StatusCode::OK, response.status());
        let response_body: serde_json::Value = response.json().await.unwrap();
        Ok(response_body["firmwareVersion"].to_string())
    }

    /// Set the state of the light panels.
    /// # Arguments
    /// * `on` - If `true`, turns the panels on. If `false`, turns them off.
    async fn turn_on_off(&self, on: bool) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/{}/state/on", &self.base_url, &self.auth_token);
        let request_body = serde_json::json!(OnOffBody { value: on });
        self.client.put(&url).json(&request_body).send().await?;
        Ok(())
    }

    /// Turn on the Nanoleaf Aurora light panels.
    pub async fn turn_on(&self) -> Result<(), Box<dyn Error>> {
        self.turn_on_off(true).await
    }

    /// Turn off the Nanoleaf Aurora light panels.
    pub async fn turn_off(&self) -> Result<(), Box<dyn Error>> {
        self.turn_on_off(false).await
    }

    /// Get the current brightness value of the Aurora light panels. The maximum
    /// value is `100` and the minimum is `0`.
    pub async fn get_brightness(&self) -> Result<u16, Box<dyn Error>> {
        let url = format!("{}/{}/state/brightness", &self.base_url, &self.auth_token);
        let response = self.client.get(&url).send().await?;
        if response.status() == reqwest::StatusCode::OK {
            let get_brightness_body: GetBrightnessBody = response.json().await?;
            Ok(get_brightness_body.value)
        } else {
            unimplemented!();
        }
    }

    /// Set the brightness of the Aurora light panels.
    ///
    /// # Arguments
    /// * `value` - The brightness value to set. Max is `100`.
    /// * `duration` - The amount of time over which to transition to the new brightness.
    pub async fn set_brightness(
        &self,
        value: u16,
        duration: Option<Duration>,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/{}/state", &self.base_url, &self.auth_token);
        let duration = if let Some(duration) = duration {
            // TODO Should just error and tell the user if they passed too big of a number
            Some(duration.as_secs() as u16)
        } else {
            None
        };

        let request_body = serde_json::json!(SetBrightnessBody {
            brightness: SetBrightnessBodySubArgs {
                value: value,
                duration: duration
            }
        });
        self.client.put(&url).json(&request_body).send().await?;
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
            let response_body: serde_json::Value = response.json().await?;
            for val in response_body.as_array().unwrap() {
                effects.push(val.as_str().unwrap().to_string());
            }
        }
        Ok(effects)
    }

    pub async fn set_effect(&self, effect: &String) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/{}/effects/select", &self.base_url, &self.auth_token);
        let request_body = serde_json::json!(SelectEffect {
            select: effect.clone()
        });
        self.client.put(&url).json(&request_body).send().await?;

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

    /// Causes the panels to flash in unison. This is typically used to help users
    /// differentiate between multiple panels.
    pub async fn identify(&self) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/{}/identify", &self.base_url, &self.auth_token);
        self.client.put(&url).send().await?;
        Ok(())
    }
}
