use bevy::prelude::*;
use rand::prelude::*;

const MINEFIELD_SIZE: (usize, usize) = (10, 10);
const BACKGROUND_COLOR: Color = Color::hsv(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Cell {
    position: UVec2,
    is_mine: bool,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(7), 9, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec2::splat(100.0).extend(0.0),
                ..default()
            },
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        Cell {
            position: UVec2::splat(0),
            is_mine: false,
        },
    ));
}
