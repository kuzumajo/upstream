use bevy::prelude::*;

#[macro_use]
pub mod attack;
pub mod health;
pub mod counter_attack;
pub mod entity;
pub mod shield;
pub mod soul;
pub mod movement;
pub mod projectile;

pub struct GameSystemPlugins;

impl PluginGroup for GameSystemPlugins {
  fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
    group
      .add(movement::MovementPlugin)
      .add(shield::ShieldPlugin)
      .add(projectile::ProjectilePlugin)
      .add(attack::AttackPlugin)
      .add(counter_attack::CounterAttackPlugin)
      .add(health::HealthPlugin);
  }
}
