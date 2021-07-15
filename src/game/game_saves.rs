use crate::consts::*;
use crate::saves::GameSave;
use bevy::prelude::*;

pub struct GameAutoSaveSlot(pub u8);

struct GameAutoSaveTimer(Timer);

fn enter_game(mut commands: Commands, save: Res<GameSave>, slot: Option<Res<GameAutoSaveSlot>>) {
  if let Some(slot) = slot {
    save.save(slot.0).expect("failed to save!");
    info!("Game saved to slot {}", slot.0);
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
) {
  if let Some(mut timer) = timer {
    if timer.0.tick(time.delta()).just_finished() {
      if let Some(slot) = slot {
        save.save(slot.0).expect("failed to save!");
        info!("Auto saved to slot {}", slot.0);
      } else {
        warn!("Auto Save enabled but save slot not found");
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
