use bevy::prelude::*;

use crate::{consts::AppState, game::GameSystemStage};

use super::{attack::{AttackArea, AttackDamage, GroupAttack}, cooldown::{AttackCoolDown, RemovalCoolDown, update_removal_cool_down}, entity::{CollideRadius, Player, PlayerState, Position, Velocity}, projectile::{BulletProps, ProjectileBundle}, soul::SoulPower};

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
    (With<Player>, Without<AttackCoolDown>, Without<ShieldPreviousAttack>)
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
    (With<Player>, Without<AttackCoolDown>)
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
    (With<Player>, Without<AttackCoolDown>)
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
    (With<Player>, Without<AttackCoolDown>, Without<ShieldPreviousAttack>),
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
    (With<Player>, Without<AttackCoolDown>),
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
    (With<Player>, Without<AttackCoolDown>),
  >,
  mouse_input: Res<Input<MouseButton>>,
) {
  if let Ok((entity, mut state, prev, mut soul)) = query.single_mut() {
    if *state == PlayerState::Stand && *prev == ShieldPreviousAttack::BB {
      if mouse_input.just_pressed(MouseButton::Right) {
        if soul.cost(30) {
          *state = PlayerState::ShieldAttackBB;
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
  mut attack: ResMut<Vec<GroupAttack>>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackA)>,
) {
  if let Ok((entity, position, mut atk)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackA>();

      attack.push(GroupAttack {
        area: AttackArea::HalfCircle {
          o: position.0,
          r: 150.0,
          // FIXME: find direction here
          v: Vec2::X,
        },
        entities: Vec::new(),
        damage: AttackDamage::Physical {
          damage: 30,
          power: 2,
        },
        from: entity,
      });
    }
  }
}

fn perform_shield_attack_aa(
  mut commands: Commands,
  time: Res<Time>,
  mut attack: ResMut<Vec<GroupAttack>>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackAA)>,
) {
  if let Ok((entity, position, mut atk)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackAA>();

      attack.push(GroupAttack {
        area: AttackArea::HalfCircle {
          o: position.0,
          r: 150.0,
          // FIXME: find direction here
          v: Vec2::X,
        },
        entities: Vec::new(),
        damage: AttackDamage::Physical {
          damage: 40,
          power: 2,
        },
        from: entity,
      });
    }
  }
}

fn perform_shield_attack_ab(
  mut commands: Commands,
  time: Res<Time>,
  mut attack: ResMut<Vec<GroupAttack>>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackAB)>,
) {
  if let Ok((entity, position, mut atk)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackAB>();

      attack.push(GroupAttack {
        area: AttackArea::Rectangle {
          o: position.0,
          w: 250.0,
          h: 110.0,
          // FIXME: find direction here
          v: Vec2::X,
        },
        entities: Vec::new(),
        damage: AttackDamage::Physical {
          damage: 40,
          power: 2,
        },
        from: entity,
      });
    }
  }
}

fn perform_shield_attack_b(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackB)>,
) {
  if let Ok((entity, position, mut atk)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackB>();

      commands.spawn_bundle(ProjectileBundle {
        position: position.clone(),
        // FIXME: correct direction
        velocity: Velocity(Vec2::X * 1800.0),
        bullet: BulletProps {
          owner: entity,
          damage: AttackDamage::Physical {
            damage: 20,
            power: 1,
          }
        },
        radius: CollideRadius(30.0),
      });
    }
  }
}

fn perform_shield_attack_bb(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackBB)>,
) {
  if let Ok((entity, position, mut atk)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackBB>();

      commands.spawn_bundle(ProjectileBundle {
        position: position.clone(),
        // FIXME: correct direction
        velocity: Velocity(Vec2::X * 1800.0),
        bullet: BulletProps {
          owner: entity,
          damage: AttackDamage::Physical {
            damage: 20,
            power: 1,
          }
        },
        radius: CollideRadius(30.0),
      });
    }
  }
}

fn perform_shield_attack_bbb(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &Position, &mut ShieldAttackBBB)>,
) {
  if let Ok((entity, position, mut atk)) = query.single_mut() {
    if atk.0.tick(time.delta()).just_finished() {
      commands.entity(entity)
        .remove::<ShieldAttackBBB>();

      commands.spawn_bundle(ProjectileBundle {
        position: position.clone(),
        // FIXME: correct direction
        velocity: Velocity(Vec2::X * 1800.0),
        bullet: BulletProps {
          owner: entity,
          damage: AttackDamage::Physical {
            damage: 35,
            power: 2,
          }
        },
        radius: CollideRadius(30.0),
      });
    }
  }
}

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameSystemStage::CoolDown)
          .with_system(update_removal_cool_down::<ShieldPreviousAttack>.system())
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameSystemStage::NormalAttack)
          .after(GameSystemStage::SpecialAttack)
          .with_system(trigger_shield_attack_a.system())
          .with_system(trigger_shield_attack_aa.system())
          .with_system(trigger_shield_attack_ab.system())
          .with_system(trigger_shield_attack_b.system())
          .with_system(trigger_shield_attack_bb.system())
          .with_system(trigger_shield_attack_bbb.system())
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameSystemStage::CreateDamage)
          .after(GameSystemStage::NormalAttack)
          .with_system(perform_shield_attack_a.system())
          .with_system(perform_shield_attack_aa.system())
          .with_system(perform_shield_attack_ab.system())
          .with_system(perform_shield_attack_b.system())
          .with_system(perform_shield_attack_bb.system())
          .with_system(perform_shield_attack_bbb.system())
      );
  }
}
