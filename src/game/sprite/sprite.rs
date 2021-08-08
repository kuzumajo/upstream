use bevy::prelude::*;

use crate::game::stages::SpriteLabel;

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
      .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
          .label(SpriteLabel::SpriteAnimation)
          .after(SpriteLabel::UpdateSpriteSheet)
          .with_system(sprite_animation)
      );
  }
}
