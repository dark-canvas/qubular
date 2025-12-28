use crate::matrix::Matrix;

use std::ops;
use std::fmt;

// TODO: change to f32 to save space and improve performance?
// TODO: do we need w here?
#[derive(Debug, Copy, Clone)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {} {} {} {} ]", self.x, self.y, self.z, self.w)
    }
}

impl ops::Mul<Matrix> for Point3D {
    type Output = Point3D;

    // TODO: this can be optimized in the context of 3d math
    fn mul(self, mat: Matrix) -> Self {
        Point3D {
            x: self.x * mat.data[0][0] + self.y * mat.data[1][0] + self.z * mat.data[2][0] + self.w * mat.data[3][0],
            y: self.x * mat.data[0][1] + self.y * mat.data[1][1] + self.z * mat.data[2][1] + self.w * mat.data[3][1],
            z: self.x * mat.data[0][2] + self.y * mat.data[1][2] + self.z * mat.data[2][2] + self.w * mat.data[3][2],
            w: self.x * mat.data[0][3] + self.y * mat.data[1][3] + self.z * mat.data[2][3] + self.w * mat.data[3][3],
        }
    }
}

impl ops::Mul<&Matrix> for Point3D {
    type Output = Point3D;

    // TODO: this can be optimized in the context of 3d math
    fn mul(self, mat: &Matrix) -> Self {
        Point3D {
            x: self.x * mat.data[0][0] + self.y * mat.data[1][0] + self.z * mat.data[2][0] + self.w * mat.data[3][0],
            y: self.x * mat.data[0][1] + self.y * mat.data[1][1] + self.z * mat.data[2][1] + self.w * mat.data[3][1],
            z: self.x * mat.data[0][2] + self.y * mat.data[1][2] + self.z * mat.data[2][2] + self.w * mat.data[3][2],
            w: self.x * mat.data[0][3] + self.y * mat.data[1][3] + self.z * mat.data[2][3] + self.w * mat.data[3][3],
        }
    }
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D{
            x:x, 
            y:y,
            z:z,
            w:1.0,
        }
    }
}
