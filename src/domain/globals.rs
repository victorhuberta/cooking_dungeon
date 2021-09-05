pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const NUM_ROOMS: usize = 20;

#[derive(Copy, Clone, PartialEq)]
pub struct RenderInfo {
    pub x: i32,
    pub y: i32,
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
    pub glyph: u16,
}
