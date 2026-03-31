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