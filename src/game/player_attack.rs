use crate::consts::*;
use bevy::prelude::*;

enum PlayerAttackState {
  /// first animation stage of A+A & A+B
  AttackA(Timer),
  /// second animation stage of A+A
  AttackAA(Timer),
  /// second animation stage of A+B
  AttackAB(Timer),
  /// first animation stage of B+B+B
  AttackB(Timer),
  /// second animation stage of B+B+B
  AttackBB(Timer),
  /// last animation stage of B+B+B
  AttackBBB(Timer),

  /// both A & B clicked
  Parry,

  /// Player assault
  Assault(Vec2, Timer),
  /// Assault + A
  AssaultA(Vec2, Timer),
  /// Assault + B
  AssaultB(Vec2, Timer),

  Stand,
}

enum PlayerAttackFirstStage {
  AttackA(Timer),
  AttackB(Timer),
}

enum PlayerAttackSecondStage {
  AttackBB(Timer),
}

struct Player;

/// Player can not attack again
/// <https://kuzumajo.github.io/wiki/#/combat/state?id=%e6%99%ae%e6%94%bb%e5%86%b7%e5%8d%b4>
struct PlayerAttackCoolDown(Timer);

/// Player can not assault again
/// <https://kuzumajo.github.io/wiki/#/combat/state?id=%e5%86%b2%e5%88%ba%e5%86%b7%e5%8d%b4>
struct PlayerAssaultCoolDown(Timer);

fn attack_cool_down(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &mut PlayerAttackCoolDown), With<Player>>,
) {
  for (entity, mut cd) in query.iter_mut() {
    if cd.0.tick(time.delta()).finished() {
      commands.entity(entity).remove::<PlayerAttackCoolDown>();
    }
  }
}

fn assault_cool_down(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(Entity, &mut PlayerAssaultCoolDown), With<Player>>,
) {
  for (entity, mut cd) in query.iter_mut() {
    if cd.0.tick(time.delta()).finished() {
      commands.entity(entity).remove::<PlayerAssaultCoolDown>();
    }
  }
}

/*

.- player clicked A
|     .- 0.2s           .- 1.5s
+-----+-----------------+

[-----]                   : PlayerAttackState::AttackA
[-----]                   : PlayerAttackCoolDown
[-----------------------] : PlayerAttackFirstStage::AttackA

when in
      [--------+--------]
               |
               `- player clicked B : enter new stage A+B
*/

/// Stand => AttackA,
/// Stand => AttackB
fn emit_first_attack_stage(
  mut commands: Commands,
  mut query: Query<
    (Entity, &mut PlayerAttackState),
    (
      With<Player>,
      Without<PlayerAttackCoolDown>,
      Without<PlayerAttackFirstStage>,
      Without<PlayerAttackSecondStage>,
    )
  >,
  mouse_button_input: Res<Input<MouseButton>>,
) {
  for (entity, mut state) in query.iter_mut() {

    match *state {
      PlayerAttackState::Stand => {
        // attack A
        if mouse_button_input.just_pressed(MouseButton::Left) {
          *state = PlayerAttackState::AttackA(Timer::from_seconds(0.2, false));
          commands
            .entity(entity)
            .insert(PlayerAttackFirstStage::AttackA(Timer::from_seconds(1.5, false)))
            .insert(PlayerAttackCoolDown(Timer::from_seconds(0.2, false)));
        }
        // attack B
        if mouse_button_input.just_pressed(MouseButton::Right) {
          *state = PlayerAttackState::AttackB(Timer::from_seconds(0.2, false));
          commands
            .entity(entity)
            .insert(PlayerAttackFirstStage::AttackB(Timer::from_seconds(1.5, false)))
            .insert(PlayerAttackCoolDown(Timer::from_seconds(0.2, false)));
        }
      }
      _ => { }
    }

  }
}

/// AttackA => AttackAA,
/// AttackA => AttackAB,
/// AttackB => AttackBB
fn emit_second_attack_stage(
  mut commands: Commands,
  mut query: Query<
    (Entity, &PlayerAttackFirstStage, &mut PlayerAttackState),
    (With<Player>, Without<PlayerAttackCoolDown>),
  >,
  mouse_button_input: Res<Input<MouseButton>>,
) {
  for (entity, prev, mut state) in query.iter_mut() {
    match prev {
      PlayerAttackFirstStage::AttackA(_) => {
        if mouse_button_input.just_pressed(MouseButton::Left) {
          // A + A
          *state = PlayerAttackState::AttackAA(Timer::from_seconds(0.4, false));
          commands.entity(entity)
            .remove::<PlayerAttackFirstStage>()
            // FIXME: here should cool down for 1.0s or 1.4s?
            .insert(PlayerAttackCoolDown(Timer::from_seconds(1.0, false)));
        } else if mouse_button_input.just_pressed(MouseButton::Right) {
          // A + B
          *state = PlayerAttackState::AttackAB(Timer::from_seconds(0.1, false));
          commands.entity(entity)
            .remove::<PlayerAttackFirstStage>()
            // FIXME: here should cool down for 1.0s or 1.1s?
            .insert(PlayerAttackCoolDown(Timer::from_seconds(1.0, false)));
        }
      }
      PlayerAttackFirstStage::AttackB(_) => {
        if mouse_button_input.just_pressed(MouseButton::Right) {
          // B + B
          *state = PlayerAttackState::AttackBB(Timer::from_seconds(0.2, false));
          commands.entity(entity)
            .remove::<PlayerAttackFirstStage>()
            .insert(PlayerAttackSecondStage::AttackBB(Timer::from_seconds(0.2, false)))
            .insert(PlayerAttackCoolDown(Timer::from_seconds(0.2, false)));
        }
      }
    }
  }
}

/// AttackBB => AttackBBB
fn emit_third_attack_stage(
  mut commands: Commands,
  mut query: Query<
    (Entity, &PlayerAttackSecondStage, &mut PlayerAttackState),
    (With<Player>, Without<PlayerAttackCoolDown>),
  >,
  mouse_button_input: Res<Input<MouseButton>>,
) {
  for (entity, prev, mut state) in query.iter_mut() {
    match prev {
      PlayerAttackSecondStage::AttackBB(_) => {
        if mouse_button_input.just_pressed(MouseButton::Right) {
          *state = PlayerAttackState::AttackBBB(Timer::from_seconds(0.4, false));
          commands.entity(entity)
            .remove::<PlayerAttackSecondStage>()
            // FIXME: here should cool down for 1.0s or 1.4s?
            .insert(PlayerAttackCoolDown(Timer::from_seconds(1.0, false)));
        }
      }
    }
  }
}

/// AttackA => Parry
/// AttackB => Parry
fn emit_parry_state(
  mut commands: Commands,
  mut query: Query<
    (Entity, &PlayerAttackFirstStage, &mut PlayerAttackState),
    With<Player>,
  >,
  mouse_button_input: Res<Input<MouseButton>>,
) {
  for (entity, stage, mut state) in query.iter_mut() {
    let button = match stage {
      &PlayerAttackFirstStage::AttackA(_) => MouseButton::Right,
      &PlayerAttackFirstStage::AttackB(_) => MouseButton::Left,
    };
    if mouse_button_input.just_pressed(button) {
      *state = PlayerAttackState::Parry;
      commands.entity(entity)
        .remove::<PlayerAttackFirstStage>();
    }
  } 
}

/// Parry => Stand
fn leave_parry_state(
  mut query: Query<(Entity, &mut PlayerAttackState), With<Player>>,
  mouse_button_input: Res<Input<MouseButton>>,
) {
  use PlayerAttackState::*;
  for (_entity, mut state) in query.iter_mut() {
    match *state {
      Parry => {
        if mouse_button_input.just_released(MouseButton::Left) {
          // TODO
          *state = Stand;
        } else if mouse_button_input.just_released(MouseButton::Right) {
          // TODO
          *state = Stand;
        }
      }
      _ => { }
    }
  }
}

/// Stand => Assault
/// Attack* => Assault
/// Parry => Assault (FIXME)
fn emit_assault_state(
  mut query: Query<&mut PlayerAttackState, With<Player>>,
  keycode_input: Res<Input<KeyCode>>,
) {
  for mut state in query.iter_mut() {
    if keycode_input.just_pressed(KeyCode::Space) {
      // FIXME: direction to mouse
      *state = PlayerAttackState::Assault(Vec2::X, Timer::from_seconds(0.5, false));
    }
  }
}

fn emit_assault_attack_state(
  mut query: Query<&mut PlayerAttackState, With<Player>>,
  mouse_button_input: Res<Input<MouseButton>>,
) {
  for mut state in query.iter_mut() {
    if let PlayerAttackState::Assault(vec, ref timer) = *state {
      if mouse_button_input.just_pressed(MouseButton::Left) {
        *state = PlayerAttackState::AssaultA(vec, timer.clone());
      } else if mouse_button_input.just_pressed(MouseButton::Right) {
        *state = PlayerAttackState::AssaultB(vec, timer.clone());
      }
    }
  }
}

fn attack_first_stage_cool_down(
  time: Res<Time>,
  mut commands: Commands,
  mut query: Query<(Entity, &mut PlayerAttackFirstStage)>,
) {
  use PlayerAttackFirstStage::*;
  for (entity, mut stage) in query.iter_mut() {
    let timer = match *stage {
      AttackA(ref mut timer) => timer,
      AttackB(ref mut timer) => timer,
    };
    if timer.tick(time.delta()).finished() {
      commands.entity(entity)
        .remove::<PlayerAttackFirstStage>();
    }
  }
}

fn attack_second_stage_cool_down(
  time: Res<Time>,
  mut commands: Commands,
  mut query: Query<(Entity, &mut PlayerAttackSecondStage)>,
) {
  use PlayerAttackSecondStage::*;
  for (entity, mut stage) in query.iter_mut() {
    let timer = match *stage {
      AttackBB(ref mut timer) => timer,
    };
    if timer.tick(time.delta()).finished() {
      commands.entity(entity)
        .remove::<PlayerAttackSecondStage>();
    }
  }
}

/// Perform player attacks
pub struct PlayerAttackPlugin;

impl Plugin for PlayerAttackPlugin {
  fn build(&self, app: &mut App) {

    app
      .add_system_set(SystemSet::on_update(AppState::InGame)
        .label(GameLabel::CoolDown)
        .with_system(attack_cool_down.system())
        .with_system(assault_cool_down.system())
        .with_system(attack_first_stage_cool_down.system())
        .with_system(attack_second_stage_cool_down.system()))
      .add_system_set(SystemSet::on_update(AppState::InGame)
        .label(GameLabel::SpecialPlayerState)
        .after(GameLabel::CoolDown)
        .with_system(emit_second_attack_stage.system())
        .with_system(emit_third_attack_stage.system())
        .with_system(emit_parry_state.system())
        .with_system(leave_parry_state.system())
        .with_system(emit_assault_state.system())
        .with_system(emit_assault_attack_state.system()))
      .add_system_set(SystemSet::on_update(AppState::InGame)
        .label(GameLabel::NormalPlayerState)
        .after(GameLabel::SpecialPlayerState)
        .with_system(emit_first_attack_stage.system()));
  }
}
