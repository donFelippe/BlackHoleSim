use rayon::prelude::*;
use crate::{vec::Vec3, ray::{Ray, rk4_step}, black_hole::BlackHole};

pub fn render(
    frame: &mut [u8],
    width: u32,
    height: u32,
    bh: &BlackHole,
    yaw: f64,
    pitch: f64,
) {
    let w = width as f64;
    let h = height as f64;

    let cam_dist = 60.0;
    let fov = 0.5;

    let cos_p = pitch.cos();
    let sin_p = pitch.sin();
    let cos_y = yaw.cos();
    let sin_y = yaw.sin();

    let rotate = |v: Vec3| -> Vec3 {
        let y1 = v.y * cos_p - v.z * sin_p;
        let z1 = v.y * sin_p + v.z * cos_p;
        let x1 = v.x;

        let x2 = x1 * cos_y + z1 * sin_y;
        let z2 = -x1 * sin_y + z1 * cos_y;
        Vec3::new(x2, y1, z2)
    };

    let cam_pos = rotate(Vec3::new(0.0, 0.0, -cam_dist));

    frame.par_chunks_exact_mut(4).enumerate().for_each(|(i, pixel)| {
        let px = (i % width as usize) as f64;
        let py = (i / width as usize) as f64;

        let uv_x = (2.0 * px - w) / w;
        let uv_y = -(2.0 * py - h) / h;

        let ray_dir_local = Vec3::new(uv_x, uv_y, fov).normalise();
        let ray_dir = rotate(ray_dir_local);

        let mut ray = Ray::new(cam_pos, ray_dir);
        let color = trace_ray(&mut ray, bh);

        pixel[0] = color[0];
        pixel[1] = color[1];
        pixel[2] = color[2];
        pixel[3] = 255;
    });
}

fn trace_ray(ray: &mut Ray, bh: &BlackHole) -> [u8; 3] {
    let max_steps = 1200; 
    let dt = 0.15;

    let rs = bh.schwarzschild_radius;
    let disk_inner = rs * 1.8;
    let disk_outer = rs * 5.0;

    let mut final_color = [0.0f64, 0.0f64, 0.0f64];
    let mut hit_disk = false;

    for _ in 0..max_steps {
        let r = ray.pos.length();

        // Singularity
        if r < rs {
            return [final_color[0].min(255.0) as u8, final_color[1].min(255.0) as u8, final_color[2].min(255.0) as u8];
        }

        // Deep space 
        if r > 100.0 {
            let bg = background_shader(ray.vel);
            if hit_disk {
                // Daca a trecut prin disc, mixam lumina discului cu fundalul
                return [
                    (final_color[0] + bg[0] as f64 * 0.2).min(255.0) as u8,
                    (final_color[1] + bg[1] as f64 * 0.2).min(255.0) as u8,
                    (final_color[2] + bg[2] as f64 * 0.2).min(255.0) as u8,
                ];
            } else {
                return bg;
            }
        }

        let next_ray = rk4_step(ray, bh, dt);

        if ray.pos.y * next_ray.pos.y < 0.0 {
            let t = ray.pos.y.abs() / (ray.pos.y.abs() + next_ray.pos.y.abs());
            let hit_pos = ray.pos + (next_ray.pos - ray.pos) * t;
            let hit_r = hit_pos.length();

            if hit_r > disk_inner && hit_r < disk_outer {
                let intensity = 1.0 - ((hit_r - disk_inner) / (disk_outer - disk_inner));

                final_color[0] += 255.0 * 0.8;
                final_color[1] += (180.0 * intensity + 50.0) * 0.8;
                final_color[2] += (50.0 * intensity) * 0.8;
                hit_disk = true;
            }
        }

        *ray = next_ray;
    }

    // Daca, printr-o minune, o raza orbiteaza de atatea ori incat ramane iar fara pasi (Sfera Fotonică)
    if hit_disk {
        [final_color[0].min(255.0) as u8, final_color[1].min(255.0) as u8, final_color[2].min(255.0) as u8]
    } else {
        background_shader(ray.vel)
    }
}
fn background_shader(dir: Vec3) -> [u8; 3] {
    let u = (dir.x.atan2(dir.z) / std::f64::consts::TAU) + 0.5;
    let v = dir.y.asin() / std::f64::consts::PI + 0.5;

    let grid_u = (u * 40.0).fract();
    let grid_v = (v * 20.0).fract();

    if grid_u < 0.03 || grid_v < 0.03 {
        [50, 60, 100]
    } else {
        [5, 5, 10]
    }
}