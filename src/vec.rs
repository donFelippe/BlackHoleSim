use std::ops::{Add, Sub, Mul, Neg};

#[derive (Debug, Copy, Clone)]
pub struct Vec2{
    pub x: f64,
    pub y: f64

}

impl Vec2 {
    pub fn new(x:f64, y:f64) -> Self{Self {x,y}}
    pub fn zero() -> Self{Self {x:0.0, y:0.0}}
    pub fn length_sq(&self) -> f64 {self.x*self.x + self.y*self.y}
    pub fn length(&self) -> f64 {self.length_sq().sqrt()}

    pub fn normalise(&self) -> Self{
        let l = self.length();
        Self{x: self.x /l, y: self.y /l}
    }
    pub fn dot(&self, o: Vec2) -> f64 {self.x*o.x + self.y*o.y}

}

impl Add for Vec2{
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 { Vec2::new (self.x + o.x, self.y +o.y)}
}

impl Sub for Vec2{
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x-rhs.x,self.y - rhs.y)
    }
}

impl Mul<f64> for Vec2{
    type Output = Vec2;
    fn mul(self, m: f64) -> Vec2 {
        Vec2::new(self.x * m, self.y*m)
    }
}

impl Neg for Vec2{
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

#[derive (Debug, Copy, Clone)]
pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64

}

impl Vec3 {
    pub fn new(x:f64, y:f64, z:f64) -> Self{Self {x,y,z}}
    pub fn zero() -> Self{Self {x:0.0, y:0.0, z:0.0}}
    pub fn length_sq(&self) -> f64 {self.x*self.x + self.y*self.y + self.z*self.z}
    pub fn length(&self) -> f64 {self.length_sq().sqrt()}

    pub fn normalise(&self) -> Self{
        let l = self.length();
        Self{x: self.x /l, y: self.y /l, z: self.z / l}
    }
    pub fn dot(&self, o: Vec3) -> f64 {self.x*o.x + self.y*o.y + self.z*o.z}

    pub fn cross(&self, o: Vec3) -> Vec3 {
        Vec3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
}

impl Add for Vec3{
    type Output = Vec3;
    fn add(self, o: Vec3) -> Vec3 { Vec3::new (self.x + o.x, self.y +o.y, self.z + o.z)}
}

impl Sub for Vec3{
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x-rhs.x,self.y - rhs.y, self.z-rhs.z)
    }
}

impl Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, m: f64) -> Vec3 {
        Vec3::new(self.x * m, self.y*m, self.z*m)
    }
}

impl Neg for Vec3{
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
