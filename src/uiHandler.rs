use bevy::prelude::*;

#[derive(Component)]
pub struct FpsText;
pub fn setup_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new("fps ->"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
        FpsText {},
    ));
}
pub fn update_ui(mut fps_text_query: Query<&mut Text, With<FpsText>>, time: Res<Time>) {
    let fps = (1f32 / time.delta_secs()).round();
    let mut fps_text = fps_text_query.single_mut();
    fps_text.0 = format!("fps: {}", fps);
}
