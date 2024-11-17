use bevy::prelude::*;
use rand::prelude::*;

const LOGO_SPEED: f32 = 750.0;
const BACKGROUND_COLOR: Color = Color::hsv(0.0, 1.0, 0.25);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_logo, cycle_clear_color))
        .run();
}

#[derive(Component)]
struct Logo;

#[derive(Component)]
struct Velocity(Vec2);

fn setup(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform::default(),
            texture: assert_server.load("bevy_bird_dark.png"),
            ..default()
        },
        Velocity(Vec2::new(1.0, random::<f32>() + 0.5).normalize()),
        Logo,
    ));
}

fn move_logo(
    mut logo: Query<(&mut Transform, &mut Velocity, &Handle<Image>), With<Logo>>,
    window: Query<&Window>,
    time: Res<Time>,
    assets: Res<Assets<Image>>,
) {
    let Ok(mut logo) = logo.get_single_mut() else {
        return;
    };

    let Ok(window) = window.get_single() else {
        return;
    };

    let Some(image) = assets.get(logo.2) else {
        return;
    };

    if logo.0.translation.x - (image.width() as f32 / 2.0) <= -(window.width() / 2.0)
        || logo.0.translation.x + (image.width() as f32 / 2.0) >= window.width() / 2.0
    {
        logo.1 .0.x *= -1.0;
    }

    if logo.0.translation.y - (image.height() as f32 / 2.0) <= -(window.height() / 2.0)
        || logo.0.translation.y + (image.height() as f32 / 2.0) >= window.height() / 2.0
    {
        logo.1 .0.y *= -1.0;
    }

    let move_delta = logo.1 .0 * LOGO_SPEED * time.delta_seconds();
    logo.0.translation += move_delta.extend(0.0);
}

fn cycle_clear_color(mut color: ResMut<ClearColor>, time: Res<Time>) {
    color.0.set_hue(90.0 * time.elapsed_seconds_wrapped());
}
