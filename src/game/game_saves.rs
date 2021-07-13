use crate::consts::*;
use bevy::prelude::*;

pub struct GameAutoSaveSlot(pub u8);

/// Manage game saves
pub struct GameSavePlugin;

impl Plugin for GameSavePlugin {
  fn build(&self, app: &mut AppBuilder) {}
}
