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
use std::thread;
use std::time::Duration;

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
            let rotate_x = Matrix::rotate_x(angle as f64 / 2.0, &trig);
            let translate = Matrix::translate(0.0, 0.0, 15.0);
            let matrix = rotate_y * rotate_x * translate;
            cube.apply(&matrix);

            // in this simple setup, the camera is hardcoded to look down the Z axis
            let view_normal = Vector::new(0.0, 0.0, -1.0);

            cube.project(WIN_WIDTH, WIN_HEIGHT, FOV);
            
            let points = cube.get_projected();
            for p in 0..cube.get_polygon_count() {
                let polygon = cube.get_polygon(p);
                
                // is this polygon visisble?
                // If the dot product is >= 0, then polygon is >= 90 degrees to view normal and thus not visible
                let normal = &cube.get_normals()[p];
                if normal.dot_product(&view_normal) >= 0.0 {
                    continue;
                }

                // TODO: project and draw the normal
                /*
                let projected_normal = &cube.get_projected_normals()[p];
                if normal.z != 0.0 {
                    //let center_x = (points[polygon[0]].x + points[polygon[2]].x) / 2;
                    //let center_y = (points[polygon[0]].y + points[polygon[2]].y) / 2;
                    let start_x = points[polygon[1]].x;
                    let start_y = points[polygon[1]].y;
                    println!("normal: {:?} projected: {:?} start: {},{}", normal, projected_normal, start_x, start_y);
                    screen.line(
                        start_x as usize, 
                        start_y as usize,
                        (start_x + projected_normal.x) as usize, 
                        (start_y + projected_normal.y) as usize);
                }
                */

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

        thread::sleep(Duration::from_millis(100));

        frames += 1;
        angle += 1;
        if angle >= 360 {
            angle = 0;
        }
        
    }


    // TODO: 
    //   add frame rate calculation and display
    //   display surface (and vertice) normals
    //   rotate normals with points? (rather than recalculating each frame)
    //   add camera-based view system
    //   add shading/texture mapping
    //   add z-buffer
}
