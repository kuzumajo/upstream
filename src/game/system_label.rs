use bevy::prelude::*;

#[derive(Debug, Hash, Clone, Eq, PartialEq, SystemLabel)]
pub enum GameSystemStage {
  /// remove cool down tags if they where finished
  CoolDown,
  /// calculate damages created in the whole map
  CreateDamage,
  /// turn all group damages to single damages
  ProcessDamage,
  /// perform damage to target entities
  RecieveDamage,
  /// do some fix after damages were calculated and modified
  PostDamage,
  /// mark health 0 as Dead
  CheckDead,
  /// remove all Dead entities
  ClearDead,

  /// Perform special attack
  SpecialAttack,
  /// Perform normal attack
  NormalAttack,
}
