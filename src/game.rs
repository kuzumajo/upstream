use bevy::{app::PluginGroupBuilder, prelude::*};

mod control_panel;
mod game_saves;
mod camera;
mod stages;

mod engine;
mod sprite;
mod entity;

use control_panel::ControlPanelPlugin;
use game_saves::GameSavePlugin;
use camera::CameraPlugin;

pub use game_saves::AutoSaveSlot;
pub use camera::GameCamera;
pub use camera::MouseDirection;

/// Whole game logics and UI performances
pub struct GameBasicPlugins;

impl PluginGroup for GameBasicPlugins {
  fn build(&mut self, app: &mut PluginGroupBuilder) {
    app
      .add(CameraPlugin)
      .add(ControlPanelPlugin)
      .add(GameSavePlugin);
  }
}

pub use engine::GameSystemPlugins;
pub use sprite::SpriteSystemPlugins;
