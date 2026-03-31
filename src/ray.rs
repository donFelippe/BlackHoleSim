use crate::vec::Vec2;
use crate::black_hole::BlackHole;

#[derive(Clone, Copy)]
pub struct Ray{
    pub pos: Vec2,
    pub vel: Vec2, //direction * speed, for light |vel| = 1
}

impl Ray {
    pub fn new(pos: Vec2, vel: Vec2) -> Self { Self { pos, vel } }
}

//rk4 integration for 1 step

// returns new pos, new svelocity
pub fn rk4_step(ray: &Ray, bh: &BlackHole, dt: f64) -> Ray{
    //k1
    let a1 = bh.acceleration(ray.pos);
    let kp1 = ray.vel;
    let kv1 = a1;

    //k2
    let p2 = ray.pos + kp1 * (dt * 0.5);
    let v2 = ray.vel + kv1 * (dt * 0.5);
    let a2 = bh.acceleration(p2);
    let kp2 = v2;
    let kv2 = a2;

    // k3
    let p3 = ray.pos + kp2 * (dt * 0.5);
    let v3 = ray.vel + kv2 * (dt * 0.5);
    let a3 = bh.acceleration(p3);
    let kp3 = v3;
    let kv3 = a3;

    // k4
    let p4 = ray.pos + kp3 * dt;
    let v4 = ray.vel + kv3 * dt;
    let a4 = bh.acceleration(p4);
    let kp4 = v4;
    let kv4 = a4;

    Ray{
        pos: ray.pos + (kp1+kp2*2.0+kp3*2.0+kp4) * (dt/6.0),
        vel: ray.vel + (kv1 + kv2 * 2.0 + kv3 * 2.0 + kv4) * (dt / 6.0), // for the accuracy of the approk, the middle samples have more meaning therefore are weighted more than the beginning and end samples.
    }

}