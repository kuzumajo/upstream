use bevy::{app::PluginGroupBuilder, prelude::*};

mod control_panel;

use control_panel::ControlPanelPlugin;

/// Whole game logics and UI performances
pub struct UpstreamGamePlugins;

impl PluginGroup for UpstreamGamePlugins {
  fn build(&mut self, app: &mut PluginGroupBuilder) {
    app.add(ControlPanelPlugin);
  }
}
