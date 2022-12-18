extern crate sdl2;

use sdl2::{pixels::Color, rect::Point};
use sdl2::event::Event;

use glam::{Vec3, vec3, vec2};

use tobj::Model;

use crate::rasterize::*;

pub const CANVAS_WIDTH: i32 = 800;
pub const CANVAS_HEIGHT: i32 = 600;

pub mod rasterize;


fn load_obj() -> Vec<Model> {
    let (models, _materials) =
        tobj::load_obj(
            "./objs/triangle.obj",
            &tobj::LoadOptions::default()
            )
        .expect("Failed to OBJ load file");

    println!("Number of models = {:?}", models.len());

    models
}

fn main() {
    // load in obj file
    load_obj();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rasterizer", CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let v0 = (vec2(0.0, 300.0), vec3(255.0, 0.0, 0.0));
    let v1 = (vec2(-150.0, 0.0), vec3(0.0, 255.0, 0.0));
    let v2 = (vec2(150.0, 0.0), vec3(0.0, 0.0, 255.0));

    draw_triangle(&mut canvas, v0, v1, v2); 
    // triangle.draw_outline(&mut canvas);

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                _ => {}
            }
        }
    }
}
