use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vector3;
pub type Point = Vector3;

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self*v.x,
            y: self*v.y,
            z: self*v.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, v: f64) -> Vector3 {
        Vector3 {
            x: self.x*v,
            y: self.y*v,
            z: self.z*v,
        }
    }
}

impl Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, v: f64) -> Vector3 {
        Vector3 {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v,
        }
    }
}

impl Vector3 {
    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }


    pub fn dot(v1: &Vector3, v2: &Vector3) -> f64 {
        v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
    }

    pub fn cross(v1: &Vector3, v2: &Vector3) -> Vector3 {
        Vector3 {
            x: v1.y*v2.z - v1.z*v2.y,
            y: v1.z*v2.x - v1.x*v2.z,
            z: v1.x*v2.y - v1.y*v2.x,
        }
    }

    pub fn unit_vector(v: &Vector3) -> Vector3 {
        v / v.length()
    }
}
