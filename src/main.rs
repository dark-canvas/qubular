extern crate sdl2;
extern crate trigr;

mod camera;
mod colour;
mod gfx;
mod matrix;
mod point2d;
mod point3d;
mod simple_object;
mod vector;
mod zbuffer;

use colour::Colour;
use gfx::Screen;
use matrix::Matrix;
use point2d::Point2D;
use point3d::Point3D;
use simple_object::SimpleObject;
use trigr::SineCosineTable;
use vector::Vector;
use zbuffer::ZBuffer;

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
    let mut cube2 = SimpleObject::cube(5);

    let mut camera = camera::Camera::new(
        Point3D::new(0.0, 0.0, -30.0), // camera's position in world space
        Point3D::new(0.0, 0.0, 15.0),  // point the camera is looking at in world space
    );

    println!("Cube:\n{}", cube);
    println!("Camera matrix:\n{}", camera.get_matrix());
    println!("View normal:\n{}", camera.get_view_normal());

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

    let mut y_angle = 0.0;
    let mut x_angle = 0.0;
    let mut z_angle = 0.0;

    let mut paused = false;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | 
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    paused = !paused;
                }
                e => {
                    println!("{:?}", e);
                }
            }
        }
        
        thread::sleep(Duration::from_millis(10));
        if paused {
            continue;
        }
        
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let mut screen = Screen::new(
                buffer,
                WIN_WIDTH,
                WIN_HEIGHT,
                3,
                pitch,
            );

            screen.clear();

            let camera_rotation = Matrix::rotate_y(y_angle, &trig);
            camera.apply(&camera_rotation);

            let rotate_y = Matrix::rotate_y(y_angle, &trig);
            let rotate_x = Matrix::rotate_x(x_angle, &trig);
            let translate = Matrix::translate(2.0, 0.0, 16.0);
            let matrix = rotate_y * rotate_x * translate * camera.get_matrix();
            //let matrix = translate * camera.get_matrix();
            cube.apply(&matrix);

            // rotate y was consumed above... recreated (TODO: borrow above)
            let rotate_y = Matrix::rotate_y(y_angle, &trig);
            let rotate_z = Matrix::rotate_z(z_angle, &trig);
            let translate2 = Matrix::translate(-2.0, 0.0, 16.0);
            let matrix2 = rotate_y * rotate_z * translate2 * camera.get_matrix();
            //let matrix2 = translate2 * camera.get_matrix();
            cube2.apply(&matrix2);

            //let view_normal = camera.get_view_normal();
            // Doesn't matter what the camera view normal is, the view normal is always 0,0,-1 
            // from the camera's point of view because everything is transformed by the camera matrix
            // to be relative to the camera.
            let view_normal = Vector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            };

            cube.project(WIN_WIDTH, WIN_HEIGHT, FOV);
            cube2.project(WIN_WIDTH, WIN_HEIGHT, FOV);

            cube.render(&mut screen, &view_normal);
            cube2.render(&mut screen, &view_normal);
        }).unwrap();

        // Copy the whole texture to the canvas...
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        frames += 1;
        y_angle += 1.0;
        if y_angle >= 360.0 {
            y_angle = 0.0;
        }

        x_angle += 0.5;
        if x_angle >= 360.0 {
            x_angle = 0.0;
        }

        z_angle += 0.25;
        if z_angle >= 360.0 {
            z_angle = 0.0;
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
