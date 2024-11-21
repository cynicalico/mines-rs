use crate::spritesheets::MinefieldSpriteIndex;
use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::prelude::*;

pub const MINEFIELD_SIZE: (usize, usize) = (30, 16);
pub const MINE_COUNT: usize = 99;

#[derive(Resource)]
pub struct Minefield {
    pub(crate) cells: Vec<Vec<u32>>,
    pub(crate) flags: Vec<Vec<bool>>,
    pub(crate) hidden: Vec<Vec<bool>>,
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

                cells[y][x] = vec![
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

        let flags = vec![vec![false; MINEFIELD_SIZE.0]; MINEFIELD_SIZE.1];
        let hidden = vec![vec![false; MINEFIELD_SIZE.0]; MINEFIELD_SIZE.1];

        Self {
            cells,
            flags,
            hidden,
        }
    }
}
