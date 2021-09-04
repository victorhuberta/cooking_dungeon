pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const MAP_TILES_N: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

pub type RenderInfo = (i32, i32, (u8, u8, u8), (u8, u8, u8), u16);