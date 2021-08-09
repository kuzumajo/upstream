use bevy::prelude::*;

use crate::{MousePosition, WindowSize, consts::{AppState, CAMERA_SYNC_SPEED}};

use super::{engine::entity::{Player, Position}, stages::GameEngineLabel};

pub struct GameCamera;

fn sync_camera_with_player(
  mut camera_query: Query<&mut Transform, With<GameCamera>>,
  player_query: Query<&Position, With<Player>>,
) {
  if let Ok(position) = player_query.single() {
    if let Ok(mut transform) = camera_query.single_mut() {
      transform.translation.x += (position.0.x - transform.translation.x) * CAMERA_SYNC_SPEED;
      transform.translation.y += (position.0.y - transform.translation.y) * CAMERA_SYNC_SPEED;
    }
  }
}

pub struct MouseDirection(pub Vec2);

fn update_mouse_direction(
  mouse_position: Res<MousePosition>,
  window_size: Res<WindowSize>,
  mut direction: ResMut<MouseDirection>,
  camera_query: Query<&Transform, With<GameCamera>>,
  player_query: Query<&Position, With<Player>>,
) {
  if let Ok(position) = player_query.single() {
    if let Ok(transform) = camera_query.single() {
      let camera = position.0 - Vec2::new(transform.translation.x, transform.translation.y);
      let mouse = mouse_position.0 - Vec2::new(window_size.width, window_size.height) / 2.0;
      let vector = (mouse - camera).normalize_or_zero();

      if vector != Vec2::ZERO {
        direction.0 = vector;
      }
    }
  }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(MouseDirection(Vec2::ZERO))
      .add_system_to_stage(CoreStage::PreUpdate, update_mouse_direction)
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .after(GameEngineLabel::UpdatePhysics)
          .with_system(sync_camera_with_player)
      );
  }
}