#![windows_subsystem = "windows"]

#[macro_use]
extern crate magic_crypt;

use bevy::window::WindowResized;
use bevy::{prelude::*, render::pass::ClearColor};
use game::GameCamera;

mod config;
mod consts;
mod crypto;
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
use crate::crypto::Crypto;
use crate::game::GameBasicPlugins;
use crate::game::GameSystemPlugins;
use crate::game::SpriteSystemPlugins;
use crate::load_game::LoadGamePlugin;
use crate::logo::StudioLogoPlugin;
use crate::menu::GameMenuPlugin;
use crate::settings::SettingsPlugin;
use crate::staff::StaffPlugin;
use crate::text_input::TextInputPlugin;

fn insert_camera(mut commands: Commands) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(GameCamera);
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

pub struct MousePosition(pub Vec2);

fn update_mouse_position(
  mut events: EventReader<CursorMoved>,
  mut res: ResMut<MousePosition>,
) {
  for event in events.iter() {
    res.0 = event.position;
  }
}

pub struct WindowSize {
  pub width: f32,
  pub height: f32,
}

fn update_window_size(
  mut res: ResMut<WindowSize>,
  mut events: EventReader<WindowResized>,
) {
  for event in events.iter() {
    res.width = event.width;
    res.height = event.height;
  }
}

/// @see <https://github.com/bevyengine/bevy/issues/1135>
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
  let window_descriptor = game_config.get_window_descriptor();

  App::new()
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .insert_resource(Crypto::new(CRYPTO_KEY))
    .insert_resource(MousePosition(Vec2::ZERO))
    .insert_resource(WindowSize {
      width: window_descriptor.width,
      height: window_descriptor.height
    })
    .insert_resource(window_descriptor)
    .insert_resource(game_config)
    .add_system_to_stage(CoreStage::PreUpdate, update_mouse_position)
    .add_system_to_stage(CoreStage::PreUpdate, update_window_size)
    .add_system_to_stage(CoreStage::PostUpdate, issue_1135_system)
    .add_plugins(DefaultPlugins)
    .init_resource::<FontAssets>()
    .add_plugin(StudioLogoPlugin)
    .add_plugin(GameMenuPlugin)
    .add_plugin(StaffPlugin)
    .add_plugin(LoadGamePlugin)
    .add_plugin(TextInputPlugin)
    .add_plugin(SettingsPlugin)
    .add_plugins(GameBasicPlugins)
    .add_plugins(GameSystemPlugins)
    .add_plugins(SpriteSystemPlugins)
    .add_startup_system(insert_camera)
    .add_state(AppState::StudioLogo)
    .run();
}
