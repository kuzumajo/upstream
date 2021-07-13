use crate::consts::*;
use crate::FontAssets;
use bevy::app::AppExit;
use bevy::prelude::*;

struct GameMenuUI;
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
        (MenuButton::Start, "start"),
        (MenuButton::Settings, "settings"),
        (MenuButton::Achievements, "achievements"),
        (MenuButton::Staff, "staff"),
        (MenuButton::Exit, "exit"),
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
    (Changed<Interaction>, With<Button>),
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
          state.replace(AppState::LoadGame).unwrap();
        }
        MenuButton::Settings => {
          // TODO
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

fn exit_menu(mut commands: Commands, query: Query<Entity, With<GameMenuUI>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<MenuMaterials>()
      .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu.system()))
      .add_system_set(
        SystemSet::on_update(AppState::Menu)
          .with_system(button_material_change.system())
          .with_system(button_click.system()),
      )
      .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(exit_menu.system()));
  }
}
