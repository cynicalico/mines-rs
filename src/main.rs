#![feature(let_chains)]

mod commands;
mod constants;
mod minefield;
mod simple_fps;
mod spritesheets;

use commands::*;
use constants::*;
use minefield::*;
use spritesheets::*;

use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
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
            // simple_fps::plugin,
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_resource::<Minefield>()
        .init_resource::<MinefieldSpriteSheet>()
        .init_resource::<BorderSpriteSheet>()
        .init_resource::<FaceSpriteSheet>()
        .init_resource::<ScoreSpriteSheet>()
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .add_systems(
            Update,
            (
                handle_minefield_click,
                update_minefield_sprites,
                update_mine_count_sprites,
            ),
        )
        .run();
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
        position: Vec2::new(0.0, CONTENT_HEIGHT),
    });

    commands.queue(SpawnBorderSprite {
        index: BorderSpriteIndex::TopRightCorner,
        position: Vec2::new(CONTENT_WIDTH - BORDER_SPRITE_SIZE.0, CONTENT_HEIGHT),
    });

    commands.queue(SpawnBorderSprite {
        index: BorderSpriteIndex::BottomLeftCorner,
        position: Vec2::new(0.0, BORDER_SPRITE_SIZE.1),
    });

    commands.queue(SpawnBorderSprite {
        index: BorderSpriteIndex::BottomRightCorner,
        position: Vec2::new(CONTENT_WIDTH - BORDER_SPRITE_SIZE.0, BORDER_SPRITE_SIZE.1),
    });

    // horizontal segments
    for i in 0..2 * MINEFIELD_SIZE.0 {
        commands.queue(SpawnBorderSprite {
            index: BorderSpriteIndex::Hori,
            position: Vec2::new(BORDER_SPRITE_SIZE.0 * (i as f32 + 1.0), CONTENT_HEIGHT),
        });

        commands.queue(SpawnBorderSprite {
            index: BorderSpriteIndex::Hori,
            position: Vec2::new(
                BORDER_SPRITE_SIZE.0 * (i as f32 + 1.0),
                CONTENT_HEIGHT - (BORDER_SPRITE_SIZE.1 * 4.0),
            ),
        });

        commands.queue(SpawnBorderSprite {
            index: BorderSpriteIndex::Hori,
            position: Vec2::new(
                BORDER_SPRITE_SIZE.0 * (i as f32 + 1.0),
                BORDER_SPRITE_SIZE.1,
            ),
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
            position: Vec2::new(
                0.0,
                CONTENT_HEIGHT - BORDER_SPRITE_SIZE.1 * (i as f32 + 1.0),
            ),
        });

        commands.queue(SpawnBorderSprite {
            index: BorderSpriteIndex::Vert,
            position: Vec2::new(
                CONTENT_WIDTH - BORDER_SPRITE_SIZE.0,
                CONTENT_HEIGHT - BORDER_SPRITE_SIZE.1 * (i as f32 + 1.0),
            ),
        });
    }

    commands.queue(SpawnBorderSprite {
        index: BorderSpriteIndex::JoinVerticalLeft,
        position: Vec2::new(0.0, CONTENT_HEIGHT - BORDER_SPRITE_SIZE.1 * 4.0),
    });

    commands.queue(SpawnBorderSprite {
        index: BorderSpriteIndex::JoinVerticalRight,
        position: Vec2::new(
            CONTENT_WIDTH - BORDER_SPRITE_SIZE.0,
            CONTENT_HEIGHT - BORDER_SPRITE_SIZE.1 * 4.0,
        ),
    });

    // infill on top
    for row in 0..3 {
        for col in 0..2 * MINEFIELD_SIZE.0 {
            commands.queue(SpawnBorderSprite {
                index: BorderSpriteIndex::Empty,
                position: Vec2::new(
                    BORDER_SPRITE_SIZE.0 + (col as f32 * BORDER_SPRITE_SIZE.0),
                    CONTENT_HEIGHT - ((row + 1) as f32 * BORDER_SPRITE_SIZE.1),
                ),
            })
        }
    }

    commands.queue(SpawnScoreFrame {
        position: Vec2::new(
            BORDER_SPRITE_SIZE.0 + 1.0,
            CONTENT_HEIGHT - (BORDER_SPRITE_SIZE.1 + 1.0),
        ),
    });

    commands.queue(SpawnScoreFrame {
        position: Vec2::new(
            CONTENT_WIDTH - (BORDER_SPRITE_SIZE.0 + 1.0 + SCORE_FRAME_SIZE.0),
            CONTENT_HEIGHT - (BORDER_SPRITE_SIZE.1 + 1.0),
        ),
    });

    commands.queue(SpawnMineCount {
        position: Vec2::new(
            BORDER_SPRITE_SIZE.0 + 2.0,
            CONTENT_HEIGHT - (BORDER_SPRITE_SIZE.1 + 2.0),
        ),
    });

    commands.queue(SpawnFaceSprite {
        index: FaceSpriteIndex::Idle,
        position: Vec2::new(
            (CONTENT_WIDTH / 2.0) - (FACE_SPRITE_SIZE.0 / 2.0),
            CONTENT_HEIGHT - BORDER_SPRITE_SIZE.1,
        ),
    });

    for row in 0..minefield.cells.len() {
        for col in 0..minefield.cells[row].len() {
            commands.queue(SpawnMinefieldSprite {
                index: MinefieldSpriteIndex::Hidden,
                minefield_data: MinefieldData {
                    position: (row, col),
                },
                position: Vec2::new(
                    col as f32 * MINEFIELD_SPRITE_SIZE.0,
                    row as f32 * MINEFIELD_SPRITE_SIZE.0,
                ),
            })
        }
    }
}

fn mouse_pos_to_minefield_coords(pos: Vec2) -> Option<(usize, usize)> {
    if pos.x >= MINEFIELD_OFFSET.0
        && pos.x < MINEFIELD_OFFSET.0 + (MINEFIELD_SIZE.0 as f32 * MINEFIELD_SPRITE_SIZE.0)
        && pos.y <= MINEFIELD_OFFSET.1
        && pos.y > MINEFIELD_OFFSET.1 - (MINEFIELD_SIZE.1 as f32 * MINEFIELD_SPRITE_SIZE.1)
    {
        Some((
            ((MINEFIELD_OFFSET.1 - pos.y) as u32 / MINEFIELD_SPRITE_SIZE.1 as u32) as usize,
            ((pos.x - MINEFIELD_OFFSET.0) as u32 / MINEFIELD_SPRITE_SIZE.0 as u32) as usize,
        ))
    } else {
        None
    }
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
            if let Some((minefield_row, minefield_col)) = mouse_pos_to_minefield_coords(pos) {
                let visibility = minefield.hidden[minefield_row][minefield_col];
                minefield.hidden[minefield_row][minefield_col] = !visibility;
            }
        } else if mouse_button_input.just_released(MouseButton::Right) {
            if let Some((minefield_row, minefield_col)) = mouse_pos_to_minefield_coords(pos) {
                let flagged = minefield.flags[minefield_row][minefield_col];
                minefield.flags[minefield_row][minefield_col] = !flagged;
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
            texture_atlas.index = if minefield.flags[data.position.0][data.position.1] {
                MinefieldSpriteIndex::Flag.into()
            } else if minefield.hidden[data.position.0][data.position.1] {
                MinefieldSpriteIndex::Hidden.into()
            } else {
                match minefield.cells[data.position.0][data.position.1] {
                    11 => MinefieldSpriteIndex::Mine as usize,
                    12 => MinefieldSpriteIndex::MineHit as usize,
                    13 => MinefieldSpriteIndex::MineMissed as usize,
                    n => MinefieldSpriteIndex::Num as usize + n as usize,
                }
            }
        }
    }
}

fn update_mine_count_sprites(
    mine_count: Query<(&mut MineCount, &Children)>,
    mut digit: Query<&mut Sprite>,
) {
    let mine_count = mine_count.single();

    let count_str = format!("{:0>3}", mine_count.0.value.to_string())
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut idx: usize = 0;
    for &c in mine_count.1.iter() {
        if idx < count_str.len() {
            if let Ok(mut d) = digit.get_mut(c) {
                if let Some(texture_atlas) = &mut d.texture_atlas {
                    texture_atlas.index = count_str[idx];
                }
            }
        }
        idx += 1;
    }
}
