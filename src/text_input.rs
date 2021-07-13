use crate::consts::*;
use crate::saves::GameSave;
use crate::FontAssets;
use crate::game::GameAutoSaveSlot;
use bevy::prelude::*;
use bevy::window::ReceivedCharacter;

pub enum TextInputFinishTask {
  StartNewGame(u8),
}

struct TextInputUI;
struct TextInputTextUI;
struct TextInputMaterials {
  background: Handle<ColorMaterial>,
}

impl FromWorld for TextInputMaterials {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

    TextInputMaterials {
      background: materials.add(Color::NONE.into()),
    }
  }
}

struct TextInputText(String);

fn setup_text_input(
  mut commands: Commands,
  materials: Res<TextInputMaterials>,
  font_assets: Res<FontAssets>,
) {
  commands.spawn_bundle(Text2dBundle {
    text: Text::with_section("Type your name", TextStyle {
      font: font_assets.default_font.clone(),
      font_size: 32.0,
      color: Color::BLACK,
    }, TextAlignment {
      vertical: VerticalAlign::Center,
      horizontal: HorizontalAlign::Center,
    }),
    transform: Transform::from_xyz(0.0, 20.0, 0.0),
    ..Default::default()
  }).insert(TextInputUI);
  commands.spawn_bundle(Text2dBundle {
    text: Text::with_section("...", TextStyle {
      font: font_assets.default_font.clone(),
      font_size: 32.0,
      color: Color::BLACK,
    }, TextAlignment {
      vertical: VerticalAlign::Center,
      horizontal: HorizontalAlign::Center,
    }),
    transform: Transform::from_xyz(0.0, -20.0, 0.0),
    ..Default::default()
  }).insert(TextInputTextUI).insert(TextInputUI);
  commands.insert_resource(TextInputText("".to_string()));
}

fn input_text(
  mut commands: Commands,
  mut text: ResMut<TextInputText>,
  mut char_input_events: EventReader<ReceivedCharacter>,
  task: Res<TextInputFinishTask>,
  mut state: ResMut<State<AppState>>,
) {
  for event in char_input_events.iter() {
    if event.char == '\u{8}' {
      let mut chars = text.0.chars();
      chars.next_back();
      text.0 = chars.as_str().to_string();
    } else if event.char == '\r' {
      match *task {
        TextInputFinishTask::StartNewGame(slot) => {
          commands.insert_resource(GameAutoSaveSlot(slot));
          commands.insert_resource(GameSave::new(text.0.clone()));
          state.replace(AppState::InGame).unwrap();
        }
      }
    } else {
      text.0.push_str(&event.char.to_string());
    }
  }
}

fn update_text(
  input: Res<TextInputText>,
  mut query: Query<&mut Text, With<TextInputTextUI>>
) {
  for mut text in query.iter_mut() {
    if input.0.is_empty() {
      text.sections[0].value = "...".to_string();
    } else {
      text.sections[0].value = input.0.clone();
    }
  }
}

fn destroy_text_input(
  mut commands: Commands,
  query: Query<Entity, With<TextInputUI>>,
) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
  commands.remove_resource::<TextInputText>();
}

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<TextInputMaterials>()
      .add_system_set(
        SystemSet::on_enter(AppState::TextInput)
          .with_system(setup_text_input.system())
      )
      .add_system_set(
        SystemSet::on_update(AppState::TextInput)
          .with_system(input_text.system())
          .with_system(update_text.system())
      )
      .add_system_set(
        SystemSet::on_exit(AppState::TextInput)
          .with_system(destroy_text_input.system())
      );
  }
}
