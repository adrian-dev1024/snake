pub mod game_context;
pub mod game_loop;
pub mod renderer;

use game_loop::run;

const GRID_X_SIZE: i32 = 40;
const GRID_Y_SIZE: i32 = 30;
const DOT_SIZE_IN_PXS: i32 = 20;
const FONT_PATH: &str = "./fonts/SourceCodePro-Regular.ttf";

pub fn main() -> Result<(), String> {
    run()?;
    Ok(())
}
