#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
  /// studio logo page
  /// game entrance
  StudioLogo,
  /// game menu
  Menu,
  /// staff page
  Staff,
  /// gaming page
  /// WARNING: make sure `Res<GameSave>` exists
  InGame,
  /// load saves or start a new game
  LoadGame,
  /// game settings
  Settings,
  /// text input page
  /// designed for type save names only
  TextInput,
}

// studio logo settings
pub const STUDIO_LOGO_WAITING_SECONDS: f32 = 5.0;

// control panel settings
pub const HEALTH_BAR_WIDTH: f32 = 300.0;
pub const ENERGY_BAR_WIDTH: f32 = 300.0;

// staff list settings
pub const STAFF_LIST_WAITING_SECONDS: f32 = 2.0;

// settings
pub const RESOLUTION_LIST: [(u32, u32); 2] = [(1280, 720), (1920, 1080)];
pub const SLIDER_LENGTH: f32 = 500.0;

// game
pub const GAME_AUTOSAVE_INTERVAL: f32 = 60.0;

// crypto

/// Crypto key, but maybe change after release
pub const CRYPTO_KEY: &str = "flag{dHrkWcgkdohUvl4IywOGpPhlHz}";

pub const PLAYER_NAME: &str = "蓿";
