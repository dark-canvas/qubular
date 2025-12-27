extern crate sdl2;
extern crate trigr;

mod gfx;
mod matrix;
mod point2d;
mod point3d;
mod simple_object;
mod vector;

use gfx::Screen;
use matrix::Matrix;
use point2d::Point2D;
use point3d::Point3D;
use simple_object::SimpleObject;
use trigr::SineCosineTable;
use vector::Vector;

use std::ops;
use std::fmt;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::PixelFormatEnum;

use std::time::SystemTime;

static WIN_WIDTH: usize = 800;
static WIN_HEIGHT: usize = 600;
static FOV: usize = 60;

fn main() {
    let mut cube = SimpleObject::cube(5);
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

            let rotate_y = Matrix::rotate_y(angle as f64, &trig);
            //let rotate_z = Matrix::rotate_z(angle as f64, &trig);
            let translate = Matrix::translate(0.0, 0.0, 10.0);
            let matrix = rotate_y * translate;
            cube.apply(&matrix);
            
            cube.project(WIN_WIDTH, WIN_HEIGHT, FOV);
            let points = cube.get_projected();
            for polygon in cube.get_polygons() {
                let mut last_point = 0usize;
                for point in 1..polygon.len() {
                    screen.line(
                        points[polygon[last_point]].x as usize, 
                        points[polygon[last_point]].y as usize,
                        points[polygon[point]].x as usize, 
                        points[polygon[point]].y as usize);
                    last_point = point;
                }
                // close the polygon by drawing a line from last to first
                screen.line(
                    points[polygon[last_point]].x as usize, 
                    points[polygon[last_point]].y as usize,
                    points[polygon[0usize]].x as usize, 
                    points[polygon[0usize]].y as usize);
            }
            /*
            for point in cube.get_projected() {
                screen.putpixel(point.x as usize, point.y as usize);
            }
            */

            /*
            for mut point in cube.get_transformed() {

                let screen_x = (dx * point.x) / point.z + (WIN_WIDTH/2) as f64;
                let screen_y = (dy * point.y) / point.z + (WIN_HEIGHT/2) as f64;

                screen.putpixel(screen_x as usize, screen_y as usize);
            }*/
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
