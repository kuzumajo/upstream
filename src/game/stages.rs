use bevy::prelude::*;

#[derive(Debug, Hash, Clone, Eq, PartialEq, SystemLabel)]
pub enum GameEngineLabel {
  /// cool down every tags
  CoolDown,
  /// update all physics
  UpdatePhysics,
  /// update attacks
  UpdateAttacks,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, SystemLabel)]
pub enum PhysicsLabel {
  /// update velocity accordings to AI / user input
  UpdateVelocity,
  /// update positions accordings to velocity
  UpdatePosition,
  /// make entity collides back if strait into wall
  CheckCollision,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, SystemLabel)]
pub enum AttackLabel {
  /// trigger attack according to user input
  TriggerAttack,
  /// perform attacks
  PerformAttack,
  /// flat group attack into single attacks
  ProcessDamage,
  /// calculate damage to entity and make effects to health
  RecieveDamage,
  /// check if somebody dies and mark it as Dead
  CheckDead,
  /// remove Dead entities
  RemoveDead,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, SystemLabel)]
pub enum TriggerAttackLabel {
  /// special attacks
  TriggerSpecialAttack,
  /// normal attacks
  TriggerNormalAttack,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, SystemLabel)]
pub enum SpriteLabel {
  /// update sprite handle
  UpdateSpriteSheet,
  /// sprite animations
  SpriteAnimation,
}
