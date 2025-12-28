
use crate::{Point2D, Point3D, Matrix, Vector};

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
    normals: Vec<Vector>,
    projected_normals: Vec<Point2D>,
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
        let mut result = SimpleObject{
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
            // define polygons in counter-clockwise order in order for the normals to point outward
            polygons: vec![
                vec![ 3, 2, 1, 0 ], // front
                vec![ 0, 4, 7, 3 ], // left
                vec![ 4, 5, 6, 7 ], // back
                vec![ 5, 1, 2, 6 ], // right
                vec![ 1, 5, 4, 0 ], // top
                vec![ 3, 7, 6, 2 ], // bottom
            ],
            normals: vec![
                Vector::new(0.0, 0.0, 0.0); 6
            ],
            projected_normals: vec![
                Point2D{ x:0, y:0 }; 6
            ],
        };
        result.calculate_normals();
        result
    }

    fn calculate_normals(&mut self) {
        // calculate normals for each polygon
        for i in 0..self.get_polygon_count() {
            let polygon = &self.polygons[i];

            let p0 = &self.transformed[polygon[0]];
            let p1 = &self.transformed[polygon[1]];
            let p2 = &self.transformed[polygon[2]];
            //let v1 = Vector::from_points(p0, p1);
            //let v2 = Vector::from_points(p0, p2);
            let v1 = Vector::from_points(p1, p0);
            let v2 = Vector::from_points(p1, p2);
            let normal = v1.cross_product(&v2).normalize();
            self.normals[i] = normal;
        }
    }


    pub fn get_polygon_count(&self) -> usize {
        self.polygons.len()
    }

    pub fn get_vertices(&self) -> &Vec<Point3D> {
        &self.vertices
    }

    pub fn get_transformed(&self) -> &Vec<Point3D> {
        &self.transformed
    }

    pub fn get_projected(&self) -> &Vec<Point2D> {
        &self.projected
    }

    pub fn get_polygons(&self) -> &Vec<Polygon> {
        &self.polygons
    }

    pub fn get_polygon(&self, index: usize) -> &Polygon {
        &self.polygons[index]
    }

    pub fn get_normals(&self) -> &Vec<Vector> {
        &self.normals
    }

    pub fn get_projected_normals(&self) -> &Vec<Point2D> {
        &self.projected_normals
    }

    pub fn apply(&mut self, mat: &Matrix) {
        for i in 0..self.vertices.len() {
            self.transformed[i] = self.vertices[i] * mat;
        }
        self.calculate_normals();
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

        for i in 0..self.normals.len() {
            let normal = &self.normals[i];
            let mut projected_normal = Point2D{
                x: ((normal.x * aspect_ratio * fov_rad) / normal.z * (win_width as f64 / 2.0) + (win_width as f64 / 2.0)) as u32,
                y: ((normal.y * fov_rad) / normal.z * (win_height as f64 / 2.0) + (win_height as f64 / 2.0)) as u32,
            };
            self.projected_normals[i] = projected_normal;
        }
    }
}