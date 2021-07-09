use bevy::{prelude::*, render::pass::ClearColor};

mod setup;

use crate::setup::setup;

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .add_startup_system(setup.system())
    .run();
}
