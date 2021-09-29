use crate::config::GameConfig;
use crate::consts::*;
use crate::text_input::TextInputText;
use crate::FontAssets;
use crate::MousePosition;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

/// Used to indentify entities in this page
struct SettingsUI;

/// Used to indentify button entities in this page
struct NavButtonUI;

/// Used materials for the settings page
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

  nav_bar_background: Handle<ColorMaterial>,

  transparent: Handle<ColorMaterial>,
}

/// Used to indentify string button entities in this page
struct SettingStringButton;

/// Used to indentify radio button entities in this page
struct SettingRadioButton;

/// Used to indentify slider entities in this page
struct SettingSlideButton;

/// Used to indentify slider button entities in this page
struct SettingSlideMovingButton;

/// Used to indentify select button entities in this page
struct SettingSelectButton;

/// Used to track dragging of the slider
struct SettingDraggingSlide(Entity);

/// Initialize materials
impl FromWorld for SettingsMaterials {
  fn from_world(world: &mut World) -> Self {
    let world_cell = world.cell();
    let mut materials = world_cell
      .get_resource_mut::<Assets<ColorMaterial>>()
      .unwrap();
    let asset_server = world_cell.get_resource::<AssetServer>().unwrap();

    SettingsMaterials {
      button_pressed: materials.add(asset_server.load("images/ui/button-pressed.png").into()),
      button_normal: materials.add(asset_server.load("images/ui/button-normal.png").into()),
      button_hover: materials.add(asset_server.load("images/ui/button-hover.png").into()),

      radio_check_pressed: materials.add(asset_server.load("images/ui/checkbox-checked.png").into()),
      radio_check_normal: materials.add(asset_server.load("images/ui/checkbox-checked.png").into()),
      radio_check_hover: materials.add(asset_server.load("images/ui/checkbox-checked.png").into()),

      radio_uncheck_pressed: materials.add(asset_server.load("images/ui/checkbox-unchecked.png").into()),
      radio_uncheck_normal: materials.add(asset_server.load("images/ui/checkbox-unchecked.png").into()),
      radio_uncheck_hover: materials.add(asset_server.load("images/ui/checkbox-unchecked.png").into()),

      slide_button: materials.add(asset_server.load("images/ui/slider-button.png").into()),
      slide_bar: materials.add(asset_server.load("images/ui/slider-bar.png").into()),

      nav_bar_background: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),

      transparent: materials.add(Color::NONE.into()),
    }
  }
}

#[derive(Clone)]
/// Setting Type
pub enum SettingType {
  /// A simple string
  String(String),
  /// A ratio button
  Ratio(bool),
  /// A slider which ranged from 0.0 to 1.0
  Slide(f32),
  /// A select button which has a list of options and a selected option
  Select(usize, Vec<String>),
}

#[derive(Clone)]
/// Different setting items
pub enum SettingItem {
  Volume,
  VolumeMusic,
  VolumeSfx,
  VolumeVoice,
  SaveDir,
  Fullscreen,
  Resolution,
  Decorations,
  AttackToMouse,
  AssaultToMouse,
  MouseSensitivity,
}

/// Navigation buttons
enum NavButton {
  /// Go to the previous page
  Back,
  /// Apply changes
  Apply,
  /// Reset changes
  Reset,
}

/// Setup settings page
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
          material: materials.nav_bar_background.clone(),
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
            size: Size::new(Val::Px(200.0), Val::Px(40.0)),
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
                text: Text::with_section("戻る", text_style.clone(), text_alignment.clone()),
                ..Default::default()
              });
            })
            .insert(NavButton::Back)
            .insert(NavButtonUI);

          // apply button
          parent
            .spawn_bundle(ButtonBundle {
              style: button_style.clone(),
              material: materials.button_normal.clone(),
              ..Default::default()
            })
            .with_children(|parent| {
              parent.spawn_bundle(TextBundle {
                text: Text::with_section("適用", text_style.clone(), text_alignment.clone()),
                ..Default::default()
              });
            })
            .insert(NavButton::Apply)
            .insert(NavButtonUI);

          // reset button
          parent
            .spawn_bundle(ButtonBundle {
              style: button_style.clone(),
              material: materials.button_normal.clone(),
              ..Default::default()
            })
            .with_children(|parent| {
              parent.spawn_bundle(TextBundle {
                text: Text::with_section("リセット", text_style.clone(), text_alignment.clone()),
                ..Default::default()
              });
            })
            .insert(NavButton::Reset)
            .insert(NavButtonUI);
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
            ("ボリューム", Volume),
            ("音楽", VolumeMusic),
            ("効果音", VolumeSfx),
            ("声", VolumeVoice),
            ("保存先", SaveDir),
            ("フルスクリーン", Fullscreen),
            ("解像度", Resolution),
            ("ウィンドウ装飾", Decorations),
            ("マウスの方向に攻撃", AttackToMouse),
            ("マウスの方向に突撃", AssaultToMouse),
            ("マウス感度", MouseSensitivity),
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
              for (_, item) in setting_list.iter().cloned() {
                let st = config.get_settings_type(&item);

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
                        .insert(item)
                        .insert(SettingStringButton);
                    }
                    Ratio(value) => {
                      parent
                        .spawn_bundle(ButtonBundle {
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
                        })
                        .insert(st)
                        .insert(item)
                        .insert(SettingRadioButton);
                    }
                    Slide(value) => {
                      parent
                        .spawn_bundle(ButtonBundle {
                          style: Style {
                            size: Size::new(Val::Auto, Val::Px(50.0)),
                            padding: Rect {
                              left: Val::Px(12.5),
                              right: Val::Px(12.5),
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
                          parent
                            .spawn_bundle(NodeBundle {
                              style: Style {
                                size: Size::new(Val::Px(SLIDER_LENGTH), Val::Px(10.0)),
                                ..Default::default()
                              },
                              material: materials.slide_bar.clone(),
                              ..Default::default()
                            })
                            .insert(FocusPolicy::Pass);
                          parent
                            .spawn_bundle(NodeBundle {
                              style: Style {
                                size: Size::new(Val::Px(25.0), Val::Px(25.0)),
                                position_type: PositionType::Absolute,
                                position: Rect {
                                  top: Val::Px(12.5),
                                  left: Val::Px(value * SLIDER_LENGTH),
                                  ..Default::default()
                                },
                                ..Default::default()
                              },
                              material: materials.slide_button.clone(),
                              ..Default::default()
                            })
                            .insert(FocusPolicy::Pass)
                            .insert(SettingSlideMovingButton);
                        })
                        .insert(st)
                        .insert(item)
                        .insert(SettingSlideButton);
                    }
                    Select(selected, list) => {
                      parent
                        .spawn_bundle(ButtonBundle {
                          material: materials.transparent.clone(),
                          ..Default::default()
                        })
                        .with_children(|parent| {
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
                        })
                        .insert(st)
                        .insert(item)
                        .insert(SettingSelectButton);
                    }
                  });
              }
            });
        });
    });
}

/// listen mouse events and update hovered/clicked material change to navigation buttons
fn nav_button_material_change(
  materials: Res<SettingsMaterials>,
  mut query: Query<
    (&Interaction, &mut Handle<ColorMaterial>),
    (Changed<Interaction>, With<NavButtonUI>),
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

/// listen click events to navigations buttons
fn nav_button_clicked(
  mut commands: Commands,
  mut state: ResMut<State<AppState>>,
  mut config: ResMut<GameConfig>,
  query: Query<(&Interaction, &NavButton), (Changed<Interaction>, With<NavButtonUI>)>,
  mut query_set: QuerySet<(
    Query<(&SettingItem, &SettingType)>,
    Query<(&SettingItem, &mut SettingType)>,
  )>,
) {
  for (interaction, button) in query.iter() {
    match *interaction {
      Interaction::Clicked => match *button {
        NavButton::Back => {
          state.pop().unwrap();
        }
        NavButton::Apply => {
          for (item, stype) in query_set.q0().iter() {
            config.apply_changes(item, stype);
          }
          config
            .save()
            .expect("failed to save display config to disk");
          commands.insert_resource(config.get_window_descriptor());
        }
        NavButton::Reset => {
          let config = GameConfig::default();
          for (item, mut stype) in query_set.q1_mut().iter_mut() {
            *stype = config.get_settings_type(item);
          }
        }
      },
      _ => {}
    }
  }
}

/// get string settings after quitting the text input page
enum SettingsInputTextReason {
  ChangeStringValue(Entity),
}

/// listen clicked events to string typed settings
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

/// listen click events to radio typed buttons
fn radio_button_clicked(
  mut query: Query<
    (&Interaction, &mut SettingType),
    (Changed<Interaction>, With<SettingRadioButton>),
  >,
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

/// update materials of radio typed button when hovered/clicked 
fn update_radio_material(
  mut query: Query<
    (&Interaction, &SettingType, &mut Handle<ColorMaterial>),
    (
      Or<(Changed<Interaction>, Changed<SettingType>)>,
      With<SettingRadioButton>,
    ),
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

/// listen click events to select typed buttons
fn select_button_clicked(
  mut query: Query<
    (&Interaction, &mut SettingType),
    (Changed<Interaction>, With<SettingSelectButton>),
  >,
) {
  for (interaction, mut stype) in query.iter_mut() {
    if let SettingType::Select(choice, list) = &*stype {
      match *interaction {
        Interaction::Clicked => {
          *stype = SettingType::Select((choice + 1) % list.len(), list.clone());
        }
        _ => {}
      }
    }
  }
}

/// update selected options of select typed button after clicked
fn update_select_button(
  query: Query<(&SettingType, &Children), (Changed<SettingType>, With<SettingSelectButton>)>,
  mut text_query: Query<&mut Text>,
) {
  for (stype, children) in query.iter() {
    if let SettingType::Select(choice, list) = stype {
      for child in children.iter() {
        if let Ok(mut text) = text_query.get_mut(*child) {
          text.sections[0].value = list[*choice].clone();
        }
      }
    }
  }
}

/// get resumed after quitting text input page
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

/// update showing string of string typed settings after text changed
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

/// update materials of slider typed button when hovered/clicked
fn slide_button_clicked(
  mut commands: Commands,
  query: Query<(&Interaction, Entity), (Changed<Interaction>, With<SettingSlideButton>)>,
) {
  for (interaction, entity) in query.iter() {
    match *interaction {
      Interaction::Clicked => {
        commands.insert_resource(SettingDraggingSlide(entity));
      }
      Interaction::None | Interaction::Hovered => {
        commands.remove_resource::<SettingDraggingSlide>();
      }
    }
  }
}

/// listen mouse movements and sync it with dragging slider
fn drag_slide_button(
  mouse_position: Res<MousePosition>,
  dragging_slide: Option<Res<SettingDraggingSlide>>,
  mut query: Query<(&GlobalTransform, &mut SettingType)>,
) {
  if let Some(slide) = dragging_slide {
    if let Ok((transform, mut stype)) = query.get_mut(slide.0) {
      let position = transform.translation;
      let value = (mouse_position.0.x - (position.x - SLIDER_LENGTH / 2.0)) / SLIDER_LENGTH;
      let value = value.max(0.0).min(1.0);
      *stype = SettingType::Slide(value);
    }
  }
}

/// update the position of the slide dragging button
fn update_slide_button(
  query: Query<(&SettingType, &Children), With<SettingSlideButton>>,
  mut child_query: Query<&mut Style, With<SettingSlideMovingButton>>,
) {
  for (stype, children) in query.iter() {
    if let SettingType::Slide(value) = &stype {
      for child in children.iter() {
        if let Ok(mut style) = child_query.get_mut(*child) {
          style.position.left = Val::Px(value * SLIDER_LENGTH);
        }
      }
    }
  }
}

/// destroy settings page
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
  fn build(&self, app: &mut App) {
    app
      .init_resource::<SettingsMaterials>()
      .add_system_set(SystemSet::on_enter(AppState::Settings).with_system(setup_settings))
      .add_system_set(
        SystemSet::on_update(AppState::Settings)
          .with_system(nav_button_material_change)
          .with_system(nav_button_clicked)
          .with_system(string_button_clicked.label("clicked"))
          .with_system(radio_button_clicked.label("clicked"))
          .with_system(select_button_clicked.label("clicked"))
          .with_system(slide_button_clicked.label("clicked"))
          .with_system(update_string_settings.after("clicked"))
          .with_system(update_radio_material.after("clicked"))
          .with_system(update_select_button.after("clicked"))
          .with_system(drag_slide_button.after("clicked"))
          .with_system(update_slide_button.after("clicked")),
      )
      .add_system_set(
        SystemSet::on_pause(AppState::Settings)
          .with_system(hide_ui)
      )
      .add_system_set(
        SystemSet::on_resume(AppState::Settings)
          .with_system(resume_ui)
          .with_system(resume_settings),
      )
      .add_system_set(
        SystemSet::on_exit(AppState::Settings)
          .with_system(destroy_settings)
      );
  }
}
