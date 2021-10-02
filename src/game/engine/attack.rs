use std::{collections::BTreeSet, iter::FromIterator};

use bevy::prelude::*;

use crate::{consts::AppState, game::{entity::attack::{AttackBundle, AttackSpriteType}, sprite::sprite::{SpriteRotation, SpriteSize}}};

use super::{entity::{CollideRadius, Position}, health::{Health, LockHealth}};

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
  /// ```text
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
  /// ```text
  ///  _
  /// | ``.
  /// |    `.
  /// o---r-|  v ---> 
  /// |    .`
  /// |_.-`
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
  /// ```text
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
fn flat_group_damage(
  mut commands: Commands,
  mut group_attacks: ResMut<Vec<GroupAttack>>,
  mut single_attacks: ResMut<Vec<SingleAttack>>,
  query: Query<(Entity, &Position, &CollideRadius)>,
) {
  for attack in group_attacks.iter() {
    let mut set: BTreeSet<Entity> = BTreeSet::from_iter(attack.entities.clone().into_iter());

    // XXX: debug here
    println!("{:?}", attack);

    // insert effects
    match &attack.area {
      &AttackArea::HalfCircle { o, r, v } => {
        commands.spawn_bundle(AttackBundle {
          position: Position(o),
          // TODO: use atan2 is not very well...
          rotation: SpriteRotation(Quat::from_rotation_z(v.y.atan2(v.x))),
          scale: SpriteSize(Vec2::new(2.0 * r, 2.0 * r)),
          area: AttackSpriteType::HalfCircle,
          ..Default::default()
        });
      }
      &AttackArea::Circle { o, r } => {
        commands.spawn_bundle(AttackBundle {
          position: Position(o),
          scale: SpriteSize(Vec2::new(2.0 * r, 2.0 * r)),
          area: AttackSpriteType::Circle,
          ..Default::default()
        });
      }
      &AttackArea::Rectangle { o, w, h, v } => {
        commands.spawn_bundle(AttackBundle {
          position: Position(o + v * w / 2.0),
          // TODO: use atan2 is not very well...
          rotation: SpriteRotation(Quat::from_rotation_z(v.y.atan2(v.x))),
          scale: SpriteSize(Vec2::new(w, h)),
          area: AttackSpriteType::Rectangle,
          ..Default::default()
        });
      }
    }

    for (entity, position, radius) in query.iter() {

      // damage doesn't hurt self
      if attack.from == Some(entity) {
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
  mut query: Query<(&mut Health, Option<&LockHealth>)>,
) {
  for attack in attacks.iter() {
    if let Ok((mut health, lock_health)) = query.get_mut(attack.entity) {

      // FIXME: very original
      let damage = match attack.damage {
        AttackDamage::Physical { damage, .. } => damage,
        AttackDamage::Magical { damage } => damage,
      };

      if lock_health.is_some() {
        health.recieve_damage_locked(damage);
      } else {
        health.recieve_damage(damage);
      }

      // TODO: play repellent here
    }
  }

  attacks.clear();
}

macro_rules! create_cool_down_system {
  ($func_name:ident, $t:ty) => {
    fn $func_name(
      time: Res<Time>,
      mut commands: Commands,
      mut query: Query<(Entity, &mut $t)>,
    ) {
      query.iter_mut().for_each(|(entity, mut cd)| {
        if cd.0.tick(time.delta()).finished() {
          commands.entity(entity).remove::<$t>();
        }
      });
    }
  };
}

pub struct AttackCoolDown(pub Timer);
pub struct AssaultCoolDown(pub Timer);

create_cool_down_system!(update_attack_cool_down, AttackCoolDown);
create_cool_down_system!(update_assault_cool_down, AssaultCoolDown);

/// Remove entity itself
pub struct RemovalCoolDown(pub Timer);

fn update_removal_cool_down(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &mut RemovalCoolDown)>
) {
  query.iter_mut().for_each(|(entity, mut cd)| {
    if cd.0.tick(time.delta()).finished() {
      commands.entity(entity).despawn_recursive();
    }
  });
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource::<Vec<GroupAttack>>(Vec::new())
      .insert_resource::<Vec<SingleAttack>>(Vec::new())
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .with_system(flat_group_damage)
          .with_system(recieve_damage)
          .with_system(update_attack_cool_down)
          .with_system(update_assault_cool_down)
          .with_system(update_removal_cool_down)
      );
  }
}
