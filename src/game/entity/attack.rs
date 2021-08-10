use bevy::prelude::*;

use crate::game::{engine::{cooldown::RemovalCoolDown, entity::Position}, sprite::sprite::{SpriteAnimateTimer, SpriteRotation, SpriteScale}};

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
  pub removal: RemovalCoolDown<Entity>,
  pub area: AttackSpriteType,

  // sprites
  #[bundle]
  pub sprite: SpriteSheetBundle,
  pub timer: SpriteAnimateTimer,
  pub rotation: SpriteRotation,
  pub scale: SpriteScale,
}

impl Default for AttackBundle {
  fn default() -> Self {
    Self {
      position: Position(Vec2::ZERO),

      removal: RemovalCoolDown::new(0.3),
      area: AttackSpriteType::default(),

      sprite: SpriteSheetBundle::default(),
      timer: SpriteAnimateTimer(Timer::from_seconds(0.1, true)),
      rotation: SpriteRotation(Quat::default()),
      scale: SpriteScale::default(),
    }
  }
}