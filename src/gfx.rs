use crate::zbuffer::ZBuffer;
use crate::colour::Colour;
use crate::point3d::Point3D;


#[derive(Debug, Copy, Clone)]
struct SpanNode {
    x: usize,
    c: Colour,
    z: f64,
}

#[derive(Debug, Copy, Clone)]
struct Span {
    start: SpanNode,
    end: SpanNode,
}

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
    zbuf: ZBuffer,
    spans: Vec<Option<Span>>,
}

impl<'a> Screen<'a> {
    pub fn new(buffer: &'a mut [u8], width: usize, height: usize, bytes_per_pixel: usize, bytes_per_line: usize) -> Screen<'a> {
        Screen {
            buffer: buffer,
            width: width,
            height: height,
            bytes_per_pixel: bytes_per_pixel,
            bytes_per_line: bytes_per_line,
            zbuf: ZBuffer::new(width, height),
            spans: vec![None; height],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
        self.zbuf.clear();
        for span in self.spans.iter_mut() {
            *span = None;
        }
    }

    // TODO: these functions need to accept a colour
    pub fn putpixel(&mut self, x: usize, y: usize, colour: Colour) {
        let offset = y * self.bytes_per_line + (x*self.bytes_per_pixel);
        if x < self.width && y < self.height {
            self.buffer[offset] = colour.r;
            self.buffer[offset+1] = colour.g;
            self.buffer[offset+2] = colour.b;
        }
    }

    // Rust translation of the Bresenham line agorithm
    // http://neuraldk.org/document.php?djgppGraphics
    pub fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, colour: Colour) {
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
                self.putpixel(x1 as usize, y1 as usize, colour);
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
                    self.putpixel(x1 as usize, y1 as usize, colour); // plot our pixel
                    delta_x -= 1;
                }
            }
            false => { // major axis is the y
                let double_delta_x = delta_x + delta_x;
                let diff_double_deltas = double_delta_x - (delta_y + delta_y);
                let mut error = double_delta_x - delta_y;
                
                // plot our first pixel
                self.putpixel( x1 as usize, y1 as usize, colour); 
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
                    self.putpixel(x1 as usize, y1 as usize, colour); // plot our pixel
                    delta_y -= 1;
                }
            }
        }
    }

    fn record_span(&mut self, x: i32, y: i32, z: f64, colour: &Colour) {
        let mut x = x;
        let mut y = y;

        // is this even possible?
        if y < 0 {
            return;
        }
        if y >= self.height as i32 {
            return;
        }
        if x < 0 {
            x = 0;
        }
        if x > (self.width as i32 - 1) {
            x = (self.width as i32 - 1);
        }
        let x = x as usize;
        let y = y as usize;
        match &mut self.spans[y] {
            Some(span) => {
                if x < span.start.x {
                    span.start.x = x;
                    span.start.c = *colour;
                    span.start.z = z;
                    return;
                }
                if x > span.end.x {
                    span.end.x = x;
                    span.end.c = *colour;
                    span.end.z = z;
                }
            }
            None => {
                // create a new span
                self.spans[y] = Some( 
                    Span {
                        start: SpanNode {
                            x: x,
                            c: *colour,
                            z: z,
                        },
                        // not used yet...
                        end: SpanNode {
                            x: x,
                            c: *colour,
                            z: z,
                        },
                    }
                );
            },
        }
    }

    fn draw_spans(&mut self) {
        for y in 0..self.height {
            match &self.spans[y] {
                Some(span) => {
                    let span_width = span.end.x - span.start.x;
                    let zinc = (span.end.z - span.start.z) / (span_width as f64);
                    let mut z = span.start.z;
                    let c = span.start.c;
                    // need to interpolate z and colour across the span
                    for x in span.start.x..=span.end.x {
                        // TODO: fetch pointer into zbuf and iterate that rather than 
                        // recalc y*width+x each time
                        if self.zbuf.set_depth(x, y, z) {
                            self.putpixel(x, y, c);
                        }
                        z += zinc;
                    }
                    self.spans[y] = None;
                }
                None => {
                    // no span to draw
                }
            }
        }
    }

    // line drawing is done with integers, with depth interpolated as f64
    // TODO: the integer line drawing may be causing visual artifacts
    fn line_to_spans(&mut self, p1: &Point3D, c1: &Colour, p2: &Point3D, c2: &Colour) {

        if p1.y > p2.y {
            // swap points so that p1 is always the topmost
            return self.line_to_spans(p2, c2, p1, c1);
        }

        // locally convert to signed (and mutable) so that we can add the (also signed) direction to them
        let mut x1: i32 = p1.x as i32;
        let mut y1: i32 = p1.y as i32;
        let mut x2: i32 = p2.x as i32;
        let mut y2: i32 = p2.y as i32;

        /*
        if(y1 > y2) {
            y1 ^= y2; // swap y1 and y2
            y2 ^= y1;
            y1 ^= y2;
            x1 ^= x2; // swap x1 and x2
            x2 ^= x1;
            x1 ^= x2;
        }
        */
        let mut delta_x = x2 - x1;  // will determine L->R or R->L
        let mut delta_y = y2 - y1;  // has to be positive because line goes T->B
        let direction = match (delta_x > 0) {
            true => 1i32,           // delta_x is positive: we're going left to right
            false => {              // delta_x is negative: we're going from right to left
                delta_x = -delta_x; // we need the absolute length of this axis later on
                -1i32
            }
        };

        if delta_y == 0 {
            // horizontal line
            if x1 < x2 {
                if x1 < 0 {
                    x1 = 0;
                }
                if x2 > (self.width as i32 - 1) {
                    x2 = (self.width as i32 - 1);
                }
                self.spans[y1 as usize] = Some( Span {
                    start: SpanNode {
                        x: x1 as usize,
                        c: *c1,
                        z: p1.z,
                    },
                    end: SpanNode {
                        x: x2 as usize,
                        c: *c2,
                        z: p2.z,
                    },
                });
                return;
            } else {
                if x2 < 0 {
                    x2 = 0;
                }
                if x1 > (self.width as i32 - 1) {
                    x1 = (self.width as i32 - 1);
                }
                self.spans[y1 as usize] = Some( Span {
                    start: SpanNode {
                        x: x2 as usize,
                        c: *c2,
                        z: p2.z,
                    },
                    end: SpanNode {
                        x: x1 as usize,
                        c: *c1,
                        z: p1.z,
                    },
                });
            }
            return;
        }

        // we're filling in spans, so no matter what the line orientation is, 
        // we need to iterate over the y axis.
    
        match delta_x > delta_y { // what is our main axis
            true => { // major axis is the x
                let double_delta_y = delta_y + delta_y;
                let diff_double_deltas = double_delta_y - (delta_x + delta_x);
                let mut error = double_delta_y - delta_x;
                let zinc = (p2.z - p1.z) / (delta_x as f64);
                let mut z = p1.z;

                // plot our first pixel
                //self.putpixel(x1 as usize, y1 as usize);
                self.record_span(x1, y1, z, c1);
                delta_x -= 1;

                // loop for the length of the major axis 
                while delta_x > 0 {
                    if(error >= 0) { // if the error is greater than or equal to zero:
                        y1 += 1; // increase the minor axis (y)
                        error += diff_double_deltas;
                        self.record_span(x1, y1, z, c1);
                    } else {
                        error += double_delta_y;
                    }
                    x1 += direction; // increase the major axis to next pixel
                    //self.putpixel(x1 as usize, y1 as usize); // plot our pixel
                    
                    delta_x -= 1;
                    z += zinc;
                }
            }
            false => { // major axis is the y
                let double_delta_x = delta_x + delta_x;
                let diff_double_deltas = double_delta_x - (delta_y + delta_y);
                let mut error = double_delta_x - delta_y;
                let zinc = (p2.z - p1.z) / (delta_y as f64);
                let mut z = p1.z;
                
                // plot our first pixel
                //self.putpixel( x1 as usize, y1 as usize); 
                self.record_span(x1, y1, z, c1);
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
                    //self.putpixel(x1 as usize, y1 as usize); // plot our pixel
                    self.record_span(x1, y1, z, c1);
                    delta_y -= 1;
                    z += zinc;
                }
            }
        }
    }

    // vertices are expected to be projected (in screen space) but with z/depth preserved.
    // TODO: use colour
    pub fn polygon(&mut self, vertices: &Vec<(Point3D, Colour)>) {
        let mut prev_point = &vertices[0];
        for point in 1..vertices.len() {
            /*
            self.line(
                prev_point.0.x as usize, 
                prev_point.0.y as usize,
                vertices[point].0.x as usize,
                vertices[point].0.y as usize);
            */
            self.line_to_spans(
                &prev_point.0, 
                &prev_point.1,
                &vertices[point].0,
                &vertices[point].1);

                prev_point = &vertices[point];
        }
        // close the polygon by drawing a line from last to first
        /*
        self.line(
            prev_point.0.x as usize, 
            prev_point.0.y as usize,
            vertices[0].0.x as usize, 
            vertices[0].0.y as usize);
            */
        self.line_to_spans(
            &prev_point.0, 
            &prev_point.1,
            &vertices[0].0,
            &vertices[0].1);

        // draw the spans...
        self.draw_spans();
    }
}