use crate::consts::*;
use crate::FontAssets;
use bevy::prelude::*;
use bevy::window::ReceivedCharacter;

/// Used to indentify entities in this page.
struct TextInputUI;

/// Used to indentify inputting text entity in this page.
struct TextInputTextUI;

/// Used to return results from the text input.
pub struct TextInputText(pub String);

/// setup the input page.
fn setup_text_input(
  mut commands: Commands,
  font_assets: Res<FontAssets>,
  option: Option<Res<TextInputText>>,
) {
  commands
    .spawn_bundle(Text2dBundle {
      text: Text::with_section(
        "Type your name",
        TextStyle {
          font: font_assets.default_font.clone(),
          font_size: 32.0,
          color: Color::BLACK,
        },
        TextAlignment {
          vertical: VerticalAlign::Center,
          horizontal: HorizontalAlign::Center,
        },
      ),
      transform: Transform::from_xyz(0.0, 20.0, 0.0),
      ..Default::default()
    })
    .insert(TextInputUI);

  commands
    .spawn_bundle(Text2dBundle {
      text: Text::with_section(
        "...",
        TextStyle {
          font: font_assets.default_font.clone(),
          font_size: 32.0,
          color: Color::BLACK,
        },
        TextAlignment {
          vertical: VerticalAlign::Center,
          horizontal: HorizontalAlign::Center,
        },
      ),
      transform: Transform::from_xyz(0.0, -20.0, 0.0),
      ..Default::default()
    })
    .insert(TextInputTextUI)
    .insert(TextInputUI);

  if option.is_none() {
    commands.insert_resource(TextInputText("".to_string()));
  }
}

/// Listen user input and update TextInputText.
fn input_text(
  mut text: ResMut<TextInputText>,
  mut char_input_events: EventReader<ReceivedCharacter>,
  mut state: ResMut<State<AppState>>,
) {
  for event in char_input_events.iter() {
    if event.char == '\u{8}' {
      let mut chars = text.0.chars();
      chars.next_back();
      text.0 = chars.as_str().to_string();
    } else if event.char == '\r' {
      if !text.0.is_empty() {
        // task finished, pop itself
        state.pop().unwrap();
      }
    } else {
      text.0.push_str(&event.char.to_string());
    }
  }
}

/// Sync TextInputTextUI with TextInputText.
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

/// exit input page.
fn exit_on_esc(
  mut commands: Commands,
  input: Res<Input<KeyCode>>,
  mut state: ResMut<State<AppState>>,
) {
  if input.just_pressed(KeyCode::Escape) {
    commands.remove_resource::<TextInputText>();
    state.pop().unwrap();
  }
}

/// destroy the input page.
fn destroy_text_input(mut commands: Commands, query: Query<Entity, With<TextInputUI>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

/// Text input page.
pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_enter(AppState::TextInput)
          .with_system(setup_text_input),
      )
      .add_system_set(
        SystemSet::on_update(AppState::TextInput)
          .with_system(input_text)
          .with_system(update_text)
          .with_system(exit_on_esc),
      )
      .add_system_set(
        SystemSet::on_exit(AppState::TextInput)
          .with_system(destroy_text_input),
      );
  }
}
