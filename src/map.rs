#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

// usize uses preffered bit size for given CPU
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn get_map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {

        for y in 0..SCREEN_HEIGHT {

            for x in 0..SCREEN_WIDTH {
                let index = get_map_index(x, y);

                match self.tiles[index] {
                    TileType::Floor => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                    TileType::Wall => {}
                }

            }
        }
    }
}

