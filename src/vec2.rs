pub struct Vec2 {
    e: [f64; 2]
}  

impl Vec2 {
    pub fn new(x: f64, y:f64) -> Vec2 {
        Vec2 { e: [x, y] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}