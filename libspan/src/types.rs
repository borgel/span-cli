use thiserror::Error;
use serde::{Deserialize};

#[derive(Error, Debug)]
pub enum Error {
   #[error("Unknown libspan error")]
   Unknown,
   #[error("Error making request")]
   Request,
}

impl From<reqwest::Error> for Error {
   // TODO break down more specifically
    fn from(err: reqwest::Error) -> Error {
       Error::Request
    }
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct PanelState {
   pub branches: Vec<BranchState>,
   pub instant_grid_power: f32,
   #[serde(rename = "grid_connect_state")]
   pub main_relay_state: RelayState,
   #[serde(rename = "current_dsm_state")]
   pub device_state: DsmState,
   #[serde(rename = "current_grid_state")]
   pub grid_state: GridState,
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct BranchState {
   #[serde(rename = "id")]
   pub id: u8,
   #[serde(rename = "instant_power")]
   pub instant_power_w: f32,

   pub relay_state: RelayState,
   pub priority: Priority,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum Priority {
   #[serde(rename = "UNKNOWN")]
   Unknown,
   #[serde(rename = "NON_ESSENTIAL")]
   NonEssential,
   #[serde(rename = "LOW")]
   NiceToHave,
   #[serde(rename = "HIGH")]
   MustHave,
}
impl Default for Priority {
   fn default() -> Self {Priority::Unknown}
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum GridState {
   #[serde(rename = "GRID_STATE_UNKNOWN")]
   Unknown,
   #[serde(rename = "GRID_STATE_GRID_UP")]
   Up,
   #[serde(rename = "GRID_STATE_GRID_DOWN")]
   Down
}
impl Default for GridState {
   fn default() -> Self {GridState::Unknown}
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum DsmState {
   #[serde(rename = "DSM_UNKNOWN")]
   Unknown,
   #[serde(rename = "DSM_ON_GRID")]
   OnGrid,
   #[serde(rename = "DSM_ISLANDED")]
   Islanded,
   // TODO check if accurate
   #[serde(rename = "DSM_FAULTED_ON_GRID")]
   FaultOnGrid,
   #[serde(rename = "DSM_FAULTED_OFF_GRID")]
   FaultOffGrid,
}
impl Default for DsmState {
   fn default() -> Self {DsmState::Unknown}
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum RelayState {
   #[serde(rename = "relay-unknown")]
   Unknown,
   #[serde(rename = "relay-open")]
   Open,
   #[serde(rename = "relay-closed")]
   Closed,
}
impl Default for RelayState {
   fn default() -> Self {RelayState::Unknown}
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct InverterState {
   pub inverter_brand: String,
   pub inverter_model: String,

   pub battery_soe_pct: u8,
   pub battery_capacity_kwh: f32,
   pub battery_empty: bool,
   pub battery_full: bool,
}

/*
"battery_capacity_kwh": 9.31,
"battery_empty": false,
"battery_full": false,
"battery_power_request_w": 0,
"battery_soe_kwh": 8.565,
"battery_soe_pct": 92,
"inverter_brand": "LG Electronics Inc.",
"inverter_model": "D007KEEN261",
"time_latest_read": "2021-01-24T03:32:37.351936"
*/
