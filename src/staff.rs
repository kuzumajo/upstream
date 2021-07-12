use crate::consts::*;
use crate::FontAssets;
use bevy::prelude::*;

mod list;
use list::*;

struct StaffUI;
struct StaffScroll;
struct StartScrollTimer(Timer);

struct StaffMaterials {
  transparent: Handle<ColorMaterial>,
}

impl FromWorld for StaffMaterials {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

    StaffMaterials {
      transparent: materials.add(Color::NONE.into()),
    }
  }
}

fn setup_staff(
  mut commands: Commands,
  font_assets: Res<FontAssets>,
  materials: Res<StaffMaterials>,
) {
  // full list
  let mut total_y = -60.0;
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
        commands.spawn_bundle(Text2dBundle {
          text: Text::with_section(message, TextStyle {
            font: font_assets.default_font.clone(),
            font_size,
            color: Color::BLACK,
          }, TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
          }),
          transform: Transform::from_xyz(0.0, -total_y, 0.0),
          ..Default::default()
        }).insert(StaffUI).insert(StaffScroll);
        total_y += font_size;
      }
    }
  }
  // buttons (right)
  commands.spawn_bundle(NodeBundle {
    style: Style {
      size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
      position_type: PositionType::Absolute,
      position: Rect {
        right: Val::Px(0.0),
        bottom: Val::Px(0.0),
        ..Default::default()
      },
      ..Default::default()
    },
    material: materials.transparent.clone(),
    ..Default::default()
  }).insert(StaffUI);
}

fn scrolling_staff_list(
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

fn exit_staff(
  mut commands: Commands,
  query: Query<Entity, With<StaffUI>>,
) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

pub struct StaffPlugin;

impl Plugin for StaffPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .insert_resource(StartScrollTimer(Timer::from_seconds(STAFF_LIST_WAITING_SECONDS, false)))
      .init_resource::<StaffMaterials>()
      .add_system_set(SystemSet::on_enter(AppState::Staff).with_system(setup_staff.system()))
      .add_system_set(SystemSet::on_update(AppState::Staff).with_system(scrolling_staff_list.system()))
      .add_system_set(SystemSet::on_exit(AppState::Staff).with_system(exit_staff.system()));
  }
}