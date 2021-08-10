use bevy::prelude::*;

use crate::game::{engine::{entity::{CollideRadius, Player, PlayerState, Position, Velocity}, health::Health, soul::SoulPower}, sprite::sprite::{SpriteAnimateTimer, SpriteRotation, SpriteScale}};

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
  pub scale: SpriteScale,
}

impl Default for PlayerBundle {
  fn default() -> Self {
    PlayerBundle {
      velocity: Velocity(Vec2::ZERO),
      position: Position(Vec2::ZERO),
      health: Health(200),
      soulpower: SoulPower(20000, 20000),
      collision_radius: CollideRadius(50.0),

      player: Player,
      player_state: PlayerState::default(),

      sprite: SpriteSheetBundle::default(),
      timer: SpriteAnimateTimer(Timer::from_seconds(0.1, true)),
      rotation: SpriteRotation(Quat::default()),
      scale: SpriteScale(Vec3::new(100.0 / 30.0, 100.0 / 30.0, 1.0)),
    }
  }
}
