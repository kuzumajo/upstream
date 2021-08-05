use bevy::prelude::*;

#[derive(Debug, Hash, Clone, Eq, PartialEq, SystemLabel)]
pub enum GameSystemStage {
  /// physical engine
  UpdatePosition,
  /// remove cool down tags if they were finished
  CoolDown,
  /// Perform special attack
  SpecialAttack,
  /// Perform normal attack
  NormalAttack,
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
}
