use bevy::prelude::*;

use crate::{consts::{AppState, PLAYER_MOVE_SPEED}, game::stages::{GameEngineLabel, PhysicsLabel}};

use super::entity::{Controlling, Position, Velocity};

fn update_controlling_velocity(
  keycode_input: Res<Input<KeyCode>>,
  mut query: Query<&mut Velocity, With<Controlling>>
) {
  for mut velocity in query.iter_mut() {
    let mut direction = Vec2::ZERO;
    if keycode_input.pressed(KeyCode::A) {
      direction -= Vec2::X;
    }
    if keycode_input.pressed(KeyCode::D) {
      direction += Vec2::X;
    }
    if keycode_input.pressed(KeyCode::W) {
      direction += Vec2::Y;
    }
    if keycode_input.pressed(KeyCode::S) {
      direction -= Vec2::Y;
    }
    let v = direction.normalize_or_zero() * PLAYER_MOVE_SPEED;
    
    if velocity.0 != v {
      velocity.0 = v;
    }
  }
}

fn update_position(
  time: Res<Time>,
  mut query: Query<(&Velocity, &mut Position)>,
) {
  for (velocity, mut position) in query.iter_mut() {
    if velocity.0 != Vec2::ZERO {
      position.0 += velocity.0 * time.delta().as_secs_f32();
    }
  }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::UpdatePhysics)
          .after(GameEngineLabel::CoolDown)
          .label(PhysicsLabel::UpdateVelocity)
          .with_system(update_controlling_velocity)
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::UpdatePhysics)
          .after(GameEngineLabel::CoolDown)
          .label(PhysicsLabel::UpdatePosition)
          .after(PhysicsLabel::UpdateVelocity)
          .with_system(update_position)
      );
  }
}
