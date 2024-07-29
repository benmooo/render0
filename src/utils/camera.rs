use glam::{Mat4, Vec3};

// Simple camera struct
pub struct Camera {
    position: Vec3,
    target: Vec3, // Point the camera looks at (usually slightly in front of the actual direction)
    up: Vec3,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl Camera {
    // Create a new camera with specified parameters
    pub fn new(
        position: Vec3,
        target: Vec3,
        up: Vec3,
        fov: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Camera {
            position,
            target,
            up,
            fov,
            aspect_ratio,
            near,
            far,
        }
    }

    // Get the view matrix (world to camera space)
    pub fn get_view_matrix(&self) -> Mat4 {
        let eye = self.position; // Camera position
        let target = self.target; // Target position
        let up = self.up; // Up vector

        Mat4::look_at_rh(eye, target, up)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }

    // Get the projection matrix (camera space to clip space)
    // fn get_projection_matrix(&self) -> Mat4 {
    //     let top = self.near * f32::tan(self.fov * 0.5);
    //     let bottom = -top;
    //     let left = bottom * self.aspect_ratio;
    //     let right = left * -self.aspect_ratio;

    //     let projection_matrix = Mat4::from_cols(
    //         2.0 * self.near / (right - left),
    //         0.0,
    //         (right + left) / (right - left),
    //         0.0,
    //         0.0,
    //         2.0 * self.near / (top - bottom),
    //         (top + bottom) / (top - bottom),
    //         0.0,
    //         0.0,
    //         0.0,
    //         -(self.far + self.near) / (self.far - self.near),
    //         -1.0,
    //     );

    //     projection_matrix
    // }
}

// fn main() {
//     // Example usage
//     let camera = Camera::new(
//         Vec3::new(0.0, 2.0, 5.0), // Camera position
//         Vec3::new(0.0, 2.0, 0.0), // Target point (slightly in front) -z
//         Vec3::new(0.0, 1.0, 0.0), // Up vector (default)
//         f32::to_radians(60.0),    // Field of view
//         16.0 / 9.0,               // Aspect ratio (16:9)
//         0.1,                      // Near clip plane
//         1000.0,                   // Far clip plane
//     );

//     let camera_transform = camera.get_camera_transform();
//     let view_matrix = camera.get_view_matrix();
//     let projection_matrix = camera.get_projection_matrix();

//     // Do something with the matrices (e.g., pass them to a graphics library)

//     println!("Camera Transform:\n{:?}", camera_transform);
//     println!("View Matrix:\n{:?}", view_matrix);
//     println!("Projection Matrix:\n{:?}", projection_matrix);
// }
