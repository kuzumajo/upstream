use crate::crypto::Crypto;
use bevy::prelude::*;
use bincode::deserialize;
use home::home_dir;
use serde::Deserialize;
use serde::Serialize;
use std::fs::create_dir_all;
use std::fs::read;
use std::fs::remove_file;
use std::fs::write;
use std::path::PathBuf;
use std::time::Duration;
use std::time::SystemTime;

/// Game Data
#[derive(Serialize, Deserialize, Debug, Clone)]
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

fn get_save_dir() -> Option<PathBuf> {
  Some(home_dir()?.join(".kuzumajo").join("upstream"))
}

impl GameSave {
  pub fn new(name: String) -> GameSave {
    GameSave {
      last_modified_time: SystemTime::now(),
      created_time: SystemTime::now(),
      total_playing_time: Duration::from_secs(0),
      saving_name: name,
      unlocked_mahou: String::from("nashi"),
      money: 0,
      health: 1500,
      health_limit: 2000,
      energy: 490,
      energy_limit: 500,
    }
  }

  pub fn load(crypto: &Crypto, slot: u8) -> Option<Self> {
    let save_dir = get_save_dir().unwrap();
    let save_path = save_dir.join(format!("save{}.dat", slot));
    if let Ok(data) = read(&save_path) {
      if let Ok(data) = crypto.decrypt(&data) {
        if let Ok(save) = deserialize::<GameSave>(&data) {
          return Some(save);
        }
      }
      warn!("save {} broken, trying to delete it", slot);
      if remove_file(&save_path).is_ok() {
        warn!("removed {:?}", save_path);
      } else {
        warn!("unable to remove {:?}", save_path);
      }
    }
    return None;
  }

  pub fn save(&self, crypto: &Crypto, slot: u8) -> std::io::Result<()> {
    if slot >= 4 {
      warn!("illegal slot: {}", slot);
    }
    let save_dir = get_save_dir().unwrap();
    create_dir_all(&save_dir)?;
    let filename = save_dir.join(format!("save{}.dat", slot));
    let data = bincode::serialize(self).unwrap();
    let data = crypto.encrypt(&data);
    write(filename, &data)?;
    info!("game saved slot {}", slot);
    Ok(())
  }
}
