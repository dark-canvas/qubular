use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Point2D {
    pub x: u32,
    pub y: u32,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {} {} ]", self.x, self.y)
    }
}

impl Point2D {
    pub fn new(x: u32, y: u32) -> Self {
        Point2D{
            x:x, 
            y:y,
        }
    }
}