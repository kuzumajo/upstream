use bevy::prelude::*;

pub struct SoundEffects {
  pub tape: Handle<AudioSource>,
}

impl FromWorld for SoundEffects {
  fn from_world(world: &mut World) -> Self {
    let assets = world.get_resource::<AssetServer>().unwrap();

    SoundEffects {
      tape: assets.load("sounds/tape.mp3"),
    }
  }
}

pub struct SoundEffectsPlugin;

impl Plugin for SoundEffectsPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<SoundEffects>();
  }
}
