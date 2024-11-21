use bevy::prelude::*;

pub const MINEFIELD_SPRITE_SIZE: (f32, f32) = (16.0, 16.0);
pub const BORDER_SPRITE_SIZE: (f32, f32) = (8.0, 8.0);
pub const FACE_SPRITE_SIZE: (f32, f32) = (24.0, 24.0);
pub const SCORE_SPRITE_SIZE: (f32, f32) = (11.0, 20.0);
pub const SCORE_FRAME_SIZE: (f32, f32) = (35.0, 22.0);

#[derive(Resource)]
pub struct MinefieldSpriteSheet(pub Handle<TextureAtlasLayout>);

impl FromWorld for MinefieldSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            UVec2::new(
                MINEFIELD_SPRITE_SIZE.0 as u32,
                MINEFIELD_SPRITE_SIZE.1 as u32,
            ),
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
pub enum MinefieldSpriteIndex {
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

#[derive(Resource)]
pub struct BorderSpriteSheet(pub Handle<TextureAtlasLayout>);

impl FromWorld for BorderSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            UVec2::new(BORDER_SPRITE_SIZE.0 as u32, BORDER_SPRITE_SIZE.1 as u32),
            9,
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
pub enum BorderSpriteIndex {
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
pub struct FaceSpriteSheet(pub Handle<TextureAtlasLayout>);

impl FromWorld for FaceSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            UVec2::new(FACE_SPRITE_SIZE.0 as u32, FACE_SPRITE_SIZE.1 as u32),
            4,
            1,
            None,
            Some((0, 56).into()),
        );
        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        Self(texture_atlases.add(texture_atlas))
    }
}

#[repr(usize)]
pub enum FaceSpriteIndex {
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

#[derive(Resource)]
pub struct ScoreSpriteSheet(pub Handle<TextureAtlasLayout>);

impl FromWorld for ScoreSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            UVec2::new(SCORE_SPRITE_SIZE.0 as u32, SCORE_SPRITE_SIZE.1 as u32),
            6,
            2,
            None,
            Some((0, 80).into()),
        );
        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        Self(texture_atlases.add(texture_atlas))
    }
}
