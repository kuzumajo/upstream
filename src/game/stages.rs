use bevy::{prelude::*, render::RenderStage};


#[derive(Debug, Hash, Clone, Eq, PartialEq, StageLabel)]
pub enum PhysicalStage {
  UpdateVelocityPre,
  UpdateVelocity,
  UpdateVelocityPost,

  UpdatePositionPre,
  UpdatePosition,
  UpdatePositionPost,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, StageLabel)]
pub enum AttackStage {
  TriggerSpecialAttackPre,
  TriggerSpecialAttack,
  TriggerSpecialAttackPost,

  TriggerNormalAttackPre,
  TriggerNormalAttack,
  TriggerNormalAttackPost,

  CreateDamagePre,
  CreateDamage,
  CreateDamagePost,
  
  ProcessDamagePre,
  ProcessDamage,
  ProcessDamagePost,

  RecieveDamagePre,
  RecieveDamage,
  RecieveDamagePost,

  ClearDeadPre,
  ClearDead,
  ClearDeadPost,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, StageLabel)]
pub enum SpritingStage {
  ChangeHandlePre,
  ChangeHandle,
  ChangeHandlePost,

  AnimateSpritePre,
  AnimateSprite,
  AnimateSpritePost,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, StageLabel)]
pub enum GameEngineStage {
  CoolDownPre,
  CoolDown,
  CoolDownPost,

  PhysicalEnginePre,
  PhysicalEnginePost,

  AttackEnginePre,
  AttackEnginePost,

  SpritingEnginePre,
  SpritingEnginePost,
}

pub struct RegisterStagesPlugin;

impl Plugin for RegisterStagesPlugin {
  fn build(&self, app: &mut App) {
    use GameEngineStage::*;
    use PhysicalStage::*;
    use AttackStage::*;
    use SpritingStage::*;

    app
      .add_stage(CoolDownPre, SystemStage::parallel())

      .add_stage_after(CoolDownPre, CoolDown, SystemStage::parallel())
      .add_stage_after(CoolDown, CoolDownPost, SystemStage::parallel())
      .add_stage_after(CoolDownPost, PhysicalEnginePre, SystemStage::parallel())

      // enter physical stage
      .add_stage_after(PhysicalEnginePre, UpdateVelocityPre, SystemStage::parallel())
      .add_stage_after(UpdateVelocityPre, UpdateVelocity, SystemStage::parallel())
      .add_stage_after(UpdateVelocity, UpdateVelocityPost, SystemStage::parallel())

      .add_stage_after(UpdateVelocityPost, UpdatePositionPre, SystemStage::parallel())
      .add_stage_after(UpdatePositionPre, UpdatePosition, SystemStage::parallel())
      .add_stage_after(UpdatePosition, UpdatePositionPost, SystemStage::parallel())

      // leave physical stage
      .add_stage_after(UpdatePositionPost, PhysicalEnginePost, SystemStage::parallel())
      .add_stage_after(PhysicalEnginePost, AttackEnginePre, SystemStage::parallel())

      // enter attack stage
      .add_stage_after(AttackEnginePre, TriggerSpecialAttackPre, SystemStage::parallel())
      .add_stage_after(TriggerSpecialAttackPre, TriggerSpecialAttack, SystemStage::parallel())
      .add_stage_after(TriggerSpecialAttack, TriggerSpecialAttackPost, SystemStage::parallel())

      .add_stage_after(TriggerSpecialAttackPost, TriggerNormalAttackPre, SystemStage::parallel())
      .add_stage_after(TriggerNormalAttackPre, TriggerNormalAttack, SystemStage::parallel())
      .add_stage_after(TriggerNormalAttack, TriggerNormalAttackPost, SystemStage::parallel())

      .add_stage_after(TriggerNormalAttackPost, CreateDamagePre, SystemStage::parallel())
      .add_stage_after(CreateDamagePre, CreateDamage, SystemStage::parallel())
      .add_stage_after(CreateDamage, CreateDamagePost, SystemStage::parallel())

      .add_stage_after(CreateDamagePost, ProcessDamagePre, SystemStage::parallel())
      .add_stage_after(ProcessDamagePre, ProcessDamage, SystemStage::parallel())
      .add_stage_after(ProcessDamage, ProcessDamagePost, SystemStage::parallel())

      .add_stage_after(ProcessDamagePost, RecieveDamagePre, SystemStage::parallel())
      .add_stage_after(RecieveDamagePre, RecieveDamage, SystemStage::parallel())
      .add_stage_after(RecieveDamage, RecieveDamagePost, SystemStage::parallel())

      .add_stage_after(RecieveDamagePost, ClearDeadPre, SystemStage::parallel())
      .add_stage_after(ClearDeadPre, ClearDead, SystemStage::parallel())
      .add_stage_after(ClearDead, ClearDeadPost, SystemStage::parallel())

      // leave attack stage
      .add_stage_after(ClearDeadPost, AttackEnginePost, SystemStage::parallel())
      .add_stage_after(AttackEnginePost, SpritingEnginePre, SystemStage::parallel())

      // enter spriting stage
      .add_stage_after(SpritingEnginePre, ChangeHandlePre, SystemStage::parallel())
      .add_stage_after(ChangeHandlePre, ChangeHandle, SystemStage::parallel())
      .add_stage_after(ChangeHandle, ChangeHandlePost, SystemStage::parallel())

      .add_stage_after(ChangeHandlePost, AnimateSpritePre, SystemStage::parallel())
      .add_stage_after(AnimateSpritePre, AnimateSprite, SystemStage::parallel())
      .add_stage_after(AnimateSprite, AnimateSpritePost, SystemStage::parallel())

      // leave spriting stage
      .add_stage_after(AnimateSpritePost, SpritingEnginePost, SystemStage::parallel())

      // define priority
      .add_stage_before(RenderStage::Draw, SpritingEnginePost, SystemStage::parallel());
  }
}
