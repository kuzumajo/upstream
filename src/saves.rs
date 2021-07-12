use crate::consts::*;
use bincode::deserialize;
use home::home_dir;
use std::fs::create_dir_all;
use std::fs::read;
use std::fs::write;
use std::path::PathBuf;
use std::time::Duration;
use std::time::SystemTime;

fn get_save_dir() -> Option<PathBuf> {
  Some(home_dir()?.join(".kuzumajo").join("upstream"))
}

pub fn load_game_saves() -> Vec<Option<GameSave>> {
  let save_dir = get_save_dir().unwrap();
  (0..4)
    .map(|i| {
      let save_path = save_dir.join(format!("save{}.dat", i));
      if let Ok(data) = read(save_path) {
        if let Ok(save) = deserialize::<GameSave>(&data[..]) {
          return Some(save);
        }
      }
      return None;
    })
    .collect()
}

pub fn save_game_save(save: &GameSave, index: u32) -> std::io::Result<()> {
  if index >= 4 {
    panic!("illegal index: {}", index);
  }
  let save_dir = get_save_dir().unwrap();
  create_dir_all(&save_dir)?;
  let filename = save_dir.join(format!("save{}.dat", index));
  let data = bincode::serialize(save).unwrap();
  write(filename, data)?;
  Ok(())
}

/// start a new game
pub fn create_new_game_save() -> GameSave {
  GameSave {
    last_modified_time: SystemTime::now(),
    created_time: SystemTime::now(),
    total_playing_time: Duration::from_secs(0),
    saving_name: String::from(""),
    unlocked_mahou: String::from("nashi"),
    money: 0,
    health: 1500,
    health_limit: 2000,
    energy: 490,
    energy_limit: 500,
  }
}
