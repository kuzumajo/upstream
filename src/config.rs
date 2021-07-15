use crate::consts::*;
use crate::settings::{SettingItem, SettingType};
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
  volume: f32,
  volume_music: f32,
  volume_sfx: f32,
  volume_voice: f32,
  save_location: String,

  fullscreen: bool,
  resolution: usize,
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
      volume: 1.0,
      volume_music: 1.0,
      volume_sfx: 1.0,
      volume_voice: 1.0,
      save_location: save_dir,

      fullscreen: false,
      resolution: 0,
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
    warn!("failed to read display config from disk, use the default config");
    GameConfig::default()
  }

  /// save to disk
  pub fn save(&self) -> std::io::Result<()> {
    write(GameConfig::CONFIG_FILE, toml::to_vec(self).unwrap())?;
    info!("display config saved to {}", GameConfig::CONFIG_FILE);
    Ok(())
  }
}

impl GameConfig {
  pub fn get_window_descriptor(&self) -> WindowDescriptor {
    let (width, height) = RESOLUTION_LIST[self.resolution as usize % RESOLUTION_LIST.len()];

    WindowDescriptor {
      width: width as f32,
      height: height as f32,
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

  #[rustfmt::skip]
  pub fn get_settings_type(&self, item: &SettingItem) -> SettingType {
    use SettingItem::*;
    use SettingType::*;
    match *item {
      Volume =>           Slide(self.volume),
      VolumeMusic =>      Slide(self.volume_music),
      VolumeSfx =>        Slide(self.volume_sfx),
      VolumeVoice =>      Slide(self.volume_voice),
      SaveDir =>          String(self.save_location.clone()),
      Fullscreen =>       Ratio(self.fullscreen),
      Resolution =>       Select(self.resolution, RESOLUTION_LIST.iter().map(|(w, h)| format!("{}x{}", w, h)).collect()),
      Decorations =>      Ratio(self.decorations),
      AttackToMouse =>    Ratio(self.attack_to_mouse),
      AssaultToMouse =>   Ratio(self.assault_to_mouse),
      MouseSensitivity => Slide(self.mouse_sensitivity),
    }
  }

  #[rustfmt::skip]
  pub fn apply_changes(&mut self, sitem: &SettingItem, stype: &SettingType) {
    use SettingItem::*;
    use SettingType::*;
    match *sitem {
      Volume           => if let Slide(value) = *stype     { self.volume = value;                },
      VolumeMusic      => if let Slide(value) = *stype     { self.volume_music = value;          },
      VolumeSfx        => if let Slide(value) = *stype     { self.volume_sfx = value;            },
      VolumeVoice      => if let Slide(value) = *stype     { self.volume_voice = value;          },
      SaveDir          => if let String(value) = &*stype   { self.save_location = value.clone(); },
      Fullscreen       => if let Ratio(value) = *stype     { self.fullscreen = value;            },
      Resolution       => if let Select(value, _) = *stype { self.resolution = value;            },
      Decorations      => if let Ratio(value) = *stype     { self.decorations = value;           },
      AttackToMouse    => if let Ratio(value) = *stype     { self.attack_to_mouse = value;       },
      AssaultToMouse   => if let Ratio(value) = *stype     { self.assault_to_mouse = value;      },
      MouseSensitivity => if let Slide(value) = *stype     { self.mouse_sensitivity = value;     },
    }
  }
}
