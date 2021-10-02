use bevy::prelude::*;

use crate::game::{engine::{attack::RemovalCoolDown, entity::Position}, sprite::sprite::{SpriteAnimateTimer, SpriteRotation, SpriteSize}};

pub enum AttackSpriteType {
  Circle,
  HalfCircle,
  Rectangle,
}

impl Default for AttackSpriteType {
  fn default() -> Self {
    Self::Circle
  }
}

#[derive(Bundle)]
pub struct AttackBundle {
  pub position: Position,

  // flags
  pub removal: RemovalCoolDown,
  pub area: AttackSpriteType,

  // sprites
  #[bundle]
  pub sprite: SpriteSheetBundle,
  pub timer: SpriteAnimateTimer,
  pub rotation: SpriteRotation,
  pub scale: SpriteSize,
}

impl Default for AttackBundle {
  fn default() -> Self {
    Self {
      position: Position(Vec2::ZERO),

      removal: RemovalCoolDown(Timer::from_seconds(0.3, false)),
      area: AttackSpriteType::default(),

      sprite: SpriteSheetBundle::default(),
      timer: SpriteAnimateTimer::default(),
      rotation: SpriteRotation(Quat::default()),
      scale: SpriteSize::default(),
    }
  }
}
