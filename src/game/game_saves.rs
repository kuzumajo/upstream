use crate::consts::*;
use crate::crypto::Crypto;
use crate::saves::GameSave;
use bevy::prelude::*;

use super::engine::entity::Controlling;
use super::entity::player::PlayerBundle;

pub struct AutoSaveSlot(pub u8);

struct AutoSaveTimer(Timer);

fn enter_game(
  mut commands: Commands,
  save: Res<GameSave>,
  slot: Option<Res<AutoSaveSlot>>,
  crypto: Res<Crypto>,
) {
  if let Some(slot) = slot {
    save.save(&crypto, slot.0).expect("failed to save!");
    commands.insert_resource(AutoSaveTimer(Timer::from_seconds(
      GAME_AUTOSAVE_INTERVAL,
      true,
    )));
  }

  // XXX: debug
  commands.spawn_bundle(PlayerBundle::default()).insert(Controlling);
}

fn update_auto_save(
  time: Res<Time>,
  timer: Option<ResMut<AutoSaveTimer>>,
  save: Res<GameSave>,
  slot: Option<Res<AutoSaveSlot>>,
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
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(enter_game))
      .add_system_set(
        SystemSet::on_update(AppState::InGame).with_system(update_auto_save),
      );
  }
}
