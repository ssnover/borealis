use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AddUserResponseBody {
    pub auth_token: String,
}

#[derive(Serialize)]
pub struct SelectEffect {
    pub select: String,
}

#[derive(Serialize)]
pub struct OnOffBody {
    pub value: bool,
}

#[derive(Deserialize)]
pub struct GetBrightnessBody {
    pub value: u16,
    pub max: u16,
    pub min: u16,
}

#[derive(Serialize)]
pub struct SetBrightnessBody {
    pub brightness: SetBrightnessBodySubArgs,
}

#[derive(Serialize)]
pub struct SetBrightnessBodySubArgs {
    pub value: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u16>,
}

#[derive(Deserialize)]
pub struct GetEffectsListResponseBody {
    #[serde(flatten)]
    pub effects: Vec<String>,
}
