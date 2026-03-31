use crate::{vec::Vec3, black_hole::BlackHole};

const CAM_DISTANCE: f64 = 60.0;
const FOV_SCALE: f64 = 800.0;

pub fn render(
    frame: &mut [u8],
    width: u32,
    height: u32,
    bh: &BlackHole,
    paths: &[Vec<Vec3>],
    time_step: usize,
    yaw: f64,
    pitch: f64,
) {
    let w = width as f64;
    let h = height as f64;

    // Stergem ecranul COMPLET la fiecare cadru
    for pixel in frame.chunks_exact_mut(4) {
        pixel[0] = 5;
        pixel[1] = 5;
        pixel[2] = 10;
        pixel[3] = 255;
    }

    let mut draw_point_3d = |pos: Vec3, color: [u8; 3]| {
        let y1 = pos.y * pitch.cos() - pos.z * pitch.sin();
        let z1 = pos.y * pitch.sin() + pos.z * pitch.cos();

        let x2 = pos.x * yaw.cos() + z1 * yaw.sin();
        let z2 = -pos.x * yaw.sin() + z1 * yaw.cos();
        let y2 = y1;

        let z_cam = z2 + CAM_DISTANCE;
        if z_cam < 0.1 { return; }

        let px = (x2 / z_cam * FOV_SCALE + w / 2.0) as isize;
        let py = (-y2 / z_cam * FOV_SCALE + h / 2.0) as isize;

        if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
            let idx = ((py as u32) * width + (px as u32)) as usize * 4;
            frame[idx] = color[0];
            frame[idx + 1] = color[1];
            frame[idx + 2] = color[2];
            frame[idx + 3] = 255;
        }
    };

    // Desenam Singularitatea
    draw_point_3d(bh.position, [255, 50, 50]);

    // Randam traiectoriile pre-calculate pana la 'time_step'
    for path in paths {
        let steps_to_draw = path.len().min(time_step);

        for i in 0..steps_to_draw {
            let pos = path[i];
            let depth_color = ((pos.z / 20.0 + 0.5).clamp(0.0, 1.0) * 255.0) as u8;
            draw_point_3d(pos, [255, 200, depth_color]);
        }
    }
}