use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{bullet::BulletSpritingPlugin, player::PlayerSpritingPlugin, sprite::SpriteAnimationPlugin};

mod player;
mod bullet;
mod sprite;

pub use sprite::SpriteAnimateTimer;

pub struct SpriteSystemPlugins;

impl PluginGroup for SpriteSystemPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group
      .add(SpriteAnimationPlugin)
      .add(BulletSpritingPlugin)
      .add(PlayerSpritingPlugin);
  }
}
