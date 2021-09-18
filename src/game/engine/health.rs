use bevy::prelude::*;

use crate::consts::AppState;

/// Health of entity.
/// When it = 0, then we will remove the entity.
pub struct Health {
  pub now: u32,
  pub max: u32,
}

impl Health {
  /// Recieve Damage
  pub fn recieve_damage(&mut self, damage: u32) {
    self.now -= damage.min(self.now);
  }

  pub fn recieve_damage_locked(&mut self, damage: u32) {
    self.now -= damage.min(self.now - 1);
  }
  
  /// Recieve Heal
  pub fn recieve_heal(&mut self, heal: u32) {
    self.now += heal.min(self.max - self.now);
  }
}

/// This entity will never reach health to 0,
/// which means we will lock its health no less than 1
pub struct LockHealth;

/// Remove entity which health is zero
fn remove_zero_health_entity(
  mut commands: Commands,
  query: Query<(Entity, &Health)>,
) {
  for (entity, health) in query.iter() {
    if health.now == 0 {
      commands.entity(entity).despawn();
    }
  }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .with_system(remove_zero_health_entity)
      );
  }
}

