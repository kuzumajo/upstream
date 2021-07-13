use bevy::{prelude::*, render::pass::ClearColor};

mod consts;
mod game;
mod logo;
mod menu;
mod saves;
mod staff;
mod load_game;
mod text_input;

use crate::consts::*;
use crate::game::UpstreamGamePlugins;
use crate::logo::StudioLogoPlugin;
use crate::menu::GameMenuPlugin;
use crate::staff::StaffPlugin;
use crate::load_game::LoadGamePlugin;
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

fn main() {
  App::build()
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .insert_resource(WindowDescriptor {
      width: 1280.0,
      height: 720.0,
      vsync: true,
      title: "Upstream".to_string(),
      ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .init_resource::<FontAssets>()
    .add_plugin(StudioLogoPlugin)
    .add_plugin(GameMenuPlugin)
    .add_plugin(StaffPlugin)
    .add_plugin(LoadGamePlugin)
    .add_plugin(TextInputPlugin)
    .add_plugins(UpstreamGamePlugins)
    .add_startup_system(insert_camera.system())
    .add_state(AppState::StudioLogo)
    .run();
}
