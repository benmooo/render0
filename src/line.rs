/// Draws a line on a buffer using Bresenham's line algorithm.
///
/// The line is drawn from the starting coordinates (x0, y0) to the ending coordinates (x1, y1).
/// The buffer is a mutable slice representing the buffer where the line will be drawn.
/// The size parameter is the size of the buffer as a tuple of (width, height).
/// The color parameter is the RGB color of the line as a tuple of (red, green, blue) values.
///
/// # Examples
///
/// ```
/// let mut buffer = [0; 100];
/// let size = (10, 10);
/// let color = (255, 0, 0);
/// line(0, 0, 9, 9, &mut buffer, size, color);
/// ```
#[allow(unused)]
pub fn draw_line0(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    buffer: &mut [u32],
    size: (u32, u32),
    color: (u32, u32, u32),
) {
    let steep = y1.abs_diff(y0) > x1.abs_diff(x0);
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let incr_e = 2 * dy;
    let incr_ne = 2 * (dy - dx);
    let mut d = 2 * dy - dx;
    let mut y = y0;
    let y_step = if y0 < y1 { 1 } else { -1 };

    for x in x0..=x1 {
        if steep {
            buffer[(x as u32 * size.0 + y as u32) as usize] =
                (color.0 << 16) | (color.1 << 8) | color.2;
        } else {
            buffer[(y as u32 * size.0 + x as u32) as usize] =
                (color.0 << 16) | (color.1 << 8) | color.2;
        }

        if d < 0 {
            d += incr_e;
        } else {
            d += incr_ne;
            y += y_step;
        }
    }
}

#[allow(unused)]
pub fn draw_line(
    mut x0: u32,
    mut y0: u32,
    mut x1: u32,
    mut y1: u32,
    buffer: &mut [u32],
    size: (u32, u32),
    color: (u32, u32, u32),
) {
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
        if steep {
            buffer[(x * size.0 + y as u32) as usize] = color;
        } else {
            buffer[(y as u32 * size.0 + x) as usize] = color;
        }

        error -= dy;
        if error < 0 {
            y += y_step;
            error += dx;
        }
    }
}
