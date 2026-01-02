use crate::matrix::Matrix;
use crate::point3d::Point3D;
use crate::vector::Vector;

pub struct Camera {
    position: Point3D,
    trasnslated_position: Point3D,
    look_at: Point3D,
    view_normal: Vector,
    matrix: Matrix,
}

impl Camera {
    pub fn new(position: Point3D, look_at: Point3D) -> Self {
        let mut result = Camera {
            position,
            trasnslated_position: position,
            look_at,
            view_normal: Vector::from_points(&look_at, &position).normalize(),
            matrix: Matrix::identity(),
        };
        result.recalculate();
        result
    }

    pub fn get_matrix(&self) -> &Matrix {
        &self.matrix
    }

    pub fn get_view_normal(&self) -> &Vector {
        &self.view_normal
    }

    pub fn apply(&mut self, matrix: &Matrix) {
        self.trasnslated_position = self.position * matrix;
        self.recalculate();
    }

    fn recalculate(&mut self) {
        let forward = Vector::from_points(&self.look_at, &self.trasnslated_position).normalize();
        let up = Vector::new(0.0, 1.0, 0.0);
        let right = forward.cross_product(&up).normalize();
        let true_up = right.cross_product(&forward).normalize();

        let camera_vector = Vector::new(
            self.trasnslated_position.x, 
            self.trasnslated_position.y, 
            self.trasnslated_position.z);
        let translate_x = Vector::dot_product(&camera_vector, &right);
        let translate_y = Vector::dot_product(&camera_vector, &true_up);
        let translate_z = Vector::dot_product(&camera_vector, &forward);

        self.matrix = Matrix {
            data: [
                [right.x, true_up.x, forward.x, 0.0],
                [right.y, true_up.y, forward.y, 0.0],
                [right.z, true_up.z, forward.z, 0.0],
                [-translate_x, -translate_y, -translate_z, 1.0],
            ],
        };

        // save off the view normal... this is the direction the camera is looking in 
        // world space
        self.view_normal = forward;
    }
}