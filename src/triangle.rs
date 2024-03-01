use glam::{Vec3, Vec3Swizzles};

use crate::{draw::draw_pixel, ndc_to_screen, screen_to_ndc, RenderContext};

pub fn draw_triangle(vertices: &[Vec3; 3], ctx: &mut RenderContext, color: u32) {
    // conver vertices to ndc coordinates
    let mut scr_v = [(0, 0); 3];
    for (i, v) in vertices.iter().enumerate() {
        scr_v[i] = ndc_to_screen((v.x, v.y), ctx.viewport);
    }

    // bounding box
    let bbox = scr_v
        .iter()
        .fold((i32::MAX, i32::MAX, i32::MIN, i32::MIN), |acc, (x, y)| {
            (acc.0.min(*x), acc.1.min(*y), acc.2.max(*x), acc.3.max(*y))
        });

    // loop through pixels in the bounding box
    for x in bbox.0..bbox.2 {
        for y in bbox.1..bbox.3 {
            // check if the pixel inside the facet
            if !inside(&scr_v, (x, y)) {
                continue;
            };

            // check if the pixel is more close to the camera
            // let i = y * ctx.viewport.0 + x;
            // if ctx.zbuf[i] >

            draw_pixel(ctx, (x, y), color);
        }
    }
}

fn inside(vertices: &[(i32, i32); 3], p: (i32, i32)) -> bool {
    let ds: Vec<i32> = vertices
        .into_iter()
        .enumerate()
        .map(|(i, &v0)| {
            let v1 = vertices[(i + 1) % 3];
            let a = (v1.0 - v0.0, v1.1 - v0.1);
            let b = (p.0 - v0.0, p.1 - v0.1);

            a.0 * b.1 - a.1 * b.0
        })
        .collect();

    ds.iter().all(|&v| v >= 0) || ds.iter().all(|&v| v <= 0)
}
