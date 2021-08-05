use std::{collections::BTreeSet, iter::FromIterator};

use bevy::prelude::*;

use crate::{consts::AppState, game::GameSystemStage};

use super::{entity::{CollideRadius, Position}, health::Health};

pub struct GroupAttack {
  pub area: AttackArea,
  pub entities: Vec<Entity>,
  pub damage: AttackDamage,
  pub from: Entity,
}

pub struct SingleAttack {
  pub entity: Entity,
  pub damage: AttackDamage,
  pub from: Entity,
}

pub enum AttackArea {
  /// Circle
  /// 
  /// ```
  ///   .-```-.
  /// .`       `.
  /// |    o--r-|
  /// `.       .`
  ///   `-._.-`
  /// ```
  Circle {
    /// center
    o: Vec2,
    /// radius
    r: f32,
  },

  /// Half circle
  /// 
  /// ```
  /// |`-.
  /// |   `.
  /// o--r-|  v ---> 
  /// |   .`
  /// |.-`
  /// ```
  HalfCircle {
    /// center
    o: Vec2,
    /// radius
    r: f32,
    /// normalized direction vector
    v: Vec2,
  },

  /// Rectangle
  /// 
  /// ```
  /// .--------w--------.
  /// |                 |
  /// o     v --->      h   
  /// |                 |
  /// `-----------------`
  /// ```
  Rectangle {
    /// left center of the area
    o: Vec2,
    /// width
    w: f32,
    /// height
    h: f32,
    /// normalized direction vector
    v: Vec2,
  }
}

#[derive(Clone, Copy)]
pub enum AttackDamage {
  Physical {
    damage: u32,
    power: u32,
  },

  Magical {
    damage: u32,
  }
}

/// convert group attack to single attack
fn process_damage(
  mut group_attacks: ResMut<Vec<GroupAttack>>,
  mut single_attacks: ResMut<Vec<SingleAttack>>,
  query: Query<(Entity, &Position, &CollideRadius)>,
) {
  for attack in group_attacks.iter() {
    let mut set: BTreeSet<Entity> = BTreeSet::from_iter(attack.entities.clone().into_iter());

    for (entity, position, radius) in query.iter() {

      // damage doesn't hurt self
      if entity == attack.from {
        continue;
      }

      let collides = match attack.area {
        AttackArea::Circle { o, r } => o.distance(position.0) <= r + radius.0,
        AttackArea::HalfCircle { o, r, v } =>
          o.distance(position.0) <= r + radius.0 && v.dot(position.0 + v * r - o) > 0.0,
        AttackArea::Rectangle { o, w, h, v } => {
          // FIXME: I don't know how to calculate...
          false
        }
      };

      if collides {
        set.insert(entity);
      }
    }

    for entity in set {
      single_attacks.push(SingleAttack {
        entity,
        damage: attack.damage,
        from: attack.from,
      });
    }
  }

  group_attacks.clear();
}

/// perform damage to targeted entity
fn recieve_damage(
  mut attacks: ResMut<Vec<SingleAttack>>,
  mut query: Query<&mut Health>,
) {
  for attack in attacks.iter() {
    if let Ok(mut health) = query.get_mut(attack.entity) {

      // FIXME: very original
      health.recieve_damage(match attack.damage {
        AttackDamage::Physical { damage, .. } => damage,
        AttackDamage::Magical { damage } => damage,
      });

      // TODO: play repellent here
    }
  }

  attacks.clear();
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource::<Vec<GroupAttack>>(Vec::new())
      .insert_resource::<Vec<SingleAttack>>(Vec::new())
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameSystemStage::ProcessDamage)
          .after(GameSystemStage::CreateDamage)
          .with_system(process_damage.system())
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameSystemStage::RecieveDamage)
          .after(GameSystemStage::ProcessDamage)
          .with_system(recieve_damage.system())
      );
  }
}