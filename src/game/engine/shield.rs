use bevy::prelude::*;

use crate::{consts::{AppState, PLAYER_SHIELD_BULLET_SPEED}, game::{MouseDirection, entity::projectile::{ProjectileBundle}, stages::{AttackLabel, GameEngineLabel, TriggerAttackLabel}}};

use super::{attack::{AttackArea, AttackDamage, GroupAttack}, cooldown::{AttackCoolDown, RemovalCoolDown, update_removal_cool_down}, entity::{CollideRadius, Controlling, PlayerState, Position, Velocity}, projectile::BulletProps, soul::SoulPower};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ShieldPreviousAttack {
  A,
  B,
  BB,
}

struct ShieldAttackA(Timer);
struct ShieldAttackAA(Timer);
struct ShieldAttackAB(Timer);
struct ShieldAttackB(Timer);
struct ShieldAttackBB(Timer);
struct ShieldAttackBBB(Timer);

fn trigger_shield_attack_a(
  mut commands: Commands,
  mut query: Query<
    (Entity, &mut PlayerState),
    (With<Controlling>, Without<AttackCoolDown>, Without<ShieldPreviousAttack>)
  >,
  mouse_input: Res<Input<MouseButton>>,
) {
  if let Ok((entity, mut state)) = query.single_mut() {
    if *state == PlayerState::Stand {
      if mouse_input.just_pressed(MouseButton::Left) {
        *state = PlayerState::ShieldAttackA;
        commands.entity(entity)
          .insert(ShieldPreviousAttack::A)
          .insert(RemovalCoolDown::<ShieldPreviousAttack>::new(1.5))
          .insert(AttackCoolDown)
          .insert(RemovalCoolDown::<AttackCoolDown>::new(0.2))
          .insert(ShieldAttackA(Timer::from_seconds(0.2, false)));
      }
    }
  }
}

fn trigger_shield_attack_aa(
  mut commands: Commands,
  mut query: Query<
    (Entity, &mut PlayerState, &ShieldPreviousAttack),
    (With<Controlling>, Without<AttackCoolDown>)
  >,
  mouse_input: Res<Input<MouseButton>>,
) {
  if let Ok((entity, mut state, prev)) = query.single_mut() {
    if *state == PlayerState::Stand && *prev == ShieldPreviousAttack::A {
      if mouse_input.just_pressed(MouseButton::Left) {
        *state = PlayerState::ShieldAttackAA;
        commands.entity(entity)
          .remove::<ShieldPreviousAttack>()
          .remove::<RemovalCoolDown<ShieldPreviousAttack>>()
          .insert(AttackCoolDown)
          .insert(RemovalCoolDown::<AttackCoolDown>::new(1.4))
          .insert(ShieldAttackAA(Timer::from_seconds(0.4, false)));
      }
    }
  }
}

fn trigger_shield_attack_ab(
  mut commands: Commands,
  mut query: Query<
    (Entity, &mut PlayerState, &ShieldPreviousAttack, &mut SoulPower),
    (With<Controlling>, Without<AttackCoolDown>)
  >,
  mouse_input: Res<Input<MouseButton>>,
) {
  if let Ok((entity, mut state, prev, mut soul)) = query.single_mut() {
    if *state == PlayerState::Stand && *prev == ShieldPreviousAttack::A {
      if mouse_input.just_pressed(MouseButton::Right) {
        if soul.cost(30) {
          *state = PlayerState::ShieldAttackAB;
          commands.entity(entity)
            .remove::<ShieldPreviousAttack>()
            .remove::<RemovalCoolDown<ShieldPreviousAttack>>()
            .insert(AttackCoolDown)
            .insert(RemovalCoolDown::<AttackCoolDown>::new(1.1))
            .insert(ShieldAttackAB(Timer::from_seconds(0.1, false)));
        }
      }
    }
  }
}

fn trigger_shield_attack_b(
  mut commands: Commands,
  mut query: Query<
    (Entity, &mut PlayerState, &mut SoulPower),
    (With<Controlling>, Without<AttackCoolDown>, Without<ShieldPreviousAttack>),
  >,
  mouse_input: Res<Input<MouseButton>>,
) {
  if let Ok((entity, mut state, mut soul)) = query.single_mut() {
    if *state == PlayerState::Stand {
      if mouse_input.just_pressed(MouseButton::Right) {
        if soul.cost(20) {
          *state = PlayerState::ShieldAttackB;
          commands.entity(entity)
            .insert(ShieldPreviousAttack::B)
            .insert(RemovalCoolDown::<ShieldPreviousAttack>::new(1.0))
            .insert(AttackCoolDown)
            .insert(RemovalCoolDown::<AttackCoolDown>::new(0.2))
            .insert(ShieldAttackB(Timer::from_seconds(0.2, false)));
        }
      }
    }
  }
}

fn trigger_shield_attack_bb(
  mut commands: Commands,
  mut query: Query<
    (Entity, &mut PlayerState, &ShieldPreviousAttack, &mut SoulPower),
    (With<Controlling>, Without<AttackCoolDown>),
  >,
  mouse_input: Res<Input<MouseButton>>,
) {
  if let Ok((entity, mut state, prev, mut soul)) = query.single_mut() {
    if *state == PlayerState::Stand && *prev == ShieldPreviousAttack::B {
      if mouse_input.just_pressed(MouseButton::Right) {
        if soul.cost(20) {
          *state = PlayerState::ShieldAttackBB;
          commands.entity(entity)
            .insert(ShieldPreviousAttack::BB)
            .insert(RemovalCoolDown::<ShieldPreviousAttack>::new(1.0))
            .insert(AttackCoolDown)
            .insert(RemovalCoolDown::<AttackCoolDown>::new(0.2))
            .insert(ShieldAttackBB(Timer::from_seconds(0.2, false)));
        }
      }
    }
  }
}

fn trigger_shield_attack_bbb(
  mut commands: Commands,
  mut query: Query<
    (Entity, &mut PlayerState, &ShieldPreviousAttack, &mut SoulPower),
    (With<Controlling>, Without<AttackCoolDown>),
  >,
  mouse_input: Res<Input<MouseButton>>,
) {
  if let Ok((entity, mut state, prev, mut soul)) = query.single_mut() {
    if *state == PlayerState::Stand && *prev == ShieldPreviousAttack::BB {
      if mouse_input.just_pressed(MouseButton::Right) {
        if soul.cost(30) {
          *state = PlayerState::ShieldAttackBBB;
          commands.entity(entity)
            .remove::<ShieldPreviousAttack>()
            .remove::<RemovalCoolDown<ShieldPreviousAttack>>()
            .insert(AttackCoolDown)
            .insert(RemovalCoolDown::<AttackCoolDown>::new(1.4))
            .insert(ShieldAttackBBB(Timer::from_seconds(0.4, false)));
        }
      }
    }
  }
}

fn perform_shield_attack_a(
  mut commands: Commands,
  time: Res<Time>,
  direction: Res<MouseDirection>,
  mut attack: ResMut<Vec<GroupAttack>>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackA, &mut PlayerState)>,
) {
  if let Ok((entity, position, mut atk, mut state)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackA>();

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

      *state = PlayerState::Stand;
    }
  }
}

fn perform_shield_attack_aa(
  mut commands: Commands,
  time: Res<Time>,
  direction: Res<MouseDirection>,
  mut attack: ResMut<Vec<GroupAttack>>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackAA, &mut PlayerState)>,
) {
  if let Ok((entity, position, mut atk, mut state)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackAA>();

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

      *state = PlayerState::Stand;
    }
  }
}

fn perform_shield_attack_ab(
  mut commands: Commands,
  time: Res<Time>,
  direction: Res<MouseDirection>,
  mut attack: ResMut<Vec<GroupAttack>>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackAB, &mut PlayerState)>,
) {
  if let Ok((entity, position, mut atk, mut state)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackAB>();

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

      *state = PlayerState::Stand;
    }
  }
}

fn perform_shield_attack_b(
  mut commands: Commands,
  time: Res<Time>,
  direction: Res<MouseDirection>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackB, &mut PlayerState)>,
) {
  if let Ok((entity, position, mut atk, mut state)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackB>();

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
        ..Default::default()
      });

      *state = PlayerState::Stand;
    }
  }
}

fn perform_shield_attack_bb(
  mut commands: Commands,
  time: Res<Time>,
  direction: Res<MouseDirection>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackBB, &mut PlayerState)>,
) {
  if let Ok((entity, position, mut atk, mut state)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackBB>();

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
        ..Default::default()
      });

      *state = PlayerState::Stand;
    }
  }
}

fn perform_shield_attack_bbb(
  mut commands: Commands,
  time: Res<Time>,
  direction: Res<MouseDirection>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackBBB, &mut PlayerState)>,
) {
  if let Ok((entity, position, mut atk, mut state)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackBBB>();

      commands.spawn_bundle(ProjectileBundle {
        position: position.clone(),
        velocity: Velocity(direction.0 * PLAYER_SHIELD_BULLET_SPEED),
        bullet: BulletProps {
          owner: Some(entity),
          damage: Some(AttackDamage::Physical {
            damage: 35,
            power: 2,
          })
        },
        radius: CollideRadius(30.0),
        ..Default::default()
      });

      *state = PlayerState::Stand;
    }
  }
}

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::CoolDown)
          .with_system(update_removal_cool_down::<ShieldPreviousAttack>)
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::UpdateAttacks)
          .after(GameEngineLabel::UpdatePhysics)
          .label(AttackLabel::TriggerAttack)
          .label(TriggerAttackLabel::TriggerNormalAttack)
          .after(TriggerAttackLabel::TriggerSpecialAttack)
          .with_system(trigger_shield_attack_a)
          .with_system(trigger_shield_attack_aa)
          .with_system(trigger_shield_attack_ab)
          .with_system(trigger_shield_attack_b)
          .with_system(trigger_shield_attack_bb)
          .with_system(trigger_shield_attack_bbb)
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::UpdateAttacks)
          .after(GameEngineLabel::UpdatePhysics)
          .label(AttackLabel::PerformAttack)
          .after(AttackLabel::TriggerAttack)
          .with_system(perform_shield_attack_a)
          .with_system(perform_shield_attack_aa)
          .with_system(perform_shield_attack_ab)
          .with_system(perform_shield_attack_b)
          .with_system(perform_shield_attack_bb)
          .with_system(perform_shield_attack_bbb)
      );
  }
}
