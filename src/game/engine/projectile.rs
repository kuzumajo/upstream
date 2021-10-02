use bevy::prelude::*;

use crate::consts::AppState;

use super::{attack::{AttackArea, AttackDamage, GroupAttack}, entity::{CollideRadius, Position}};

#[derive(Default)]
pub struct BulletProps {
  pub owner: Option<Entity>,
  pub damage: Option<AttackDamage>,
}

/// if bullets collides with other entity (with another owner),
/// then destory itself and perform attack to the specific entity
/// or the whole area.
fn bullet_collision(
  mut commands: Commands,
  mut attack: ResMut<Vec<GroupAttack>>,
  query: Query<(Entity, &Position, &CollideRadius, &BulletProps)>,
  obj_query: Query<(Entity, &Position, &CollideRadius)>,
) {
  for (entity1, position1, radius1, props) in query.iter() {
    for (entity2, position2, radius2) in obj_query.iter() {
      if entity2 == entity1 || (props.owner.is_some() && props.owner == Some(entity2)) {
        continue;
      }

      // check if bullet collides with some other entities
      if position1.0.distance(position2.0) < radius1.0 + radius2.0 {

        commands.entity(entity1).despawn_recursive();

        if let Some(damage) = props.damage {
          attack.push(GroupAttack {
            area: AttackArea::Circle {
              o: position1.0,
              r: radius1.0,
            },
            entities: Vec::new(),
            damage,
            from: props.owner,
          });
        }

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
        .with_system(bullet_collision)
    );
  }
}
