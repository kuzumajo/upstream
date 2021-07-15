use bevy::{prelude::*, render::pass::ClearColor, render::RenderSystem};

mod config;
mod consts;
mod game;
mod load_game;
mod logo;
mod menu;
mod saves;
mod settings;
mod staff;
mod text_input;

use crate::config::GameConfig;
use crate::consts::*;
use crate::game::UpstreamGamePlugins;
use crate::load_game::LoadGamePlugin;
use crate::logo::StudioLogoPlugin;
use crate::menu::GameMenuPlugin;
use crate::settings::SettingsPlugin;
use crate::staff::StaffPlugin;
use crate::text_input::TextInputPlugin;

fn insert_camera(mut commands: Commands) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());
}

pub struct FontAssets {
  default_font: Handle<Font>,
}

impl FromWorld for FontAssets {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world.get_resource::<AssetServer>().unwrap();

    FontAssets {
      default_font: asset_server.load("fonts/hanyi.ttf"),
    }
  }
}

/// @see https://github.com/bevyengine/bevy/issues/1135
fn issue_1135_system(mut query: Query<(&Node, &mut Visible), With<Text>>) {
  for (node, mut visible) in query.iter_mut() {
    if node.size == Vec2::ZERO {
      visible.is_visible = false;
    } else {
      visible.is_visible = true;
    }
  }
}

fn main() {
  let game_config = GameConfig::load();

  App::build()
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .insert_resource(game_config.get_window_descriptor())
    .insert_resource(game_config)
    .add_system(
      issue_1135_system
        .system()
        .before(RenderSystem::VisibleEntities),
    )
    .add_plugins(DefaultPlugins)
    .init_resource::<FontAssets>()
    .add_plugin(StudioLogoPlugin)
    .add_plugin(GameMenuPlugin)
    .add_plugin(StaffPlugin)
    .add_plugin(LoadGamePlugin)
    .add_plugin(TextInputPlugin)
    .add_plugin(SettingsPlugin)
    .add_plugins(UpstreamGamePlugins)
    .add_startup_system(insert_camera.system())
    .add_state(AppState::StudioLogo)
    .run();
}
