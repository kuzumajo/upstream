use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{attack::AttackPlugin, bullet::BulletSpritingPlugin, player::PlayerSpritingPlugin, sprite::SpriteAnimationPlugin};

mod player;
mod bullet;
mod attack;

pub mod sprite;

pub struct SpriteSystemPlugins;

impl PluginGroup for SpriteSystemPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group
      .add(AttackPlugin)
      .add(SpriteAnimationPlugin)
      .add(BulletSpritingPlugin)
      .add(PlayerSpritingPlugin);
  }
}
