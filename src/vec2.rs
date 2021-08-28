use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 { Vec2{x, y} }
    pub fn zero() -> Vec2 { Vec2::new(0.0, 0.0) }
    pub fn sub(&self, other: Vec2) -> Vec2 { Vec2::new(self.x - other.x, self.y - other.y) }
    pub fn add(&self, other: Vec2) -> Vec2 { Vec2::new(self.x + other.x, self.y + other.y) }
    pub fn mul_scalar(&self, scalar: f32) -> Vec2 { Vec2::new(self.x * scalar, self.y * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec2 { Vec2::new(self.x / scalar, self.y / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y).sqrt() }
    pub fn normalize(&self) -> Vec2 { self.div_scalar(self.magnitude()) }
    pub fn lerp(&self, other: Vec2, t: f32) -> Vec2 { Vec2::new(self.x*(1.0-t) + other.x*(t), self.y*(1.0-t) + other.y*(t)) }
    pub fn rotate(&self, radians: f32) -> Vec2 { 
        Vec2::new(
            self.x * radians.cos() - self.y * radians.sin(), 
            self.x * radians.sin() + self.y * radians.cos()
        ) 
    }
    
    pub fn spread(&self, amount: f32) -> Vec2 {
        let roll = rand::thread_rng().gen_range(-amount..amount);
        return self.rotate(roll);
    }
}