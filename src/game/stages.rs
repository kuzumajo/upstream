use bevy::prelude::*;

#[derive(Debug, Hash, Clone, Eq, PartialEq, SystemLabel)]
pub enum PhysicsLabel {
  /// update velocity accordings to AI / user input
  UpdateVelocity,
  /// update positions accordings to velocity.
  /// also checks if it collides with other entities.
  UpdatePosition,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, SystemLabel)]
/// Priority of attacks to trigger.
/// Trigger from higher to lower.
pub enum AttackPriority {
  /// low priority
  Low,
  /// normal priority
  Normal,
  /// high priority
  High,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, SystemLabel)]
pub enum SpriteLabel {
  /// update sprite handle to entities
  UpdateSpriteSheet,
  /// run sprite animations.
  /// including sprite switching and re-loading.
  SpriteAnimation,
}
