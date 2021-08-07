use bevy::{app::PluginGroupBuilder, prelude::*};

mod control_panel;
mod game_saves;
mod stages;

mod engine;
mod sprite;
mod entity;

use control_panel::ControlPanelPlugin;
use game_saves::GameSavePlugin;
// use player_attack::PlayerAttackPlugin;

pub use game_saves::AutoSaveSlot;

/// Whole game logics and UI performances
pub struct GameBasicPlugins;

impl PluginGroup for GameBasicPlugins {
  fn build(&mut self, app: &mut PluginGroupBuilder) {
    app
      .add(ControlPanelPlugin)
      .add(GameSavePlugin);
  }
}

pub use engine::GameSystemPlugins;
pub use sprite::SpriteSystemPlugins;
