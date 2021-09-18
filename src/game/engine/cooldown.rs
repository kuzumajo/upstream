use std::marker::PhantomData;

use bevy::{ecs::component::Component, prelude::*};

use crate::consts::AppState;

/// disable entity attack action
pub struct AttackCoolDown;

/// disable entity assault action
pub struct AssaultCoolDown;

pub struct RemovalCoolDown<T>(Timer, PhantomData<T>);

impl<T> RemovalCoolDown<T> {
  pub fn new(seconds: f32) -> Self {
    Self(Timer::from_seconds(seconds, false), PhantomData)
  }
}

pub fn update_removal_cool_down<T: Component>(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &mut RemovalCoolDown<T>)>,
) {
  for (entity, mut cooldown) in query.iter_mut() {
    if cooldown.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<RemovalCoolDown<T>>()
        .remove::<T>();
    }
  }
}

fn update_removal_entity_cool_down(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &mut RemovalCoolDown<Entity>)>,
) {
  for (entity, mut cooldown) in query.iter_mut() {
    if cooldown.0.tick(time.delta()).just_finished() {
      commands.entity(entity).despawn();
    }
  }
}

pub struct CoolDownPlugin;

impl Plugin for CoolDownPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .with_system(update_removal_cool_down::<AttackCoolDown>)
          .with_system(update_removal_cool_down::<AssaultCoolDown>)
          .with_system(update_removal_entity_cool_down)
      );
  }
}

/// useless
/// TODO: remove it
macro_rules! _cool_down_system {
  ($func_name:ident, $t:ty) => {
    fn $func_name(
      mut query: Query<(Entity, &mut $t)>,
    ) {
      for (entity, mut cooldown) in query.iter_mut() {
        if cooldown.0.tick(time.delta()).just_finished() {
          commands.entity(entity).remove::<$t>();
        }
      }
    }
  };
}
