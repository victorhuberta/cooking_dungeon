use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(world: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    <(&Point, &Render)>::query()
        .iter(world)
        .for_each(|(pos, render)| {
            let camera_offset = Point::new(camera.left(), camera.top());
            draw_batch.set(*pos - camera_offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Draw batch error");
}
