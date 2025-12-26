
use crate::{Point2D, Point3D, Matrix};

// TODO: descibe on the proper coordinate system?
// OpenGL supposedly uses a right handed system:
//
//      y axis
//        ^
//        |
//        |
//        /--------->   x axis
//       /  
//      /
//     V 
//  z axis


// A polygon is a list of points, where is point is represented as 
// an index into the vertices array
// NOTE: this doesn't need to be usize, and is probably better as u32 (or even a u16), 
// but it's used as an index into a Vec, which uses usize, so this saves a bunch of 
// "as usize" conversions.
pub type Polygon = Vec<usize>;

pub struct SimpleObject {
    // TODO: will need some way to expose this in order to allow it to be transformed
    vertices: Vec<Point3D>,
    transformed: Vec<Point3D>,
    projected: Vec<Point2D>,
    polygons: Vec<Polygon>,
}

impl SimpleObject {

    pub fn cube(size: u32) -> Self {
        /*
         *     4 _____________5
         *      /.           /|
         *     / .          / |
         *    /__._________/  |
         *  0|   .        |1  |
         *   |   ........ |...|
         *   |  . 7       |  /6
         *   | .          | /
         *   |____________|/
         *   3            2
         */


        let d : f64 = size as f64 / 2.0;
        SimpleObject{
            vertices: vec![
                // front most (+z) sqaure, from top-left point and going clock-wise, followed by the same square in behind (-z)
                Point3D::new(-d,  d,  d), Point3D::new(d,  d,  d), Point3D::new(d, -d,  d), Point3D::new(-d, -d,  d), 
                Point3D::new(-d,  d, -d), Point3D::new(d,  d, -d), Point3D::new(d, -d, -d), Point3D::new(-d, -d, -d), 
            ],
            transformed: vec![
                Point3D{ x:0.0, y:0.0, z:0.0, w:0.0 }; 8
            ],
            projected: vec![
                Point2D{ x:0, y:0 }; 8
            ],
            polygons: vec![
                vec![ 0, 1, 2, 3 ], // front
                vec![ 0, 4, 7, 3 ], // left
                vec![ 4, 5, 6, 7 ], // back
                vec![ 1, 5, 6, 2 ], // right
                vec![ 0, 4, 5, 1 ], // top
                vec![ 3, 7, 6, 2 ], // bottom
            ]
        }
    }

    // TODO: return references rather than clones
    pub fn get_vertices(&self) -> Vec<Point3D> {
        self.vertices.clone()
    }

    pub fn get_transformed(&self) -> Vec<Point3D> {
        self.transformed.clone()
    }

    pub fn get_projected(&self) -> Vec<Point2D> {
        self.projected.clone()
    }

    pub fn get_polygons(&self) -> Vec<Polygon> {
        self.polygons.clone()
    }

    pub fn apply(&mut self, mat: &Matrix) {
        for i in 0..self.vertices.len() {
            self.transformed[i] = self.vertices[i] * mat;
        }
    }

    // NOTE: this is the what co-pilot produced as a projection function.  It's more complex than 
    // mine, as it takes into account aspect ratio.
    // TODO: compare the output of the two projection methods.
    // TODO: if using this one, pre-calc fov (and use trigr)
    pub fn project(&mut self, win_width: usize, win_height: usize, fov: usize) {
        let aspect_ratio = win_width as f64 / win_height as f64;
        let fov_rad = 1.0 / (fov as f64 * 0.5 / 180.0 * std::f64::consts::PI).tan();

        for i in 0..self.transformed.len() {
            let point = &self.transformed[i];
            let mut projected = Point2D{
                x: ((point.x * aspect_ratio * fov_rad) / point.z * (win_width as f64 / 2.0) + (win_width as f64 / 2.0)) as u32,
                y: ((point.y * fov_rad) / point.z * (win_height as f64 / 2.0) + (win_height as f64 / 2.0)) as u32,
            };
            self.projected[i] = projected;
        }
    }
}