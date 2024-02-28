mod draw;
mod model;
mod triangle;

use draw::draw_line;
use glam::Vec3;
use model::load_model;
use rand::{thread_rng, Rng};
use std::num::NonZeroU32;
use std::rc::Rc;
use tobj::Model;
use triangle::draw_triangle;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() -> anyhow::Result<()> {
    // setup window
    let el = EventLoop::new()?;
    let window = Rc::new(WindowBuilder::new().with_title("render0").build(&el)?);
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    // model
    let models = load_model();

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

                let mut buffer = surface.buffer_mut().unwrap();
                let mut ctx = RenderContext::new(&mut buffer, (width, height));
                // draw_triangle(
                //     [(0, 0), (width / 2, height / 2), (width - 1, 0)],
                //     &mut ctx,
                //     (230, 100, 180),
                // );

                // render_wireframe(&models, &mut ctx);
                render_triangles(&models, &mut ctx);
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
    let viewport = ctx.viewport;
    for model in models {
        let mesh = &model.mesh;

        // Loop through the faces by indices
        for f in mesh.indices.windows(3).step_by(3) {
            // Access vertices of the face using face_indices
            let v1 = &mesh.positions[(f[0] as usize * 3)..(f[0] as usize * 3 + 3)];
            let v2 = &mesh.positions[(f[1] as usize * 3)..(f[1] as usize * 3 + 3)];
            let v3 = &mesh.positions[(f[2] as usize * 3)..(f[2] as usize * 3 + 3)];

            let v1_screen = to_screen_coords((v1[0], v1[1]), viewport);
            let v2_screen = to_screen_coords((v2[0], v2[1]), viewport);
            let v3_screen = to_screen_coords((v3[0], v3[1]), viewport);

            let vertices = [v1_screen, v2_screen, v3_screen];
            let color = (220, 100, 120);

            for (i, v0) in vertices.iter().enumerate() {
                let v1 = vertices[(i + 1) % 3];
                draw_line(*v0, v1, ctx.buffer, viewport, color);
            }
        }
    }
}

fn render_triangles(models: &Vec<Model>, ctx: &mut RenderContext) {
    let viewport = ctx.viewport;

    let light_dir = -Vec3::Z;

    for model in models {
        let mesh = &model.mesh;

        // Loop through the faces by indices
        for f in mesh.indices.windows(3).step_by(3) {
            let v1 = &mesh.positions[(f[0] as usize * 3)..(f[0] as usize * 3 + 3)];
            let v2 = &mesh.positions[(f[1] as usize * 3)..(f[1] as usize * 3 + 3)];
            let v3 = &mesh.positions[(f[2] as usize * 3)..(f[2] as usize * 3 + 3)];

            let v1_screen = to_screen_coords((v1[0], v1[1]), viewport);
            let v2_screen = to_screen_coords((v2[0], v2[1]), viewport);
            let v3_screen = to_screen_coords((v3[0], v3[1]), viewport);
            // Access vertices of the face using face_indices
            let vertices = [v1_screen, v2_screen, v3_screen];

            let n = (Vec3::from_slice(v3) - Vec3::from_slice(v1))
                .cross(Vec3::from_slice(v2) - Vec3::from_slice(v1));
            let intensity = light_dir.dot(n.normalize());

            if intensity > 0. {
                let volumn = (intensity * 255.).floor() as u32;
                let color = (volumn, volumn, volumn);

                draw_triangle(&vertices, ctx, color)
            }
        }
    }
}

// Convert vertices from NDC to screen coordinates
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
    viewport: (u32, u32),
}

impl<'a> RenderContext<'a> {
    pub fn new(buffer: &'a mut [u32], viewport: (u32, u32)) -> Self {
        Self { buffer, viewport }
    }
}
