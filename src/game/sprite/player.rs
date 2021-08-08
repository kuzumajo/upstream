use bevy::prelude::*;

use crate::{consts::AppState, game::{engine::entity::{Player, Position, Velocity}, stages::{GameEngineLabel, SpriteLabel}}};

struct PlayerSprites {
  stand: Handle<TextureAtlas>,
  walk: Handle<TextureAtlas>,
}

impl FromWorld for PlayerSprites {
  fn from_world(world: &mut World) -> Self {
    let cell = world.cell();
    let mut texture_atlases = cell.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
    let asset_server = cell.get_resource::<AssetServer>().unwrap();

    PlayerSprites {
      stand: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/char/shuku/shuku-stand.png"),
          Vec2::new(28.0, 30.0),
          1,
          1,
        )),
      walk: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/char/shuku/shuku-walk.png"),
          Vec2::new(28.0, 30.0),
          2,
          1,
        )),
    }
  }
}

fn change_player_sprite(
  sprites: Res<PlayerSprites>,
  mut query: Query<(&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &Velocity), (With<Player>, Changed<Velocity>)>
) {
  for (mut handle, mut sprite, velocity) in query.iter_mut() {
    // TODO: here is alot of more animations needed
    let new_texture = if velocity.0 == Vec2::ZERO {
      &sprites.stand
    } else {
      &sprites.walk
    };

    if *handle != *new_texture {
      *handle = new_texture.clone();
      sprite.index = 0;
    }

    if velocity.0.x > 0.0 {
      sprite.flip_x = true;
    } else if velocity.0.x < 0.0 {
      sprite.flip_x = false;
    }
  }
}

fn sync_player_sprite(
  mut query: Query<(&Position, &mut Transform), (With<Player>, Changed<Position>)>,
) {
  for (position, mut transform) in query.iter_mut() {
    transform.translation.x = position.0.x;
    transform.translation.y = position.0.y;
    // FIXME: z-index fix
    // now if z < 0 it disappears
    // transform.translation.z = position.0.y;
  }
}

pub struct PlayerSpritingPlugin;

impl Plugin for PlayerSpritingPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<PlayerSprites>()
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .label(GameEngineLabel::UpdateSprites)
          .after(GameEngineLabel::UpdateAttacks)
          .label(SpriteLabel::UpdateSpriteSheet)
          .with_system(change_player_sprite)
      )
      .add_system_to_stage(CoreStage::PostUpdate, sync_player_sprite);
  }
}
