#![feature(let_chains)]

use std::fmt::Debug;
use bevy::{prelude::*, sprite::Anchor, window::{PrimaryWindow, WindowResolution}};
use rand::prelude::*;

const TILE_SIZE: (u32, u32) = (16, 16);
const SCALE: f32 = 1.0;
const MINEFIELD_SIZE: (usize, usize) = (30, 16);
const BACKGROUND_COLOR: Color = Color::hsv(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "mines-rs".into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(
                            ((MINEFIELD_SIZE.0 as u32) * (TILE_SIZE.0)) as f32 * SCALE,
                            ((MINEFIELD_SIZE.1 as u32) * (TILE_SIZE.1)) as f32 * SCALE,
                        ),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(RandomizeTimer(Timer::from_seconds(
            0.25,
            TimerMode::Repeating,
        )))
        .init_resource::<Minefield>()
        .init_resource::<MinesSpriteSheet>()
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .add_systems(
            Update,
            (
                handle_click,
                (random_shuffle_sprite, update_minefield_sprites).chain(),
            ),
        )
        .run();
}

#[derive(Resource)]
struct RandomizeTimer(Timer);

#[derive(Resource)]
struct MinesSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for MinesSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(TILE_SIZE.into(), 4, 3, None, None);
        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        Self(texture_atlases.add(texture_atlas))
    }
}

#[derive(Resource)]
struct Minefield {
    cells: Vec<Vec<u32>>,
    hidden: Vec<Vec<bool>>,
}

impl FromWorld for Minefield {
    fn from_world(_world: &mut World) -> Self {
        let cells = vec![vec![0; MINEFIELD_SIZE.0]; MINEFIELD_SIZE.1];
        let hidden = vec![vec![false; MINEFIELD_SIZE.0]; MINEFIELD_SIZE.1];

        Self { cells, hidden }
    }
}

#[derive(Component)]
struct MinefieldData {
    position: (usize, usize),
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

fn setup(
    mut commands: Commands,
    minefield: Res<Minefield>,
    texture_atlas: Res<MinesSpriteSheet>,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window.single();

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.0,
            near: -1000.0,
            viewport_origin: Vec2::ZERO,
            ..default()
        },
        ..default()
    });

    let texture = asset_server.load("spritesheet.png");

    let center = (window.width() / 2.0, window.height() / 2.0);
    let minefield_w = (MINEFIELD_SIZE.0 as u32 * TILE_SIZE.0) as f32 * SCALE;
    let minefield_h = (MINEFIELD_SIZE.1 as u32 * TILE_SIZE.1) as f32 * SCALE;
    let top_left = (
        center.0 - (minefield_w / 2.0),
        center.1 + (minefield_h / 2.0),
    );

    for row in 0..minefield.cells.len() {
        for col in 0..minefield.cells[row].len() {
            let x = top_left.0 + (col as u32 * TILE_SIZE.0) as f32 * SCALE;
            let y = top_left.1 - (row as u32 * TILE_SIZE.0) as f32 * SCALE;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    transform: Transform {
                        translation: (x, y, 0.0).into(),
                        scale: Vec2::splat(SCALE).extend(0.0),
                        ..default()
                    },
                    texture: texture.clone(),
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas.0.clone(),
                    index: minefield.cells[row][col] as usize,
                },
                MinefieldData {
                    position: (row, col),
                },
            ));
        }
    }
}

fn random_shuffle_sprite(
    time: Res<Time>,
    mut timer: ResMut<RandomizeTimer>,
    mut minefield: ResMut<Minefield>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let row = thread_rng().gen_range(0..minefield.cells.len());
        let col = thread_rng().gen_range(0..minefield.cells[row].len());
        minefield.cells[row][col] = thread_rng().gen_range(0..12);
    }
}

fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
) {
    if let Ok((camera, camera_transform)) = camera.get_single()
        && let Ok(window) = window.get_single()
        && let Some(pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            info!("{:?}", pos);
        }
    }
}

fn update_minefield_sprites(
    mut sprites: Query<(&mut TextureAtlas, &MinefieldData)>,
    minefield: Res<Minefield>,
) {
    for (mut texture, data) in sprites.iter_mut() {
        texture.index = if minefield.hidden[data.position.0][data.position.1] {
            0
        } else {
            minefield.cells[data.position.0][data.position.1] as usize
        }
    }
}
