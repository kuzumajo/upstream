use std::{collections::BTreeSet, iter::FromIterator};

use bevy::prelude::*;

use crate::{consts::AppState, game::stages::{AttackLabel, GameEngineLabel}};

use super::{entity::{CollideRadius, Position}, health::Health};

#[derive(Debug)]
pub struct GroupAttack {
  pub area: AttackArea,
  pub entities: Vec<Entity>,
  pub damage: AttackDamage,
  pub from: Option<Entity>,
}

#[derive(Debug)]
pub struct SingleAttack {
  pub entity: Entity,
  pub damage: AttackDamage,
  pub from: Option<Entity>,
}

#[derive(Debug)]
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

#[derive(Clone, Copy, Debug)]
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
  mut attacks: ResMut<Vec<GroupAttack>>,
  mut single_attacks: ResMut<Vec<SingleAttack>>,
  query: Query<(Entity, &Position, &CollideRadius)>,
) {
  for attack in attacks.iter() {
    let mut set: BTreeSet<Entity> = BTreeSet::from_iter(attack.entities.clone().into_iter());

    // XXX: debug here
    println!("{:?}", attack);

    for (entity, position, radius) in query.iter() {

      // damage doesn't hurt self
      if attack.from.is_some() && attack.from == Some(entity) {
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

  attacks.clear();
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
          .label(GameEngineLabel::UpdateAttacks)
          .after(GameEngineLabel::UpdatePhysics)
          .label(AttackLabel::ProcessDamage)
          .after(AttackLabel::PerformAttack)
          .with_system(process_damage)
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::UpdateAttacks)
          .after(GameEngineLabel::UpdatePhysics)
          .label(AttackLabel::RecieveDamage)
          .after(AttackLabel::ProcessDamage)
          .with_system(recieve_damage)
      );
  }
}
