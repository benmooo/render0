use glam::{IVec2, IVec3, Vec2, Vec3};

use crate::{ndc_to_screen, utils::draw::draw_pixel, RenderContext};

#[allow(unused)]
pub fn draw_triangle(vertices: &[Vec3; 3], tex_coords: &[Vec2; 3], ctx: &mut RenderContext) {
    let (width, _heigh) = (ctx.viewport.width as i32, ctx.viewport.height as i32);

    // conver vertices to ndc coordinates
    let mut scr_vs = [IVec2::default(); 3];
    for (i, &v) in vertices.iter().enumerate() {
        let scr = ndc_to_screen(v, ctx.viewport);
        scr_vs[i] = (scr - 0.5).as_ivec2();
    }

    // axias aligned bounding box AABB
    let bbox = scr_vs.iter().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |acc, IVec2 { x, y }| (acc.0.min(*x), acc.1.min(*y), acc.2.max(*x), acc.3.max(*y)),
    );

    // loop through pixels in the bounding box
    for x in bbox.0..bbox.2 {
        for y in bbox.1..bbox.3 {
            // check if the pixel inside the facet
            let bc_screen = barycentric(&scr_vs, &IVec2::new(x, y));
            if bc_screen.x < 0. || bc_screen.y < 0. || bc_screen.z < 0. {
                continue;
            }

            let mut z = 0.;
            for i in 0..3 {
                z += bc_screen[i] * vertices[i].z;
            }

            // check if the pixel is more close to the camera
            let z_index = (x + y * width) as usize;
            if ctx.zbuf[z_index] > z {
                continue;
            }

            // get the vertex texture color

            let tex_coord = tex_coords
                .iter()
                .enumerate()
                .fold(Vec2::ZERO, |acc, (i, &v)| acc + bc_screen[i] * v);

            let size = ctx.diffuse_texture.size;
            let tex_coord = (
                (tex_coord.x * size.0 as f32) as u32,
                (tex_coord.y * size.1 as f32) as u32,
            );

            let index = (tex_coord.1 * size.0 + tex_coord.0) as usize;
            let color = ctx.diffuse_texture.pixels[index];

            // println!("x: {}, y: {}, index: {}", tex_coord.0, tex_coord.1, index);

            // update zbuffer;
            ctx.zbuf[z_index] = z;
            // iterpolate (x,y) with barycentric coordinates
            draw_pixel(ctx, (x, y), color);
        }
    }
}

fn barycentric(vs: &[IVec2; 3], p: &IVec2) -> Vec3 {
    let u = IVec3::new(vs[2].x - vs[0].x, vs[1].x - vs[0].x, vs[0].x - p.x)
        .cross(IVec3::new(
            vs[2].y - vs[0].y,
            vs[1].y - vs[0].y,
            vs[0].y - p.y,
        ))
        .as_vec3();

    if u.z.abs() < 1. {
        Vec3::new(-1., 1., 1.)
    } else {
        Vec3::new(1. - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
    }
}

#[allow(unused)]
pub fn draw_triangle0(vertices: &[Vec3; 3], ctx: &mut RenderContext, color: u32) {
    // sort the vertices
    let mut scr_v = [IVec2::default(); 3];
    for (i, &v) in vertices.iter().enumerate() {
        let scr = ndc_to_screen(v, ctx.viewport);
        scr_v[i] = (scr - 0.5).as_ivec2();
    }
    scr_v.sort_by(|a, b| a.y.cmp(&b.y));
    let (v0, v1, v2) = (scr_v[0], scr_v[1], scr_v[2]);

    let total_height = v2.y - v0.y;
    if total_height == 0 {
        return;
    }
    let segment_height = v1.y - v0.y + 1;
    for y in v0.y..v1.y {
        let alpha = (y - v0.y) as f32 / total_height as f32;
        let beta = (y - v0.y) as f32 / segment_height as f32;

        let mut a = v0 + ((v2 - v0).as_vec2() * alpha).as_ivec2();
        let mut b = v0 + ((v1 - v0).as_vec2() * beta).as_ivec2();

        if a.x > b.x {
            (a, b) = (b, a)
        }
        for x in a.x..=b.x {
            draw_pixel(ctx, (x, y), color);
        }
    }

    let segment_height = v2.y - v1.y + 1;
    for y in v1.y..v2.y {
        let alpha = (y - v0.y) as f32 / total_height as f32;
        let beta = (y - v1.y) as f32 / segment_height as f32;

        let mut a = v0 + ((v2 - v0).as_vec2() * alpha).as_ivec2();
        let mut b = v1 + ((v2 - v1).as_vec2() * beta).as_ivec2();

        if a.x > b.x {
            (a, b) = (b, a)
        }
        for x in a.x..=b.x {
            draw_pixel(ctx, (x, y), color);
        }
    }
}

#[allow(unused)]
pub fn draw_triangle1(vertices: &[Vec3; 3], ctx: &mut RenderContext, color: u32) {
    // conver vertices to ndc coordinates
    let mut scr_v = [(0, 0); 3];
    for (i, &v) in vertices.iter().enumerate() {
        let v = (ndc_to_screen(v, ctx.viewport) - 0.5).as_ivec2();
        scr_v[i] = (v.x, v.y);
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
