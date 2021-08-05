use bevy::prelude::*;

use crate::{consts::AppState, game::GameSystemStage};

/// Indentify player
pub struct Player;

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

fn movement(
  mut query: Query<(&Velocity, &mut Position)>,
) {
  for (velocity, mut position) in query.iter_mut() {
    position.0 += velocity.0;
  }
}

/// most basic systems
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
  fn build(&self, app: &mut App) {
    app.add_system_set(
      SystemSet::on_update(AppState::InGame)
        .label(GameSystemStage::UpdatePosition)
        .before(GameSystemStage::CreateDamage)
        .with_system(movement)
    );
  }
}
