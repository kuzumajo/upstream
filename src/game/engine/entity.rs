use bevy::prelude::*;

/// Indentify player
pub struct Player;

/// Indentity monster
pub struct Monster;

/// Indentify NPC
pub struct NPC;

/// entity which is under control
pub struct Controlling;

#[derive(Clone, Copy)]
pub struct Position(pub Vec2);

#[derive(Clone, Copy)]
pub struct Velocity(pub Vec2);

pub struct CollideRadius(pub f32);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerState {
  ShieldAttackA,
  ShieldAttackAA,
  ShieldAttackAB,
  ShieldAttackB,
  ShieldAttackBB,
  ShieldAttackBBB,

  ShieldAssault,
  ShieldAssaultA,
  ShieldAssaultB,

  Stand,
}

impl Default for PlayerState {
  fn default() -> Self {
    Self::Stand
  }
}
