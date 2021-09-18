use sdl2::rect::Point;
use std::ops::Add;

pub fn point_to_angle(p: Point) -> f64 {
    (p.y() as f64).atan2(p.x() as f64)
}

pub struct PolarVector {
    pub r: f64,
    pub theta: f64,  // in radians
}

impl PolarVector {
    pub fn set_r(mut self, r: f64) -> Self {
        self.r = r;
        self
    }

    pub fn set_theta(mut self, theta: f64) -> Self {
        self.theta = theta;
        self
    }
}

impl From<(f64, f64)> for PolarVector {
    fn from(tuple: (f64, f64)) -> Self {
        Self { r: tuple.0, theta: tuple.1 }
    }
}

impl Add for PolarVector {
    type Output = PolarVector;

    fn add(self, rhs: Self) -> Self::Output {
        // from https://math.stackexchange.com/questions/1365622/adding-two-polar-vectors

        let delta = rhs.theta - self.theta;
        Self::Output {
            r: (self.r.powi(2) + rhs.r.powi(2) + 2. * self.r * rhs.r * delta.cos()).sqrt(),
            theta: (self.r + rhs.theta * delta.sin()).atan2(self.r + rhs.r * delta.cos()),
        }
    }
}