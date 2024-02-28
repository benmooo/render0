use crate::{draw::draw_pixel, RenderContext};

pub fn draw_triangle(vertices: &[(u32, u32)], ctx: &mut RenderContext, color: (u32, u32, u32)) {
    let vx = vertices.iter().map(|v| v.0);
    let vy = vertices.iter().map(|v| v.1);

    let bbox = [
        (vx.to_owned().min().unwrap(), vy.to_owned().min().unwrap()),
        (vx.max().unwrap(), vy.max().unwrap()),
    ];

    for x in bbox[0].0..bbox[1].0 {
        for y in bbox[0].1..bbox[1].1 {
            if inside(vertices, (x, y)) {
                draw_pixel(ctx, (x, y), color);
            }
        }
    }

    // for (i, v0) in vertices.iter().enumerate() {
    //     let v1 = vertices[(i + 1) % 3];
    //     draw_line(*v0, v1, &mut ctx.buffer, ctx.viewport, color);
    // }
}

fn inside(vertices: &[(u32, u32)], p: (u32, u32)) -> bool {
    let ds: Vec<i32> = vertices
        .into_iter()
        .enumerate()
        .map(|(i, v0)| {
            let v1 = vertices[(i + 1) % 3];
            let a = (v1.0 as i32 - v0.0 as i32, v1.1 as i32 - v0.1 as i32);
            let b = (p.0 as i32 - v0.0 as i32, p.1 as i32 - v0.1 as i32);

            a.0 * b.1 - a.1 * b.0
        })
        .collect();

    ds.iter().all(|&v| v >= 0) || ds.iter().all(|&v| v <= 0)
}
