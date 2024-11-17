use bevy::prelude::*;
use rand::prelude::*;

const LOGO_SPEED: f32 = 750.0;
const BACKGROUND_COLOR: Color = Color::hsv(0.0, 1.0, 0.25);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<HitEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (check_hit, (move_logo, cycle_clear_color), play_hit_sound).chain(),
        )
        .run();
}

#[derive(Component)]
struct Logo;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct JustHit(bool);

#[derive(Resource)]
struct HitSound(Handle<AudioSource>);

#[derive(Event, Default)]
struct HitEvent;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform::default(),
            texture: asset_server.load("bevy_bird_dark.png"),
            ..default()
        },
        Velocity(Vec2::new(1.0, random::<f32>() + 0.5).normalize()),
        JustHit(false),
        Logo,
    ));

    let hit_sound = asset_server.load("hit.wav");
    commands.insert_resource(HitSound(hit_sound));
}

fn check_hit(
    mut logo: Query<(&mut Transform, &mut Velocity, &mut JustHit, &Handle<Image>), With<Logo>>,
    window: Query<&Window>,
    assets: Res<Assets<Image>>,
    mut hit_events: EventWriter<HitEvent>,
) {
    let Ok(mut logo) = logo.get_single_mut() else {
        return;
    };

    let Ok(window) = window.get_single() else {
        return;
    };

    let Some(image) = assets.get(logo.3) else {
        return;
    };

    if logo.2 .0 {
        logo.2 .0 = false;
    } else if logo.0.translation.x - (image.width() as f32 / 2.0) <= -(window.width() / 2.0)
        || logo.0.translation.x + (image.width() as f32 / 2.0) >= window.width() / 2.0
    {
        logo.1 .0.x *= -1.0;
        logo.2 .0 = true;
        hit_events.send_default();
    } else if logo.0.translation.y - (image.height() as f32 / 2.0) <= -(window.height() / 2.0)
        || logo.0.translation.y + (image.height() as f32 / 2.0) >= window.height() / 2.0
    {
        logo.1 .0.y *= -1.0;
        logo.2 .0 = true;
        hit_events.send_default();
    }
}

fn move_logo(
    mut logo: Query<(&mut Transform, &mut Velocity, &Handle<Image>), With<Logo>>,
    time: Res<Time>,
) {
    let Ok(mut logo) = logo.get_single_mut() else {
        return;
    };

    let move_delta = logo.1 .0 * LOGO_SPEED * time.delta_seconds();
    logo.0.translation += move_delta.extend(0.0);
}

fn cycle_clear_color(mut color: ResMut<ClearColor>, time: Res<Time>) {
    color.0.set_hue(90.0 * time.elapsed_seconds_wrapped());
}

fn play_hit_sound(
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
    sound: Res<HitSound>,
) {
    if !hit_events.is_empty() {
        hit_events.clear();
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            settings: PlaybackSettings::DESPAWN,
            ..default()
        });
    }
}
