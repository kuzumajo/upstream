use crate::config::GameConfig;
use crate::consts::*;
use crate::text_input::TextInputText;
use crate::FontAssets;
use bevy::prelude::*;

struct SettingsUI;
struct SettingsButtonUI;
struct SettingsMaterials {
  button_pressed: Handle<ColorMaterial>,
  button_normal: Handle<ColorMaterial>,
  button_hover: Handle<ColorMaterial>,

  radio_check_pressed: Handle<ColorMaterial>,
  radio_check_normal: Handle<ColorMaterial>,
  radio_check_hover: Handle<ColorMaterial>,

  radio_uncheck_pressed: Handle<ColorMaterial>,
  radio_uncheck_normal: Handle<ColorMaterial>,
  radio_uncheck_hover: Handle<ColorMaterial>,

  slide_button: Handle<ColorMaterial>,
  slide_bar: Handle<ColorMaterial>,

  transparent: Handle<ColorMaterial>,
}

struct SettingStringButton;
struct SettingRadioButton;
struct SettingSlideButton;
struct SettingSelectButton;

impl FromWorld for SettingsMaterials {
  fn from_world(world: &mut World) -> Self {
    let world_cell = world.cell();
    let mut materials = world_cell
      .get_resource_mut::<Assets<ColorMaterial>>()
      .unwrap();
    let asset_server = world_cell.get_resource::<AssetServer>().unwrap();

    SettingsMaterials {
      button_pressed: materials.add(Color::BLUE.into()),
      button_normal: materials.add(Color::GREEN.into()),
      button_hover: materials.add(Color::RED.into()),

      radio_check_pressed: materials.add(Color::BLUE.into()),
      radio_check_normal: materials.add(Color::GREEN.into()),
      radio_check_hover: materials.add(Color::RED.into()),

      radio_uncheck_pressed: materials.add(Color::BLUE.into()),
      radio_uncheck_normal: materials.add(Color::RED.into()),
      radio_uncheck_hover: materials.add(Color::GREEN.into()),

      slide_button: materials.add(asset_server.load("images/slidebutton.png").into()),
      slide_bar: materials.add(asset_server.load("images/slidebar.png").into()),

      transparent: materials.add(Color::NONE.into()),
    }
  }
}

#[derive(Clone)]
pub enum SettingType {
  String(String),
  Ratio(bool),
  /// [0.0, 1.0]
  Slide(f32),
  /// (selected, total)
  /// 0 <= selected < total
  Select(u32, Vec<String>),
}

pub enum SettingItem {
  Volumn,
  VolumnMusic,
  VolumnSfx,
  VolumnVoice,
  SaveDir,
  Fullscreen,
  Resolution,
  Decorations,
  AttackToMouse,
  AssaultToMouse,
  MouseSensitivity,
}

enum SettingsButton {
  Back,
  Apply,
  Reset,
}

fn setup_settings(
  mut commands: Commands,
  materials: Res<SettingsMaterials>,
  font_assets: Res<FontAssets>,
  config: Res<GameConfig>,
) {
  commands
    .spawn_bundle(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        justify_content: JustifyContent::SpaceBetween,
        ..Default::default()
      },
      material: materials.transparent.clone(),
      ..Default::default()
    })
    .insert(SettingsUI)
    .with_children(|parent| {
      // left <div>
      parent
        .spawn_bundle(NodeBundle {
          style: Style {
            size: Size::new(Val::Px(250.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            ..Default::default()
          },
          material: materials.radio_check_hover.clone(),
          ..Default::default()
        })
        .with_children(|parent| {
          // button default styles
          let text_style = TextStyle {
            font: font_assets.default_font.clone(),
            font_size: 32.0,
            color: Color::BLACK,
          };
          let text_alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
          };
          let button_style = Style {
            size: Size::new(Val::Percent(80.0), Val::Px(40.0)),
            margin: Rect {
              top: Val::Px(20.0),
              ..Default::default()
            },
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
          };

          // back button
          parent
            .spawn_bundle(ButtonBundle {
              style: button_style.clone(),
              material: materials.button_normal.clone(),
              ..Default::default()
            })
            .with_children(|parent| {
              parent.spawn_bundle(TextBundle {
                text: Text::with_section("<", text_style.clone(), text_alignment.clone()),
                ..Default::default()
              });
            })
            .insert(SettingsButton::Back)
            .insert(SettingsButtonUI);

          // apply button
          parent
            .spawn_bundle(ButtonBundle {
              style: button_style.clone(),
              material: materials.button_normal.clone(),
              ..Default::default()
            })
            .with_children(|parent| {
              parent.spawn_bundle(TextBundle {
                text: Text::with_section("Apply", text_style.clone(), text_alignment.clone()),
                ..Default::default()
              });
            })
            .insert(SettingsButton::Apply)
            .insert(SettingsButtonUI);

          // reset button
          parent
            .spawn_bundle(ButtonBundle {
              style: button_style.clone(),
              material: materials.button_normal.clone(),
              ..Default::default()
            })
            .with_children(|parent| {
              parent.spawn_bundle(TextBundle {
                text: Text::with_section("Reset", text_style.clone(), text_alignment.clone()),
                ..Default::default()
              });
            })
            .insert(SettingsButton::Reset)
            .insert(SettingsButtonUI);
        });

      // right <div>
      parent
        .spawn_bundle(NodeBundle {
          style: Style {
            size: Size::new(Val::Undefined, Val::Percent(100.0)),
            margin: Rect::all(Val::Auto),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
          },
          material: materials.transparent.clone(),
          ..Default::default()
        })
        .with_children(|parent| {
          use SettingItem::*;
          use SettingType::*;
          let setting_list = [
            ("声", Volumn),
            ("乐声", VolumnMusic),
            ("效声", VolumnSfx),
            ("音声", VolumnVoice),
            ("存址", SaveDir),
            ("全屏", Fullscreen),
            ("解像度", Resolution),
            ("窗框", Decorations),
            ("攻于鼠", AttackToMouse),
            ("冲于鼠", AssaultToMouse),
            ("鼠敏", MouseSensitivity),
          ];

          // left name
          parent
            .spawn_bundle(NodeBundle {
              style: Style {
                size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                ..Default::default()
              },
              material: materials.transparent.clone(),
              ..Default::default()
            })
            .with_children(|parent| {
              for (name, _) in setting_list.iter() {
                parent
                  .spawn_bundle(NodeBundle {
                    style: Style {
                      size: Size::new(Val::Auto, Val::Px(50.0)),
                      margin: Rect {
                        right: Val::Px(20.0),
                        ..Default::default()
                      },
                      align_items: AlignItems::Center,
                      justify_content: JustifyContent::Center,
                      ..Default::default()
                    },
                    material: materials.transparent.clone(),
                    ..Default::default()
                  })
                  .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                      text: Text::with_section(
                        *name,
                        TextStyle {
                          font: font_assets.default_font.clone(),
                          font_size: 32.0,
                          color: Color::BLACK,
                        },
                        Default::default(),
                      ),
                      ..Default::default()
                    });
                  });
              }
            });

          // right elements
          parent
            .spawn_bundle(NodeBundle {
              style: Style {
                size: Size::new(Val::Undefined, Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                ..Default::default()
              },
              material: materials.transparent.clone(),
              ..Default::default()
            })
            .with_children(|parent| {
              // right controllers
              for (_, item) in setting_list.iter() {
                let st = config.get_settings_type(item);

                parent
                  .spawn_bundle(NodeBundle {
                    style: Style {
                      size: Size::new(Val::Auto, Val::Px(50.0)),
                      margin: Rect {
                        left: Val::Px(20.0),
                        right: Val::Px(20.0),
                        ..Default::default()
                      },
                      align_items: AlignItems::Center,
                      justify_content: JustifyContent::Center,
                      ..Default::default()
                    },
                    material: materials.transparent.clone(),
                    ..Default::default()
                  })
                  .with_children(|parent| match &st {
                    String(value) => {
                      // string values
                      parent
                        .spawn_bundle(ButtonBundle {
                          material: materials.transparent.clone(),
                          ..Default::default()
                        })
                        .with_children(|parent| {
                          parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                              value,
                              TextStyle {
                                font: font_assets.default_font.clone(),
                                font_size: 32.0,
                                color: Color::BLACK,
                              },
                              Default::default(),
                            ),
                            ..Default::default()
                          });
                        })
                        .insert(st)
                        .insert(SettingStringButton);
                    }
                    Ratio(value) => {
                      parent.spawn_bundle(ButtonBundle {
                        style: Style {
                          size: Size::new(Val::Px(25.0), Val::Px(25.0)),
                          ..Default::default()
                        },
                        material: if *value {
                          materials.radio_check_normal.clone()
                        } else {
                          materials.radio_uncheck_normal.clone()
                        },
                        ..Default::default()
                      }).insert(st).insert(SettingRadioButton);
                    }
                    Slide(value) => {
                      parent.spawn_bundle(NodeBundle {
                        style: Style {
                          size: Size::new(Val::Px(500.0), Val::Px(10.0)),
                          ..Default::default()
                        },
                        material: materials.slide_bar.clone(),
                        ..Default::default()
                      });
                      parent.spawn_bundle(NodeBundle {
                        style: Style {
                          size: Size::new(Val::Px(25.0), Val::Px(25.0)),
                          position_type: PositionType::Absolute,
                          position: Rect {
                            top: Val::Px(12.5),
                            left: Val::Px(value * 500.0 - 12.5),
                            ..Default::default()
                          },
                          ..Default::default()
                        },
                        material: materials.slide_button.clone(),
                        ..Default::default()
                      });
                    }
                    Select(selected, list) => {
                      parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                          list[*selected as usize].clone(),
                          TextStyle {
                            font: font_assets.default_font.clone(),
                            font_size: 32.0,
                            color: Color::BLACK,
                          },
                          Default::default(),
                        ),
                        ..Default::default()
                      });
                    }
                  });
              }
            });
        });
    });
}

fn button_material_change(
  materials: Res<SettingsMaterials>,
  mut query: Query<
    (&Interaction, &mut Handle<ColorMaterial>),
    (Changed<Interaction>, With<SettingsButtonUI>),
  >,
) {
  for (interaction, mut material) in query.iter_mut() {
    *material = match *interaction {
      Interaction::Clicked => materials.button_pressed.clone(),
      Interaction::Hovered => materials.button_hover.clone(),
      Interaction::None => materials.button_normal.clone(),
    }
  }
}

fn nav_button_clicked(
  mut state: ResMut<State<AppState>>,
  query: Query<(&Interaction, &SettingsButton), (Changed<Interaction>, With<SettingsButtonUI>)>,
) {
  for (interaction, button) in query.iter() {
    match *interaction {
      Interaction::Clicked => match *button {
        SettingsButton::Back => {
          state.pop().unwrap();
        }
        SettingsButton::Apply => {
          println!("apply!!");
        }
        SettingsButton::Reset => {
          println!("reset!!");
        }
      },
      _ => {}
    }
  }
}

enum SettingsInputTextReason {
  ChangeStringValue(Entity),
}

fn string_button_clicked(
  mut commands: Commands,
  mut state: ResMut<State<AppState>>,
  query: Query<(&Interaction, Entity, &SettingType), With<SettingStringButton>>,
) {
  for (interaction, entity, stype) in query.iter() {
    match *interaction {
      Interaction::Clicked => {
        commands.insert_resource(SettingsInputTextReason::ChangeStringValue(entity.clone()));
        if let SettingType::String(string) = stype {
          commands.insert_resource(TextInputText(string.clone()));
        }
        state.push(AppState::TextInput).unwrap();
      }
      _ => {}
    }
  }
}

fn radio_button_clicked(
  mut query: Query<(&Interaction, &mut SettingType), (Changed<Interaction>, With<SettingRadioButton>)>,
) {
  for (interaction, mut stype) in query.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        if let SettingType::Ratio(checked) = *stype {
          *stype = SettingType::Ratio(!checked);
        }
      }
      _ => {}
    }
  }
}

fn update_radio_material(
  mut query: Query<
    (&Interaction, &SettingType, &mut Handle<ColorMaterial>),
    (Or<(Changed<Interaction>, Changed<SettingType>)>, With<SettingRadioButton>)
  >,
  materials: Res<SettingsMaterials>,
) {
  for (interaction, stype, mut material) in query.iter_mut() {
    if let SettingType::Ratio(checked) = &stype {
      *material = match *interaction {
        Interaction::Clicked => {
          if *checked {
            materials.radio_check_pressed.clone()
          } else {
            materials.radio_uncheck_pressed.clone()
          }
        }
        Interaction::Hovered => {
          if *checked {
            materials.radio_check_hover.clone()
          } else {
            materials.radio_uncheck_hover.clone()
          }
        }
        Interaction::None => {
          if *checked {
            materials.radio_check_normal.clone()
          } else {
            materials.radio_uncheck_normal.clone()
          }
        }
      }
    }
  }
}

fn resume_settings(
  mut commands: Commands,
  reason: Option<Res<SettingsInputTextReason>>,
  input: Option<Res<TextInputText>>,
  mut query: Query<&mut SettingType>,
) {
  if let Some(reason) = reason {
    match *reason {
      SettingsInputTextReason::ChangeStringValue(entity) => {
        if let Ok(mut stype) = query.get_mut(entity) {
          if let Some(input) = input {
            *stype = SettingType::String(input.0.clone());
            commands.remove_resource::<TextInputText>();
          }
        }
      }
    }
  }
}

fn update_string_settings(
  query: Query<(&SettingType, &Children), (Changed<SettingType>, With<SettingStringButton>)>,
  mut text_query: Query<&mut Text>,
) {
  for (stype, children) in query.iter() {
    if let SettingType::String(value) = stype {
      for child in children.iter() {
        if let Ok(mut text) = text_query.get_mut(*child) {
          text.sections[0].value = value.clone();
        }
      }
    }
  }
}

fn destroy_settings(mut commands: Commands, query: Query<Entity, With<SettingsUI>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

fn hide_ui(mut query: Query<&mut Style, With<SettingsUI>>) {
  for mut style in query.iter_mut() {
    style.display = Display::None;
  }
}

fn resume_ui(mut query: Query<&mut Style, With<SettingsUI>>) {
  for mut style in query.iter_mut() {
    style.display = Display::default();
  }
}

/// settings page
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<SettingsMaterials>()
      .add_system_set(SystemSet::on_enter(AppState::Settings).with_system(setup_settings.system()))
      .add_system_set(
        SystemSet::on_update(AppState::Settings)
          .with_system(button_material_change.system())
          .with_system(nav_button_clicked.system())
          .with_system(string_button_clicked.system())
          .with_system(update_string_settings.system())
          .with_system(radio_button_clicked.system().label("renew"))
          .with_system(update_radio_material.system().after("renew")),
      )
      .add_system_set(SystemSet::on_exit(AppState::Settings).with_system(destroy_settings.system()))
      .add_system_set(SystemSet::on_pause(AppState::Settings).with_system(hide_ui.system()))
      .add_system_set(
        SystemSet::on_resume(AppState::Settings)
          .with_system(resume_ui.system())
          .with_system(resume_settings.system()),
      );
  }
}
