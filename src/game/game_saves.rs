use crate::consts::*;
use crate::crypto::Crypto;
use crate::saves::GameSave;
use bevy::prelude::*;

pub struct GameAutoSaveSlot(pub u8);

struct GameAutoSaveTimer(Timer);

fn enter_game(
  mut commands: Commands,
  save: Res<GameSave>,
  slot: Option<Res<GameAutoSaveSlot>>,
  crypto: Res<Crypto>,
) {
  if let Some(slot) = slot {
    save.save(&crypto, slot.0).expect("failed to save!");
    commands.insert_resource(GameAutoSaveTimer(Timer::from_seconds(
      GAME_AUTOSAVE_INTERVAL,
      true,
    )));
  }
}

fn update_auto_save(
  time: Res<Time>,
  timer: Option<ResMut<GameAutoSaveTimer>>,
  save: Res<GameSave>,
  slot: Option<Res<GameAutoSaveSlot>>,
  crypto: Res<Crypto>,
) {
  if let Some(mut timer) = timer {
    if timer.0.tick(time.delta()).just_finished() {
      if let Some(slot) = slot {
        save.save(&crypto, slot.0).expect("failed to save!");
      } else {
        warn!("autosave enabled but save slot not found");
      }
    }
  }
}

/// Manage game saves
pub struct GameSavePlugin;

impl Plugin for GameSavePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(enter_game.system()))
      .add_system_set(
        SystemSet::on_update(AppState::InGame).with_system(update_auto_save.system()),
      );
  }
}
