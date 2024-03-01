#[allow(unused)]
pub fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> (f32, f32, f32) {
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let hue_prime = hue / 60.0;
    let x = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());

    let (r, g, b) = match hue_prime.floor() as i32 {
        0 => (chroma, x, 0.0),
        1 => (x, chroma, 0.0),
        2 => (0.0, chroma, x),
        3 => (0.0, x, chroma),
        4 => (x, 0.0, chroma),
        5 => (chroma, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    let m = lightness - chroma / 2.0;
    (r + m, g + m, b + m)
}
