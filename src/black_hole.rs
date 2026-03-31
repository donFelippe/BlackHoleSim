use crate::vec::Vec3;

pub struct BlackHole{
    pub mass: f64,
    pub schwarzschild_radius: f64,
    pub position : Vec3,
}

impl BlackHole{
    pub fn new(mass: f64, position: Vec3) -> Self{
        Self{
            mass, schwarzschild_radius: 2.0 * mass , position
        }
    }

    pub fn acceleration(&self, pos: Vec3) -> Vec3 {
        let offset = self.position - pos; // vector towards bh
        let r_sq = offset.length_sq();
        let r = r_sq.sqrt();
        if r < 0.001 { return Vec3::zero(); } // avoid singularity
        let magnitude = self.mass / r_sq;
        offset.normalise() * magnitude
    }
}