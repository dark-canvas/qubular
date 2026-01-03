use crate::point3d::Point3D;

pub struct LightSource {
    position: Point3D
}

impl LightSource {
    pub fn new(position: &Point3D) -> Self {
        LightSource {
            position: position.clone(),
        }
    }

    pub fn get_position(&self) -> &Point3D {
        &self.position
    }
}