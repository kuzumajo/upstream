use bevy::prelude::*;

pub fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  let font = asset_server.load("font/hanyi.ttf");
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  
  let text_style = TextStyle {
    font,
    font_size: 60.0,
    color: Color::BLACK,
  };
  let text_alignment = TextAlignment {
    vertical: VerticalAlign::Center,
    horizontal: HorizontalAlign::Center,
  };

  commands.spawn_bundle(Text2dBundle {
    text: Text::with_section(
      "Kuzumajo Studio",
      text_style.clone(),
      text_alignment.clone(),
    ),
    transform: Transform::from_xyz(0.0, 50.0, 0.0),
    ..Default::default()
  });

  commands.spawn_bundle(Text2dBundle {
    text: Text::with_section(
      "屑魔女工作室",
      text_style.clone(),
      text_alignment.clone(),
    ),
    transform: Transform::from_xyz(0.0, -20.0, 0.0),
    ..Default::default()
  });

}
