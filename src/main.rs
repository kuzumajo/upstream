use bevy::{input::system::exit_on_esc_system, prelude::*, render::pass::ClearColor};

mod consts;
mod logo;
mod menu;

use crate::consts::*;
use crate::logo::StudioLogoPlugin;
use crate::menu::GameMenuPlugin;

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
    .add_plugins(DefaultPlugins)
    .init_resource::<FontAssets>()
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .add_plugin(StudioLogoPlugin)
    .add_plugin(GameMenuPlugin)
    .add_startup_system(insert_camera.system())
    .add_state(AppState::StudioLogo)
    .add_system(exit_on_esc_system.system())
    .run();
}
