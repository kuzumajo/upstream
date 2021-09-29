use bevy::prelude::*;

use crate::game::{engine::{entity::{CollideRadius, Monster, Position, Velocity}, health::Health}, sprite::sprite::{SpriteAnimateTimer, SpriteRotation, SpriteSize}};

#[derive(Bundle)]
pub struct MonsterBundle {
  pub velocity: Velocity,
  pub position: Position,
  pub health: Health,
  pub collision_radius: CollideRadius,

  /// flags
  pub monster: Monster,

  /// sprites
  #[bundle]
  pub sprite: SpriteSheetBundle,
  pub timer: SpriteAnimateTimer,
  pub rotation: SpriteRotation,
  pub scale: SpriteSize,
}

impl Default for MonsterBundle {
  fn default() -> Self {
    MonsterBundle {
      velocity: Velocity(Vec2::ZERO),
      position: Position(Vec2::ZERO),
      health: Health { now: 500, max: 500 },
      collision_radius: CollideRadius(50.0),

      monster: Monster,

      sprite: SpriteSheetBundle::default(),
      timer: SpriteAnimateTimer::default(),
      rotation: SpriteRotation(Quat::default()),
      scale: SpriteSize(Vec2::new(100.0, 100.0)),
    }
  }
}
