use crate::{vec::Vec2, ray::{Ray, rk4_step}, black_hole::BlackHole};

const WORLD_HALF: f64 = 20.0;
const DT: f64 = 0.05;
const ESCAPE_R: f64 = 40.0;
// pasi pe cadru
const STEPS_PER_FRAME: u32 = 4;

pub fn render(
    frame: &mut [u8],
    width: u32,
    height: u32,
    bh: &BlackHole,
    rays: &mut [Ray],
    first_frame: bool,
) {
    let w = width as f64;
    let h = height as f64;

    if first_frame {
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 10;
            pixel[1] = 10;
            pixel[2] = 20;
            pixel[3] = 255;
        }

        // event horison
        let bh_steps = 150;
        for i in 0..bh_steps {
            let angle = (i as f64 / bh_steps as f64) * std::f64::consts::TAU;
            let pos = bh.position + Vec2::new(angle.cos(), angle.sin()) * bh.schwarzschild_radius;

            let px = ((pos.x / (2.0 * WORLD_HALF) + 0.5) * w) as isize;
            let py = ((0.5 - pos.y / (2.0 * WORLD_HALF)) * h) as isize;

            if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                let idx = ((py as u32) * width + (px as u32)) as usize * 4;
                frame[idx] = 255;
                frame[idx + 1] = 50;
                frame[idx + 2] = 50;
                frame[idx + 3] = 255;
            }
        }
    }


    for ray in rays.iter_mut() {
        let r = (ray.pos - bh.position).length();
        if r < bh.schwarzschild_radius || r > ESCAPE_R {
            continue; //escaped
        }

        for _ in 0..STEPS_PER_FRAME {
            *ray = rk4_step(ray, bh, DT);

            // Conversia în pixeli
            let px = ((ray.pos.x / (2.0 * WORLD_HALF) + 0.5) * w) as isize;
            let py = ((0.5 - ray.pos.y / (2.0 * WORLD_HALF)) * h) as isize;

            if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                let idx = ((py as u32) * width + (px as u32)) as usize * 4;
                frame[idx] = 255;     // R
                frame[idx + 1] = 230; // G
                frame[idx + 2] = 150; // B
                frame[idx + 3] = 255; // A
            }
        }
    }
}