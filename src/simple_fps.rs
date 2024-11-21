use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Component)]
struct FpsRoot;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct FpsThresholds {
    max: f64,
    stable: f64,
    low: f64,
}

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
            FpsThresholds {
                max: 60.0,
                stable: 45.0,
                low: 30.0,
            },
        ))
        .id();

    commands
        .entity(root)
        .add_children(&[text_fps_label, text_fps]);
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&mut Text, &mut TextColor, &FpsThresholds), With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.0 .0 = format!("{value:>4.0}");

            text.1 .0 = if value >= text.2.max {
                Color::srgb(0.0, 1.0, 0.0)
            } else if value >= text.2.stable {
                Color::srgb(
                    (1.0 - (value - text.2.stable) / (text.2.max - text.2.stable)) as f32,
                    1.0,
                    0.0,
                )
            } else if value >= text.2.low {
                Color::srgb(
                    1.0,
                    ((value - text.2.low) / (text.2.stable - text.2.low)) as f32,
                    0.0,
                )
            } else {
                Color::srgb(1.0, 0.0, 0.0)
            }
        } else {
            text.0 .0 = " N/A".into();
            text.1 .0 = Color::WHITE;
        }
    }
}
