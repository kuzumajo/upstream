use crate::consts::*;
use crate::FontAssets;
use crate::sounds::SoundEffects;
use bevy::app::AppExit;
use bevy::prelude::*;

struct GameMenuUI;
struct GameMenuButtonUI;

enum MenuButton {
  Start,
  Settings,
  Achievements,
  Staff,
  Exit,
}

struct MenuMaterials {
  normal: Handle<ColorMaterial>,
  hovered: Handle<ColorMaterial>,
  pressed: Handle<ColorMaterial>,
}

impl FromWorld for MenuMaterials {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

    MenuMaterials {
      normal: materials.add(Color::NONE.into()),
      hovered: materials.add(Color::rgba(0.1, 0.1, 0.1, 0.1).into()),
      pressed: materials.add(Color::rgba(0.1, 0.1, 0.1, 0.2).into()),
    }
  }
}

fn setup_menu(mut commands: Commands, materials: Res<MenuMaterials>, font_assets: Res<FontAssets>) {
  commands
    .spawn_bundle(NodeBundle {
      style: Style {
        size: Size::new(Val::Px(400.0), Val::Auto),
        margin: Rect {
          left: Val::Px(200.0),
          top: Val::Auto,
          bottom: Val::Auto,
          ..Default::default()
        },
        flex_direction: FlexDirection::ColumnReverse,
        ..Default::default()
      },
      material: materials.normal.clone(),
      ..Default::default()
    })
    .insert(GameMenuUI)
    .with_children(|parent| {
      vec![
        (MenuButton::Start, "始める"),
        (MenuButton::Settings, "設定"),
        (MenuButton::Achievements, "成就"),
        (MenuButton::Staff, "スタッフ"),
        (MenuButton::Exit, "終了"),
      ]
      .into_iter()
      .for_each(|(button, title)| {
        parent
          .spawn_bundle(ButtonBundle {
            style: Style {
              size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
              margin: Rect::all(Val::Auto),
              justify_content: JustifyContent::FlexStart,
              align_items: AlignItems::Center,
              ..Default::default()
            },
            material: materials.normal.clone(),
            ..Default::default()
          })
          .insert(button)
          .insert(GameMenuButtonUI)
          .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
              text: Text::with_section(
                title,
                TextStyle {
                  font: font_assets.default_font.clone(),
                  font_size: 40.0,
                  color: Color::BLACK,
                },
                Default::default(),
              ),
              style: Style {
                margin: Rect {
                  left: Val::Px(20.0),
                  ..Default::default()
                },
                ..Default::default()
              },
              ..Default::default()
            });
          });
      });
    });
}

fn button_material_change(
  materials: Res<MenuMaterials>,
  mut interaction_query: Query<
    (&Interaction, &mut Handle<ColorMaterial>),
    (Changed<Interaction>, With<GameMenuButtonUI>),
  >,
) {
  for (interaction, mut material) in interaction_query.iter_mut() {
    *material = match *interaction {
      Interaction::Clicked => materials.pressed.clone(),
      Interaction::Hovered => materials.hovered.clone(),
      Interaction::None => materials.normal.clone(),
    }
  }
}

fn button_click(
  query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
  mut state: ResMut<State<AppState>>,
  mut app_exit_events: EventWriter<AppExit>,
) {
  for (interaction, menu_button) in query.iter() {
    match *interaction {
      Interaction::Clicked => match *menu_button {
        MenuButton::Start => {
          state.push(AppState::LoadGame).unwrap();
        }
        MenuButton::Settings => {
          state.push(AppState::Settings).unwrap();
        }
        MenuButton::Achievements => {
          // TODO
        }
        MenuButton::Staff => {
          state.replace(AppState::Staff).unwrap();
        }
        MenuButton::Exit => {
          app_exit_events.send(AppExit);
        }
      },
      _ => {}
    }
  }
}

fn button_hover_sfx(
  sounds: Res<SoundEffects>,
  audio: Res<Audio>,
  query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
) {
  for interaction in query.iter() {
    if interaction == &Interaction::Hovered {
      audio.play(sounds.tape.clone());
    }
  }
}

fn hide_ui(mut query: Query<&mut Style, With<GameMenuUI>>) {
  for mut style in query.iter_mut() {
    style.display = Display::None;
  }
}

fn resume_ui(mut query: Query<&mut Style, With<GameMenuUI>>) {
  for mut style in query.iter_mut() {
    style.display = Display::default();
  }
}

fn destroy_menu(mut commands: Commands, query: Query<Entity, With<GameMenuUI>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<MenuMaterials>()
      .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
      .add_system_set(
        SystemSet::on_update(AppState::Menu)
          .with_system(button_material_change)
          .with_system(button_click)
          .with_system(button_hover_sfx)
      )
      .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(destroy_menu))
      .add_system_set(SystemSet::on_pause(AppState::Menu).with_system(hide_ui))
      .add_system_set(SystemSet::on_resume(AppState::Menu).with_system(resume_ui));
  }
}
