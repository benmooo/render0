mod line;
mod model;

use line::draw_line;
use model::load_model;
use std::num::NonZeroU32;
use std::rc::Rc;
use tobj::Model;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() -> anyhow::Result<()> {
    // setup window
    let el = EventLoop::new()?;
    let window = Rc::new(WindowBuilder::new().build(&el)?);
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
                // draw_line(
                //     0,
                //     0,
                //     width - 1,
                //     height - 1,
                //     &mut buffer,
                //     (width, height),
                //     (255, 0, 0),
                // );
                draw_model_lines(&models, &mut buffer, width, height);
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

/// Draws lines between the vertices of the faces of the models on the buffer.
///
/// # Arguments
///
/// * `models` - A vector of models, where each model contains a mesh with vertex positions and face indices.
/// * `buffer` - A mutable slice of u32 values representing the buffer where the lines will be drawn.
/// * `width` - The width of the buffer.
/// * `height` - The height of the buffer.
fn draw_model_lines(models: &Vec<Model>, buffer: &mut [u32], width: u32, height: u32) {
    for model in models {
        let mesh = &model.mesh;

        // Loop through the faces by indices
        for f in 0..mesh.indices.len() / 3 {
            let face_indices = &mesh.indices[3 * f..(3 * f + 3)];
            // Access vertices of the face using face_indices
            let v1 =
                &mesh.positions[(face_indices[0] as usize * 3)..(face_indices[0] as usize * 3 + 3)];
            let v2 =
                &mesh.positions[(face_indices[1] as usize * 3)..(face_indices[1] as usize * 3 + 3)];
            let v3 =
                &mesh.positions[(face_indices[2] as usize * 3)..(face_indices[2] as usize * 3 + 3)];

            // Convert vertices from NDC to screen coordinates
            let to_screen_coords = |v: (f32, f32)| {
                (
                    ((v.0 + 1.0) * (width - 1) as f32 / 2.0) as u32,
                    ((v.1 + 1.0) * (height - 1) as f32 / 2.0) as u32,
                )
            };
            let v1_screen = to_screen_coords((v1[0], v1[1]));
            let v2_screen = to_screen_coords((v2[0], v2[1]));
            let v3_screen = to_screen_coords((v3[0], v3[1]));

            let vertices = vec![v1_screen, v2_screen, v3_screen];
            let face_edges: Vec<(&(u32, u32), &(u32, u32))> = vertices
                .iter()
                .zip(vertices.iter().cycle().skip(1))
                .collect();
            let color = (200, 150, 150);

            for edge in face_edges {
                draw_line(
                    edge.0 .0,
                    edge.0 .1,
                    edge.1 .0,
                    edge.1 .1,
                    buffer,
                    (width, height),
                    color,
                );
            }
        }
    }
}
