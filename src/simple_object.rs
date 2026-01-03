
use crate::{Point2D, Point3D, Matrix, Vector, Colour, LightSource};
use crate::gfx::Screen;
use std::fmt;

// TODO: confirm this...
//
//            -y 
//             ^    ^ +z
//             |   /
//             |  /
//             | /
//  -x --------|--------->  +x
//           / |  
//          /  |
//         V   |
//      -z     v +y


// A polygon is a list of points, where is point is represented as 
// an index into the vertices array
// NOTE: this doesn't need to be usize, and is probably better as u32 (or even a u16), 
// but it's used as an index into a Vec, which uses usize, so this saves a bunch of 
// "as usize" conversions.
pub type Polygon = Vec<usize>;
pub type VertexColours = Vec<Colour>;

pub struct SimpleObject {
    // TODO: will need some way to expose this in order to allow it to be transformed
    vertices: Vec<Point3D>,
    transformed: Vec<Point3D>,
    projected: Vec<Point3D>,
    polygons: Vec<Polygon>,
    normals: Vec<Vector>,
    colours: Vec<Colour>,
    lit_colours: Vec<VertexColours>,
    projected_normals: Vec<(Point2D, Point2D)>,
}

impl fmt::Display for SimpleObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in 0..self.get_polygon_count() {
            let polygon = self.get_polygon(p);
            write!(f, "Polygon {}:\n", p)?;
            for v in 0..polygon.len() {
                let vi = polygon[v];
                let overtex = &self.vertices[vi];
                let tvertex = &self.transformed[vi];
                let lit_colour = &self.lit_colours[p][v];
                write!(f, "  Vertex {}: {} {} {}\n", vi, overtex, tvertex, lit_colour)?;
            }
            write!(f, "  Normal: {}\n", self.normals[p])?;
        }
        Ok(())
    }
}

impl SimpleObject {

    pub fn cube(size: u32) -> Self {
        /*
         *     0 _____________1
         *      /.           /|
         *     / .          / |
         *    /__._________/  |
         *  4|   .       5|   |
         *   |   ........ |...|
         *   |  . 3       |  /2
         *   | .          | /
         *   |____________|/
         *   7            6
         */


        let d : f64 = size as f64 / 2.0;
        let cube = vec![
                Point3D::new(-d,  d,  d), Point3D::new(d,  d,  d), Point3D::new(d, -d,  d), Point3D::new(-d, -d,  d), 
                Point3D::new(-d,  d, -d), Point3D::new(d,  d, -d), Point3D::new(d, -d, -d), Point3D::new(-d, -d, -d), 
            ];
        let mut result = SimpleObject {
            vertices: cube.clone(),
            transformed: cube.clone(), // same as the original at start
            projected: vec![
                Point3D{ x:0.0, y:0.0, z:0.0, w:0.0 }; 8
            ],
            // define polygons in counter-clockwise order in order for the normals to point outward
            // TODO: merge all polygon data into a single structure
            polygons: vec![
                vec![ 0, 1, 2, 3 ], 
                vec![ 0, 3, 7, 4 ], 
                vec![ 7, 6, 5, 4 ], 
                vec![ 1, 5, 6, 2 ], 
                vec![ 0, 4, 5, 1 ], 
                vec![ 2, 6, 7, 3 ], 
            ],
            normals: vec![
                Vector::new(0.0, 0.0, 0.0); 6
            ],
            colours: vec![
                Colour::new(255, 0, 0),   // red
                Colour::new(0, 255, 0),   // green
                Colour::new(0, 0, 255),   // blue
                Colour::new(255, 255, 0), // yellow
                Colour::new(0, 255, 255), // cyan
                Colour::new(255, 0, 255), // magenta
            ],
            lit_colours: vec![ 
                vec![Colour::new(255,255,255); 4],
                vec![Colour::new(255,255,255); 4],
                vec![Colour::new(255,255,255); 4],
                vec![Colour::new(255,255,255); 4],
                vec![Colour::new(255,255,255); 4],
                vec![Colour::new(255,255,255); 4],
            ],
            projected_normals: vec![
                (Point2D{ x:0, y:0 }, Point2D{ x:0, y:0 }); 6
            ],
        };
        result.calculate_normals();
        result
    }

    fn calculate_normals(&mut self) {
        // TODO: do these need to be recalculated or can they be translated with the object?
        // calculate normals for each polygon
        for i in 0..self.get_polygon_count() {
            let polygon = &self.polygons[i];

            let p0 = &self.transformed[polygon[0]];
            let p1 = &self.transformed[polygon[1]];
            let p2 = &self.transformed[polygon[2]];
            let v1 = Vector::from_points(p0, p1);
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

    pub fn get_projected(&self) -> &Vec<Point3D> {
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

    pub fn get_colours(&self) -> &Vec<Colour> {
        &self.colours
    }

    pub fn apply(&mut self, mat: &Matrix) {
        for i in 0..self.vertices.len() {
            self.transformed[i] = self.vertices[i] * mat;
        }
        self.calculate_normals();
    }

    pub fn light(&mut self, light: &LightSource) {
        for p in 0..self.polygons.len() {
            let polygon = &self.polygons[p];
            let n = &self.normals[p];

            for v in 0..polygon.len() {
                let point = &self.vertices[ polygon[v] ];

                // calculate the normal from the light source to the vertex
                let l = Vector::from_points(point, light.get_position()).normalize();

                // The dot product of the light vector (l) and the polygon normal (n) produces 
                // a value in the range of -1 to 1, with the following properties:
                //      == 1 is a parallel vector
                //       > 0 is an acute (<90) degress
                //      == 0 is a right angle == 90 degresst aw
                //       < 0 is an obtuse (>90 degree) angle
                // This is ideal; we only care about positive values, as they showcase the 
                // situations where the polygon is oriented towards the light, reflecting the 
                // most light when the polygon normal and light normal are parallel (dot product is 
                // 1.0) and reflection decreases as the normals tilt away from each other.
                // We can then simply "scale" the polygon's colour by this value of the dot product 
                // to have the polygon's colour/intensity affected by the light.
                let mut d = n.dot_product(&l);
                if d < 0.0 {
                    d = 0.0;
                }

                let poly_colour = &self.colours[p];
                self.lit_colours[p][v] = Colour {
                    r: (poly_colour.r as f64 * d) as u8,
                    g: (poly_colour.g as f64 * d) as u8,
                    b: (poly_colour.b as f64 * d) as u8,
                };
                
            }
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
            let mut projected = Point3D{
                x: ((point.x * aspect_ratio * fov_rad) / point.z * (win_width as f64 / 2.0) + (win_width as f64 / 2.0)),
                y: ((point.y * fov_rad) / point.z * (win_height as f64 / 2.0) + (win_height as f64 / 2.0)),
                z: point.z,
                w: point.w,
            };
            self.projected[i] = projected;
        }

        // TODO embed the normal into the array of points so that it's rotated with everything else

        // for each polygon, find the center point, and project the normal there, and calculate the 
        // start and end of the normal line in screen space
        for p in 0..self.get_polygon_count() {
            let polygon = self.get_polygon(p);
            let first_vertex = &self.transformed[polygon[0]];

            let mut min_x = first_vertex.x as f64;
            let mut max_x = first_vertex.x as f64;
            let mut min_y = first_vertex.y as f64;
            let mut max_y = first_vertex.y as f64;
            let mut min_z = first_vertex.z as f64;
            let mut max_z = first_vertex.z as f64;

            for &vi in polygon.iter().skip(1) {
                min_x = min_x.min(self.transformed[vi].x);
                max_x = max_x.max(self.transformed[vi].x);
                min_y = min_y.min(self.transformed[vi].y);
                max_y = max_y.max(self.transformed[vi].y);
                min_z = min_z.min(self.transformed[vi].z);
                max_z = max_z.max(self.transformed[vi].z);
            }
            let center = Point3D{
                x: (min_x + max_x) / 2.0,
                y: (min_y + max_y) / 2.0,
                z: (min_z + max_z) / 2.0,
                w: 1.0,
            };
            let normal_end = Point3D{
                x: center.x + self.normals[p].x,
                y: center.y + self.normals[p].y,
                z: center.z + self.normals[p].z,
                w: 1.0,
            };

            let mut projected_start = Point2D{
                x: ((center.x * aspect_ratio * fov_rad) / center.z * (win_width as f64 / 2.0) + (win_width as f64 / 2.0)) as u32,
                y: ((center.y * fov_rad) / center.z * (win_height as f64 / 2.0) + (win_height as f64 / 2.0)) as u32,
            };
            let mut projected_end = Point2D{
                x: ((normal_end.x * aspect_ratio * fov_rad) / normal_end.z * (win_width as f64 / 2.0) + (win_width as f64 / 2.0)) as u32,
                y: ((normal_end.y * fov_rad) / normal_end.z * (win_height as f64 / 2.0) + (win_height as f64 / 2.0)) as u32,
            };
            self.projected_normals[p] = (projected_start, projected_end);
        }
    }

    pub fn render(&self, screen: &mut crate::gfx::Screen, view_normal: &Vector) {
        let points = self.get_projected();
        for p in 0..self.get_polygon_count() {
            let polygon = self.get_polygon(p);
            
            // is this polygon visisble?
            // If the dot product is >= 0, then polygon is >= 90 degrees to view normal and thus not visible
            let normal = &self.get_normals()[p];
            if normal.dot_product(&view_normal) >= 0.0 {
                continue;
            }

            
             // pull together the relevant parts to draw the polygon
             // NOTE: this copies the points - could be optimized? (4xf64 = 32 bytes per vertex)
            let colours = &self.lit_colours[p];
            let polygon_points: Vec<(Point3D, Colour)> = (0..polygon.len())
                .map(|pi| ( points[ polygon[pi]], colours[pi] ) )
                .collect();

            screen.polygon(&polygon_points);
            screen.line(self.projected_normals[p].0.x as usize, 
                        self.projected_normals[p].0.y as usize, 
                        self.projected_normals[p].1.x as usize, 
                        self.projected_normals[p].1.y as usize,
                        Colour::new(255, 255, 255));
        }
    }
}