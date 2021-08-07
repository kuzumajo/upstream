use bevy::prelude::*;

/// Indentify player
pub struct Player;

/// entity which is under control
pub struct Controlling;

#[derive(Clone, Copy)]
pub struct Position(pub Vec2);

#[derive(Clone, Copy)]
pub struct Velocity(pub Vec2);

pub struct CollideRadius(pub f32);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
  ShieldAttackA,
  ShieldAttackAA,
  ShieldAttackAB,
  ShieldAttackB,
  ShieldAttackBB,
  ShieldAttackBBB,

  ShieldAssaultA,
  ShieldAssaultB,

  Assault,
  Stand,
}

impl Default for PlayerState {
  fn default() -> Self {
    Self::Stand
  }
}
