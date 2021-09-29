use bevy::prelude::*;

use crate::game::stages::SpriteLabel;

/// control animate interval
pub struct SpriteAnimateTimer(pub Timer);

impl Default for SpriteAnimateTimer {
  fn default() -> Self {
    Self(Timer::from_seconds(0.1, true))
  }
}

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

fn sprite_replay_when_handle_changed(
  mut query: Query<(&mut SpriteAnimateTimer, &mut TextureAtlasSprite), Changed<Handle<TextureAtlas>>>,
) {
  for (mut timer, mut sprite) in query.iter_mut() {
    timer.0.reset();
    sprite.index = 0;
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

pub struct SpriteSize(pub Vec2);

impl Default for SpriteSize {
  fn default() -> Self {
    Self(Vec2::splat(1.0))
  }
}

fn sprite_sync_size(
  materials: Res<Assets<TextureAtlas>>,
  mut query: Query<(&SpriteSize, &mut Transform, &Handle<TextureAtlas>)>,
) {
  for (size, mut transform, handle) in query.iter_mut() {
    if let Some(material) = materials.get(handle) {
      if let Some(rect) = material.textures.get(0) {
        let texture_size = rect.max - rect.min;
        transform.scale = Vec3::new(size.0.x / texture_size.x, size.0.y / texture_size.y, 1.0);
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
          .with_system(sprite_replay_when_handle_changed.label("reset"))
          .with_system(sprite_sheet_next_frame.after("reset"))
          .with_system(sprite_sync_rotation)
          .with_system(sprite_sync_size)
      );
  }
}
