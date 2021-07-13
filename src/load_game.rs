use crate::consts::*;
use crate::saves::GameSave;
use crate::FontAssets;
use bevy::prelude::*;
use crate::saves::load_game_saves;
use crate::game::GameAutoSaveSlot;
use crate::text_input::TextInputFinishTask;

struct LoadGameUI;
struct LoadGameMaterials {
  transparent: Handle<ColorMaterial>,
  slot_pressed: Handle<ColorMaterial>,
  slot_normal: Handle<ColorMaterial>,
  slot_hover: Handle<ColorMaterial>,
}

impl FromWorld for LoadGameMaterials {
  fn from_world(world: &mut World) -> Self {
    let mut material = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

    LoadGameMaterials {
      transparent: material.add(Color::NONE.into()),
      slot_pressed: material.add(Color::rgb(0.75, 0.75, 0.75).into()),
      slot_normal: material.add(Color::rgb(0.85, 0.85, 0.85).into()),
      slot_hover: material.add(Color::rgb(0.8, 0.8, 0.8).into()),
    }
  }
}

struct GameSaveSlot(Option<GameSave>, u8);

fn make_save_slot(
  parent: &mut ChildBuilder,
  slot: GameSaveSlot,
  materials: &Res<LoadGameMaterials>,
  font_assets: &Res<FontAssets>,
) {
  parent.spawn_bundle(ButtonBundle {
    style: Style {
      size: Size::new(Val::Percent(100.0), Val::Percent(45.0)),
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      flex_direction: FlexDirection::ColumnReverse,
      ..Default::default()
    },
    material: materials.slot_normal.clone(),
    ..Default::default()
  }).with_children(|parent| {
    if let Some(save) = &slot.0 {
      parent.spawn_bundle(TextBundle {
        text: Text::with_section(save.saving_name.clone(), TextStyle {
          font: font_assets.default_font.clone(),
          font_size: 20.0,
          color: Color::BLACK,
        }, Default::default()),
        ..Default::default()
      });
    } else {
      parent.spawn_bundle(TextBundle {
        text: Text::with_section("(empty)", TextStyle {
          font: font_assets.default_font.clone(),
          font_size: 20.0,
          color: Color::BLACK,
        }, Default::default()),
        ..Default::default()
      });
      parent.spawn_bundle(TextBundle {
        text: Text::with_section("click to create new", TextStyle {
          font: font_assets.default_font.clone(),
          font_size: 20.0,
          color: Color::rgb(0.3, 0.3, 0.3),
        }, Default::default()),
        ..Default::default()
      });
    }
  }).insert(slot);
}

fn setup_loadgame(
  mut commands: Commands,
  font_assets: Res<FontAssets>,
  materials: Res<LoadGameMaterials>,
) {

  let mut saves = load_game_saves();
  assert_eq!(saves.len(), 4);
  let save3 = saves.pop().unwrap();
  let save2 = saves.pop().unwrap();
  let save1 = saves.pop().unwrap();
  let save0 = saves.pop().unwrap();
  assert_eq!(saves.len(), 0);

  commands.spawn_bundle(NodeBundle {
    style: Style {
      size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
      margin: Rect::all(Val::Auto),
      justify_content: JustifyContent::SpaceBetween,
      ..Default::default()
    },
    material: materials.transparent.clone(),
    ..Default::default()
  }).insert(LoadGameUI)
    .with_children(|parent| {
      // left
      parent.spawn_bundle(NodeBundle {
        style: Style {
          size: Size::new(Val::Percent(45.0), Val::Percent(100.0)),
          flex_direction: FlexDirection::ColumnReverse,
          justify_content: JustifyContent::SpaceBetween,
          ..Default::default()
        },
        material: materials.transparent.clone(),
        ..Default::default()
      }).with_children(|parent| {
        make_save_slot(parent, GameSaveSlot(save0, 0), &materials, &font_assets);
        make_save_slot(parent, GameSaveSlot(save2, 2), &materials, &font_assets);
      });
      // right
      parent.spawn_bundle(NodeBundle {
        style: Style {
          size: Size::new(Val::Percent(45.0), Val::Percent(100.0)),
          flex_direction: FlexDirection::ColumnReverse,
          justify_content: JustifyContent::SpaceBetween,
          ..Default::default()
        },
        material: materials.transparent.clone(),
        ..Default::default()
      }).with_children(|parent| {
        make_save_slot(parent, GameSaveSlot(save1, 1), &materials, &font_assets);
        make_save_slot(parent, GameSaveSlot(save3, 3), &materials, &font_assets);
      });
    });
}

fn slot_material_change(
  materials: Res<LoadGameMaterials>,
  mut interaction_query: Query<
    (&Interaction, &mut Handle<ColorMaterial>),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut material) in interaction_query.iter_mut() {
    *material = match *interaction {
      Interaction::Clicked => materials.slot_pressed.clone(),
      Interaction::Hovered => materials.slot_hover.clone(),
      Interaction::None => materials.slot_normal.clone(),
    }
  }
}

fn slot_button_click(
  mut commands: Commands,
  mut interaction_query: Query<
    (&Interaction, &GameSaveSlot),
    (Changed<Interaction>, With<Button>),
  >,
  mut state: ResMut<State<AppState>>,
) {
  for (interaction, save_slot) in interaction_query.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        commands.insert_resource(GameAutoSaveSlot(save_slot.1));
        if let Some(save) = &save_slot.0 {
          // start the game directly
          commands.insert_resource(save.clone());
          state.replace(AppState::InGame).unwrap();
        } else {
          // start the game after type its saving name
          commands.insert_resource(TextInputFinishTask::StartNewGame);
          state.replace(AppState::TextInput).unwrap();
        }
      }
      _ => {}
    }
  }
}

fn destroy_loadgame(
  mut commands: Commands,
  query: Query<Entity, With<LoadGameUI>>,
) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

pub struct LoadGamePlugin;

impl Plugin for LoadGamePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<LoadGameMaterials>()
      .add_system_set(
        SystemSet::on_enter(AppState::LoadGame)
          .with_system(setup_loadgame.system())
      )
      .add_system_set(
        SystemSet::on_update(AppState::LoadGame)
          .with_system(slot_material_change.system())
          .with_system(slot_button_click.system())
      )
      .add_system_set(
        SystemSet::on_exit(AppState::LoadGame)
          .with_system(destroy_loadgame.system())
      );
  }
}
