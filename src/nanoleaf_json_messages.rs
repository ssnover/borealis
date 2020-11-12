#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AddUserResponseBody {
    pub auth_token: String,
}

#[derive(Serialize)]
pub struct SelectEffect {
    pub select: String,
}

#[derive(Deserialize, Serialize)]
pub struct OnOffBody {
    pub value: bool,
}

#[derive(Deserialize)]
pub struct PanelStateVariable {
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
pub struct GetAllPanelInfoResponseBody {
    pub name: String,
    pub serialNo: String,
    pub manufacturer: String,
    pub firmwareVersion: String,
    pub model: String,
    pub state: AuroraState,
    pub effects: AuroraEffects,
    pub panelLayout: AuroraPanelLayout,
    pub rhythm: RhythmState,
}

#[derive(Deserialize)]
pub struct AuroraState {
    pub on: OnOffBody,
    pub brightness: PanelStateVariable,
    pub hue: PanelStateVariable,
    pub sat: PanelStateVariable,
    pub ct: PanelStateVariable,
    pub colorMode: String,
}

#[derive(Deserialize)]
pub struct AuroraEffects {
    pub select: String,
    pub effectsList: Vec<String>
}

#[derive(Deserialize)]
pub struct AuroraPanelLayout {
    pub layout: Layout,
    pub globalOrientation: PanelStateVariable,
}

#[derive(Deserialize)]
pub struct Layout {
    pub numPanels: u8,
    pub sideLength: u16,
    pub positionData: Vec<PanelPositionInfo>,
}

#[derive(Deserialize)]
pub struct PanelPositionInfo {
    pub panelId: u16,
    #[serde(flatten)]
    pub position: Position,
    pub shapeType: u8, // TODO This should be an enum
}

#[derive(Deserialize)]
pub struct RhythmState {
    pub rhythmConnected: bool,
    pub rhythmActive: bool,
    pub rhythmId: u32,
    pub hardwareVersion: String,
    pub firmwareVersion: String,
    pub auxAvailable: bool,
    pub rhythmMode: u8, // TODO This should be an enum
    pub rhythmPos: Position,
}

#[derive(Deserialize)]
pub struct Position {
    pub x: i16,
    pub y: i16,
    pub o: i16,
}