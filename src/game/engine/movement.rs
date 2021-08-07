use bevy::prelude::*;

use crate::{consts::{AppState, PLAYER_MOVE_SPEED}, game::stages::PhysicalStage};

use super::entity::{Controlling, Position, Velocity};

fn update_controlling_velocity(
  keycode_input: Res<Input<KeyCode>>,
  mut query: Query<&mut Velocity, With<Controlling>>
) {
  for mut velocity in query.iter_mut() {
    let mut direction = Vec2::ZERO;
    if keycode_input.pressed(KeyCode::A) {
      direction += - Vec2::X;
    }
    if keycode_input.pressed(KeyCode::D) {
      direction += Vec2::X;
    }
    if keycode_input.pressed(KeyCode::W) {
      direction += Vec2::Y;
    }
    if keycode_input.pressed(KeyCode::S) {
      direction += - Vec2::Y;
    }
    velocity.0 = direction.normalize_or_zero() * PLAYER_MOVE_SPEED;
  }
}

fn update_position(
  time: Res<Time>,
  mut query: Query<(&Velocity, &mut Position)>,
) {
  for (velocity, mut position) in query.iter_mut() {
    position.0 += velocity.0 * time.delta().as_secs_f32();
  }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set_to_stage(
        PhysicalStage::UpdateVelocity,
        SystemSet::on_update(AppState::InGame)
          .with_system(update_controlling_velocity)
      )
      .add_system_set_to_stage(
        PhysicalStage::UpdatePosition,
        SystemSet::on_update(AppState::InGame)
          .with_system(update_position)
      );
  }
}
