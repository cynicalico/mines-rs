use crate::constants::*;
use crate::minefield::*;
use crate::spritesheets::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct SpawnMinefieldSprite {
    pub index: MinefieldSpriteIndex,
    pub minefield_data: MinefieldData,
    pub position: Vec2,
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

pub struct SpawnBorderSprite {
    pub index: BorderSpriteIndex,
    pub position: Vec2,
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

pub struct SpawnFaceSprite {
    pub index: FaceSpriteIndex,
    pub position: Vec2,
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
            Transform::from_translation(self.position.extend(1.0)),
        ));
    }
}

#[derive(Component)]
pub struct MineCount {
    pub value: usize,
}

pub struct SpawnMineCount {
    pub position: Vec2,
}

impl Command for SpawnMineCount {
    fn apply(self, world: &mut World) {
        let texture: Handle<Image> = world.load_asset("spritesheet.png");
        let texture_atlas: Handle<TextureAtlasLayout> =
            world.resource::<ScoreSpriteSheet>().0.clone();

        world
            .spawn((
                MineCount { value: MINE_COUNT },
                Transform::default(),
                Visibility::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Sprite {
                        image: texture.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas.clone(),
                            index: 10,
                        }),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(self.position.x, self.position.y, 2.0),
                        ..default()
                    },
                ));
                parent.spawn((
                    Sprite {
                        image: texture.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas.clone(),
                            index: 10,
                        }),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(
                            self.position.x + SCORE_SPRITE_SIZE.0,
                            self.position.y,
                            2.0,
                        ),
                        ..default()
                    },
                ));
                parent.spawn((
                    Sprite {
                        image: texture.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas.clone(),
                            index: 10,
                        }),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(
                            self.position.x + SCORE_SPRITE_SIZE.0 * 2.0,
                            self.position.y,
                            2.0,
                        ),
                        ..default()
                    },
                ));
            });
    }
}

pub struct SpawnScoreFrame {
    pub position: Vec2,
}

impl Command for SpawnScoreFrame {
    fn apply(self, world: &mut World) {
        let texture: Handle<Image> = world.load_asset("spritesheet.png");

        world.spawn((
            Sprite {
                image: texture.clone(),
                rect: Some(Rect::new(
                    0.0,
                    120.0,
                    SCORE_FRAME_SIZE.0,
                    120.0 + SCORE_FRAME_SIZE.1,
                )),
                anchor: Anchor::TopLeft,
                ..default()
            },
            Transform::from_translation(self.position.extend(1.0)),
        ));
    }
}
