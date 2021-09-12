use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top()..=camera.bottom() {
        for x in camera.left()..=camera.right() {
            let tile_pos = Point::new(x, y);
            let camera_offset = Point::new(camera.left(), camera.top());

            if let Some(tile) = map.tile(tile_pos) {
                let glyph = match tile {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(
                    tile_pos - camera_offset,
                    ColorPair::new(WHITE, BLACK),
                    glyph,
                );
            }
        }
    }

    draw_batch.submit(0).expect("Draw batch error");
}
