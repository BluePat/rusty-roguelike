#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

// usize uses preffered bit size for given CPU
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
