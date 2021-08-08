use bevy::prelude::*;

use crate::{consts::AppState, game::{engine::{entity::Position, projectile::BulletProps}, stages::{GameEngineLabel, SpriteLabel}}};

struct BulletSprites {
  sprite: Handle<TextureAtlas>,
}

impl FromWorld for BulletSprites {
  fn from_world(world: &mut World) -> Self {
    let cell = world.cell();
    let mut texture_atlases = cell.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
    let asset_server = cell.get_resource::<AssetServer>().unwrap();

    BulletSprites {
      sprite: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/item/bullet/bullet.png"),
          Vec2::new(20.0, 20.0),
          3,
          1,
        )),
    }
  }
}

fn change_bullet_sprite(
  sprites: Res<BulletSprites>,
  mut query: Query<&mut Handle<TextureAtlas>, Changed<BulletProps>>
) {
  for mut handle in query.iter_mut() {
    *handle = sprites.sprite.clone();
  }
}

fn sync_bullet_sprite(
  mut query: Query<(&Position, &mut Transform), (With<BulletProps>, Changed<Position>)>,
) {
  for (position, mut transform) in query.iter_mut() {
    transform.translation.x = position.0.x;
    transform.translation.y = position.0.y;
    // FIXME: z-index fix
    // now if z < 0 it disappears
    // transform.translation.z = position.0.y;
  }
}

pub struct BulletSpritingPlugin;

impl Plugin for BulletSpritingPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<BulletSprites>()
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::UpdateSprites)
          .after(GameEngineLabel::UpdateAttacks)
          .label(SpriteLabel::UpdateSpriteSheet)
          .with_system(change_bullet_sprite)
      )
      .add_system_to_stage(CoreStage::PostUpdate, sync_bullet_sprite);
  }
}

