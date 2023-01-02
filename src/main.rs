#![warn(clippy::all, clippy::pedantic)]

mod map;

// Simplifying module access
mod prelude {
    // publicly using bracket_lib prelude re-exports it inside prelude
    pub use bracket_lib::prelude::*;

    pub use crate::map::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct State {}

impl GameState for State {

    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }

}

fn main() -> BError {

    let context = BTermBuilder::simple80x50()
    .with_title("Rusty Roguelike")
    .build()?;

    main_loop(context, State{})
}

