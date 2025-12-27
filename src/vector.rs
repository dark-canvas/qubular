use crate::point3d::Point3D;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector{
            x:x, 
            y:y,
            z:z,
        }
    }

    fn from_points(p1: &Point3D, p2: &Point3D) -> Self {
        Vector{
            x: p2.x - p1.x,
            y: p2.y - p1.y,
            z: p2.z - p1.z,
        }
    }

    fn normalize(&self) -> Vector {
        let length = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    fn cross_product(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn dot_product(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}