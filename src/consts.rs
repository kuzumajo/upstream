use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
  StudioLogo,
  Menu,
  Staff,
  InGame,
  LoadGame,
}

/// Game Data
#[derive(Serialize, Deserialize, Debug)]
pub struct GameSave {
  /// last modified time
  pub last_modified_time: SystemTime,
  /// save creating time
  pub created_time: SystemTime,
  /// simple counting
  pub total_playing_time: Duration,
  /// save name (seems useless)
  pub saving_name: String,
  /// unlocked hahou (?)
  pub unlocked_mahou: String,
  /// money
  pub money: u32,
  /// health
  pub health: u32,
  /// max health
  pub health_limit: u32,
  /// energy
  pub energy: u32,
  /// max energy
  pub energy_limit: u32,
}

// studio logo settings
pub const STUDIO_LOGO_WAITING_SECONDS: f32 = 5.0;

// control panel settings
pub const HEALTH_BAR_WIDTH: f32 = 300.0;
pub const ENERGY_BAR_WIDTH: f32 = 300.0;

// staff list settings
pub const STAFF_LIST_WAITING_SECONDS: f32 = 2.0;

pub const PLAYER_NAME: &str = "蓿";
