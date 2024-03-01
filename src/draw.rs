use crate::RenderContext;

/// Draws a line on a buffer using Bresenham's line algorithm.
///
/// The line is drawn from the starting coordinates (x0, y0) to the ending coordinates (x1, y1).
/// The buffer is a mutable slice representing the buffer where the line will be drawn.
/// The size parameter is the size of the buffer as a tuple of (width, height).
/// The color parameter is the RGB color of the line as a tuple of (red, green, blue) values.
pub fn draw_line(p0: (i32, i32), p1: (i32, i32), ctx: &mut RenderContext, color: u32) {
    let steep = (p1.1 - p0.1).abs() > (p1.0 - p0.0).abs();
    if steep {
        draw_steep_line(p0, p1, ctx, color);
    } else {
        draw_flat_line(p0, p1, ctx, color);
    }
}

fn draw_flat_line(mut p0: (i32, i32), mut p1: (i32, i32), ctx: &mut RenderContext, color: u32) {
    if p0.0 > p1.0 {
        (p0, p1) = (p1, p0);
    }

    let dx = p1.0 - p0.0;
    let mut dy = p1.1 - p0.1;

    let mut y_step = 1;
    if dy < 0 {
        y_step = -1;
        dy = -dy;
    }

    let incr_e = 2 * dy;
    let incr_ne = 2 * (dy - dx);

    let mut d = 2 * dy - dx;
    let mut y = p0.1;

    for x in p0.0..=p1.0 {
        let index = (ctx.viewport.height - y as u32 - 1) * ctx.viewport.width + x as u32;
        ctx.buffer[index as usize] = color;
        if d < 0 {
            d += incr_e;
        } else {
            y += y_step;
            d += incr_ne;
        }
    }
}

fn draw_steep_line(mut p0: (i32, i32), mut p1: (i32, i32), ctx: &mut RenderContext, color: u32) {
    if p0.1 > p1.1 {
        (p0, p1) = (p1, p0);
    }

    let mut dx = p1.0 - p0.0;
    let dy = p1.1 - p0.1;

    let mut x_step = 1;
    if dx < 0 {
        x_step = -1;
        dx = -dx;
    }

    let incr_e = 2 * dx;
    let incr_ne = 2 * (dx - dy);

    let mut d = 2 * dx - dy;
    let mut x = p0.0;

    for y in p0.1..=p1.1 {
        let index = (ctx.viewport.height - y as u32 - 1) * ctx.viewport.width + x as u32;
        ctx.buffer[index as usize] = color;
        if d < 0 {
            d += incr_e;
        } else {
            x += x_step;
            d += incr_ne;
        }
    }
}

#[allow(unused)]
pub fn draw_pixel(ctx: &mut RenderContext, p: (i32, i32), color: u32) {
    let width = ctx.viewport.width;
    let height = ctx.viewport.height;
    let index = (height as i32 - p.1 - 1) * width as i32 + p.0;
    ctx.buffer[index as usize] = color;
}
