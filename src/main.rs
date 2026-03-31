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
mod renderer_2d;

use black_hole::BlackHole;
use vec::Vec3;
use ray::Ray;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    bh: BlackHole,
    // whole trajectory + color
    paths: Vec<(Vec<Vec3>, [u8; 3])>,
    time_step: usize,

    camera_yaw: f64,
    camera_pitch: f64,
    mouse_held: bool,
    last_mouse_pos: Option<(f64, f64)>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Black Hole 3D - Continuous Engine")
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
                        self.camera_yaw -= dx * 0.01;
                        self.camera_pitch -= dy * 0.01;
                    }
                    self.last_mouse_pos = Some((position.x, position.y));
                }
            }
            WindowEvent::RedrawRequested => {
                // 15 steps redraw
                self.time_step += 15;

                if let Some(pixels) = self.pixels.as_mut() {
                    renderer_2d::render(
                        pixels.frame_mut(),
                        WIDTH,
                        HEIGHT,
                        &self.bh,
                        &self.paths,
                        self.time_step,
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

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Cerem sistemului sa randeze non-stop cat de repede poate
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}
fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let bh = BlackHole::new(5.0, Vec3::new(0.0, 0.0, 0.0));
    let mut paths = Vec::new();

    // Am crescut enorm numarul de raze pentru detaliu fin,
    // deoarece majoritatea vor fi filtrate si sterse!
    let num_rays_y = 100;
    let num_rays_z = 100;
    let initial_dir = Vec3::new(1.0, 0.0, 0.0);
    let world_half = 20.0;

    println!("Calculez si filtrez spatiul-timp... te rog asteapta putin.");

    for i in 0..num_rays_y {
        for j in 0..num_rays_z {
            let normalized_y = i as f64 / (num_rays_y - 1) as f64;
            let normalized_z = j as f64 / (num_rays_z - 1) as f64;
            let start_y = (normalized_y - 0.5) * 2.0 * world_half;
            let start_z = (normalized_z - 0.5) * 2.0 * world_half;

            let mut ray = Ray::new(Vec3::new(-world_half * 1.5, start_y, start_z), initial_dir);
            let mut path = Vec::with_capacity(2000);
            let mut captured = false;

            // Calculam fizica
            for _ in 0..2000 {
                path.push(ray.pos);
                let r = (ray.pos - bh.position).length();
                if r <= bh.schwarzschild_radius {
                    captured = true;
                    break;
                }
                if r > 50.0 { break; }
                ray = ray::rk4_step(&ray, &bh, 0.05);
            }

            // FILTRUL: Calculam cata deviatie a suferit raza (Produs scalar intre directia initiala si finala)
            // 0.0 inseamna linie dreapta perfecta, > 0.0 inseamna curbata
            let final_vel = ray.vel.normalise();
            let deflection = 1.0 - initial_dir.dot(final_vel);

            if captured {
                // Razele capturate (care pica in gaura neagra) -> Rosu aprins
                paths.push((path, [255, 40, 40]));
            } else if deflection > 0.01 {
                // Razele care ocolesc dar sunt curbate masiv (Haloul) -> Nuante de Galben/Portocaliu
                let intensity = (deflection * 5.0).clamp(0.0, 1.0); // 0.0 = putin curbat, 1.0 = super curbat
                let r_color = 255;
                let g_color = (150.0 + 105.0 * (1.0 - intensity)) as u8;
                let b_color = (50.0 + 205.0 * (1.0 - intensity)) as u8;

                paths.push((path, [r_color, g_color, b_color]));
            }
            // Cele cu deflection <= 0.01 sunt plictisitoare (linii drepte).
            // Nu le adaugam in vector deloc! Asa eliberam centrul imaginii.
        }
    }

    println!("Fizica gata! Au supravietuit {} raze interesante.", paths.len());

    let mut app = App {
        window: None,
        pixels: None,
        bh,
        paths,
        time_step: 0,
        camera_yaw: 0.0,
        camera_pitch: 0.0,
        mouse_held: false,
        last_mouse_pos: None,
    };

    event_loop.run_app(&mut app).unwrap();
}