use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::{PrimaryWindow, WindowResolution};
use rand::prelude::*;

const TILE_SIZE: (u32, u32) = (7, 7);
const SCALE: f32 = 10.0;
const MINEFIELD_SIZE: (usize, usize) = (10, 10);
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
                            (MINEFIELD_SIZE.0 as u32 * (TILE_SIZE.0 + 2)) as f32 * SCALE,
                            (MINEFIELD_SIZE.1 as u32 * (TILE_SIZE.1 + 2)) as f32 * SCALE,
                        ),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_resource::<Minefield>()
        .init_resource::<MinesSpriteSheet>()
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

#[derive(Resource)]
struct MinesSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for MinesSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(TILE_SIZE.into(), 9, 2, None, None);
        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        Self(texture_atlases.add(texture_atlas))
    }
}

#[derive(Resource)]
struct Minefield {
    cells: Vec<Vec<bool>>,
    hidden: Vec<Vec<bool>>,
}

impl FromWorld for Minefield {
    fn from_world(world: &mut World) -> Self {
        let cells = vec![vec![false; MINEFIELD_SIZE.0]; MINEFIELD_SIZE.1];
        let hidden = vec![vec![false; MINEFIELD_SIZE.0]; MINEFIELD_SIZE.1];

        Self { cells, hidden }
    }
}

#[derive(Component)]
struct MinefieldData {
    position: (usize, usize),
}

fn setup(
    mut commands: Commands,
    minefield: Res<Minefield>,
    texture_atlas: Res<MinesSpriteSheet>,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("spritesheet.png");

    let window = window.single();

    let center = (0.0, 0.0);
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
                    index: 1,
                },
                MinefieldData {
                    position: (row, col),
                },
            ));
        }
    }
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
