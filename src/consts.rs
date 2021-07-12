use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
  StudioLogo,
  Menu,
  Staff,
  InGame,
}

/// Game Data
#[derive(Serialize, Deserialize, Debug)]
pub struct GameSave {
  last_modified_time: SystemTime,
  created_time: SystemTime,
  total_playing_time: Duration,
  saving_name: String,
  unlocked_mahou: String,
  money: u32,
}

// studio logo settings
pub const STUDIO_LOGO_WAITING_SECONDS: f32 = 5.0;

// control panel settings
pub const HEALTH_BAR_WIDTH: f32 = 300.0;
pub const ENERGY_BAR_WIDTH: f32 = 300.0;

// staff list settings
pub const STAFF_LIST_WAITING_SECONDS: f32 = 2.0;

pub const PLAYER_NAME: &str = "蓿";
