
use crate::Point;

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
type Polygon = Vec<u32>;

pub struct SimpleObject {
    // TODO: will need some way to expose this in order to allow it to be transformed
    vertices: Vec<Point>,
    polygons: Vec<Polygon>,
}

impl SimpleObject {
    pub fn get_vertices(&self) -> Vec<Point> {
        self.vertices.clone()
    }

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
                Point::new(-d,  d,  d), Point::new(d,  d,  d), Point::new(d, -d,  d), Point::new(-d, -d,  d), 
                Point::new(-d,  d, -d), Point::new(d,  d, -d), Point::new(d, -d, -d), Point::new(-d, -d, -d), 
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
}