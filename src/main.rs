extern crate sdl2;
extern crate trigr;

mod simple_object;

use trigr::SineCosineTable;
use simple_object::SimpleObject;
use std::ops;
use std::fmt;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::PixelFormatEnum;

use std::time::SystemTime;

static WIN_WIDTH: usize = 800;
static WIN_HEIGHT: usize = 600;
static FOV: usize = 60;

struct Screen<'a> {
    buffer: &'a mut [u8],
    width: usize,
    height: usize,
    bytes_per_pixel: usize,
    bytes_per_line: usize,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {} {} {} {} ]", self.x, self.y, self.z, self.w)
    }
}

impl ops::Mul<Matrix> for Point {
    type Output = Point;

    // TODO: this can be optimized in the context of 3d math
    fn mul(self, mat: Matrix) -> Self {
        Point {
            x: self.x * mat.data[0][0] + self.y * mat.data[1][0] + self.z * mat.data[2][0] + self.w * mat.data[3][0],
            y: self.x * mat.data[0][1] + self.y * mat.data[1][1] + self.z * mat.data[2][1] + self.w * mat.data[3][1],
            z: self.x * mat.data[0][2] + self.y * mat.data[1][2] + self.z * mat.data[2][2] + self.w * mat.data[3][2],
            w: self.x * mat.data[0][3] + self.y * mat.data[1][3] + self.z * mat.data[2][3] + self.w * mat.data[3][3],
        }
    }
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point{
            x:x, 
            y:y,
            z:z,
            w:1.0,
        }
    }
}

struct Matrix {
    data:  [[f64; 4]; 4],
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..4 {
            write!(f, "[ {} {} {} {} ]\n", self.data[i][0], self.data[i][1], self.data[i][2], self.data[i][3] )?;
        }
        Ok(())
    }
}


impl Matrix {
    fn identity() -> Matrix {
        Matrix {
            data: [ [ 1.0, 0.0, 0.0, 0.0, ],
                    [ 0.0, 1.0, 0.0, 0.0, ],
                    [ 0.0, 0.0, 1.0, 0.0, ],
                    [ 0.0, 0.0, 0.0, 1.0, ] ]
        }
    }

    fn translate(x: f64, y: f64, z: f64) -> Matrix {
        Matrix {
            data: [ [ 1.0, 0.0, 0.0, 0.0, ],
                    [ 0.0, 1.0, 0.0, 0.0, ],
                    [ 0.0, 0.0, 1.0, 0.0, ],
                    [   x,   y,   z, 1.0, ] ]
        }
    }

    fn scale(x: f64, y: f64, z: f64) -> Matrix {
        Matrix {
            data: [ [   x, 0.0, 0.0, 0.0, ],
                    [ 0.0,   y, 0.0, 0.0, ],
                    [ 0.0, 0.0,   z, 0.0, ],
                    [ 0.0, 0.0, 0.0, 1.0, ] ]
        }
    }

    fn rotate_x(angle: f64, lookup: &SineCosineTable) -> Matrix {
        let sin_a = lookup.sine(angle);
        let cos_a = lookup.cosine(angle);
        Matrix {
            data: [ [  1.0,    0.0,    0.0,    0.0, ],
                    [  0.0,  cos_a,  sin_a,    0.0, ],
                    [  0.0, -sin_a,  cos_a,    0.0, ],
                    [  0.0,    0.0,    0.0,    1.0, ] ]
        }
    }

    fn rotate_y(angle: f64, lookup: &SineCosineTable) -> Matrix {
        let sin_a = lookup.sine(angle);
        let cos_a = lookup.cosine(angle);
        Matrix {
            data: [ [ cos_a,    0.0, -sin_a,    0.0, ],
                    [   0.0,    1.0,    0.0,    0.0, ],
                    [ sin_a,    0.0,  cos_a,    0.0, ],
                    [   0.0,    0.0,    0.0,    1.0, ] ]
        }
    }

    fn rotate_z(angle: f64, lookup: &SineCosineTable) -> Matrix {
        let sin_a = lookup.sine(angle);
        let cos_a = lookup.cosine(angle);
        Matrix {
            data: [ [  cos_a,  sin_a,    0.0,    0.0, ],
                    [ -sin_a,  cos_a,    0.0,    0.0, ],
                    [    0.0,    0.0,    1.0,    0.0, ],
                    [    0.0,    0.0,    0.0,    1.0, ] ]
        }
    }

    // TODO: 
    // write an inverse function?
    // https://stackoverflow.com/questions/1148309/inverting-a-4x4-matrix
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    // TODO: this can be optimized in the context of 3d math
    fn mul(self, rhs: Self) -> Self {
        let mut result = Matrix::identity();
        for r in 0..4 {
            for c in 0..4 {
                result.data[r][c] = 0.0;
                for i in 0..4 {
                    result.data[r][c] += self.data[r][i] + rhs.data[i][c];
                }
            }
        }
        result
    }
}

// TODO: these functions need to accept a colour
pub fn putpixel(screen: &mut Screen, x: usize, y: usize) {
    let offset = y * screen.bytes_per_line + (x*screen.bytes_per_pixel);
    if x < screen.width && y < screen.height {
        screen.buffer[offset] = 0xff;
        screen.buffer[offset+1] = 0xff;
        screen.buffer[offset+2] = 0xff;
    }
}

// Rust translation of the Bresenham line agorithm
// http://neuraldk.org/document.php?djgppGraphics
pub fn line(screen: &mut Screen, x1: usize, y1: usize, x2: usize, y2: usize) {
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
            putpixel(screen, x1 as usize, y1 as usize); // plot our first pixel
            //while(delta_x -= 1) { // loop for the length of the major axis
            while delta_x > 0 {
                if(error >= 0) { // if the error is greater than or equal to zero:
                y1 += 1; // increase the minor axis (y)
                error += diff_double_deltas;
                } else {
                    error += double_delta_y;
                }
                x1 += direction; // increase the major axis to next pixel
                putpixel(screen, x1 as usize, y1 as usize); // plot our pixel
                delta_x -= 1;
            }
        }
        false => { // major axis is the y
            let double_delta_x = delta_x + delta_x;
            let diff_double_deltas = double_delta_x - (delta_y + delta_y);
            let mut error = double_delta_x - delta_y;
            putpixel(screen, x1 as usize, y1 as usize); // plot our first pixel
            //while(delta_y -= 1) { // loop for the length of the major axis
            while delta_y > 0 {
                if(error >= 0) { // if the error is greater than or equal to zero:
                x1 += direction; // increase the minor axis (x)
                error += diff_double_deltas;
                } else  {
                    error += double_delta_x;
                }
                y1 += 1; // increase major axis to next pixel
                putpixel(screen, x1 as usize, y1 as usize); // plot our pixel
                delta_y -= 1;
            }
        }
    }
}

fn main() {
    let cube = SimpleObject::cube(5);
    let trig = SineCosineTable::new(360*4);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("qubular", WIN_WIDTH as u32, WIN_HEIGHT as u32)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string()).unwrap();

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIN_WIDTH as u32, WIN_HEIGHT as u32)
        .map_err(|e| e.to_string()).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut frames: u64 = 0;
    let start_time = SystemTime::now();

    let tan_half_fov = trig.tangent( (FOV/2) as f64 );
    let dx = (WIN_WIDTH/2) as f64 / tan_half_fov;
    let dy = (WIN_HEIGHT/2) as f64 / tan_half_fov;

    let mut angle = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | 
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                e => {
                    println!("{:?}", e);
                }
            }
        }
        
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            // clear the buffer to black...
            buffer.fill(0);

            let mut screen = Screen {
                buffer: buffer,
                width: WIN_WIDTH,
                height: WIN_HEIGHT,
                bytes_per_pixel: 3,
                bytes_per_line: pitch,
            };

            for mut point in cube.get_vertices() {

                let matrix = Matrix::rotate_y(angle as f64, &trig);
                let mut rotated_point = point * matrix;

                rotated_point.z += 10.0;
                let screen_x = (dx * rotated_point.x) / rotated_point.z + (WIN_WIDTH/2) as f64;
                let screen_y = (dy * rotated_point.y) / rotated_point.z + (WIN_HEIGHT/2) as f64;

                putpixel(&mut screen, screen_x as usize, screen_y as usize);
            }
        }).unwrap();

        // Copy the whole texture to the canvas...
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        frames += 1;
        angle += 1;
        if angle >= 360 {
            angle = 0;
        }
        
    }


    // TODO: 
    //   move the point and matrix code into separate files
    //   add camera-based view system
    //   add shading/texture mapping
    //   back-face culling
}
