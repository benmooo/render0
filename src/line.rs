/// Draws a line on a buffer using Bresenham's line algorithm.
///
/// The line is drawn from the starting coordinates (x0, y0) to the ending coordinates (x1, y1).
/// The buffer is a mutable slice representing the buffer where the line will be drawn.
/// The size parameter is the size of the buffer as a tuple of (width, height).
/// The color parameter is the RGB color of the line as a tuple of (red, green, blue) values.
pub fn draw_line(
    p0: (u32, u32),
    p1: (u32, u32),
    buffer: &mut [u32],
    viewport: (u32, u32),
    color: (u32, u32, u32),
) {
    let (mut x0, mut y0) = p0;
    let (mut x1, mut y1) = p1;

    let steep = y1.abs_diff(y0) > x1.abs_diff(x0);
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 as i32 - x0 as i32;
    let dy = (y1 as i32 - y0 as i32).abs();
    let mut error = dx / 2;
    let y_step = if y0 < y1 { 1 } else { -1 };
    let mut y = y0 as i32;

    let color = (color.0 << 16) | (color.1 << 8) | color.2;

    for x in x0..=x1 {
        let index = if steep {
            rh_coord_to_buffer_index((y as u32, x), viewport)
        } else {
            rh_coord_to_buffer_index((x, y as u32), viewport)
        };
        buffer[index as usize] = color;

        error -= dy;
        if error < 0 {
            y += y_step;
            error += dx;
        }
    }
}

// screen space right handed coordinate to viewport buffer index
fn rh_coord_to_buffer_index(coord: (u32, u32), viewport: (u32, u32)) -> u32 {
    (viewport.1 - coord.1 - 1) * viewport.0 + coord.0
}
