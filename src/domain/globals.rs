pub const WORLD_WIDTH: i32 = 80;
pub const WORLD_HEIGHT: i32 = 50;
pub const DISPLAY_WIDTH: i32 = WORLD_WIDTH / 2;
pub const DISPLAY_HEIGHT: i32 = WORLD_HEIGHT / 2;
pub const NUM_ROOMS: usize = 20;

#[derive(Copy, Clone, PartialEq)]
pub struct RenderInfo {
    pub x: i32,
    pub y: i32,
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
    pub glyph: u16,
}
