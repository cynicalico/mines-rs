use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Component)]
struct FpsRoot;

#[derive(Component)]
struct FpsText;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup_fps_text)
        .add_systems(Update, update_fps_text);
}

fn setup_fps_text(mut commands: Commands) {
    let root = commands
        .spawn((
            FpsRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(1.),
                right: Val::Auto,
                top: Val::Percent(1.),
                bottom: Val::Auto,
                padding: UiRect::all(Val::Px(4.0)),
                ..Default::default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            ZIndex(i32::MAX),
        ))
        .id();

    let text_fps_label = commands
        .spawn((
            Text::new("FPS: "),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();

    let text_fps = commands
        .spawn((
            Text::new("N/A"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            FpsText,
        ))
        .id();

    commands
        .entity(root)
        .add_children(&[text_fps_label, text_fps]);
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&mut Text, &mut TextColor), With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.0 .0 = format!("{value:>4.0}");

            text.1 .0 = if value >= 120.0 {
                Color::srgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                Color::srgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                Color::srgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                Color::srgb(1.0, 0.0, 0.0)
            }
        } else {
            text.0 .0 = " N/A".into();
            text.1 .0 = Color::WHITE;
        }
    }
}
