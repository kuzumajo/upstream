use bevy::prelude::*;

use crate::game::{engine::{entity::{CollideRadius, Position, Velocity}, projectile::BulletProps}, sprite::sprite::{SpriteAnimateTimer, SpriteRotation, SpriteScale}};


#[derive(Bundle)]
pub struct ProjectileBundle {
  pub position: Position,
  pub velocity: Velocity,
  pub bullet: BulletProps,
  pub radius: CollideRadius,

  /// sprites
  #[bundle]
  pub sprite: SpriteSheetBundle,
  pub timer: SpriteAnimateTimer,
  pub rotation: SpriteRotation,
  pub scale: SpriteScale,
}

impl Default for ProjectileBundle {
  fn default() -> Self {
    Self {
      position: Position(Vec2::default()),
      velocity: Velocity(Vec2::default()),
      bullet: BulletProps::default(),
      radius: CollideRadius(20.0),

      sprite: SpriteSheetBundle::default(),
      timer: SpriteAnimateTimer(Timer::from_seconds(0.1, true)),
      rotation: SpriteRotation(Quat::default()),
      scale: SpriteScale::default(),
    }
  }
}
