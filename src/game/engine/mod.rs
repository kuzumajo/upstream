use bevy::prelude::*;

mod health;
mod counter_attack;
mod cooldown;
mod entity;
mod attack;
mod shield;
mod soul;
mod projectile;

pub struct GameSystemPlugins;

impl PluginGroup for GameSystemPlugins {
  fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
    group
      .add(entity::EntityPlugin)
      .add(shield::ShieldPlugin)
      .add(projectile::ProjectilePlugin)
      .add(attack::AttackPlugin)
      .add(cooldown::CoolDownPlugin)
      .add(counter_attack::CounterAttackPlugin)
      .add(health::HealthPlugin);
  }
}
