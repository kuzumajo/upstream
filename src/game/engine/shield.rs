use bevy::prelude::*;

use crate::{consts::{AppState, PLAYER_SHIELD_BULLET_SPEED}, game::{MouseDirection, entity::projectile::ProjectileBundle, sprite::sprite::SpriteSize, stages::AttackPriority}};

use super::{attack::{AttackArea, AttackCoolDown, AttackDamage, GroupAttack}, entity::{CollideRadius, Controlling, PlayerState, Position, Velocity}, projectile::BulletProps, soul::SoulPower};

/// Used to check trigger result
enum ShieldAttackType {
  A,
  AA,
  AB,
  B,
  BB,
  BBB,
}

/// Used to refer next attack
struct ShieldAttackPrefix(Timer, ShieldAttackType);

/// Used to trigger attack
struct ShieldAttackAnimation(Timer);

/// Trigger all kinds of common attacks in shield
fn trigger_shield_common_attack(
  mut commands: Commands,
  mut query: Query<
    (Entity, &mut PlayerState, Option<&ShieldAttackPrefix>, &mut SoulPower),
    (With<Controlling>, Without<AttackCoolDown>)
  >,
  mouse_input: Res<Input<MouseButton>>,
) {
  if let Ok((entity, mut state, prev, mut soul)) = query.single_mut() {
    if *state != PlayerState::Stand {
      return;
    }

    // Here should be a Trie Tree, but I'm lazy
    let result = match prev {
      None =>
        if mouse_input.just_pressed(MouseButton::Left) {
          Some(ShieldAttackType::A)
        } else if mouse_input.just_pressed(MouseButton::Right) && soul.cost(20) {
          Some(ShieldAttackType::B)
        } else {
          None
        }
      Some(&ShieldAttackPrefix(_, ShieldAttackType::A)) => 
        if mouse_input.just_pressed(MouseButton::Left) {
          Some(ShieldAttackType::AA)
        } else if mouse_input.just_pressed(MouseButton::Right) && soul.cost(30) {
          Some(ShieldAttackType::AB)
        } else {
          None
        }
      Some(&ShieldAttackPrefix(_, ShieldAttackType::B)) =>
        if mouse_input.just_pressed(MouseButton::Left) {
          Some(ShieldAttackType::A)
        } else if mouse_input.just_pressed(MouseButton::Right) && soul.cost(20) {
          Some(ShieldAttackType::BB)
        } else {
          None
        }
      Some(&ShieldAttackPrefix(_, ShieldAttackType::BB)) =>
        if mouse_input.just_pressed(MouseButton::Left) {
          Some(ShieldAttackType::A)
        } else if mouse_input.just_pressed(MouseButton::Right) && soul.cost(20) {
          Some(ShieldAttackType::BBB)
        } else {
          None
        }
      _ => None,
    };

    match result {
      None => { }
      Some(ShieldAttackType::A) => {
        *state = PlayerState::ShieldAttackA;
        commands.entity(entity)
          .insert(AttackCoolDown(Timer::from_seconds(0.2, false)))
          .insert(ShieldAttackAnimation(Timer::from_seconds(0.2, false)))
          .insert(ShieldAttackPrefix(Timer::from_seconds(1.5, false), ShieldAttackType::A));
      }
      Some(ShieldAttackType::AA) => {
        *state = PlayerState::ShieldAttackAA;
        commands.entity(entity)
          .remove::<ShieldAttackPrefix>()
          .insert(AttackCoolDown(Timer::from_seconds(1.4, false)))
          .insert(ShieldAttackAnimation(Timer::from_seconds(0.4, false)));
      }
      Some(ShieldAttackType::AB) => {
        *state = PlayerState::ShieldAttackAB;
        commands.entity(entity)
          .remove::<ShieldAttackPrefix>()
          .insert(AttackCoolDown(Timer::from_seconds(1.1, false)))
          .insert(ShieldAttackAnimation(Timer::from_seconds(0.1, false)));
      }
      Some(ShieldAttackType::B) => {
        *state = PlayerState::ShieldAttackB;
        commands.entity(entity)
          .insert(AttackCoolDown(Timer::from_seconds(0.2, false)))
          .insert(ShieldAttackAnimation(Timer::from_seconds(0.2, false)))
          .insert(ShieldAttackPrefix(Timer::from_seconds(1.0, false), ShieldAttackType::B));
      }
      Some(ShieldAttackType::BB) => {
        *state = PlayerState::ShieldAttackBB;
        commands.entity(entity)
          .insert(AttackCoolDown(Timer::from_seconds(0.2, false)))
          .insert(ShieldAttackAnimation(Timer::from_seconds(0.2, false)))
          .insert(ShieldAttackPrefix(Timer::from_seconds(1.0, false), ShieldAttackType::BB));
      }
      Some(ShieldAttackType::BBB) => {
        *state = PlayerState::ShieldAttackBBB;
        commands.entity(entity)
          .remove::<ShieldAttackPrefix>()
          .insert(AttackCoolDown(Timer::from_seconds(1.4, false)))
          .insert(ShieldAttackAnimation(Timer::from_seconds(0.4, false)));
      }
    }
  }
}

fn perform_shield_common_attack(
  mut commands: Commands,
  time: Res<Time>,
  direction: Res<MouseDirection>,
  mut attack: ResMut<Vec<GroupAttack>>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackAnimation, &mut PlayerState)>,
) {
  if let Ok((entity, position, mut animation, mut state)) = query.single_mut() {
    if animation.0.tick(time.delta()).finished() {
      commands.entity(entity).remove::<ShieldAttackAnimation>();

      match *state {
        PlayerState::ShieldAttackA => {
          attack.push(GroupAttack {
            area: AttackArea::HalfCircle {
              o: position.0,
              r: 150.0,
              v: direction.0,
            },
            entities: Vec::new(),
            damage: AttackDamage::Physical {
              damage: 30,
              power: 2,
            },
            from: Some(entity),
          });
        }
        PlayerState::ShieldAttackAA => {
          attack.push(GroupAttack {
            area: AttackArea::HalfCircle {
              o: position.0,
              r: 150.0,
              v: direction.0,
            },
            entities: Vec::new(),
            damage: AttackDamage::Physical {
              damage: 40,
              power: 2,
            },
            from: Some(entity),
          });
        }
        PlayerState::ShieldAttackAB => {
          attack.push(GroupAttack {
            area: AttackArea::Rectangle {
              o: position.0,
              w: 250.0,
              h: 110.0,
              v: direction.0,
            },
            entities: Vec::new(),
            damage: AttackDamage::Physical {
              damage: 40,
              power: 2,
            },
            from: Some(entity),
          });
        }
        PlayerState::ShieldAttackB => {
          commands.spawn_bundle(ProjectileBundle {
            position: position.clone(),
            velocity: Velocity(direction.0 * PLAYER_SHIELD_BULLET_SPEED),
            bullet: BulletProps {
              owner: Some(entity),
              damage: Some(AttackDamage::Physical {
                damage: 20,
                power: 1,
              })
            },
            radius: CollideRadius(30.0),
            scale: SpriteSize(Vec2::new(60.0, 60.0)),
            ..Default::default()
          });
        }
        PlayerState::ShieldAttackBB => {
          commands.spawn_bundle(ProjectileBundle {
            position: position.clone(),
            velocity: Velocity(direction.0 * PLAYER_SHIELD_BULLET_SPEED),
            bullet: BulletProps {
              owner: Some(entity),
              damage: Some(AttackDamage::Physical {
                damage: 20,
                power: 1,
              })
            },
            radius: CollideRadius(30.0),
            scale: SpriteSize(Vec2::new(60.0, 60.0)),
            ..Default::default()
          });
        }
        PlayerState::ShieldAttackBBB => {
          commands.spawn_bundle(ProjectileBundle {
            position: position.clone(),
            velocity: Velocity(direction.0 * PLAYER_SHIELD_BULLET_SPEED),
            bullet: BulletProps {
              owner: Some(entity),
              damage: Some(AttackDamage::Physical {
                damage: 35,
                power: 2,
              }),
            },
            radius: CollideRadius(30.0),
            scale: SpriteSize(Vec2::new(60.0, 60.0)),
            ..Default::default()
          });
        }
        _ => { warn!("unexpected PlayerState detected"); println!("{:?}", *state); }
      }
      *state = PlayerState::Stand;
    }
  }
}

create_cool_down_system!(update_shield_attack_prefix, ShieldAttackPrefix);

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .with_system(trigger_shield_common_attack.label(AttackPriority::Normal))
          .with_system(perform_shield_common_attack)
          .with_system(update_shield_attack_prefix)
      );
  }
}
