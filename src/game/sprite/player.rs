use bevy::prelude::*;

use crate::{game::{MouseDirection, engine::entity::{Player, PlayerState, Position, Velocity}, stages::SpriteLabel}};

struct PlayerSprites {
  stand: Handle<TextureAtlas>,
  walk: Handle<TextureAtlas>,
  attack_a: Handle<TextureAtlas>,
  attack_aa: Handle<TextureAtlas>,
  attack_ab: Handle<TextureAtlas>,
  attack_b: Handle<TextureAtlas>,
  attack_bb: Handle<TextureAtlas>,
  attack_bbb: Handle<TextureAtlas>,
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
        )
      ),
      walk: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/char/shuku/shuku-walk.png"),
          Vec2::new(28.0, 30.0),
          2,
          1,
        )
      ),
      attack_a: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/char/shuku/shuku-attack-a.png"),
          Vec2::new(28.0, 30.0),
          3,
          1,
        )
      ),
      attack_aa: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/char/shuku/shuku-attack-aa.png"),
          Vec2::new(28.0, 30.0),
          5,
          1,
        )
      ),
      attack_ab: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/char/shuku/shuku-attack-ab.png"),
          Vec2::new(28.0, 30.0),
          2,
          1,
        )
      ),
      attack_b: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/char/shuku/shuku-attack-b.png"),
          Vec2::new(28.0, 30.0),
          3,
          1,
        )
      ),
      attack_bb: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/char/shuku/shuku-attack-bb.png"),
          Vec2::new(28.0, 30.0),
          3,
          1,
        )
      ),
      attack_bbb: texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("images/char/shuku/shuku-attack-bbb.png"),
          Vec2::new(28.0, 30.0),
          5,
          1,
        )
      ),
    }
  }
}

fn change_player_sprite(
  sprites: Res<PlayerSprites>,
  mut query: Query<(&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &Velocity, &PlayerState), (With<Player>, Or<(Changed<Velocity>, Changed<PlayerState>)>)>
) {
  for (mut handle, mut sprite, velocity, state) in query.iter_mut() {
    // TODO: here is alot of more animations needed
    let new_texture = match state {
      &PlayerState::Stand => if velocity.0 == Vec2::ZERO {
        &sprites.stand
      } else {
        &sprites.walk
      }
      &PlayerState::ShieldAttackA   => &sprites.attack_a,
      &PlayerState::ShieldAttackAA  => &sprites.attack_aa,
      &PlayerState::ShieldAttackAB  => &sprites.attack_ab,
      &PlayerState::ShieldAttackB   => &sprites.attack_b,
      &PlayerState::ShieldAttackBB  => &sprites.attack_bb,
      &PlayerState::ShieldAttackBBB => &sprites.attack_bbb,
      _ => &sprites.walk,
    };

    if *handle != *new_texture {
      *handle = new_texture.clone();
    }
  }
}

fn change_player_sprite_direction(
  direction: Res<MouseDirection>,
  mut query: Query<&mut TextureAtlasSprite, With<Player>>,
) {
  for mut sprite in query.iter_mut() {
    if direction.0.x > 0.0 {
      sprite.flip_x = true;
    } else if direction.0.x < 0.0 {
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
    transform.translation.z = 1.0;
  }
}

pub struct PlayerSpritingPlugin;

impl Plugin for PlayerSpritingPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<PlayerSprites>()
      .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
          .label(SpriteLabel::UpdateSpriteSheet)
          .with_system(change_player_sprite)
          .with_system(change_player_sprite_direction)
      )
      .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
          .label(SpriteLabel::SpriteAnimation)
          .after(SpriteLabel::UpdateSpriteSheet)
          .with_system(sync_player_sprite)
      );
  }
}
