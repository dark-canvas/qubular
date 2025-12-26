/*
 * Some simple graphics routines
 *
 * TODO: move to a separate/sharable repo
 */

pub struct Screen<'a> {
    pub buffer: &'a mut [u8],
    pub width: usize,
    pub height: usize,
    pub bytes_per_pixel: usize,
    pub bytes_per_line: usize,
}

impl<'a> Screen<'a> {
    // TODO: these functions need to accept a colour
    pub fn putpixel(&mut self, x: usize, y: usize) {
        let offset = y * self.bytes_per_line + (x*self.bytes_per_pixel);
        if x < self.width && y < self.height {
            self.buffer[offset] = 0xff;
            self.buffer[offset+1] = 0xff;
            self.buffer[offset+2] = 0xff;
        }
    }

    // Rust translation of the Bresenham line agorithm
    // http://neuraldk.org/document.php?djgppGraphics
    pub fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        // locally convert to signed (and mutable) so that we can add the (also signed) direction to them
        let mut x1: i32 = x1 as i32;
        let mut y1: i32 = y1 as i32;
        let mut x2: i32 = x2 as i32;
        let mut y2: i32 = y2 as i32;

        if(y1 > y2) {
            y1 ^= y2; // swap y1 and y2
            y2 ^= y1;
            y1 ^= y2;
            x1 ^= x2; // swap x1 and x2
            x2 ^= x1;
            x1 ^= x2;
        }
        let mut delta_x = x2 - x1;  // will determine L->R or R->L
        let mut delta_y = y2 - y1;  // has to be positive because line goes T->B
        let direction = match (delta_x > 0) { 
            true => 1i32,           // delta_x is positive: we're going left to right
            false => {              // delta_x is negative: we're going from right to left
                delta_x = -delta_x; // we need the absolute length of this axis later on
                -1i32
            }
        };
    
        match delta_x > delta_y { // what is our main axis
            true => { // major axis is the x
                let double_delta_y = delta_y + delta_y;
                let diff_double_deltas = double_delta_y - (delta_x + delta_x);
                let mut error = double_delta_y - delta_x;

                // plot our first pixel
                self.putpixel(x1 as usize, y1 as usize);
                delta_x -= 1;

                // loop for the length of the major axis 
                while delta_x > 0 {
                    if(error >= 0) { // if the error is greater than or equal to zero:
                        y1 += 1; // increase the minor axis (y)
                        error += diff_double_deltas;
                    } else {
                        error += double_delta_y;
                    }
                    x1 += direction; // increase the major axis to next pixel
                    self.putpixel(x1 as usize, y1 as usize); // plot our pixel
                    delta_x -= 1;
                }
            }
            false => { // major axis is the y
                let double_delta_x = delta_x + delta_x;
                let diff_double_deltas = double_delta_x - (delta_y + delta_y);
                let mut error = double_delta_x - delta_y;
                
                // plot our first pixel
                self.putpixel( x1 as usize, y1 as usize); 
                delta_y -= 1;
                
                // loop for the length of the major axis
                while delta_y > 0 {
                    if(error >= 0) { // if the error is greater than or equal to zero:
                        x1 += direction; // increase the minor axis (x)
                        error += diff_double_deltas;
                    } else  {
                        error += double_delta_x;
                    }
                    y1 += 1; // increase major axis to next pixel
                    self.putpixel(x1 as usize, y1 as usize); // plot our pixel
                    delta_y -= 1;
                }
            }
        }
    }
}