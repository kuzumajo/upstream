use bevy::prelude::*;

use crate::{consts::AppState, game::GameSystemStage};

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
fn update_health_death(
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
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameSystemStage::CheckDead)
          .after(GameSystemStage::PostDamage)
          .with_system(update_health_death.system())
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameSystemStage::PostDamage)
          .after(GameSystemStage::RecieveDamage)
          .with_system(update_lock_health.system())
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameSystemStage::ClearDead)
          .after(GameSystemStage::CheckDead)
          .with_system(remove_dead_entity.system())
      );
  }
}

