use bevy::prelude::*;

use crate::game::{engine::{entity::{Player, PlayerState, Position, Velocity}, health::Health, soul::SoulPower}, sprite::SpriteAnimateTimer};

#[derive(Bundle)]
pub struct PlayerBundle {
  pub velocity: Velocity,
  pub position: Position,
  pub health: Health,
  pub soulpower: SoulPower,

  /// flags
  pub player: Player,
  pub player_state: PlayerState,

  /// sprites
  #[bundle]
  pub sprite: SpriteSheetBundle,
  pub timer: SpriteAnimateTimer,
}

impl Default for PlayerBundle {
  fn default() -> Self {
    PlayerBundle {
      velocity: Velocity(Vec2::ZERO),
      position: Position(Vec2::ZERO),
      health: Health(200),
      soulpower: SoulPower(200, 200),

      player: Player,
      player_state: PlayerState::default(),

      sprite: SpriteSheetBundle::default(),
      timer: SpriteAnimateTimer(Timer::from_seconds(0.1, true)),
    }
  }
}
