use bevy::prelude::*;

use crate::{consts::AppState, game::{engine::entity::Controlling, stages::{AttackLabel, GameEngineLabel, TriggerAttackLabel}}};

use super::{attack::{AttackArea, AttackDamage, GroupAttack}, cooldown::{AttackCoolDown, RemovalCoolDown}, entity::Position};

pub struct CounterAttack;

pub struct CounterAttackObject;

/// Trigger counter attack
fn trigger_counter_attack(
  mut commands: Commands,
  mut attacks: ResMut<Vec<GroupAttack>>,
  mouse_input: Res<Input<MouseButton>>,
  query: Query<(Entity, &Position), (With<Controlling>, Without<AttackCoolDown>, With<CounterAttack>)>,
  obj_query: Query<Entity, With<CounterAttackObject>>,
) {
  if mouse_input.just_pressed(MouseButton::Left) {
    if let Ok((entity, position)) = query.single() {
      commands.entity(entity)
        // add 0s attack cd, which will be removed in next frame
        .insert(AttackCoolDown)
        .insert(RemovalCoolDown::<AttackCoolDown>::new(0.0))
        // remove counter attack
        .remove::<CounterAttack>();

      attacks.push(GroupAttack {
        area: AttackArea::Circle {
          o: position.0,
          r: 350.0,
        },
        // find all enermies with CounterAttackObject
        entities: obj_query.iter().collect(),
        damage: AttackDamage::Physical {
          damage: 50,
          power: 2,
        },
        from: Some(entity),
      });
    }
  }
}

pub struct CounterAttackPlugin;

impl Plugin for CounterAttackPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::UpdateAttacks)
          .after(GameEngineLabel::UpdatePhysics)
          .label(AttackLabel::TriggerAttack)
          .label(TriggerAttackLabel::TriggerSpecialAttack)
          .with_system(trigger_counter_attack)
      );
  }
}
