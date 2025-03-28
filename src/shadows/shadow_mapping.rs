use glam::Vec3;

/// Shadow mapping is a technique used in real-time rendering to simulate shadows in a scene.
/// It involves rendering the scene from the light's perspective to create a depth map,
/// which stores the distance from the light to the nearest surface. This depth information
/// is then used during the main rendering pass to determine whether a pixel is in shadow
/// or illuminated.
///
/// The need for shadow mapping arises from the desire to enhance the realism of 3D scenes.
/// Without shadows, objects can appear flat and less believable, as shadows provide depth
/// and context to the visual composition.
///
/// The high-level overview of the shadow mapping algorithm includes the following steps:
/// 1. Render the scene from the light's point of view to create a depth texture.
/// 2. For each pixel in the scene, transform the pixel's position into the light's coordinate space.
/// 3. Compare the transformed depth with the depth value stored in the depth texture.
/// 4. If the pixel's depth is greater than the value in the depth texture, it is in shadow;
///    otherwise, it is illuminated.
pub struct ShadowMapping {
    // The depth texture stores the distance from the light to the nearest surface.
    pub depth_texture: Vec<f32>,
    // The light's position in the scene.
    pub light_position: Vec3,
    // The light's direction in the scene.
    pub light_direction: Vec3,
    // The camera's position in the scene.
    pub camera_position: Vec3,
    // The camera's direction in the scene.
    pub camera_direction: Vec3,
}
