use bevy::{app::PluginGroupBuilder, prelude::*};

mod control_panel;
mod game_saves;
mod system_label;

mod engine;

use control_panel::ControlPanelPlugin;
use game_saves::GameSavePlugin;
// use player_attack::PlayerAttackPlugin;

pub use game_saves::GameAutoSaveSlot;
pub use system_label::GameSystemStage;

/// Whole game logics and UI performances
pub struct UpstreamGamePlugins;

impl PluginGroup for UpstreamGamePlugins {
  fn build(&mut self, app: &mut PluginGroupBuilder) {
    app
      .add(ControlPanelPlugin)
      .add(GameSavePlugin);
  }
}
