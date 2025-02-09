use bevy::prelude::*;

use crate::game::{engine::{entity::{CollideRadius, Player, PlayerState, Position, Velocity}, health::Health, soul::SoulPower}, sprite::sprite::{SpriteAnimateTimer, SpriteRotation, SpriteSize}};

#[derive(Bundle)]
pub struct PlayerBundle {
  pub velocity: Velocity,
  pub position: Position,
  pub health: Health,
  pub soulpower: SoulPower,
  pub collision_radius: CollideRadius,

  /// flags
  pub player: Player,
  pub player_state: PlayerState,

  /// sprites
  #[bundle]
  pub sprite: SpriteSheetBundle,
  pub timer: SpriteAnimateTimer,
  pub rotation: SpriteRotation,
  pub scale: SpriteSize,
}

impl Default for PlayerBundle {
  fn default() -> Self {
    PlayerBundle {
      velocity: Velocity(Vec2::ZERO),
      position: Position(Vec2::ZERO),
      health: Health { now: 200, max: 200 },
      soulpower: SoulPower { now: 2000, max: 2000 },
      collision_radius: CollideRadius(50.0),

      player: Player,
      player_state: PlayerState::default(),

      sprite: SpriteSheetBundle::default(),
      timer: SpriteAnimateTimer::default(),
      rotation: SpriteRotation(Quat::default()),
      scale: SpriteSize(Vec2::new(100.0, 100.0)),
    }
  }
}
