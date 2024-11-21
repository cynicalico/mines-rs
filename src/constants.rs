use crate::minefield::*;
use crate::spritesheets::*;
use bevy::color::Color;

pub const BACKGROUND_COLOR: Color = Color::hsv(0.0, 0.0, 0.0);

pub const SCALE: f32 = 2.0;

pub const CONTENT_WIDTH: f32 =
    (2.0 * BORDER_SPRITE_SIZE.0) + (MINEFIELD_SIZE.0 as f32 * MINEFIELD_SPRITE_SIZE.0);

pub const CONTENT_HEIGHT: f32 =
    (6.0 * BORDER_SPRITE_SIZE.1) + (MINEFIELD_SIZE.1 as f32 * MINEFIELD_SPRITE_SIZE.1);

pub const MINEFIELD_OFFSET: (f32, f32) = (
    BORDER_SPRITE_SIZE.0,
    CONTENT_HEIGHT - (BORDER_SPRITE_SIZE.1 * 5.0),
);
