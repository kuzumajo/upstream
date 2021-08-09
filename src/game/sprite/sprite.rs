use bevy::prelude::*;

use crate::game::stages::SpriteLabel;

/// control animate interval
pub struct SpriteAnimateTimer(pub Timer);

fn sprite_sheet_next_frame(
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

pub struct SpriteRotation(pub Quat);

fn sprite_sync_rotation(
  mut query: Query<(&SpriteRotation, &mut Transform), Changed<SpriteRotation>>,
) {
  for (rotation, mut transform) in query.iter_mut() {
    transform.rotation = rotation.0;
  }
}

pub struct SpriteScale(pub Vec3);

impl Default for SpriteScale {
  fn default() -> Self {
    Self(Vec3::splat(1.0))
  }
}

fn sprite_sync_scale(
  mut query: Query<(&SpriteScale, &mut Transform), Changed<SpriteScale>>,
) {
  for (scale, mut transform) in query.iter_mut() {
    transform.scale = scale.0;
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
          .with_system(sprite_sheet_next_frame)
          .with_system(sprite_sync_rotation)
          .with_system(sprite_sync_scale)
      );
  }
}
