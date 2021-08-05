use bevy::prelude::*;

use crate::{consts::AppState, game::GameSystemStage};

use super::{attack::{AttackArea, AttackDamage, GroupAttack}, entity::{CollideRadius, Position, Velocity}};

pub struct BulletProps {
  pub owner: Entity,
  pub damage: AttackDamage,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
  pub position: Position,
  pub velocity: Velocity,
  pub bullet: BulletProps,
  pub radius: CollideRadius,
}

fn bullet_collision(
  mut commands: Commands,
  mut attack: ResMut<Vec<GroupAttack>>,
  query: Query<(Entity, &Position, &CollideRadius, &BulletProps)>,
  obj_query: Query<(Entity, &Position, &CollideRadius)>,
) {
  for (entity1, position1, radius1, props) in query.iter() {
    for (entity2, position2, radius2) in obj_query.iter() {
      if entity2 == entity1 || entity2 == props.owner {
        continue;
      }

      // check if bullet collides with some other entities
      if position1.0.distance(position2.0) < radius1.0 + radius2.0 {

        commands.entity(entity1).despawn();

        attack.push(GroupAttack {
          area: AttackArea::Circle {
            o: position1.0,
            r: radius1.0,
          },
          entities: Vec::new(),
          damage: props.damage,
          from: props.owner,
        });

        break;
      }
    }
  }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
  fn build(&self, app: &mut App) {
    app.add_system_set(
      SystemSet::on_update(AppState::InGame)
        .label(GameSystemStage::CreateDamage)
        .after(GameSystemStage::UpdatePosition)
        .with_system(bullet_collision.system())
    );
  }
}
