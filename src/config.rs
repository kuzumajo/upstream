use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::window::WindowMode;
use serde::{Deserialize, Serialize};
use std::fs::{read, write};

#[derive(Serialize, Deserialize)]
pub struct KeyBindings {
  attack_1: MouseButton,
  attack_2: MouseButton,
  special_status_launch: KeyCode,
  assault: KeyCode,
  move_upwards: KeyCode,
  move_downwards: KeyCode,
  move_leftwards: KeyCode,
  move_rightwards: KeyCode,
  interact: KeyCode,
  open_handbook: KeyCode,
  open_plugins: KeyCode,
  start_mahou: KeyCode,
  item_1: KeyCode,
  item_2: KeyCode,
  item_3: KeyCode,
  item_4: KeyCode,
}

impl Default for KeyBindings {
  fn default() -> Self {
    KeyBindings {
      attack_1: MouseButton::Left,
      attack_2: MouseButton::Right,
      special_status_launch: KeyCode::Q,
      assault: KeyCode::Space,
      move_upwards: KeyCode::W,
      move_downwards: KeyCode::S,
      move_leftwards: KeyCode::A,
      move_rightwards: KeyCode::D,
      interact: KeyCode::E,
      open_handbook: KeyCode::H,
      open_plugins: KeyCode::P,
      start_mahou: KeyCode::LAlt,
      item_1: KeyCode::Key1,
      item_2: KeyCode::Key2,
      item_3: KeyCode::Key3,
      item_4: KeyCode::Key4,
    }
  }
}

/// Game config
#[derive(Serialize, Deserialize)]
pub struct GameConfig {
  volumn: f32,
  volumn_music: f32,
  volumn_sfx: f32,
  volumn_voice: f32,
  save_location: String,

  fullscreen: bool,
  resolution: (f32, f32),
  decorations: bool,

  attack_to_mouse: bool,
  assault_to_mouse: bool,
  mouse_sensitivity: f32,
  key_bindings: KeyBindings,
}

impl Default for GameConfig {
  fn default() -> Self {
    let save_dir = home::home_dir()
      .unwrap()
      .join(".kuzumajo")
      .join("upstream")
      .to_str()
      .unwrap()
      .to_string();

    GameConfig {
      volumn: 1.0,
      volumn_music: 1.0,
      volumn_sfx: 1.0,
      volumn_voice: 1.0,
      save_location: save_dir,

      fullscreen: false,
      resolution: (1280.0, 720.0),
      decorations: true,

      attack_to_mouse: true,
      assault_to_mouse: true,
      mouse_sensitivity: 1.0,
      key_bindings: KeyBindings::default(),
    }
  }
}

impl GameConfig {
  const CONFIG_FILE: &'static str = "display.cfg";

  /// load from disk or generate a default one
  pub fn load() -> GameConfig {
    if let Ok(data) = read(GameConfig::CONFIG_FILE) {
      if let Ok(config) = toml::from_slice::<GameConfig>(&data[..]) {
        return config;
      }
    }
    GameConfig::default()
  }

  /// save to disk
  pub fn save(&self) -> std::io::Result<()> {
    write(GameConfig::CONFIG_FILE, toml::to_vec(self).unwrap())
  }
}

impl GameConfig {
  pub fn get_window_descriptor(&self) -> WindowDescriptor {
    WindowDescriptor {
      width: self.resolution.0,
      height: self.resolution.1,
      resizable: true,
      title: "Upstream".to_string(),
      vsync: true,
      decorations: self.decorations,
      mode: if self.fullscreen {
        WindowMode::Fullscreen { use_size: false }
      } else {
        WindowMode::Windowed
      },
      ..Default::default()
    }
  }
}
