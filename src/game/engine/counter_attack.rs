use bevy::prelude::*;

use crate::{consts::AppState, game::{engine::entity::Controlling, stages::AttackPriority}};

use super::{attack::{AttackArea, AttackDamage, GroupAttack, AttackCoolDown}, entity::Position};

/// Entity which is at counter attack state
pub struct CounterAttack;

/// Entities which will recieve counter attack from player
pub struct CounterAttackTarget;

/// Trigger counter attack
/// this is a high priority attack
fn trigger_counter_attack(
  mut commands: Commands,
  mut attacks: ResMut<Vec<GroupAttack>>,
  mut mouse_input: ResMut<Input<MouseButton>>,
  query: Query<(Entity, &Position), (With<Controlling>, Without<AttackCoolDown>, With<CounterAttack>)>,
  obj_query: Query<Entity, With<CounterAttackTarget>>,
) {
  if mouse_input.just_pressed(MouseButton::Left) {
    if let Ok((entity, position)) = query.single() {
      mouse_input.clear_just_pressed(MouseButton::Left);

      commands.entity(entity)
        .remove::<CounterAttack>();

      attacks.push(GroupAttack {
        area: AttackArea::Circle {
          o: position.0,
          r: 350.0,
        },
        // find all enermies with CounterAttackTarget
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
          .label(AttackPriority::High)
          .with_system(trigger_counter_attack)
      );
  }
}
