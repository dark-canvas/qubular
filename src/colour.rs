use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour { r: r, g: g, b: b }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "rgb: [ {:x} {:x} {:x} ]", self.r, self.g, self.b);
    }
}