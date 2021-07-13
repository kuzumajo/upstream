use crate::consts::*;
use bevy::prelude::*;

struct SettingsUI;
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

  transparent: Handle<ColorMaterial>,
}

impl FromWorld for SettingsMaterials {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

    SettingsMaterials {
      button_pressed: materials.add(Color::BLUE.into()),
      button_normal: materials.add(Color::GREEN.into()),
      button_hover: materials.add(Color::RED.into()),

      radio_check_pressed: materials.add(Color::BLUE.into()),
      radio_check_normal: materials.add(Color::GREEN.into()),
      radio_check_hover: materials.add(Color::RED.into()),

      radio_uncheck_pressed: materials.add(Color::BLUE.into()),
      radio_uncheck_normal: materials.add(Color::GREEN.into()),
      radio_uncheck_hover: materials.add(Color::RED.into()),

      transparent: materials.add(Color::NONE.into()),
    }
  }
}

fn setup_settings(mut commands: Commands) {
  commands
    .spawn_bundle(NodeBundle {
      style: Style {
        ..Default::default()
      },
      ..Default::default()
    })
    .insert(SettingsUI);
}

/// settings page
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system_set(SystemSet::on_enter(AppState::Settings).with_system(setup_settings.system()));
  }
}
