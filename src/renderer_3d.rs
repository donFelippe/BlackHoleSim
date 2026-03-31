use crate::{vec::Vec3, ray::{Ray, rk4_step}, black_hole::BlackHole};

const DT: f64 = 0.05;
const ESCAPE_R: f64 = 50.0;
const STEPS_PER_FRAME: u32 = 4;

// Parametri pentru Camera 3D
const CAM_DISTANCE: f64 = 60.0;
const FOV_SCALE: f64 = 800.0; // Controleaza cat de "zoomata" e imaginea

pub fn render(
    frame: &mut [u8],
    width: u32,
    height: u32,
    bh: &BlackHole,
    rays: &mut [Ray],
    clear_screen: bool,
    yaw: f64,
    pitch: f64,
) {
    let w = width as f64;
    let h = height as f64;

    // Cand rotim camera, trebuie sa stergem "darele" vechi
    if clear_screen {
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 5;
            pixel[1] = 5;
            pixel[2] = 10;
            pixel[3] = 255;
        }
    }

    // Functie de proiectie 3D -> 2D
    let mut draw_point_3d = |pos: Vec3, color: [u8; 3]| {
        // 1. Rotim scena in jurul axei X (Pitch - sus/jos)
        let y1 = pos.y * pitch.cos() - pos.z * pitch.sin();
        let z1 = pos.y * pitch.sin() + pos.z * pitch.cos();

        // 2. Rotim scena in jurul axei Y (Yaw - stanga/dreapta)
        let x2 = pos.x * yaw.cos() + z1 * yaw.sin();
        let z2 = -pos.x * yaw.sin() + z1 * yaw.cos();
        let y2 = y1;

        // 3. Impingem scena in fata camerei
        let z_cam = z2 + CAM_DISTANCE;

        // Daca punctul e in spatele camerei, nu il desenam
        if z_cam < 0.1 { return; }

        // 4. Proiectie de perspectiva (Impartim x si y la adancimea z)
        let px = (x2 / z_cam * FOV_SCALE + w / 2.0) as isize;
        let py = (-y2 / z_cam * FOV_SCALE + h / 2.0) as isize; // Minus ca sa inversam axa Y a ecranului

        if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
            let idx = ((py as u32) * width + (px as u32)) as usize * 4;
            frame[idx] = color[0];
            frame[idx + 1] = color[1];
            frame[idx + 2] = color[2];
            frame[idx + 3] = 255;
        }
    };

    // Desenam gaura neagra (Optional, poti face o sfera 3D, aici desenez doar un punct central grozav)
    if clear_screen {
        draw_point_3d(bh.position, [255, 50, 50]);
    }

    // Avansam razele in spatiul fizic 3D
    for ray in rays.iter_mut() {
        let r = (ray.pos - bh.position).length();
        if r < bh.schwarzschild_radius || r > ESCAPE_R {
            continue;
        }

        for _ in 0..STEPS_PER_FRAME {
            *ray = rk4_step(ray, bh, DT);

            // Culorile pot depinde de pozitia pe axa Z ca sa le deosebim in spatiu
            let depth_color = ((ray.pos.z / 20.0 + 0.5).clamp(0.0, 1.0) * 255.0) as u8;
            draw_point_3d(ray.pos, [255, 200, depth_color]);
        }
    }
}