use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new (x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length (&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn unit (&self) -> Vec3 {
        let k = 1f32 / self.length();

        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k
        }
    }

    pub fn normalize (&self) -> Vec3 {
        Vec3 {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length()
        }
    }

    pub fn dot (&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}
