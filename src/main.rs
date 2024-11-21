#![feature(let_chains)]

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::utils::HashSet;
use bevy::window::PresentMode;
use bevy::{prelude::*, sprite::Anchor, window::WindowResolution};
use rand::prelude::*;

const BACKGROUND_COLOR: Color = Color::hsv(0.0, 0.0, 0.0);

const SCALE: f32 = 2.0;
const MINEFIELD_SIZE: (usize, usize) = (30, 16);
const MINE_COUNT: usize = 99;

const BORDER_SIZE: (f32, f32) = (8.0, 8.0);
const TILE_SIZE: (f32, f32) = (16.0, 16.0);
const FACE_SIZE: (f32, f32) = (24.0, 24.0);
const CONTENT_WIDTH: f32 = (2.0 * BORDER_SIZE.0) + (MINEFIELD_SIZE.0 as f32 * TILE_SIZE.0);
const CONTENT_HEIGHT: f32 = (6.0 * BORDER_SIZE.1) + (MINEFIELD_SIZE.1 as f32 * TILE_SIZE.1);
const MINEFIELD_OFFSET: (f32, f32) = (BORDER_SIZE.0, CONTENT_HEIGHT - (BORDER_SIZE.1 * 5.0));

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        title: "mines-rs".into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(
                            CONTENT_WIDTH * SCALE,
                            CONTENT_HEIGHT * SCALE,
                        ),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_resource::<Minefield>()
        .init_resource::<MinefieldSpriteSheet>()
        .init_resource::<BorderSpriteSheet>()
        .init_resource::<FaceSpriteSheet>()
        .add_systems(Startup, setup)
        .add_systems(Update, (close_on_esc, fps_text_update_system))
        .add_systems(
            Update,
            ((handle_minefield_click, update_minefield_sprites).chain(),),
        )
        .run();
}

#[derive(Component)]
struct FpsRoot;

#[derive(Component)]
struct FpsText;

#[derive(Resource)]
struct MinefieldSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for MinefieldSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            UVec2::new(TILE_SIZE.0 as u32, TILE_SIZE.1 as u32),
            5,
            3,
            None,
            None,
        );
        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        Self(texture_atlases.add(texture_atlas))
    }
}

#[repr(usize)]
enum MinefieldSpriteIndex {
    Num,
    Hidden = 9,
    Flag = 10,
    Mine = 11,
    MineHit = 12,
    MineMissed = 13,
}

impl From<MinefieldSpriteIndex> for usize {
    fn from(value: MinefieldSpriteIndex) -> Self {
        match value {
            MinefieldSpriteIndex::Num => 0,
            MinefieldSpriteIndex::Hidden => 9,
            MinefieldSpriteIndex::Flag => 10,
            MinefieldSpriteIndex::Mine => 11,
            MinefieldSpriteIndex::MineHit => 12,
            MinefieldSpriteIndex::MineMissed => 13,
        }
    }
}

struct SpawnMinefieldSprite {
    index: MinefieldSpriteIndex,
    minefield_data: MinefieldData,
    position: Vec2,
}

impl Command for SpawnMinefieldSprite {
    fn apply(self, world: &mut World) {
        let texture: Handle<Image> = world.load_asset("spritesheet.png");
        let texture_atlas: &MinefieldSpriteSheet = world.resource();

        world.spawn((
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas.0.clone(),
                    index: self.index.into(),
                }),
                anchor: Anchor::TopLeft,
                ..default()
            },
            Transform::from_xyz(
                MINEFIELD_OFFSET.0 + self.position.x,
                MINEFIELD_OFFSET.1 - self.position.y,
                0.0,
            ),
            self.minefield_data,
        ));
    }
}

#[derive(Resource)]
struct BorderSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for BorderSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            UVec2::new(BORDER_SIZE.0 as u32, BORDER_SIZE.1 as u32),
            2,
            5,
            None,
            Some((80, 0).into()),
        );
        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        Self(texture_atlases.add(texture_atlas))
    }
}

#[repr(usize)]
enum BorderSpriteIndex {
    Vert,
    Hori,
    JoinVerticalLeft,
    JoinVerticalRight,
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
    Empty,
}

impl From<BorderSpriteIndex> for usize {
    fn from(value: BorderSpriteIndex) -> Self {
        match value {
            BorderSpriteIndex::Vert => 0,
            BorderSpriteIndex::Hori => 1,
            BorderSpriteIndex::JoinVerticalLeft => 2,
            BorderSpriteIndex::JoinVerticalRight => 3,
            BorderSpriteIndex::TopLeftCorner => 4,
            BorderSpriteIndex::TopRightCorner => 5,
            BorderSpriteIndex::BottomLeftCorner => 6,
            BorderSpriteIndex::BottomRightCorner => 7,
            BorderSpriteIndex::Empty => 8,
        }
    }
}

#[derive(Resource)]
struct FaceSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for FaceSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            UVec2::new(FACE_SIZE.0 as u32, FACE_SIZE.1 as u32),
            4,
            1,
            None,
            Some((0, 48).into()),
        );
        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        Self(texture_atlases.add(texture_atlas))
    }
}

#[repr(usize)]
enum FaceSpriteIndex {
    Idle,
    Pressed,
    Lose,
    Win,
}

impl From<FaceSpriteIndex> for usize {
    fn from(value: FaceSpriteIndex) -> Self {
        match value {
            FaceSpriteIndex::Idle => 0,
            FaceSpriteIndex::Pressed => 1,
            FaceSpriteIndex::Lose => 2,
            FaceSpriteIndex::Win => 3,
        }
    }
}

struct SpawnFaceSprite {
    index: FaceSpriteIndex,
    position: Vec2,
}

impl Command for SpawnFaceSprite {
    fn apply(self, world: &mut World) {
        let texture: Handle<Image> = world.load_asset("spritesheet.png");
        let texture_atlas: &FaceSpriteSheet = world.resource();

        world.spawn((
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas.0.clone(),
                    index: self.index.into(),
                }),
                anchor: Anchor::TopLeft,
                ..default()
            },
            Transform::from_translation(self.position.extend(0.0)),
        ));
    }
}

struct SpawnBorderSprite {
    index: BorderSpriteIndex,
    position: Vec2,
}

impl Command for SpawnBorderSprite {
    fn apply(self, world: &mut World) {
        let texture: Handle<Image> = world.load_asset("spritesheet.png");
        let texture_atlas: &BorderSpriteSheet = world.resource();

        world.spawn((
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas.0.clone(),
                    index: self.index.into(),
                }),
                anchor: Anchor::TopLeft,
                ..default()
            },
            Transform::from_translation(self.position.extend(0.0)),
        ));
    }
}

#[derive(Resource)]
struct Minefield {
    cells: Vec<Vec<u32>>,
    hidden: Vec<Vec<bool>>,
}

impl FromWorld for Minefield {
    fn from_world(_world: &mut World) -> Self {
        let mut rng = thread_rng();

        let mut cells = vec![
            vec![MinefieldSpriteIndex::Num as usize as u32; MINEFIELD_SIZE.0];
            MINEFIELD_SIZE.1
        ];

        let mut mine_locs: HashSet<UVec2> = HashSet::new();
        while mine_locs.len() < MINE_COUNT {
            mine_locs.insert(UVec2::new(
                rng.gen_range(0..MINEFIELD_SIZE.0) as u32,
                rng.gen_range(0..MINEFIELD_SIZE.1) as u32,
            ));
        }
        for loc in mine_locs {
            cells[loc.y as usize][loc.x as usize] = MinefieldSpriteIndex::Mine as u32;
        }

        for y in 0..MINEFIELD_SIZE.1 {
            for x in 0..MINEFIELD_SIZE.0 {
                if cells[y][x] == MinefieldSpriteIndex::Mine as u32 {
                    continue;
                }

                cells[y][x] = [
                    [0, 1],
                    [0, -1],
                    [1, 0],
                    [-1, 0],
                    [1, 1],
                    [1, -1],
                    [-1, 1],
                    [-1, -1],
                ]
                .into_iter()
                .map(|offset| (x as i32 + offset[1], y as i32 + offset[0]))
                .filter(|coord| {
                    coord.0 >= 0
                        && coord.0 < MINEFIELD_SIZE.0 as i32
                        && coord.1 >= 0
                        && coord.1 < MINEFIELD_SIZE.1 as i32
                })
                .map(|coord| {
                    cells[coord.1 as usize][coord.0 as usize] == MinefieldSpriteIndex::Mine as u32
                })
                .filter(|is_mine| *is_mine)
                .count() as u32;
            }
        }

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

fn setup(mut commands: Commands, minefield: Res<Minefield>) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            far: 1000.0,
            near: -1000.0,
            viewport_origin: Vec2::ZERO,
            scale: 1.0 / SCALE,
            ..OrthographicProjection::default_2d()
        },
    ));

    commands.queue(SpawnBorderSprite {
        index: BorderSpriteIndex::TopLeftCorner,
        position: (0.0, CONTENT_HEIGHT).into(),
    });

    commands.queue(SpawnBorderSprite {
        index: BorderSpriteIndex::TopRightCorner,
        position: (CONTENT_WIDTH - BORDER_SIZE.0, CONTENT_HEIGHT).into(),
    });

    commands.queue(SpawnBorderSprite {
        index: BorderSpriteIndex::BottomLeftCorner,
        position: (0.0, BORDER_SIZE.1).into(),
    });

    commands.queue(SpawnBorderSprite {
        index: BorderSpriteIndex::BottomRightCorner,
        position: (CONTENT_WIDTH - BORDER_SIZE.0, BORDER_SIZE.1).into(),
    });

    // horizontal segments
    for i in 0..2 * MINEFIELD_SIZE.0 {
        commands.queue(SpawnBorderSprite {
            index: BorderSpriteIndex::Hori,
            position: (BORDER_SIZE.0 * (i as f32 + 1.0), CONTENT_HEIGHT).into(),
        });

        commands.queue(SpawnBorderSprite {
            index: BorderSpriteIndex::Hori,
            position: (
                BORDER_SIZE.0 * (i as f32 + 1.0),
                CONTENT_HEIGHT - (BORDER_SIZE.1 * 4.0),
            )
                .into(),
        });

        commands.queue(SpawnBorderSprite {
            index: BorderSpriteIndex::Hori,
            position: (BORDER_SIZE.0 * (i as f32 + 1.0), BORDER_SIZE.1).into(),
        });
    }

    // vertical segments
    for i in 0..(2 * MINEFIELD_SIZE.1 + 4) {
        if i == 3 {
            // skip the join
            continue;
        }

        commands.queue(SpawnBorderSprite {
            index: BorderSpriteIndex::Vert,
            position: (0.0, CONTENT_HEIGHT - BORDER_SIZE.1 * (i as f32 + 1.0)).into(),
        });

        commands.queue(SpawnBorderSprite {
            index: BorderSpriteIndex::Vert,
            position: (
                CONTENT_WIDTH - BORDER_SIZE.0,
                CONTENT_HEIGHT - BORDER_SIZE.1 * (i as f32 + 1.0),
            )
                .into(),
        });
    }

    commands.queue(SpawnBorderSprite {
        index: BorderSpriteIndex::JoinVerticalLeft,
        position: (0.0, CONTENT_HEIGHT - BORDER_SIZE.1 * 4.0).into(),
    });

    commands.queue(SpawnBorderSprite {
        index: BorderSpriteIndex::JoinVerticalRight,
        position: (
            CONTENT_WIDTH - BORDER_SIZE.0,
            CONTENT_HEIGHT - BORDER_SIZE.1 * 4.0,
        )
            .into(),
    });

    commands.queue(SpawnFaceSprite {
        index: FaceSpriteIndex::Idle,
        position: (
            (CONTENT_WIDTH / 2.0) - (FACE_SIZE.0 / 2.0),
            CONTENT_HEIGHT - BORDER_SIZE.1,
        )
            .into(),
    });

    for row in 0..minefield.cells.len() {
        for col in 0..minefield.cells[row].len() {
            commands.queue(SpawnMinefieldSprite {
                index: MinefieldSpriteIndex::Hidden,
                minefield_data: MinefieldData {
                    position: (row, col),
                },
                position: (col as f32 * TILE_SIZE.0, row as f32 * TILE_SIZE.0).into(),
            })
        }
    }

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

fn handle_minefield_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
    mut minefield: ResMut<Minefield>,
) {
    if let Ok((camera, camera_transform)) = camera.get_single()
        && let Ok(window) = window.get_single()
        && let Some(pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| Vec2::new(ray.origin.x.floor(), ray.origin.y.ceil()))
    {
        if mouse_button_input.just_released(MouseButton::Left) {
            if pos.x >= MINEFIELD_OFFSET.0
                && pos.x < MINEFIELD_OFFSET.0 + (MINEFIELD_SIZE.0 as f32 * TILE_SIZE.0)
                && pos.y <= MINEFIELD_OFFSET.1
                && pos.y > MINEFIELD_OFFSET.1 - (MINEFIELD_SIZE.1 as f32 * TILE_SIZE.1)
            {
                let minefield_row =
                    ((MINEFIELD_OFFSET.1 - pos.y) as u32 / TILE_SIZE.1 as u32) as usize;
                let minefield_col =
                    ((pos.x - MINEFIELD_OFFSET.0) as u32 / TILE_SIZE.0 as u32) as usize;

                let visibility = minefield.hidden[minefield_row][minefield_col];
                minefield.hidden[minefield_row][minefield_col] = !visibility;
            }
        }
    }
}

fn update_minefield_sprites(
    mut sprites: Query<(&mut Sprite, &MinefieldData)>,
    minefield: Res<Minefield>,
) {
    for (ref mut sprite, data) in sprites.iter_mut() {
        if let Some(texture_atlas) = &mut sprite.texture_atlas {
            texture_atlas.index = if minefield.hidden[data.position.0][data.position.1] {
                MinefieldSpriteIndex::Hidden.into()
            } else {
                match minefield.cells[data.position.0][data.position.1] {
                    9 => MinefieldSpriteIndex::Hidden as usize,
                    10 => MinefieldSpriteIndex::Flag as usize,
                    11 => MinefieldSpriteIndex::Mine as usize,
                    12 => MinefieldSpriteIndex::MineHit as usize,
                    13 => MinefieldSpriteIndex::MineMissed as usize,
                    n => MinefieldSpriteIndex::Num as usize + n as usize,
                }
            }
        }
    }
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&mut Text, &mut TextColor), With<FpsText>>,
) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.0 .0 = format!("{value:>4.0}");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            text.1 .0 = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::srgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::srgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::srgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                // Below 30 FPS, use red color
                Color::srgb(1.0, 0.0, 0.0)
            }
        } else {
            // display "N/A" if we can't get an FPS measurement
            // add an extra space to preserve alignment
            text.0 .0 = " N/A".into();
            text.1 .0 = Color::WHITE;
        }
    }
}
