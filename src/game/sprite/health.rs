use std::fmt::DebugList;

use bevy::prelude::*;

use crate::{consts::AppState, game::engine::entity::CollideRadius};

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
      empty: materials.add(Color::rgba(0., 0., 0., 0.05).into()),
    }
  }
}

struct HealthBarItem;

struct HealthBarCurrentHealth;
struct HealthBarDecendingHealth;
struct HealthBarMaxHealth;

fn insert_health_bar(
  mut commands: Commands,
  materials: Res<HealthBarMaterials>,
  query: Query<(Entity, &CollideRadius), With<HealthBar>>,
) {
  for (entity, radius) in query.iter() {
    let mut entity = commands.entity(entity);
    entity.with_children(|parent| {
      parent
        .spawn()
        .insert(HealthBarItem)
        .insert(Transform {
          translation: Vec3::new(0.0, radius.0, 5.0),
          ..Default::default()
        })
        .insert(GlobalTransform::default())
        .with_children(|parent| {

          // max health
          parent
            .spawn_bundle(SpriteBundle {
              sprite: Sprite {
                size: Vec2::new(100.0, 10.0),
                ..Default::default()
              },
              material: materials.empty.clone(),
              ..Default::default()
            })
            .insert(HealthBarMaxHealth);

          // decending health
          parent
            .spawn_bundle(SpriteBundle {
              sprite: Sprite {
                size: Vec2::new(100.0, 10.0),
                ..Default::default()
              },
              material: materials.decending.clone(),
              ..Default::default()
            })
            .insert(HealthBarDecendingHealth);

          // current health
          parent
            .spawn_bundle(SpriteBundle {
              sprite: Sprite {
                size: Vec2::new(100.0, 10.0),
                ..Default::default()
              },
              material: materials.red.clone(),
              ..Default::default()
            })
            .insert(HealthBarCurrentHealth);
        });
    });
    entity.remove::<HealthBar>();
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
      );
  }
}
