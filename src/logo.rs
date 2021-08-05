use crate::consts::*;
use crate::FontAssets;
use bevy::prelude::*;

/// Timer to skip to the next page
struct StudioLogoTimer(Timer);

struct StudioLogoUI;

/// Open Studio Logo Page
fn setup_logo(mut commands: Commands, font_assets: Res<FontAssets>) {
  let text_style = TextStyle {
    font: font_assets.default_font.clone(),
    font_size: 60.0,
    color: Color::BLACK,
  };
  let text_alignment = TextAlignment {
    vertical: VerticalAlign::Center,
    horizontal: HorizontalAlign::Center,
  };

  commands
    .spawn_bundle(Text2dBundle {
      text: Text::with_section(
        "Kuzumajo Studio",
        text_style.clone(),
        text_alignment.clone(),
      ),
      transform: Transform::from_xyz(0.0, 50.0, 0.0),
      ..Default::default()
    })
    .insert(StudioLogoUI);

  commands
    .spawn_bundle(Text2dBundle {
      text: Text::with_section("屑魔女工作室", text_style.clone(), text_alignment.clone()),
      transform: Transform::from_xyz(0.0, -20.0, 0.0),
      ..Default::default()
    })
    .insert(StudioLogoUI);

  commands.insert_resource(StudioLogoTimer(Timer::from_seconds(
    STUDIO_LOGO_WAITING_SECONDS,
    false,
  )));
}

fn logic_logo(
  time: Res<Time>,
  mut timer: ResMut<StudioLogoTimer>,
  mut state: ResMut<State<AppState>>,
  mouse_button_input: Res<Input<MouseButton>>,
) {
  if timer.0.tick(time.delta()).just_finished()
    || mouse_button_input.just_pressed(MouseButton::Left)
  {
    state.replace(AppState::Menu).unwrap();
  }
}

fn destroy_logo(mut commands: Commands, query: Query<Entity, With<StudioLogoUI>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
  commands.remove_resource::<StudioLogoTimer>();
}

/// The starter studio logo page
pub struct StudioLogoPlugin;

impl Plugin for StudioLogoPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(AppState::StudioLogo).with_system(setup_logo))
      .add_system_set(SystemSet::on_update(AppState::StudioLogo).with_system(logic_logo))
      .add_system_set(SystemSet::on_exit(AppState::StudioLogo).with_system(destroy_logo));
  }
}
