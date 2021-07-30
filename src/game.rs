use bevy::{app::PluginGroupBuilder, prelude::*};

mod control_panel;
mod game_saves;
mod player_attack;

use control_panel::ControlPanelPlugin;
use game_saves::GameSavePlugin;
use player_attack::PlayerAttackPlugin;

pub use game_saves::GameAutoSaveSlot;

/// Whole game logics and UI performances
pub struct UpstreamGamePlugins;

impl PluginGroup for UpstreamGamePlugins {
  fn build(&mut self, app: &mut PluginGroupBuilder) {
    app
      .add(ControlPanelPlugin)
      .add(GameSavePlugin)
      .add(PlayerAttackPlugin);
  }
}
