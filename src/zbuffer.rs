pub struct ZBuffer {
    width: usize,
    height: usize,
    buffer: Vec<f32>,
}

impl ZBuffer {
    pub fn new(width: usize, height: usize) -> ZBuffer {
        ZBuffer {
            width,
            height,
            buffer: vec![f32::INFINITY; width * height],
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = f32::INFINITY;
        }
    }

    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        self.buffer[y * self.width + x]
    }

    pub fn set_depth(&mut self, x: usize, y: usize, depth: f32) -> bool {
        if depth >= self.buffer[y * self.width + x] {
            return false;
        }
        self.buffer[y * self.width + x] = depth;
        true
    }
}