use bevy::prelude::*;

use crate::{consts::AppState, game::stages::{AttackLabel, GameEngineLabel}};

pub struct Health(pub u32);

impl Health {
  pub fn recieve_damage(&mut self, damage: u32) {
    self.0 -= self.0.min(damage);
  }
}

/// Entity is dead, and should be removed
pub struct Dead;

/// Entity will never reach health to 0
pub struct LockHealth;

/// Make sure LockHealth effects
fn update_lock_health(
  mut query: Query<&mut Health, With<LockHealth>>,
) {
  for mut health in query.iter_mut() {
    if health.0 == 0 {
      health.0 = 1;
    }
  }
}

/// Mark entity Dead if its health is 0
fn mark_health_0_as_dead(
  mut commands: Commands,
  query: Query<(Entity, &Health)>,
) {
  for (entity, health) in query.iter() {
    if health.0 == 0 {
      commands
        .entity(entity)
        .insert(Dead);
    }
  }
}

/// Remove entity which is marked as Dead
fn remove_dead_entity(
  mut commands: Commands,
  query: Query<Entity, With<Dead>>,
) {
  for entity in query.iter() {
    commands.entity(entity).despawn();
  }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::UpdateAttacks)
          .after(GameEngineLabel::UpdatePhysics)
          .before(AttackLabel::CheckDead)
          .after(AttackLabel::RecieveDamage)
          .with_system(update_lock_health)
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::UpdateAttacks)
          .after(GameEngineLabel::UpdatePhysics)
          .label(AttackLabel::CheckDead)
          .after(AttackLabel::RecieveDamage)
          .with_system(mark_health_0_as_dead)
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::UpdateAttacks)
          .after(GameEngineLabel::UpdatePhysics)
          .label(AttackLabel::RemoveDead)
          .after(AttackLabel::CheckDead)
          .with_system(remove_dead_entity)
      );
  }
}

