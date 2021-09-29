use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{attack::AttackPlugin, bullet::BulletSpritingPlugin, health::HealthBarPlugin, player::PlayerSpritingPlugin, sprite::SpriteAnimationPlugin};

mod player;
mod bullet;
mod attack;

pub mod sprite;
pub mod health;

pub struct SpriteSystemPlugins;

impl PluginGroup for SpriteSystemPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group
      .add(HealthBarPlugin)
      .add(AttackPlugin)
      .add(SpriteAnimationPlugin)
      .add(BulletSpritingPlugin)
      .add(PlayerSpritingPlugin);
  }
}
