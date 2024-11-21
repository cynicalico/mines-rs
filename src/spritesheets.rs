use crate::{MinefieldData, MINEFIELD_OFFSET};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub const TILE_SIZE: (f32, f32) = (16.0, 16.0);
pub const BORDER_SIZE: (f32, f32) = (8.0, 8.0);
pub const FACE_SIZE: (f32, f32) = (24.0, 24.0);

#[derive(Resource)]
pub struct MinefieldSpriteSheet(Handle<TextureAtlasLayout>);

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

#[derive(Resource)]
pub struct BorderSpriteSheet(Handle<TextureAtlasLayout>);

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

#[derive(Resource)]
pub struct FaceSpriteSheet(Handle<TextureAtlasLayout>);

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

pub struct SpawnMinefieldSprite {
    pub(crate) index: MinefieldSpriteIndex,
    pub(crate) minefield_data: MinefieldData,
    pub(crate) position: Vec2,
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

pub struct SpawnFaceSprite {
    pub(crate) index: FaceSpriteIndex,
    pub(crate) position: Vec2,
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

pub struct SpawnBorderSprite {
    pub(crate) index: BorderSpriteIndex,
    pub(crate) position: Vec2,
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
