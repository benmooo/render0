mod camera;
mod color;
mod draw;
mod model;
mod triangle;

use camera::Camera;
use draw::draw_line;
use glam::{Mat4, Vec2, Vec3, Vec4Swizzles};
use model::{load_model, load_texture};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;
use std::num::NonZeroU32;
use std::rc::Rc;
use tobj::Model;
use triangle::draw_triangle;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() -> anyhow::Result<()> {
    // setup window
    let el = EventLoop::new()?;
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("render0")
            .with_inner_size(PhysicalSize::new(1200, 1200))
            .build(&el)?,
    );
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    // model
    let models = load_model();

    let texture = load_texture();
    let (w, h) = (texture.size().width, texture.size().height);

    let mut ps = vec![];
    ps.resize_with((w * h) as usize, || 0);
    for p in texture.pixels() {
        let index = ((h as i32 - p.position.y - 1) * w as i32 + p.position.x) as usize;
        ps[index] = p.color;
    }

    // Get some information from the header

    let diffuse_texture = DiffuseTexture {
        pixels: &ps,
        size: (w, h),
    };

    el.run(|event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::RedrawRequested,
            } => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let viewport = Viewport::new(width, height, (0, 0));

                let mut buffer = surface.buffer_mut().unwrap();
                let mut zbuf = vec![f32::MIN; (width * height) as usize];

                let mut ctx =
                    RenderContext::new(&mut buffer, viewport, &mut zbuf, &diffuse_texture);

                // let color = (230, 100, 180);
                // let color = (color.0 << 16) | (color.1 << 8) | color.2;
                // draw_line(
                //     (0, 0),
                //     (width as i32 - 1, height as i32 - 1),
                //     &mut ctx,
                //     color,
                // );
                render_triangles(&models, &mut ctx);
                // render_wireframe(&models, &mut ctx);
                buffer.present().unwrap();
            }
            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::Resized(ps),
            } => {
                println!("resize: {:?}", ps);
                surface
                    .resize(
                        NonZeroU32::new(ps.width).unwrap(),
                        NonZeroU32::new(ps.height).unwrap(),
                    )
                    .unwrap()
            }
            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::CloseRequested,
            } => {
                elwt.exit();
            }
            _ => {}
        }
    })?;
    Ok(())
}

/// Wireframe renderer
/// Draws lines between the vertices of the faces of the models on the buffer.
///
/// # Arguments
/// * `models` - A vector of models, where each model contains a mesh with vertex positions and face indices.
/// * `buffer` - A mutable slice of u32 values representing the buffer where the lines will be drawn.
/// * `viewport` - The screen viewport
#[allow(unused)]
fn render_wireframe(models: &Vec<Model>, ctx: &mut RenderContext) {
    for model in models {
        let mesh = &model.mesh;

        // Loop through the faces by indices
        for f in mesh.indices.windows(3).step_by(3) {
            // Access vertices of the face using face_indices
            let mut vertices = [Vec2::default(); 3];
            for i in 0..3 {
                let indices = &mesh.positions[(f[i] as usize * 3)..(f[i] as usize * 3 + 3)];
                let v = Vec3::from_slice(indices);
                vertices[i] = ndc_to_screen(v, ctx.viewport);
            }

            let color = (250, 240, 210);
            let color = (color.0 << 16) | (color.1 << 8) | color.2;

            for (i, v0) in vertices.iter().enumerate() {
                let v1 = vertices[(i + 1) % 3];
                draw_line(*v0, v1, ctx, color);
            }
        }
    }
}

#[allow(unused)]
fn render_triangles(models: &Vec<Model>, ctx: &mut RenderContext) {
    let light_dir = -Vec3::Z;

    // world space model transform
    let scale = Mat4::from_scale(Vec3::new(0.8, 0.8, 0.8));
    let rotation = Mat4::from_rotation_y(PI / 6.);
    let translation = Mat4::from_translation(Vec3::new(0.1, 0.1, 0.));
    let model_transform = translation * rotation * scale;

    // view transform
    let camera = Camera::new(
        Vec3::new(0., 5., 10.),
        Default::default(),
        Vec3::Y,
        PI / 3.,
        4. / 3.,
        0.1,
        1000.,
    );
    let view_transform = camera.get_view_matrix();

    // projection transform
    let project_transform = camera.get_projection_matrix();

    for model in models {
        let mesh = &model.mesh;

        // Loop through the facets

        for i in 0..mesh.indices.len() / 3 {
            let mut vertices = [Vec3::default(); 3];
            let mut tex_coords = [Vec2::default(); 3];

            let v_indices = &mesh.indices[i * 3..(i + 1) * 3];
            let t_indices = &mesh.texcoord_indices[i * 3..(i + 1) * 3];

            for j in 0..3 {
                let v_index = v_indices[j] as usize;
                let t_index = t_indices[j] as usize;

                let v = &mesh.positions[v_index * 3..(v_index + 1) * 3];
                let t = &mesh.texcoords[t_index * 2..(t_index + 1) * 2];
                vertices[j] = Vec3::from_slice(v);
                tex_coords[j] = Vec2::from_slice(t);
            }

            // model transform
            for i in 0..3 {
                vertices[i] = (model_transform * vertices[i].extend(1.)).truncate();
            }

            // view transform
            for i in 0..3 {
                vertices[i] = (view_transform * vertices[i].extend(1.)).truncate();
            }

            // projection transform
            for i in 0..3 {
                vertices[i] = (project_transform * vertices[i].extend(1.)).truncate();
            }

            let n = (vertices[2] - vertices[0]).cross(vertices[1] - vertices[0]);
            let intensity = light_dir.dot(n.normalize());

            if intensity < 0. {
                continue;
            }

            // let volumn = (intensity * 255.).floor() as u32;
            // let color = (volumn, volumn, volumn);
            // let color = (color.0 << 16) | (color.1 << 8) | color.2;

            // draw_triangle(&vertices, &tex_coords, ctx, color);
            draw_triangle(&vertices, &tex_coords, ctx);
        }
    }
}

// Convert vertices from NDC to screen coordinates
#[allow(unused)]
fn to_screen_coords(p: (f32, f32), viewport: (u32, u32)) -> (u32, u32) {
    (
        ((p.0 + 1.0) * (viewport.0 - 1) as f32 / 2.0) as u32,
        ((p.1 + 1.0) * (viewport.1 - 1) as f32 / 2.0) as u32,
    )
}

#[allow(unused)]
fn generate_random_color() -> (u32, u32, u32) {
    let mut rng = thread_rng();
    (
        rng.gen_range(0..256),
        rng.gen_range(0..256),
        rng.gen_range(0..256),
    )
}

struct RenderContext<'a> {
    buffer: &'a mut [u32],
    viewport: Viewport,
    zbuf: &'a mut [f32],
    diffuse_texture: &'a DiffuseTexture<'a>,
}

struct DiffuseTexture<'a> {
    pixels: &'a Vec<u32>,
    size: (u32, u32),
}

impl<'a> RenderContext<'a> {
    pub fn new(
        buffer: &'a mut [u32],
        viewport: Viewport,
        zbuf: &'a mut [f32],
        diffuse_texture: &'a DiffuseTexture,
    ) -> Self {
        Self {
            buffer,
            viewport,
            zbuf,
            diffuse_texture,
        }
    }
}

#[derive(Clone, Copy)]
struct Viewport {
    width: u32,
    height: u32,
    center: (i32, i32),
}
impl Viewport {
    pub fn new(width: u32, height: u32, center: (i32, i32)) -> Self {
        Self {
            width,
            height,
            center,
        }
    }
}

fn ndc_to_screen(ndc: Vec3, v: Viewport) -> Vec2 {
    let scale = Mat4::from_scale(Vec3::new(v.width as f32, v.height as f32, 1.));
    let coord = (ndc + 1.) / 2.;
    (scale * coord.extend(1.)).xy()
}

#[allow(unused)]
fn screen_to_ndc(p: (i32, i32), viewport: Viewport) -> Vec3 {
    let (x, y) = p;
    let x = x as f32 / (viewport.width as f32 - 1.) * 2. - 1.;
    let y = y as f32 / (viewport.height as f32 - 1.) * 2. - 1.;

    Vec3::from_array([x, y, 1.])
}

// for f in mesh.indices.windows(3).step_by(3) {
//     let mut vertices = [Vec3::default(); 3];
//     let mut tex_coords = [Vec2::default(); 3];

//     // Access vertices of the face using face_indices
//     for (i, v) in f
//         .iter()
//         .map(|&v| &mesh.positions[v as usize * 3..(v + 1) as usize * 3])
//         .enumerate()
//     {
//         vertices[i] = Vec3::from_slice(v);
//     }

//     for (i, v) in f
//         .iter()
//         .map(|&v| &mesh.texcoords[v as usize * 2..(v + 1) as usize * 2])
//         .enumerate()
//     {
//         tex_coords[i] = Vec2::from_slice(v);
//     }

//     let n = (vertices[2] - vertices[0]).cross(vertices[1] - vertices[0]);
//     let intensity = light_dir.dot(n.normalize());

//     if intensity < 0. {
//         continue;
//     }

//     let volumn = (intensity * 255.).floor() as u32;
//     let color = (volumn, volumn, volumn);
//     let color = (color.0 << 16) | (color.1 << 8) | color.2;

//     draw_triangle(&vertices, &tex_coords, ctx, color);
//     // draw_triangle(&vertices, &tex_coords, ctx);
// }
