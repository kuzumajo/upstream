use crate::consts::*;
use crate::FontAssets;
use bevy::prelude::*;

struct GameMenuUI;

struct MenuButtonMaterials {
  normal: Handle<ColorMaterial>,
  hovered: Handle<ColorMaterial>,
  pressed: Handle<ColorMaterial>,
}

impl FromWorld for MenuButtonMaterials {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

    MenuButtonMaterials {
      normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
      hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
      pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
    }
  }
}

/// Open Studio Logo Page
fn setup_menu(
  mut commands: Commands,
  menu_materials: Res<MenuButtonMaterials>,
  font_assets: Res<FontAssets>,
) {
  commands
    .spawn_bundle(ButtonBundle {
      style: Style {
        size: Size::new(Val::Px(300.0), Val::Px(65.0)),
        margin: Rect::all(Val::Auto),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
      },
      material: menu_materials.normal.clone(),
      ..Default::default()
    })
    .insert(GameMenuUI)
    .with_children(|parent| {
      parent.spawn_bundle(TextBundle {
        text: Text::with_section(
          "Start",
          TextStyle {
            font: font_assets.default_font.clone(),
            font_size: 40.0,
            color: Color::WHITE,
          },
          Default::default(),
        ),
        ..Default::default()
      });
    });

  trace!("initialized menu page");
}

fn button_system(
  button_materials: Res<MenuButtonMaterials>,
  mut interaction_query: Query<
    (&Interaction, &mut Handle<ColorMaterial>),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut material) in interaction_query.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        *material = button_materials.pressed.clone();
      }
      Interaction::Hovered => {
        *material = button_materials.hovered.clone();
      }
      Interaction::None => {
        *material = button_materials.normal.clone();
      }
    }
  }
}

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<MenuButtonMaterials>()
      .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu.system()))
      .add_system_set(SystemSet::on_update(AppState::Menu).with_system(button_system.system()));
  }
}
