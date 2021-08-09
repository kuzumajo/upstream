use bevy::prelude::*;

use crate::game::{engine::entity::Position, entity::attack::AttackSpriteType, stages::SpriteLabel};

struct AttackSprites {
  circle_attack: Handle<TextureAtlas>,
  half_circle_attack: Handle<TextureAtlas>,
  rectangle_attack: Handle<TextureAtlas>,
}

impl FromWorld for AttackSprites {
  fn from_world(world: &mut World) -> Self {
    let cell = world.cell();
    let mut texture_atlases = cell.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
    let asset_server = cell.get_resource::<AssetServer>().unwrap();

    AttackSprites {
      circle_attack: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/other/attack_effects/circle.png"),
          Vec2::new(50.0, 50.0),
          3,
          1,
        )
      ),
      half_circle_attack: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/other/attack_effects/half_circle.png"),
          Vec2::new(50.0, 50.0),
          3,
          1,
        )
      ),
      rectangle_attack: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/other/attack_effects/rectangle.png"),
          Vec2::new(100.0, 25.0),
          3,
          1,
        )
      ),
    }
  }
}

fn change_attack_sprite(
  sprites: Res<AttackSprites>,
  mut query: Query<(&AttackSpriteType, &mut Handle<TextureAtlas>), Changed<AttackSpriteType>>
) {
  for (attack_type, mut handle) in query.iter_mut() {
    *handle = match &attack_type {
      &AttackSpriteType::HalfCircle => {
        sprites.half_circle_attack.clone()
      }
      &AttackSpriteType::Circle => {
        sprites.circle_attack.clone()
      }
      &AttackSpriteType::Rectangle => {
        sprites.rectangle_attack.clone()
      }
    }
  }
}

fn sync_attack_sprite(
  mut query: Query<(&Position, &mut Transform), (With<AttackSpriteType>, Changed<Position>)>
) {
  for (position, mut transform) in query.iter_mut() {
    transform.translation.x = position.0.x;
    transform.translation.y = position.0.y;
    // FIXME: set z-index
  }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<AttackSprites>()
      .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
          .label(SpriteLabel::UpdateSpriteSheet)
          .with_system(change_attack_sprite)
      )
      .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
          .label(SpriteLabel::SpriteAnimation)
          .with_system(sync_attack_sprite)
      );
  }
}
