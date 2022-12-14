use std::ops::{Add, Sub, Mul, Neg};

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn zero() {
        Vector3::from_one(0.0);
        }
    

    pub fn from_one(v: f64) -> Vector3 {
        Vector3 {
            x: v,
            y: v,
            z: v,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Vector3 {
        let l = self.length();
        Vector3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

    pub fn dot_prod(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_prod(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}


impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
