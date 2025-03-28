mod enviornment_mapping;
pub mod shadow_mapping;

pub fn pcf() {
    // PCF (Percentage Closer Filtering) is a technique used to determine shadow intensity by averaging
    // the depth values of multiple samples. The basic equation for PCF can be represented as:
    //
    // S(x, y) = 1 / N * ∑ I(z_i)
    //
    // where:
    // - S(x, y) is the shadow intensity at pixel (x, y),
    // - N is the number of samples taken,
    // - I(z_i) is an indicator function that returns 1 if the sample at depth z_i is in shadow,
    //   and 0 otherwise.
    //
    // This results in a smoother transition of shadows by blending the results of multiple samples,
    // reducing the harshness of shadow edges.
    todo!()
}

pub fn pcss() {
    // PCSS (Percentage Closer Soft Shadows) enhances PCF by incorporating the concept of penumbra
    // to simulate soft shadows. The equation for PCSS can be expressed as:
    //
    // S(x, y) = 1 / N * ∑ I(z_i) * P(d_i)
    //
    // where:
    // - S(x, y) is the shadow intensity at pixel (x, y),
    // - N is the number of samples taken,
    // - I(z_i) is the same indicator function as in PCF,
    // - P(d_i) is a function that represents the penumbra effect based on the distance d_i
    //   from the occluder, which gradually transitions the shadow from dark to light.
    //
    // This method allows for a more realistic representation of shadows, particularly in scenes
    // with large light sources, as it calculates the percentage of shadowed samples within a
    // defined area and creates a soft transition between light and shadow.
    unimplemented!()
}

// Variance soft shadow mapping

// MIPMAP and summed-area variance shadow maps

// moment shadow mapping

// distance field soft shadows

// Variance soft shadow mapping

// MIPMAP and summed-area variance shadow maps

// moment shadow mapping

// distance field soft shadows
