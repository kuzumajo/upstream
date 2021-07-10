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

  commands.insert_resource(StudioLogoTimer(Timer::from_seconds(5.0, false)));

  trace!("initialized studio logo page");
}

/// run timer system
fn logic_logo(
  time: Res<Time>,
  mut timer: ResMut<StudioLogoTimer>,
  mut state: ResMut<State<AppState>>,
) {
  if timer.0.tick(time.delta()).just_finished() {
    state.replace(AppState::Menu).unwrap();
  }
}

/// exit the logo page
fn exit_logo(mut commands: Commands, query: Query<Entity, With<StudioLogoUI>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn();
  }
  commands.remove_resource::<StudioLogoTimer>();
}

pub struct StudioLogoPlugin;

impl Plugin for StudioLogoPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system_set(SystemSet::on_enter(AppState::StudioLogo).with_system(setup_logo.system()))
      .add_system_set(SystemSet::on_update(AppState::StudioLogo).with_system(logic_logo.system()))
      .add_system_set(SystemSet::on_exit(AppState::StudioLogo).with_system(exit_logo.system()));
  }
}
