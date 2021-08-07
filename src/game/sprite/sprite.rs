use bevy::prelude::*;

use crate::{consts::AppState, game::system_label::GameSpriteStage};

/// control animate interval
pub struct SpriteAnimateTimer(pub Timer);

fn sprite_animation(
  assets: Res<Assets<TextureAtlas>>,
  time: Res<Time>,
  mut query: Query<(
    &mut SpriteAnimateTimer,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>
  )>
) {
  for (mut timer, mut sprite, handle) in query.iter_mut() {
    if timer.0.tick(time.delta()).just_finished() {
      if let Some(texture) = assets.get(handle) {
        sprite.index = (sprite.index + 1) % texture.textures.len() as u32;
      }
    }
  }
}

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameSpriteStage::AnimateSprite)
          .after(GameSpriteStage::ChangeHandle)
          .with_system(sprite_animation)
      );
  }
}
