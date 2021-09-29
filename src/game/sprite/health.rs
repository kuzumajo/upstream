use std::fmt::DebugList;

use bevy::prelude::*;

use crate::{consts::AppState, game::engine::{entity::CollideRadius, health::Health}};

/// Insert a health bar for this entity
pub struct HealthBar;

struct HealthBarMaterials {
  green: Handle<ColorMaterial>,
  red: Handle<ColorMaterial>,
  decending: Handle<ColorMaterial>,
  empty: Handle<ColorMaterial>,
}

impl FromWorld for HealthBarMaterials {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

    HealthBarMaterials {
      green: materials.add(Color::GREEN.into()),
      red: materials.add(Color::RED.into()),
      decending: materials.add(Color::rgba(1., 1., 1., 0.8).into()),
      empty: materials.add(Color::rgba(0., 0., 0., 0.2).into()),
    }
  }
}

enum HealthBarType {
  MaxHealth,
  CurrentHealth,
  DecendingHealth,
}

fn insert_health_bar(
  mut commands: Commands,
  materials: Res<HealthBarMaterials>,
  query: Query<(Entity, &CollideRadius), With<HealthBar>>,
) {
  for (entity, radius) in query.iter() {
    let mut entity = commands.entity(entity);
    entity.with_children(|parent| {
      // max health
      parent
        .spawn_bundle(SpriteBundle {
          sprite: Sprite {
            size: Vec2::new(100.0, 10.0),
            ..Default::default()
          },
          material: materials.empty.clone(),
          transform: Transform {
            translation: Vec3::new(0.0, radius.0, 5.0),
            ..Default::default()
          },
          ..Default::default()
        })
        .insert(HealthBarType::MaxHealth);

      // decending health
      parent
        .spawn_bundle(SpriteBundle {
          sprite: Sprite {
            size: Vec2::new(100.0, 10.0),
            ..Default::default()
          },
          material: materials.decending.clone(),
          transform: Transform {
            translation: Vec3::new(0.0, radius.0, 5.0),
            ..Default::default()
          },
          ..Default::default()
        })
        .insert(HealthBarType::DecendingHealth);

      // current health
      parent
        .spawn_bundle(SpriteBundle {
          sprite: Sprite {
            size: Vec2::new(100.0, 10.0),
            ..Default::default()
          },
          material: materials.red.clone(),
          transform: Transform {
            translation: Vec3::new(0.0, radius.0, 5.0),
            ..Default::default()
          },
          ..Default::default()
        })
        .insert(HealthBarType::CurrentHealth);
    });
    entity.remove::<HealthBar>();
  }
}

fn update_health_bar(
  query: Query<(Entity, &Health), Changed<Health>>,
  query_child: Query<&Children>,
  mut query_type: Query<(&HealthBarType, &mut Sprite, &mut Transform)>,
) {
  for (entity, health) in query.iter() {
    if let Ok(children) = query_child.get(entity) {
      for entity in children.iter() {
        if let Ok((tp, mut sprite, mut transform)) = query_type.get_mut(*entity) {
          let now = health.now as f32 / health.max as f32 * 100.0;
          match tp {
            &HealthBarType::MaxHealth => {
              // max is max...
            }
            &HealthBarType::CurrentHealth => {
              sprite.size.x = now;
            }
            &HealthBarType::DecendingHealth => {
              sprite.size.x = now + (sprite.size.x - now) * 0.1;
            }
          };
          transform.translation.x = - (100.0 - sprite.size.x) / 2.0;
        }
      }
    }
  }
}

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<HealthBarMaterials>()
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .with_system(insert_health_bar)
          .with_system(update_health_bar)
      );
  }
}
