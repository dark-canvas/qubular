extern crate sdl2;
extern crate trigr;

mod gfx;
mod simple_object;

use gfx::Screen;
use simple_object::SimpleObject;
use trigr::SineCosineTable;

use std::ops;
use std::fmt;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::PixelFormatEnum;

use std::time::SystemTime;

static WIN_WIDTH: usize = 800;
static WIN_HEIGHT: usize = 600;
static FOV: usize = 60;

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

                screen.putpixel(screen_x as usize, screen_y as usize);
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
