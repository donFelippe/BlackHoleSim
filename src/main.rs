use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{WindowEvent, ElementState, MouseButton},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod vec;
mod ray;
mod black_hole;
mod renderer_2d; // Inca il lasam asa ca nume, dar acum face 3D Raytracing!

use black_hole::BlackHole;
use vec::Vec3;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

const WINDOW_DISPLAY_SIZE_X: f64 = 1020.0;
const WINDOW_DISPLAY_SIZE_Y: f64=  800.0;
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    bh: BlackHole,

    camera_yaw: f64,
    camera_pitch: f64,
    mouse_held: bool,
    last_mouse_pos: Option<(f64, f64)>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Interstellar - Performance Mode")
                // Aici folosim dimensiunea mare pentru fereastra
                .with_inner_size(LogicalSize::new(WINDOW_DISPLAY_SIZE_X, WINDOW_DISPLAY_SIZE_Y))
                .with_resizable(false);

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let size = window.inner_size();

            let surface = SurfaceTexture::new(size.width, size.height, window.clone());
            // Dar aici buffer-ul de pixeli ramane mic (400x400)
            let pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

            window.request_redraw();
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
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left {
                    self.mouse_held = state == ElementState::Pressed;
                    if !self.mouse_held {
                        self.last_mouse_pos = None;
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.mouse_held {
                    if let Some((last_x, last_y)) = self.last_mouse_pos {
                        let dx = position.x - last_x;
                        let dy = position.y - last_y;
                        self.camera_yaw -= dx * 0.005; // Sensibilitate ajustata
                        self.camera_pitch -= dy * 0.005;

                        // Cand miscam mouse-ul, cerem un cadru nou!
                        if let Some(window) = self.window.as_ref() {
                            window.request_redraw();
                        }
                    }
                    self.last_mouse_pos = Some((position.x, position.y));
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = self.pixels.as_mut() {
                    renderer_2d::render(
                        pixels.frame_mut(),
                        WIDTH,
                        HEIGHT,
                        &self.bh,
                        self.camera_yaw,
                        self.camera_pitch,
                    );

                    if pixels.render().is_err() {
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait); // Procesorul se odihneste cand nu misti mouse-ul!

    let mut app = App {
        window: None,
        pixels: None,
        bh: BlackHole::new(5.0, Vec3::new(0.0, 0.0, 0.0)),
        camera_yaw: 0.0,
        camera_pitch: 0.1, // Ne uitam putin de sus
        mouse_held: false,
        last_mouse_pos: None,
    };

    event_loop.run_app(&mut app).unwrap();
}