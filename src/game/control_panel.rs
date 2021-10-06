use crate::consts::*;
use crate::saves::GameSave;
use crate::FontAssets;
use bevy::prelude::*;

use super::engine::entity::Player;
use super::engine::health::Health;
use super::engine::soul::SoulPower;

struct ControlPanelUI;

/// white bar to indicate how much damage recieved
struct HealthLosing(f32);

struct ControlPanelMaterials {
  transparent: Handle<ColorMaterial>,
  health_full: Handle<ColorMaterial>,
  health_losing: Handle<ColorMaterial>,
  health_now_safe: Handle<ColorMaterial>,
  health_now_danger: Handle<ColorMaterial>,
  energy_full: Handle<ColorMaterial>,
  energy_now: Handle<ColorMaterial>,
}

impl FromWorld for ControlPanelMaterials {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

    ControlPanelMaterials {
      transparent: materials.add(Color::NONE.into()),
      health_full: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
      health_losing: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
      health_now_safe: materials.add(Color::rgb(0.26, 0.73, 0.51).into()),
      health_now_danger: materials.add(Color::rgb(0.90, 0.18, 0.18).into()),
      energy_full: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
      energy_now: materials.add(Color::rgb(0.16, 0.72, 0.96).into()),
    }
  }
}

struct HealthBarUILosing;
struct HealthBarUI;
struct EnergyBarUI;

fn setup_control_panel(
  mut commands: Commands,
  font_assets: Res<FontAssets>,
  materials: Res<ControlPanelMaterials>,
) {
  // Root node <body>
  commands
    .spawn_bundle(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        ..Default::default()
      },
      material: materials.transparent.clone(),
      ..Default::default()
    })
    .insert(ControlPanelUI)
    // left top part <div>
    .with_children(|parent| {
      parent
        .spawn_bundle(NodeBundle {
          style: Style {
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::ColumnReverse,
            margin: Rect::all(Val::Px(20.0)),
            ..Default::default()
          },
          material: materials.transparent.clone(),
          ..Default::default()
        })
        .with_children(|parent| {
          // player name <text>
          parent.spawn_bundle(TextBundle {
            text: Text::with_section(
              PLAYER_NAME,
              TextStyle {
                font: font_assets.default_font.clone(),
                font_size: 32.0,
                color: Color::BLACK,
              },
              Default::default(),
            ),
            ..Default::default()
          });
          // health bar
          parent
            .spawn_bundle(NodeBundle {
              style: Style {
                size: Size::new(Val::Px(HEALTH_BAR_WIDTH), Val::Px(15.0)),
                margin: Rect {
                  top: Val::Px(10.0),
                  ..Default::default()
                },
                ..Default::default()
              },
              material: materials.health_full.clone(),
              ..Default::default()
            })
            .with_children(|parent| {
              // health bar (losing)
              parent
                .spawn_bundle(NodeBundle {
                  style: Style {
                    size: Size::new(Val::Px(HEALTH_BAR_WIDTH * 0.82), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                      top: Val::Px(0.0),
                      left: Val::Px(0.0),
                      ..Default::default()
                    },
                    ..Default::default()
                  },
                  material: materials.health_losing.clone(),
                  ..Default::default()
                })
                .insert(HealthBarUILosing);
              // health bar (now)
              parent
                .spawn_bundle(NodeBundle {
                  style: Style {
                    size: Size::new(Val::Px(HEALTH_BAR_WIDTH * 0.56), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                      top: Val::Px(0.0),
                      left: Val::Px(0.0),
                      ..Default::default()
                    },
                    ..Default::default()
                  },
                  material: materials.health_now_safe.clone(),
                  ..Default::default()
                })
                .insert(HealthBarUI);
            });
          // energy bar
          parent
            .spawn_bundle(NodeBundle {
              style: Style {
                size: Size::new(Val::Px(ENERGY_BAR_WIDTH), Val::Px(15.0)),
                margin: Rect {
                  top: Val::Px(10.0),
                  ..Default::default()
                },
                ..Default::default()
              },
              material: materials.energy_full.clone(),
              ..Default::default()
            })
            .with_children(|parent| {
              // energy bar (now)
              parent
                .spawn_bundle(NodeBundle {
                  style: Style {
                    size: Size::new(Val::Px(ENERGY_BAR_WIDTH * 0.24), Val::Percent(100.0)),
                    ..Default::default()
                  },
                  material: materials.energy_now.clone(),
                  ..Default::default()
                })
                .insert(EnergyBarUI);
            });
        });
    });
  commands.insert_resource(HealthLosing(0.0));
}

fn update_health_bar(
  mut query: Query<(&mut Style, &mut Handle<ColorMaterial>), With<HealthBarUI>>,
  save: Res<GameSave>,
  materials: Res<ControlPanelMaterials>,
) {
  for (mut style, mut material) in query.iter_mut() {
    let percent = save.health as f32 / save.health_limit as f32;
    style.size.width = Val::Px(HEALTH_BAR_WIDTH * percent);
    *material = if percent > 0.2 {
      materials.health_now_safe.clone()
    } else {
      materials.health_now_danger.clone()
    };
  }
}

fn update_health_losing_bar(
  mut query: Query<&mut Style, With<HealthBarUILosing>>,
  save: Res<GameSave>,
  mut losing: ResMut<HealthLosing>,
) {
  for mut style in query.iter_mut() {
    let percent = save.health as f32 / save.health_limit as f32;
    let last = losing.0;
    let now = last - (last - percent) * 0.05;
    style.size.width = Val::Px(HEALTH_BAR_WIDTH * now);
    losing.0 = now;
  }
}

fn update_energy_bar(mut query: Query<&mut Style, With<EnergyBarUI>>, save: Res<GameSave>) {
  for mut style in query.iter_mut() {
    let percent = save.energy as f32 / save.energy_limit as f32;
    style.size.width = Val::Px(ENERGY_BAR_WIDTH * percent);
  }
}

fn destroy_control_panel(mut commands: Commands, query: Query<Entity, With<ControlPanelUI>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
  commands.remove_resource::<HealthLosing>();
}

fn sync_player_status(
  mut save: ResMut<GameSave>,
  query: Query<(&Health, &SoulPower), (With<Player>, Or<(Changed<Health>, Changed<SoulPower>)>)>,
) {
  if let Ok((health, soul)) = query.single() {
    save.health = health.now;
    save.health_limit = health.max;
    save.energy = soul.now;
    save.energy_limit = soul.max;
  }
}

/// Manage the whole player control panel
pub struct ControlPanelPlugin;

impl Plugin for ControlPanelPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<ControlPanelMaterials>()
      .add_system_set(
        SystemSet::on_enter(AppState::InGame)
          .with_system(setup_control_panel)
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .with_system(update_health_bar)
          .with_system(update_energy_bar)
          .with_system(update_health_losing_bar)
          .with_system(sync_player_status)
      )
      .add_system_set(
        SystemSet::on_exit(AppState::InGame)
          .with_system(destroy_control_panel)
      );
  }
}
