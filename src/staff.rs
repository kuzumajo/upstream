use crate::consts::*;
use crate::FontAssets;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

mod list;
use list::*;

/// Used to indentify UI entities in this page
struct StaffUI;

/// Used to indentify scrolling entities in this page
struct StaffScroll;

/// Used wait some seconds before starting scrolling staff list
struct StartScrollTimer(Timer);

/// Setup staff page
fn setup_staff(mut commands: Commands, font_assets: Res<FontAssets>) {
  // full list
  let mut total_y = -98.0;
  for text in get_staff_text() {
    match text {
      StaffTextLine::Space(height) => {
        total_y += height;
      }
      StaffTextLine::Text(message, size) => {
        let font_size = match size {
          StaffTextSize::Ultra => 98.0,
          StaffTextSize::Large => 60.0,
          StaffTextSize::Medium => 32.0,
          StaffTextSize::Small => 24.0,
        };
        total_y += font_size;
        commands
          .spawn_bundle(Text2dBundle {
            text: Text::with_section(
              message,
              TextStyle {
                font: font_assets.default_font.clone(),
                font_size,
                color: Color::BLACK,
              },
              TextAlignment {
                vertical: VerticalAlign::Bottom,
                horizontal: HorizontalAlign::Center,
              },
            ),
            transform: Transform::from_xyz(0.0, -total_y, 0.0),
            ..Default::default()
          })
          .insert(StaffUI)
          .insert(StaffScroll);
      }
    }
  }
  commands.insert_resource(StartScrollTimer(Timer::from_seconds(
    STAFF_LIST_WAITING_SECONDS,
    false,
  )));
}

/// make list scroll
fn scroll_staff_list(
  time: Res<Time>,
  mut timer: ResMut<StartScrollTimer>,
  mut query: Query<&mut Transform, With<StaffScroll>>,
) {
  if timer.0.tick(time.delta()).finished() {
    for mut transform in query.iter_mut() {
      transform.translation.y += time.delta().as_millis() as f32 / 20.0;
    }
  }
}

/// exit staff page
fn esc_to_exit(keyevent: Res<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
  if keyevent.pressed(KeyCode::Escape) {
    state.replace(AppState::Menu).unwrap();
  }
}

/// destory staff page
fn destory_staff(mut commands: Commands, query: Query<Entity, With<StaffUI>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
  commands.remove_resource::<StartScrollTimer>();
}

/// staff page
pub struct StaffPlugin;

impl Plugin for StaffPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_enter(AppState::Staff)
          .with_system(setup_staff)
      )
      .add_system_set(
        SystemSet::on_update(AppState::Staff)
          .with_system(scroll_staff_list)
          .with_system(esc_to_exit),
      )
      .add_system_set(
        SystemSet::on_exit(AppState::Staff)
          .with_system(destory_staff)
      );
  }
}
