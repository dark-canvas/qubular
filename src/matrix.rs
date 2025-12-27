use std::ops;
use std::fmt;

use trigr::SineCosineTable;

// Do we want this?
//#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    // TODO: make this private and add getter methods?
    pub data:  [[f64; 4]; 4],
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..4 {
            write!(f, "[ {} {} {} {} ]\n", self.data[i][0], self.data[i][1], self.data[i][2], self.data[i][3] )?;
        }
        Ok(())
    }
}

impl Matrix {
    pub fn identity() -> Matrix {
        Matrix {
            data: [ [ 1.0, 0.0, 0.0, 0.0, ],
                    [ 0.0, 1.0, 0.0, 0.0, ],
                    [ 0.0, 0.0, 1.0, 0.0, ],
                    [ 0.0, 0.0, 0.0, 1.0, ] ]
        }
    }

    pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
        Matrix {
            data: [ [ 1.0, 0.0, 0.0, 0.0, ],
                    [ 0.0, 1.0, 0.0, 0.0, ],
                    [ 0.0, 0.0, 1.0, 0.0, ],
                    [   x,   y,   z, 1.0, ] ]
        }
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
        Matrix {
            data: [ [   x, 0.0, 0.0, 0.0, ],
                    [ 0.0,   y, 0.0, 0.0, ],
                    [ 0.0, 0.0,   z, 0.0, ],
                    [ 0.0, 0.0, 0.0, 1.0, ] ]
        }
    }

    pub fn rotate_x(angle: f64, lookup: &SineCosineTable) -> Matrix {
        let sin_a = lookup.sine(angle);
        let cos_a = lookup.cosine(angle);
        Matrix {
            data: [ [  1.0,    0.0,    0.0,    0.0, ],
                    [  0.0,  cos_a,  sin_a,    0.0, ],
                    [  0.0, -sin_a,  cos_a,    0.0, ],
                    [  0.0,    0.0,    0.0,    1.0, ] ]
        }
    }

    pub fn rotate_y(angle: f64, lookup: &SineCosineTable) -> Matrix {
        let sin_a = lookup.sine(angle);
        let cos_a = lookup.cosine(angle);
        Matrix {
            data: [ [ cos_a,    0.0, -sin_a,    0.0, ],
                    [   0.0,    1.0,    0.0,    0.0, ],
                    [ sin_a,    0.0,  cos_a,    0.0, ],
                    [   0.0,    0.0,    0.0,    1.0, ] ]
        }
    }

    pub fn rotate_z(angle: f64, lookup: &SineCosineTable) -> Matrix {
        let sin_a = lookup.sine(angle);
        let cos_a = lookup.cosine(angle);
        Matrix {
            data: [ [  cos_a,  sin_a,    0.0,    0.0, ],
                    [ -sin_a,  cos_a,    0.0,    0.0, ],
                    [    0.0,    0.0,    1.0,    0.0, ],
                    [    0.0,    0.0,    0.0,    1.0, ] ]
        }
    }

    // TODO: 
    // write an inverse function?
    // https://stackoverflow.com/questions/1148309/inverting-a-4x4-matrix
}

/*
impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    // TODO: this can be optimized in the context of 3d math
    fn mul(self, rhs: &Self) -> Self {
        let mut result = Matrix::identity();
        for r in 0..4 {
            for c in 0..4 {
                result.data[r][c] = 0.0;
                for i in 0..4 {
                    result.data[r][c] += self.data[r][i] * rhs.data[i][c];
                }
            }
        }
        result
    }
}
    */

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    // TODO: this can be optimized in the context of 3d math
    // NOTE: this consumes self and rhs which isn't ideal (although 
    // it looks better in code, eg: let c = a * b; vs let c = &a * &b;)
    fn mul(self, rhs: Self) -> Self {
        let mut result = Matrix::identity();
        for r in 0..4 {
            for c in 0..4 {
                result.data[r][c] = 0.0;
                for i in 0..4 {
                    result.data[r][c] += self.data[r][i] * rhs.data[i][c];
                }
            }
        }
        result
    }
}
