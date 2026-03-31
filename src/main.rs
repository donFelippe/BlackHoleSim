use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod vec;
mod ray;
mod black_hole;
mod renderer_2d;

use black_hole::BlackHole;
use vec::Vec2;
use ray::Ray; // Am adăugat Ray aici

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    bh: BlackHole,
    rays: Vec<Ray>,
    first_frame: bool,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Black Hole Sim - Real Time")
                .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
                .with_resizable(false);

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let size = window.inner_size();

            let surface = SurfaceTexture::new(size.width, size.height, window.clone());
            let pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

            self.window = Some(window);
            self.pixels = Some(pixels);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = self.pixels.as_mut() {

                    renderer_2d::render(
                        pixels.frame_mut(),
                        WIDTH,
                        HEIGHT,
                        &self.bh,
                        &mut self.rays,
                        self.first_frame,
                    );
                    self.first_frame = false;

                    if pixels.render().is_err() {
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut rays = Vec::new();
    let num_rays = 150;
    let ray_dir = Vec2::new(1.0, 0.0);
    let world_half = 20.0;

    for i in 0..num_rays {
        let normalized_y = i as f64 / (num_rays - 1) as f64;
        let start_y = (normalized_y - 0.5) * 2.0 * world_half;
        rays.push(Ray::new(Vec2::new(-world_half * 0.95, start_y), ray_dir));
    }

    let mut app = App {
        window: None,
        pixels: None,
        bh: BlackHole::new(5.0, Vec2::new(0.0, 0.0)),
        rays,
        first_frame: true,
    };

    event_loop.run_app(&mut app).unwrap();
}